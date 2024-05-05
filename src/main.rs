use clap::Parser;
use colored::Colorize;
use std::{env::var, path::PathBuf};
use sysinfo::System;
use whoami::*;

/// A simple and adorable sysinfo utility written in Rust.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Show ASCII art only
    #[arg(short, long)]
    ascii_only: bool,

    /// Show CPU architecture
    #[arg(short = 'x', long)]
    arch: bool,

    /// Show the kernel version
    #[arg(short = 'k', long)]
    kernel: bool,

    /// Hide terminal colours
    #[arg(short = 'z', long, default_value_t = false)]
    hide_colours: bool,
}

// Display the CPU architecture
fn cpu_arch(args: &Args) {
    if args.arch {
        let arch = arch();
        println!("{:>17} {}", "Arch".cyan().bold(), arch);
    }
}

// Display the kernel version
fn display_kernel(args: &Args) {
    if args.kernel {
        let kernel = System::kernel_version().unwrap_or(String::from("N/A"));
        println!("{:>19} {}", "Kernel".yellow().bold(), kernel);
    }
}

fn main() {
    let args = Args::parse();

    let bottom = format!("c({})({})", "\"".red(), "\"".red()).to_string();
    let ascii = ["(\\ /)", "( . .)", &bottom];

    let hostname = fallible::hostname().unwrap_or(String::from("N/A"));
    let user = username();

    let combined = format!("{}@{}", user.italic(), hostname.italic());

    let pretty = distro();

    let wm: String;

    if cfg!(windows) {
        wm = "Aero".to_string();
    } else if cfg!(unix) {
        let xdg_current_desktop = var("XDG_CURRENT_DESKTOP");
        let desktop = desktop_env().to_string();
        if desktop != "Unknown: Unknown" {
            wm = desktop;
        } else if xdg_current_desktop.is_ok() {
            wm = xdg_current_desktop.unwrap();
        } else {
            wm = "N/A".to_string();
        }
    } else {
        wm = "N/A".to_string();
    }

    let shell_path = PathBuf::from(var("SHELL").unwrap_or(String::from("N/A")));
    let shell = shell_path
        .file_name()
        .expect("Could not get $SHELL path")
        .to_str()
        .unwrap();

    println!();
    if !args.ascii_only {
        println!("{: <13}{}", "", combined);
        cpu_arch(&args);
        display_kernel(&args);
        println!("{:>8} {:>6} {}", ascii[0], "OS".red().bold(), pretty);
        println!("{:>9} {:>8} {}", ascii[1], "Shell".green().bold(), shell);
        println!("{:>28} {:>4} {}", ascii[2], "WM".blue().bold(), wm);
    } else {
        for i in ascii {
            println!("  {}", i);
        }
    }

    if !args.hide_colours && !args.ascii_only {
        println!();
        let colors = [
            "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
        ];
        let mut color_string: String = "    ".to_owned();
        for color in colors {
            color_string.push_str(&format!("{:>3}", "●".color(color)));
        }
        println!("{}", color_string);
    }

    println!();
}
