use std::io::Read;
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

static RUNNING: AtomicBool = AtomicBool::new(true);

pub fn socket_path() -> PathBuf {
    if let Some(runtime) = std::env::var_os("XDG_RUNTIME_DIR") {
        PathBuf::from(runtime).join("srwm.sock")
    } else {
        PathBuf::from("/tmp/srwm.sock")
    }
}

pub fn is_running() -> bool {
    RUNNING.load(Ordering::SeqCst)
}

pub fn set_running(value: bool) {
    RUNNING.store(value, Ordering::SeqCst);
}

fn create_self_pipe() -> Result<(i32, i32), std::io::Error> {
    let mut fds = [0i32, 0i32];
    unsafe {
        if libc::pipe(fds.as_mut_ptr()) != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok((fds[0], fds[1]))
}

pub fn start_ipc_server() {
    let path = socket_path();

    std::thread::spawn(move || {
        if path.exists() {
            let _ = std::fs::remove_file(&path);
        }

        let listener = match UnixListener::bind(&path) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("srwm: failed to bind socket {}: {}", path.display(), e);
                return;
            }
        };

        let (wake_read, wake_write) = match create_self_pipe() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("srwm: failed to create self-pipe: {}", e);
                return;
            }
        };

        let mut poll_fds = [
            libc::pollfd {
                fd: listener.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            },
            libc::pollfd {
                fd: wake_read,
                events: libc::POLLIN,
                revents: 0,
            },
        ];

        loop {
            let timeout_ms = if is_running() { -1 } else { 0 };
            let ret = unsafe {
                libc::poll(
                    poll_fds.as_mut_ptr(),
                    poll_fds.len() as libc::nfds_t,
                    timeout_ms,
                )
            };

            if ret < 0 {
                if std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
                    continue;
                }
                eprintln!("srwm: poll error");
                break;
            }

            if !is_running() {
                break;
            }

            if poll_fds[0].revents & libc::POLLIN != 0 {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        std::thread::spawn(move || {
                            handle_connection(&mut stream);
                        });
                    }
                    Err(e) => {
                        eprintln!("srwm: socket accept error: {}", e);
                    }
                }
            }

            if poll_fds[1].revents & libc::POLLIN != 0 {
                break;
            }
        }

        unsafe {
            let _ = std::fs::remove_file(&path);
            libc::close(wake_read);
            libc::close(wake_write);
        }
    });
}

fn handle_connection(stream: &mut impl Read) {
    let mut buf = [0u8; 256];
    let mut command = String::new();

    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                command.push_str(&String::from_utf8_lossy(&buf[..n]));
                if command.contains('\n') {
                    break;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            Err(e) => {
                eprintln!("srwm: IPC read error: {}", e);
                return;
            }
        }
    }

    let cmd = command.trim();
    match cmd {
        "shutdown" => {
            eprintln!("srwm: IPC received shutdown");
            SHOULD_QUIT.store(true, Ordering::SeqCst);
            unsafe {
                crate::ffi::srwm_quit();
            }
        }
        "restart" => {
            eprintln!("srwm: IPC received restart");
            unsafe {
                crate::ffi::srwm_request_restart();
            }
        }
        _ => {
            eprintln!("srwm: unknown IPC command: {}", cmd);
        }
    }
}

static SHOULD_QUIT: AtomicBool = AtomicBool::new(false);

pub fn send_command(command: &str) -> Result<(), String> {
    let path = socket_path();

    use std::io::Write;
    use std::os::unix::net::UnixStream;

    let mut stream = UnixStream::connect(&path)
        .map_err(|e| format!("failed to connect to srwm: {} (is it running?)", e))?;

    stream
        .write_all(command.as_bytes())
        .map_err(|e| format!("failed to send: {}", e))?;
    stream
        .flush()
        .map_err(|e| format!("failed to flush: {}", e))?;

    Ok(())
}
