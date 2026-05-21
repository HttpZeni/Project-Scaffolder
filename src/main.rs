mod debug;
mod languages;

use languages::Language;

use languages::cpp::Cpp;
use languages::python::Python;
use languages::rust::Rust;

use std::collections::HashMap;
use std::env::{args, current_dir};
use std::fs;
use std::io::Write;
use std::process::Command;

fn get_input() -> Vec<String> {
    args().skip(1).collect()
}

fn all_languages_hash_map() -> HashMap<String, Box<dyn Language>> {
    let mut valid_langs: HashMap<String, Box<dyn Language>> = HashMap::new();

    valid_langs.insert("python".to_string(), Box::new(Python));
    valid_langs.insert("rust".to_string(), Box::new(Rust));
    valid_langs.insert("cpp".to_string(), Box::new(Cpp));

    valid_langs
}

fn check_for_language(valid_args: &Vec<String>) -> Option<Box<dyn Language>> {
    let mut valid_langs = all_languages_hash_map();
    let input = valid_args[1].to_lowercase();
    valid_langs.remove(&input)
}

fn create_git_init(path: String) {
    Command::new("git")
        .arg("init")
        .current_dir(&path)
        .output()
        .ok();
}

fn create_file(
    file_ending: String,
    language: Box<dyn Language>,
    path: Option<String>,
) -> std::io::Result<()> {
    let mut main_file;
    let mut gitignore_file;
    let mut readme_file;

    if let Some(path) = path {
        let folder_path = format!("{}/src", &path);

        fs::create_dir_all(&folder_path)?;

        let full_path = format!("{}/main{}", folder_path, file_ending);

        main_file = fs::File::create(full_path)?;
        gitignore_file = fs::File::create(format!("{}/.gitignore", path))?;
        readme_file = fs::File::create(format!("{}/README.md", path))?;

        create_git_init(path);
    } else {
        main_file = fs::File::create(format!("src/main{}", file_ending))?;
        gitignore_file = fs::File::create(".gitignore")?;
        readme_file = fs::File::create("README.md")?;

        create_git_init(current_dir()?.to_string_lossy().to_string());
    }

    debug::log("Created git init!");

    write!(main_file, "{}", language.main_file())?;
    debug::log("Created main file!");
    write!(gitignore_file, "{}", language.gitignore())?;
    debug::log("Created gitignore file!");
    write!(readme_file, "{}", language.readme())?;
    debug::log("Created readme file!");

    Ok(())
}

fn create_got_called(valid_args: &Vec<String>) {
    if let Some(lang) = check_for_language(valid_args) {
        debug::log("Got Language!");
        let file = create_file(
            lang.extention(),
            lang,
            if valid_args.len() >= 3 {
                Some(valid_args[2].clone())
            } else {
                None
            },
        );
        if let Ok(_) = file {
            if let Some(path) = valid_args.get(3) {
                debug::log(&format!("Created Files at {}", path));
            } else {
                debug::log("Created Files in current directory");
            }
        }
    } else {
        debug::error("Language is not valid!");
    }
}

fn help() {
    let help_texts: [&str; 1] = ["create -> [language] [path]"];

    for text in help_texts {
        println!("{text}");
    }
}

fn check_for_use(valid_args: &Vec<String>) {
    if valid_args.is_empty() {
        help();
        return;
    }

    let mut uses: HashMap<&str, fn(&Vec<String>)> = HashMap::new();

    uses.insert("create", create_got_called);

    let call = valid_args[0].to_lowercase();

    if let Some(use_fn) = uses.get(&call.as_str()) {
        use_fn(&valid_args);
    } else if call != "help" {
        debug::error("Use doesn't exist!");
    } else {
        help();
    }
}

fn main() {
    let valid_args = get_input();
    check_for_use(&valid_args);
}
