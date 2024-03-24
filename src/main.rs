use colored::Colorize;
use std::env::var;
use sysinfo::System;
use whoami::*;
use clap::Parser;

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

    /// Hide terminal colours 
    #[arg(short = 'z', long, default_value_t = false)]
    hide_colours: bool,
}

fn cpu_arch(args: &Args) {
    if args.arch {
        let arch = arch();
        println!("{:>17} {}", "Arch".cyan().bold(), arch);
    }
}

fn main() {
    let args = Args::parse();

    let bottom = format!("c({})({})", "\"".red(), "\"".red()).to_string();
    let ascii = ["(\\ /)", "( . .)", &bottom];

    let hostname = fallible::hostname().unwrap_or(String::from("N/A"));
    let user = username();

    let combined = format!("{}@{}", user.italic(), hostname.italic());

    let kernel = System::kernel_version().unwrap_or(String::from("N/A"));
    let pretty = distro();

    let wm: String;

    if cfg!(windows) {
        wm = "Aero".to_string();
    } else if cfg!(unix) {
        wm = var("XDG_CURRENT_DESKTOP").unwrap();
    } else {
        wm = "N/A".to_string();
    }

    println!();
    if !args.ascii_only {
        println!("{:>48}", combined);
        cpu_arch(&args);
        println!("{:>8} {:>6} {}", ascii[0], "OS".blue().bold(), pretty);
        println!("{:>9} {:>9} {}", ascii[1], "Kernel".red().bold(), kernel);
        println!("{:>28} {:>4} {}", ascii[2], "WM".green().bold(), wm);
    } else {
        for i in ascii {
            println!("  {}", i);
        }
    }

    if !args.hide_colours && !args.ascii_only {
        println!();
        let colors = ["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"];
        let mut color_string: String = "    ".to_owned();
        for color in colors {
            color_string.push_str(&format!("{:>3}", "●".color(color)));
        }
        println!("{}", color_string);
    }

    println!();
}
