use docopt::Docopt;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

extern crate dirs;
extern crate docopt;

pub mod file;
pub mod utils;

#[derive(Debug, Deserialize)]
pub struct Args {
    cmd_new:      bool,
    cmd_n:        bool,
    cmd_list:     bool,
    cmd_l:        bool,
    cmd_delete:   bool,
    cmd_d:        bool,
    cmd_config:   bool,
    cmd_c:        bool,
    cmd_edit:     bool,
    cmd_e:        bool,
    arg_filename: String,
    flag_help:    bool,
    flag_version: bool,
    flag_t:       String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    editor: String,
    path: String,
    extension: String,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

static USAGE: &'static str = "
Usage:
  rusmo
  rusmo (new    | n) [-t <filename>]
  rusmo (edit   | e)
  rusmo (list   | l)
  rusmo (config | c)
  rusmo (delete | d) <filename>
  rusmo (--help | --version)

Options:
  -h, --help     Show this screen
  -v, --version  Show version
  -t <filename>  create with title

";

fn main() {

    utils::check_dir_exsists(&utils::get_toml_env().path()).expect("faild create dir");

    let raw_path         = dirs::home_dir().unwrap();
    let path             = raw_path.into_os_string().into_string().unwrap();
    let config_dir       = format!("{}{}", path, "/rusmo/");
    let config_file_path = format!("{}{}", config_dir, "Setting.toml");


    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_new && !args.flag_t.is_empty() || args.cmd_n && !args.flag_t.is_empty() {
        file::create_with_filename(args.flag_t).expect("failed create file");
    }

    if args.cmd_new || args.cmd_n {
        println!("{}", args.arg_filename);
        file::create();
    }

    if args.cmd_list || args.cmd_l {
        list(&utils::get_toml_env().path());
    }

    if args.cmd_delete || args.cmd_d {
        file::delete(&args.arg_filename).expect("faild delete file");
    }

    if args.cmd_config || args.cmd_c {
        file::config(&config_file_path);
    }

    if args.cmd_e || args.cmd_edit {
        file::edit().expect("faild open file");
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

    for filename in filenames {
        if filename.starts_with('.') {
            continue;
        }
        println!("{}", filename);
    }
}
