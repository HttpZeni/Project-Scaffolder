mod debug;

use std::collections::HashMap;
use std::env::{args, current_dir};
use std::fs;
use std::process::Command;

fn get_input() -> Vec<String> {
    args().skip(1).collect()
}

fn all_languages_hash_map() -> HashMap<String, String> {
    let mut valid_langs: HashMap<String, String> = HashMap::new();

    valid_langs.insert("python".to_string(), ".py".to_string());
    valid_langs.insert("cpp".to_string(), ".cpp".to_string());
    valid_langs.insert("rust".to_string(), ".rs".to_string());
    valid_langs.insert("javascript".to_string(), ".js".to_string());
    valid_langs.insert("typescript".to_string(), ".ts".to_string());
    valid_langs.insert("bash".to_string(), ".sh".to_string());
    valid_langs.insert("powershell".to_string(), ".ps1".to_string());
    valid_langs.insert("java".to_string(), ".java".to_string());
    valid_langs.insert("kotlin".to_string(), ".kt".to_string());
    valid_langs.insert("scala".to_string(), ".scala".to_string());
    valid_langs.insert("c".to_string(), ".c".to_string());
    valid_langs.insert("csharp".to_string(), ".cs".to_string());
    valid_langs.insert("go".to_string(), ".go".to_string());
    valid_langs.insert("lua".to_string(), ".lua".to_string());
    valid_langs.insert("ruby".to_string(), ".rb".to_string());
    valid_langs.insert("php".to_string(), ".php".to_string());

    valid_langs
}

fn check_for_language(valid_args: &Vec<String>) -> (String, String) {
    let valid_langs: HashMap<String, String> = all_languages_hash_map();
    let mut valid_lang: String = String::new();
    let mut valid_ending: String = String::new();

    for (lang, ending) in valid_langs {
        if valid_args[1].to_lowercase() == lang {
            valid_lang = lang;
            valid_ending = ending;
        }
    }

    (valid_lang, valid_ending)
}

fn create_git_init(path: String) {
    Command::new("git")
        .arg("init")
        .current_dir(&path)
        .output()
        .ok();
}

fn create_file(file_ending: String, path: Option<String>) -> std::io::Result<()> {
    if let Some(path) = path {
        let folder_path = format!("{}/src", &path);

        fs::create_dir_all(&folder_path)?;

        let full_path = format!("{}/main{}", folder_path, file_ending);
        fs::File::create(full_path)?;
        fs::File::create(format!("{}/.gitignore", path))?;
        fs::File::create(format!("{}/README.md", path))?;
        create_git_init(path);
    } else {
        fs::File::create(format!("src/main{}", file_ending))?;
        fs::File::create(".gitignore")?;
        fs::File::create("README.md")?;
        create_git_init(current_dir()?.to_string_lossy().to_string());
    }

    debug::log("Created git init!");

    Ok(())
}

fn create_got_called(valid_args: &Vec<String>) {
    let lang = check_for_language(valid_args);

    if lang.0.len() != 0 {
        debug::log("Got Language!");
        let file = create_file(
            lang.1,
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
