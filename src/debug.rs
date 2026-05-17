pub fn log(str: &str) {
    println!("[Debug] > {}", str);
}
pub fn error(str: &str) {
    eprintln!("[Error] > {}", str);
}
