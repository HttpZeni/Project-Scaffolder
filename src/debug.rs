use colored::Colorize;

pub fn log(str: &str) {
    println!("{} > {}", "[Debug]".blue(), str)
}
pub fn error(str: &str) {
    eprintln!("{} > {}", "[Error]".red(), str);
}
