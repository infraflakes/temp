use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command};

#[cfg(feature = "embedded-compositor")]
const SRCOM_BIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/srcom"));

#[cfg(not(feature = "embedded-compositor"))]
const SRCOM_BIN: &[u8] = &[];

static mut COMPOSITOR_PROCESS: Option<Child> = None;

fn cache_dir() -> PathBuf {
    dirs::home_dir()
        .expect("cannot determine home directory")
        .join(".local/share/srwm")
}

fn ensure_extracted() -> Option<PathBuf> {
    eprintln!(
        "srwm: ensure_extracted called, embedded={}",
        !SRCOM_BIN.is_empty()
    );

    if SRCOM_BIN.is_empty() {
        // Not embedded — try PATH
        return which::which("srcom").ok();
    }

    let dir = cache_dir();
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("srwm: failed to create cache dir {}: {}", dir.display(), e);
        return None;
    }
    let path = dir.join("srcom");
    eprintln!("srwm: cache path = {}", path.display());

    // Write if missing or different size
    let needs_write = match std::fs::metadata(&path) {
        Ok(m) => m.len() != SRCOM_BIN.len() as u64,
        Err(_) => true,
    };

    if needs_write {
        eprintln!(
            "srwm: extracting compositor ({} bytes) to {}",
            SRCOM_BIN.len(),
            path.display()
        );
        let mut f = std::fs::File::create(&path).expect("failed to create srcom file");
        f.write_all(SRCOM_BIN)
            .expect("failed to write srcom binary");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).ok()?;
        }
    }

    Some(path)
}

pub fn start(config_path: &str) {
    eprintln!("srwm: compositor::start called with config={}", config_path);
    let Some(bin) = ensure_extracted() else {
        eprintln!("srwm: compositor binary not available");
        return;
    };

    match Command::new(&bin)
        .args(["--config", config_path])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
    {
        Ok(child) => {
            eprintln!("srwm: compositor started (pid={})", child.id());
            unsafe { COMPOSITOR_PROCESS = Some(child) }
        }
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
