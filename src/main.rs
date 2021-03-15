extern crate dirs;
extern crate docopt;
extern crate toml;
use ansi_term::Colour;
use docopt::Docopt;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

pub mod file;

#[derive(Debug, Deserialize)]
pub struct Args {
    cmd_new: bool,
    cmd_n: bool,
    cmd_list: bool,
    cmd_l: bool,
    cmd_delete: bool,
    cmd_d: bool,
    cmd_config: bool,
    cmd_c: bool,
    cmd_edit: bool,
    cmd_e: bool,
    arg_filename: String,
    flag_help: bool,
    flag_version: bool,
    flag_t: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    editor: String,
    path: String,
    expand: String,
}

fn read_setting_info(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

static USAGE: &'static str = "
Usage:
  rlsmemo
  rlsmemo (new    | n) [-t <filename>] 
  rlsmemo (edit   | e) <filename>
  rlsmemo (list   | l)
  rlsmemo (config | c) 
  rlsmemo (delete | d) <filename>
  rlsmemo (--help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  Show version
  -t <filename>  create with title

";

fn main() {
    let raw_path = dirs::home_dir().unwrap();
    let path = raw_path.into_os_string().into_string().unwrap();
    let config_dir = format!("{}{}", path, "/rlsmemo/");
    let config_file_path = format!("{}{}", config_dir, "Setting.toml");

    file::check_config_exsists(&config_file_path);

    let s = match read_setting_info(format!("{}{}", config_dir, "Setting.toml").to_owned()) {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };

    //Read setting from Setting.toml
    let setting: Result<Setting, toml::de::Error> = toml::from_str(&s);
    let info = match setting {
        Ok(p) => p,
        Err(e) => panic!("fail to parse toml: {}", e),
    };

    let full_path = info.path;
    let editor = info.editor;
    let expand = info.expand;

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_new && !args.flag_t.is_empty() || args.cmd_n && !args.flag_t.is_empty() {
        file::create_with_filename(&full_path, &editor, args.flag_t, &expand);
    }

    if args.cmd_new || args.cmd_n {
        println!("{}", args.arg_filename);
        file::create(&full_path, &editor, &expand);
    }

    if args.cmd_list || args.cmd_l {
        list(&full_path);
    }

    if args.cmd_delete || args.cmd_d {
        file::delete(&full_path, &args.arg_filename);
    }

    if args.cmd_config || args.cmd_c {
        file::config(&config_file_path, &editor);
    }

    if args.cmd_e || args.cmd_edit {
        file::edit(&full_path, &args.arg_filename, &editor);
    }

    if args.flag_version {
        println!("{}", VERSION);
    }

    if !args.cmd_new
        && !args.cmd_n
        && !args.cmd_delete
        && !args.cmd_d
        && !args.cmd_list
        && !args.cmd_l
        && !args.cmd_c
        && !args.cmd_config
        && args.arg_filename == ""
        && !args.flag_version
    {
        println!("{}", USAGE);
    }
}

fn list(path: &str) {
    let dir = fs::read_dir(&Path::new(&path)).unwrap();
    let filenames = dir
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();

    let list_color = Colour::RGB(84, 189, 199).on(Colour::Black);
    for filename in filenames {
        if filename.starts_with('.') {
            continue;
        }
        println!("{}", list_color.paint(filename));
    }
}
