use std::process::{Child, Command};
use std::sync::Mutex;

static BAR_PROCESS: Mutex<Option<Child>> = Mutex::new(None);

pub fn start(cmd: &str) {
    eprintln!("srwm: starting external bar: {}", cmd);

    stop();

    match Command::new("sh").args(["-c", cmd]).spawn() {
        Ok(child) => {
            eprintln!("srwm: external bar started (pid={})", child.id());
            let mut guard = BAR_PROCESS.lock().unwrap_or_else(|e| e.into_inner());
            *guard = Some(child);
        }
        Err(e) => eprintln!("srwm: failed to start external bar: {}", e),
    }
}

pub fn stop() {
    let mut guard = BAR_PROCESS.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(ref mut child) = *guard {
        eprintln!("srwm: stopping external bar (pid={})", child.id());
        let _ = child.kill();
        let _ = child.wait();
    }
    *guard = None;
}
