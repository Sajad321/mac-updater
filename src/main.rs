use std::env;
use std::path::Path;
use std::process::Command;
use std::{thread, time::Duration};

fn kill_process(name: &str) {
  Command::new("killall")
    .arg(name)
    .output()
    .expect("failed to execute process");
}

fn open_app(name: &str) {
  let full_app_name = format!("{}.app", name);
   Command::new("open")
    .arg("-a")
    .arg(full_app_name)
    .output()
    .expect("failed to execute process");
}

fn hpatchz_app(hpatchz_path: &str, delta_path: &str, app_name: &str) {
  let path = Path::new(hpatchz_path);
  let app_path = format!("/Applications/{}.app", app_name);

   Command::new(path)
    .arg("-C-all")
    .arg(&app_path)
    .arg(delta_path)
    .arg(&app_path)
    .arg("-f")
    .output()
    .expect("failed to execute hpatchz process");
}

fn help() {
  print!("Usage: mac-updater <app-name> <delta-path> <hpatchz-path>");
}

fn main() {

  let args: Vec<String> = env::args().collect();

  match args.len() {
    1..=3 => help(),
    4 => {
      let app_name = &args[1];
      let delta_path = &args[2];
      let hpatchz_path = &args[3];

      kill_process(app_name);
      hpatchz_app(hpatchz_path, delta_path, app_name);
      thread::sleep(Duration::from_secs(1));
      open_app(app_name);
    }
    _ => help(),
  }
}
