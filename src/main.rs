use clap::Parser;
use owo_colors::{AnsiColors::*, OwoColorize};
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

    /// Hide the username and hostname
    #[arg(short = 'u', long)]
    hide_combined: bool,

    /// Hide terminal colours
    #[arg(short = 'z', long, default_value_t = false)]
    hide_colours: bool,

    /// Use the alternative ASCII art
    #[arg(short = 'A', long, default_value_t = false)]
    alt_art: bool,
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

// Hide the username@hostname text
fn hide_combined(args: &Args) {
    if !args.hide_combined {
        let hostname = hostname().unwrap_or(String::from("N/A"));
        let user = username().unwrap_or(String::from("N/A"));
        let combined = format!("{}@{}", user, hostname);
        println!("{: <13}{}", "", combined.bold());
    }
}

// Display the ASCII art
fn ascii_art(args: &Args) -> [String; 3] {
    if !args.alt_art {
        let bottom = format!("c({})({})", "\"".red(), "\"".red()).to_string();
        ["(\\ /)".to_string(), "( . .)".to_string(), bottom]
    } else {
        let top = format!("{}", ".".red()).to_string();
        [top, "\\\\  /\\".to_string(), " \\\\//V".to_string()]
    }
}

fn main() {
    let args = Args::parse();

    let pretty = distro().unwrap_or(String::from("N/A"));

    let wm: String;

    if cfg!(windows) {
        wm = "Aero".to_string();
    } else if cfg!(unix) {
        let xdg_current_desktop = var("XDG_CURRENT_DESKTOP");
        let desktop = desktop_env().unwrap_or(whoami::DesktopEnv::Unknown(String::from("N/A"))).to_string();
        if desktop != "Unknown: N/A" {
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
        hide_combined(&args);
        cpu_arch(&args);
        display_kernel(&args);
        let ascii = ascii_art(&args);

        let spacings: [usize; 3] = if args.alt_art {
            [
                (7 - (ascii[0].len() - 10)) + 2,
                (7 - ascii[1].len()) + 2,
                (7 - ascii[2].len()) + 2,
            ]
        } else {
            [
                (7 - ascii[0].len()) + 2,
                (7 - ascii[1].len()) + 2,
                (7 - (ascii[2].len() - 20)) + 2,
            ]
        };

        println!(
            "   {}{} {} {}",
            ascii[0],
            " ".to_string().repeat(spacings[0]),
            "OS".red().bold(),
            pretty
        );
        println!(
            "   {}{} {} {}",
            ascii[1],
            " ".to_string().repeat(spacings[1]),
            "Shell".green().bold(),
            shell
        );
        println!(
            "   {}{} {} {}",
            ascii[2],
            " ".to_string().repeat(spacings[2]),
            "WM".blue().bold(),
            wm
        );
    } else {
        for i in ascii_art(&args) {
            println!("  {}", i);
        }
    }

    if !args.hide_colours && !args.ascii_only {
        println!();
        let colors = [Black, Red, Green, Yellow, Blue, Magenta, Cyan, White];
        let mut color_string: String = "    ".to_string();
        for color in colors {
            color_string.push_str(&format!("{:>3}", "●".color(color)));
        }
        println!("{}", color_string);
    }
}
