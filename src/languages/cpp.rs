use super::Language;

pub struct Cpp;

impl Language for Cpp {
    fn extention(&self) -> String {
        ".cpp".to_string()
    }

    fn main_file(&self) -> String {
        r#"#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}"#
        .to_string()
    }

    fn gitignore(&self) -> String {
        "build/\n*.o\n*.out\n*.exe".to_string()
    }

    fn readme(&self) -> String {
        String::new()
    }
}
