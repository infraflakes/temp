use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static SIGUSR1_RECEIVED: AtomicBool = AtomicBool::new(false);

pub struct Server {
    child: Child,
    display: String,
    stty_state: Option<String>,
}

pub fn start() -> Result<Server, String> {
    let tty_path =
        std::fs::read_link("/proc/self/fd/0").map_err(|e| format!("cannot determine TTY: {e}"))?;
    let tty_str = tty_path.to_string_lossy();
    let tty_num = tty_str
        .strip_prefix("/dev/tty")
        .ok_or_else(|| format!("stdin is not a TTY: {tty_str}"))?;

    let display = format!(":{tty_num}");

    let stty_state = Command::new("stty").arg("-g").output().ok().and_then(|o| {
        if o.status.success() {
            Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
        } else {
            None
        }
    });

    let data_dir = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        format!("{home}/.local/share")
    });
    let sx_data_dir = format!("{data_dir}/sx");
    let _ = std::fs::create_dir_all(&sx_data_dir);

    let xauth_path =
        std::env::var("XAUTHORITY").unwrap_or_else(|_| format!("{sx_data_dir}/xauthority"));

    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&xauth_path)
        .map_err(|e| format!("cannot create xauthority file: {e}"))?;

    unsafe {
        std::env::set_var("XAUTHORITY", &xauth_path);
    }

    let mut cookie = [0u8; 16];
    getrandom::getrandom(&mut cookie).map_err(|e| format!("cannot generate auth cookie: {e}"))?;
    let cookie_hex: String = cookie.iter().map(|b| format!("{b:02x}")).collect();

    let output = Command::new("xauth")
        .args(["add", &display, "MIT-MAGIC-COOKIE-1", &cookie_hex])
        .output()
        .map_err(|e| format!("xauth add failed: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "xauth add failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    SIGUSR1_RECEIVED.store(false, Ordering::SeqCst);
    unsafe {
        libc::signal(
            libc::SIGUSR1,
            sigusr1_handler as *const () as libc::sighandler_t,
        );
    }

    let shell_cmd = format!(
        "trap '' USR1 && exec Xorg {display} vt{tty_num} -keeptty -noreset -auth \"{xauth_path}\""
    );
    let child = Command::new("sh")
        .args(["-c", &shell_cmd])
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .map_err(|e| {
            let _ = Command::new("xauth").args(["remove", &display]).output();
            format!("failed to start Xorg: {e}")
        })?;

    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        if SIGUSR1_RECEIVED.load(Ordering::SeqCst) {
            break;
        }
        if std::time::Instant::now() >= deadline {
            let mut child = child;
            let _ = child.kill();
            let _ = child.wait();
            let _ = Command::new("xauth").args(["remove", &display]).output();
            return Err("timed out waiting for Xorg to start".into());
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    unsafe {
        libc::signal(libc::SIGUSR1, libc::SIG_DFL);
    }

    unsafe {
        std::env::set_var("DISPLAY", &display);
    }
    eprintln!("srwm: X server started on {display} (vt{tty_num})");

    Ok(Server {
        child,
        display,
        stty_state,
    })
}

impl Server {
    pub fn stop(&mut self) {
        unsafe {
            libc::kill(self.child.id() as i32, libc::SIGTERM);
        }
        let _ = self.child.wait();

        let _ = Command::new("xauth")
            .args(["remove", &self.display])
            .output();

        if let Some(ref state) = self.stty_state {
            if Command::new("stty").arg(state).status().is_err() {
                let _ = Command::new("stty").arg("sane").status();
            }
        }

        eprintln!("srwm: X server stopped");
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop();
    }
}

extern "C" fn sigusr1_handler(_sig: libc::c_int) {
    SIGUSR1_RECEIVED.store(true, Ordering::SeqCst);
}
