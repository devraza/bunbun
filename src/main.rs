use colored::Colorize;
use sysinfo::System;
use whoami::*;
use std::env;

fn main() {
    let bottom = format!("c({})({})", "\"".red(), "\"".red()).to_string();
    let ascii = vec!["(\\ /)", "( . .)", &bottom];

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

    println!("  {}      {} {}", ascii[0], "Kernel".red(), kernel);
    println!("  {}     {} {}", ascii[1], "WM".green(), wm);
    println!("  {}    {} {}", ascii[2], "OS".blue(), pretty);
}
