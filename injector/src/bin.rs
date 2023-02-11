use std::path::{PathBuf};
use argh::FromArgs;
use eyre::Result;
use sysinfo::{System, SystemExt};

#[derive(FromArgs, PartialEq, Debug)]
/// Inject a library into a remote process.
struct Args {
    /// process id of the target process
    #[argh(option, short = 'p')]
    target_pid: usize,

    /// path to the library to inject
    #[argh(positional)]
    library_path: String,
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let mut library_path = PathBuf::new();
    library_path.push(&args.library_path);
    if !library_path.exists() {
        return Err(eyre::eyre!("Library path does not exist"));
    }

    let pid = args.target_pid as u32;
    let system = System::new_all();
    system.process((pid as usize).into())
        .ok_or_else(|| eyre::eyre!(format!("Process {} not found", pid)))?;

    println!("Target PID: {}", args.target_pid);
    println!("Library path: {}", args.library_path);

    injector::inject(pid, library_path)?;

    Ok(())
}