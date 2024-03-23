use colored::Colorize;

fn main() {
    let bottom = format!("c({})({})", "\"".red(), "\"".red()).to_string();
    let ascii = vec!["(\\ /)", "( . .)", &bottom];

    for i in ascii {
        println!("  {}", i);
    }
}
