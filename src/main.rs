use std::collections::HashMap;
use std::fs::{self};
use walkdir::DirEntry;
use std::path::PathBuf;
use regex::Regex;
use walkdir::WalkDir;
use whoami;
use std::env;
use clap::{Parser};

fn get_directories_in_directory(dir_path: &str) -> Vec<String> {
    let mut directories = vec![];

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(dir_name) = entry.file_name().to_str() {
                            directories.push(String::from(dir_name));
                        }
                    }
                }
            }
        }
    }

    directories
}

fn filter_directories(dir_names: Vec<String>, regex_str: &str) -> Vec<String> {
    let regex = Regex::new(regex_str).unwrap();
    dir_names.into_iter()
        .filter(|dir_name| regex.is_match(dir_name))
        .collect()
}

// fn get_file_path(start_dir: &PathBuf, file_name: &str) -> Option<PathBuf> {
//     for entry in fs::read_dir(start_dir).unwrap() {
//         let entry = entry.unwrap();
//         let path = entry.path();
//
//         if path.is_dir() {
//             if let Some(file_path) = get_file_path(&path, file_name) {
//                 return Some(file_path);
//             }
//         } else if path.file_name().unwrap() == file_name {
//             return Some(path);
//         }
//     }
//     None
// }

fn path_buf_to_string(path_buf: PathBuf) -> String {
    let os_string = path_buf.into_os_string();
    os_string.into_string().unwrap_or_else(|os_string| {
        panic!("Failed to convert OsString to String: {:?}", os_string)
    })
}

fn get_directories_map(root_firefox_path: &str, dir_names: Vec<String>) -> HashMap<String, String> {
    let mut directories_map = HashMap::new();

    for dir_name in dir_names {
        let dir_path = PathBuf::from(&root_firefox_path).join(&dir_name);
        if let Some(dir_str) = dir_path.to_str() {
            directories_map.insert(dir_name, dir_str.to_string());
        }
    }

    directories_map
}

fn get_profile_database_map(root_firefox_path: &str, profiles: Vec<String>) -> HashMap<String, String> {
    let mut places_databases = HashMap::new();

    for profile in &profiles {
        let mut start_path = PathBuf::new();
        start_path.push(format!("{}/{}", root_firefox_path, profile));

        match get_file_path(&start_path, "places.sqlite") {
            Some(path) => {
                places_databases.insert(profile.clone(), path);
            },
            None => {}
        };
    }

    places_databases
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn get_file_path(start_path: &PathBuf, file_name: &str) -> Option<String> {
    let walker = WalkDir::new(start_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        if let Ok(entry) = entry {
            if entry.file_name() == file_name {
                if let Some(file_path_str) = entry.path().to_str() {
                    return Some(file_path_str.to_string());
                }
            }
        }
    }
    None
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    browser: u8,

    #[arg(short, long, default_value = "false")]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    println!("Browser: {}\nDebug: {}", args.browser, args.debug);

    let operating_system = env::consts::OS;
    println!("OS: {}", operating_system);

    let user = whoami::username();
    let root_firefox_path: String;
    let root_firefox_directories: Vec<String>;

    if operating_system == "windows" { // TODO: windows moment
        root_firefox_path = format!("C:\\Users\\{}\\AppData\\Roaming\\Mozilla\\Firefox\\Profiles", user);
        root_firefox_directories = get_directories_in_directory(&root_firefox_path);
    } else {
        root_firefox_path = format!("/home/{}/.mozilla/firefox", user);
        root_firefox_directories = get_directories_in_directory(&root_firefox_path);
    }

    let profiles = filter_directories(root_firefox_directories.clone(), "(?i)(safe|default)");

    println!();

    println!("root_firefox_directories: {:?}", root_firefox_directories);
    println!("profiles: {:?}", profiles);

    println!();

    for (dir_name, dir_path) in get_profile_database_map(root_firefox_path.as_str(), profiles) {
        println!("{}: {}", dir_name, dir_path);
    }

}