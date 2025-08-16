use std::fs::File;
use std::io::{prelude::*, BufReader};

#[cfg(target_os = "linux")]
pub fn kernel() -> std::io::Result<String> {
    let file = File::open("/proc/version")?;
    let mut contents = String::new();
    BufReader::new(file).read_to_string(&mut contents)?;

    let version = contents.split_whitespace().collect::<Vec<&str>>()[2];

    Ok(version.to_string())
}

#[cfg(target_os = "macos")]
pub fn kernel() -> std::io::Result<String> {
    let output_bytes = std::process::Command::new("uname")
        .arg("-r")
        .output()?
        .stdout;
    let trimmed: Vec<u8> = output_bytes
        .iter()
        .take(output_bytes.len() - 1)
        .map(|b| *b)
        .collect();

    Ok(String::from_utf8(trimmed).unwrap())
}
