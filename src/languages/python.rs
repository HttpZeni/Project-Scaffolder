use super::Language;

pub struct Python;

impl Language for Python {
    fn extention(&self) -> String {
        ".py".to_string()
    }

    fn main_file(&self) -> String {
        r#"def main():
    print("Hello World!")

if __name__ == "__main__":
    main()"#
            .to_string()
    }

    fn gitignore(&self) -> String {
        ".gitignore\n__pycache__/\n.venv/\n*.pyc".to_string()
    }

    fn readme(&self) -> String {
        String::new()
    }
}
