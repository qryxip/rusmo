use std::fs;
use std::io::{BufReader,Read,stdin};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]

pub struct Setting {
     editor: String,
     path: String,
     extension: String,
}

impl Setting{
    pub fn editor(&mut self)->String{
        return self.editor.clone();
    }
    pub fn path(&mut self)->String{
        return self.path.clone();
    }
    pub fn extension(&mut self)->String{
        return self.extension.clone();
    }
}


pub fn home_dir()->String{
    let raw_path = dirs::home_dir().unwrap();
    let path     = raw_path.into_os_string().into_string().unwrap();
    return path
}

fn fetch_toml_content(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

pub fn get_toml_env()->Setting{

    let s = match fetch_toml_content(format!("{}/rusmo/{}",home_dir(), "Setting.toml").to_owned()) {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };

    let setting: Result<Setting, toml::de::Error> = toml::from_str(&s);

    let info = match setting {
        Ok(p) => p,
        Err(e) => panic!("fail to parse toml: {}", e),
    };

    return info;
}

pub fn need_input() -> String {
    let mut word = String::new();
    stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}

pub fn check_dir_exsists(path: &str) -> std::io::Result<()> {
    if !Path::new(&path).exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(())
}


