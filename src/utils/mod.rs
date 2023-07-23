use std::collections::HashMap;
use std::{fs};

use std::path::{Path, PathBuf};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};
use supports_color::Stream;

pub fn filter_directories(dir_names: Vec<String>, regex_str: &str) -> Vec<String> {
    let regex = Regex::new(regex_str).unwrap();
    let profiles: Vec<String> = dir_names.into_iter()
        .filter(|dir_name| regex.is_match(dir_name))
        .collect();

    profiles
}

pub fn get_directories_in_directory(dir_path: &str) -> Vec<String> {
    let mut directories = vec![];
    let path = Path::new(dir_path);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    if let Some(dir_name) = entry.file_name().to_str() {
                        directories.push(String::from(dir_name));
                    }
                }
            }
        }
    }

    directories
}

pub fn get_profile_database_map(root_firefox_path: &str, profile_names: &Vec<String>) -> HashMap<String, String> {
    let mut places_databases = HashMap::new();

    for profile in profile_names {
        let mut start_path = PathBuf::new();
        start_path.push(format!("{}/{}", root_firefox_path, profile));

        if let Some(path) = get_file_path(&start_path, "places.sqlite") {
            places_databases.insert(profile.clone(), path);
        };
    }

    places_databases
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn get_file_path(start_path: &PathBuf, file_name: &str) -> Option<String> {
    let walker = WalkDir::new(start_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)).flatten() {
        if entry.file_name() == file_name {
            if let Some(file_path_str) = entry.path().to_str() {
                return Some(file_path_str.to_string());
            }
        }
    }
    None
}

pub fn supports_ansi() -> bool {
    supports_color::on(Stream::Stdout).is_some()
}
