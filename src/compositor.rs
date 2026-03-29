use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command};

#[cfg(feature = "embedded-compositor")]
const SRCOM_BIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/srcom"));

#[cfg(not(feature = "embedded-compositor"))]
const SRCOM_BIN: &[u8] = &[];

static mut COMPOSITOR_PROCESS: Option<Child> = None;

fn cache_dir() -> PathBuf {
    dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".local/share")))
        .expect("cannot determine data directory")
        .join("srwm")
}

fn ensure_extracted() -> Option<PathBuf> {
    if SRCOM_BIN.is_empty() {
        // Not embedded — try PATH
        return which::which("srcom").ok();
    }

    let dir = cache_dir();
    std::fs::create_dir_all(&dir).ok()?;
    let path = dir.join("srcom");

    // Write if missing or different size
    let needs_write = match std::fs::metadata(&path) {
        Ok(m) => m.len() != SRCOM_BIN.len() as u64,
        Err(_) => true,
    };

    if needs_write {
        let mut f = std::fs::File::create(&path).ok()?;
        f.write_all(SRCOM_BIN).ok()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).ok()?;
        }
    }

    Some(path)
}

pub fn start(config_path: &str) {
    let Some(bin) = ensure_extracted() else {
        eprintln!("srwm: compositor binary not available");
        return;
    };

    match Command::new(&bin).args(["--config", config_path]).spawn() {
        Ok(child) => unsafe { COMPOSITOR_PROCESS = Some(child) },
        Err(e) => eprintln!("srwm: failed to start compositor: {}", e),
    }
}

pub fn stop() {
    unsafe {
        if let Some(ref mut child) = COMPOSITOR_PROCESS {
            let _ = child.kill();
            let _ = child.wait();
        }
        COMPOSITOR_PROCESS = None;
    }
}
