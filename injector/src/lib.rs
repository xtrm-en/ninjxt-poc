use std::path::{PathBuf};
use eyre::Result;

#[cfg(target_os = "windows")]
extern crate dll_injector;
#[cfg(target_family = "unix")]
extern crate intruducer;

pub fn inject(pid: u32, library_path: PathBuf) -> Result<()> {
    // Unix implementation using `intruducer`
    #[cfg(target_family = "unix")]
    {
        return match intruducer::intruduce(pid, library_path) {
            Err(e) => Err(eyre::eyre!(format!("{:?}", e))),
            _ => Ok(()),
        }
    }

    // Windows implementation using `dll-injector`
    #[cfg(target_family = "windows")]
    {
        use dll_injector::inject_dll_load_library;
        if let Some(library_path) = library_path.to_str() {
            inject_dll_load_library(pid, library_path)?;
        } else {
            return Err(eyre::eyre!("Library path is not valid"));
        }
        return Ok(());
    }

    // Unsupported platform, throw a compilation error
    #[cfg(not(any(target_family = "unix", target_family = "windows")))]
    compile_error!(eyre::eyre!("Unsupported platform"));
}