use ansi_term::Colour;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::*;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::path::Path;
use rusmo::*;


extern crate dirs;
extern crate serde;
extern crate toml;
extern crate cmd_lib;


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
    let _posts_dir = format!("{}{}", utils::home_dir(), "/rlsmemo/_posts/");
    let config_dir = format!("{}{}", utils::home_dir(), "/rlsmemo/");

    utils::check_dir_exsists(&config_dir)?;
    utils::check_dir_exsists(&_posts_dir)?;

    let setting = Setting {
        editor: "vim".into(),
        path: _posts_dir.into(),
        extension: "md".into(),
    };


    let mut file = File::create(format!("{}{}", config_dir, "Setting.toml"))?;
    let toml = toml::to_string(&setting).unwrap();
    write!(file, "{}", toml)?;
    file.flush()?;

    Ok(())
}



pub fn open_editor(title: String) {
    let filename = format!("{}/{}", utils::get_toml_env().path(), &title);
    Command::new(utils::get_toml_env().editor()).arg(filename).exec();
}

pub fn create_with_filename(name: String) -> std::io::Result<()> {
    open_editor(format_title(name));
    Ok(())
}

pub fn create() {
    print!("Title :");
    stdout().flush().unwrap();
    let title = utils::need_input();
    open_editor(format_title(title));
}


fn format_title(mut title:String)->String{
    if title.contains(" ") {
        title = title.replace(" ", "-");
    }else if title.is_empty(){
        title = Utc::now().format("%Y-%m-%d").to_string();
    }

    return format!("{}.{}",&title,utils::get_toml_env().extension());
}

pub fn delete(filename: &str) -> std::io::Result<()> {

    print!("{}",Colour::Red.paint("Are you sure you want to delete the file? (y/n):"));
    stdout().flush().unwrap();

    let res = utils::need_input();

    if res == "y" {
        let deleted_file_path = format!("{}/{}", utils::get_toml_env().path(), &filename);
        fs::remove_file(&deleted_file_path).unwrap();
        println!("{}",Colour::Yellow.paint(format!("Deleted : {}",deleted_file_path)));
    }
    Ok(())
}

pub fn edit() -> std::io::Result<()>{
    let target_filename = cmd_lib::run_fun!("ls {} | fzf ",utils::get_toml_env().path())?;

    Command::new(utils::get_toml_env().editor())
        .arg(format!("{}/{}",utils::get_toml_env().path(),target_filename.trim()))
        .exec();
    Ok(())
}

pub fn config(file_path: &str) {
    Command::new(utils::get_toml_env().editor()).arg(&file_path).exec();
}
