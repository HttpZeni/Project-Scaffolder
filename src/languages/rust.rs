use super::Language;

pub struct Rust;

impl Language for Rust {
    fn extention(&self) -> String {
        ".rs".to_string()
    }

    fn main_file(&self) -> String {
        r#"fn main() {
    println!("Hello, World!");
}"#
        .to_string()
    }

    fn gitignore(&self) -> String {
        "/target\nCargo.lock".to_string()
    }

    fn readme(&self) -> String {
        String::new()
    }
}
