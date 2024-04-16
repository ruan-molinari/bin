use std::{ffi::OsString, path::PathBuf};

use clap::{Parser, Subcommand};

/// Simple CLI tool to "install" compiled binaries into a local directory
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pattern: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install a binary in a defined personal folder
    Install {
        /// Path to the binary being installed
        #[arg(short, long, required = true)]
        binary: PathBuf,

        /// Directory in which the binary will be relocated to
        #[arg(short, long, default_value = None)]
        target_dir: Option<PathBuf>,
    },
    /// Lists all binaries in the personal folder
    List,
}

fn main() {
    let cli = Cli::parse();
    let pattern = cli.pattern.expect("could not parse command");

    match pattern {
        Commands::Install { binary, target_dir } => {
            let bin_path = binary.canonicalize().expect("Path to binary does not exist.");
            let _ = std::process::Command::new("chmod")
                .args(["+x", bin_path.to_str().unwrap()])
                .output()
                .expect("Failed to chmod binary.");

            let mut target_path = target_dir
                .map(|relative_path| relative_path.canonicalize().expect("Invalid target dir."))
                .or(get_default_install_dir()).unwrap();
            target_path.push(bin_path.file_name().unwrap());
           
            let _ = std::fs::copy(binary, target_path);
        },
        Commands::List => {
            let binaries = get_installed_binaries();
            if !binaries.is_empty() {
                for bin in binaries {
                    println!("{}", bin.to_str().unwrap());
                }
            }
        }
    }
}

/// Returns a list with the name of all binaries in the default install dir
fn get_installed_binaries() -> Vec<OsString> {
    // TODO: print a more informative error
    let path = get_default_install_dir().expect("unexpected error parsing system variables.");
    let dir = std::fs::read_dir(path).expect("Failed to parse dir.");
    let binaries: Vec<OsString> = dir.map(|entry| {
        entry.expect("unexpected error parsing binary paths")
            .file_name()
    }).collect();
    
    binaries
}

/// Gets default installation directory
fn get_default_install_dir() -> Option<PathBuf> {
    if let Some((_var, path)) = std::env::vars_os()
        .find(|(var, _path)| var.eq("PERSONAL_BIN"))
        .or_else(|| {
            if let Some((var, path)) = std::env::vars_os().find(|(var, _path)| var.eq("HOME")) {
                let mut new_path = PathBuf::from(path.into_string().expect("Failed to parse $PERSONAL_BIN"));
                new_path.push(".bin");
                return Some((var, new_path.into()));
            };
            None
        })
    {
        let path = PathBuf::from(path);
        return Some(path);
    }
    None
}
