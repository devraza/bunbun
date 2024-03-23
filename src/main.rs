use colored::Colorize;
use sysinfo::System;
use whoami::*;
use std::env;

fn main() {
    let bottom = format!("c({})({})", "\"".red(), "\"".red()).to_string();
    let ascii = vec!["(\\ /)", "( . .)", &bottom];

    let hostname = fallible::hostname().unwrap_or(String::from("N/A"));
    let user = env!("USER");

    let arch = arch();

    let combined = format!("{}@{}", user, hostname);

    let kernel = System::kernel_version().unwrap_or(String::from("N/A"));
    let pretty = distro();
    let wm: &str;

    if cfg!(windows) {
        wm = "Aero";
    } else if cfg!(unix) {
        wm = env!("XDG_CURRENT_DESKTOP");
    } else {
        wm = "N/A";
    }

    println!("{:>34}", combined.italic());
    println!("{:>19} {}", "Arch".cyan().bold(), arch);
    println!("{:>9} {:>11} {}", ascii[0], "Kernel".red().bold(), kernel);
    println!("{:>10} {:>6} {}", ascii[1], "WM".green().bold(), wm);
    println!("{:>29} {:>5} {}", ascii[2], "OS".blue().bold(), pretty);
}
