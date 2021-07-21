use ansi_term::Colour;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

extern crate dirs;
extern crate serde;
extern crate toml;

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    editor: String,
    path: String,
    extension: String,
}

pub fn check_config_exsists(path: &str) -> std::io::Result<()> {
    if !Path::new(&path).exists() {
        generate_config()?;
    }
    Ok(())
}

fn generate_config() -> std::io::Result<()> {
    let raw_path = dirs::home_dir().unwrap();
    let path = raw_path.into_os_string().into_string().unwrap();
    let _posts_dir = format!("{}{}", path, "/rlsmemo/_posts/");
    let config_dir = format!("{}{}", path, "/rlsmemo/");

    check_dir_exsists(&config_dir)?;
    check_dir_exsists(&_posts_dir)?;

    let setting = Setting {
        editor: "vim".into(),
        path: _posts_dir.into(),
        extension: "md".into(),
    };

    let config_file_path = format!("{}{}", config_dir, "Setting.toml");

    let mut file = File::create(config_file_path)?;
    let toml = toml::to_string(&setting).unwrap();
    write!(file, "{}", toml)?;
    file.flush()?;

    Ok(())
}

fn need_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}

fn check_dir_exsists(path: &str) -> std::io::Result<()> {
    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(())
}

pub fn open_editor(path: &str, title: String, editor: &str) {
    let filename = format!("{}{}", &path, &title);

    Command::new(editor).arg(filename).exec();
}

pub fn create_with_filename(
    path: &str,
    editor: &str,
    name: String,
    extension: &str,
) -> std::io::Result<()> {
    check_dir_exsists(&path)?;
    open_editor(&path, format!("{}.{}", name, &extension), &editor);
    Ok(())
}

pub fn create(path: &str, editor: &str, extension: &str) {
    check_dir_exsists(&path);

    print!("Title :");
    io::stdout().flush().unwrap();
    let mut title = need_input();

    if title.is_empty() {
        title = format!(
            "{}.{}",
            Utc::now().format("%Y-%m-%d").to_string(),
            &extension
        );
        println!("{}", title);
    } else if !title.is_empty() {
        title = format!("{}.{}", title, &extension);
    }

    if title.contains(" ") {
        title = title.replace(" ", "-");
    }

    open_editor(&path, title, &editor);
}

pub fn delete(path: &str, filename: &str) -> std::io::Result<()> {
    print!(
        "{}",
        Colour::Red.paint("Are you sure you want to delete the file? (y/n):")
    );
    io::stdout().flush().unwrap();

    let res = need_input();

    if res == "y" {
        fs::remove_file(format!("{}{}", &path, &filename)).unwrap();
        let deleted_file_path = format!("{}{}", &path, &filename);

        println!(
            "{}{}",
            Colour::Yellow.paint("Deleted :"),
            Colour::Yellow.paint(deleted_file_path)
        );
    }
    Ok(())
}

pub fn edit(path: &str, editor: &str) {
    Command::new("sh").arg("-c").arg(format!("{} $(/bin/ls {} | fzf)", editor,path )).exec();

}

pub fn config(file_path: &str, editor: &str) {
    Command::new(&editor).arg(&file_path).exec();
}
