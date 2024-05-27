use std::env;
use std::path::Path;
use std::process::{Command, Output};
use std::{thread, time::Duration};

fn kill_process(name: &str) -> Result<Output, std::io::Error> {
    Command::new("killall").arg(name).output()
}

fn open_app(name: &str) -> Result<Output, std::io::Error> {
    let full_app_name = format!("{}.app", name);
    Command::new("open").arg("-a").arg(&full_app_name).output()
}

fn hpatchz_app(hpatchz_path: &str, delta_path: &str, app_name: &str) -> Result<Output, std::io::Error> {
    let path = Path::new(hpatchz_path);
    let app_path = format!("/Applications/{}.app", app_name);

    Command::new(path)
        .arg("-C-all")
        .arg(&app_path)
        .arg(delta_path)
        .arg(&app_path)
        .arg("-f")
        .output()
}

fn print_usage() {
    println!("Usage: mac-updater <app-name> <delta-path> <hpatchz-path>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        print_usage();
        return;
    }

    let app_name = &args[1];
    let delta_path = &args[2];
    let hpatchz_path = &args[3];

    if let Err(e) = kill_process(app_name) {
        eprintln!("Failed to kill process {}: {}", app_name, e);
        return;
    }

    if let Err(e) = hpatchz_app(hpatchz_path, delta_path, app_name) {
        eprintln!("Failed to patch app {}: {}", app_name, e);
        return;
    }

    thread::sleep(Duration::from_secs(1));

    if let Err(e) = open_app(app_name) {
        eprintln!("Failed to open app {}: {}", app_name, e);
        return;
    }
}
