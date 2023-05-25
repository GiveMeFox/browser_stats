use std::collections::HashMap;
use std::env;
use std::path::Path;
use owo_colors::{OwoColorize, colors::*};
use crate::Args;
use std::error::Error;
use std::fmt;

use crate::utils::*;
use crate::utils::supports_ansi;

pub struct Firefox {
    pub root_firefox_path: String,
    pub root_firefox_directories: Vec<String>,
    pub profiles: Vec<String>,
    pub database_map: HashMap<String, String>,
}

#[derive(Debug)]
pub struct FirefoxError(String);

impl fmt::Display for FirefoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for FirefoxError {}

impl Firefox {
    pub fn new() -> Result<Firefox, FirefoxError> {
        let root_firefox_path = match Self::get_path() {
            Some(value) => value.to_string(),
            None => "".to_string(),
        };

        if root_firefox_path == "".to_string() {
            return Err(FirefoxError(format!("Unsupported Operating System: {}", env::consts::OS)));
        }

        if !Path::new(&root_firefox_path).exists() {
            return Err(FirefoxError(format!("Firefox Not Found At: {}", root_firefox_path)));
        }

        let root_firefox_directories = get_directories_in_directory(&root_firefox_path);
        let profiles = filter_directories(root_firefox_directories.clone(), "(?i)(safe|default)");
        let database_map = get_profile_database_map(&root_firefox_path, &profiles);

        Ok(Firefox {
            root_firefox_path,
            root_firefox_directories,
            profiles,
            database_map,
        })
    }

    fn get_path() -> Option<String> {
        match env::consts::OS {
            "windows" => Some(format!("C:\\Users\\{}\\AppData\\Roaming\\Mozilla\\Firefox\\Profiles", whoami::username())),
            "linux" => Some(format!("/home/{}/.mozilla/firefox", whoami::username())),
            _ => None,
        }
    }

    pub fn print_info(&self, args: &Args) {
        let use_color = supports_ansi();

        let browser = if use_color {
            format!("{} {} {}{}",
                    "Browser:".bold(),
                    args.browser.fg::<Green>(),
                    "| ".bold(),
                    args.verbose.fg::<Green>()
            )
        } else {
            format!("Browser: {} | {}", args.browser, args.verbose)
        };

        let root = if use_color {
            format!("{}{}", "Root: ".bold(), self.root_firefox_path.fg::<Cyan>())
        } else {
            format!("Root: {}", &self.root_firefox_path)
        };

        let directories = if use_color {
            format!(
                "{}{}{}{}",
                "root_firefox_directories:".bold(),
                " [".fg::<Black>(),
                self.root_firefox_directories
                    .iter()
                    .map(|dir| dir.fg::<BrightBlack>().to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                "]".fg::<Black>(),
            )
        } else {
            format!(
                "root_firefox_directories: {:?}\n",
                &self.root_firefox_directories
            )
        };

        let profiles = if use_color {
            format!(
                "{}{}{}{}",
                "profiles:".bold(),
                " [".fg::<Black>(),
                self.profiles
                    .iter()
                    .map(|prof| prof.fg::<BrightBlack>().to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                "]".fg::<Black>(),
            )
        } else {
            format!("profiles: {:?}\n", &self.profiles)
        };

        let database_map = if use_color {
            self.database_map
                .iter()
                .map(|(dir_name, dir_path)| {
                    format!(
                        "{}{}{}{}{}",
                        dir_name.fg::<Yellow>(),
                        ": ".bright_white(),
                        "[".fg::<Black>(),
                        dir_path.as_str().fg::<Magenta>(),
                        "]\n".fg::<Black>()
                    )
                })
                .collect::<Vec<_>>()
                .join("")
        } else {
            self.database_map
                .iter()
                .map(|(dir_name, dir_path)| format!("{}: {}\n", dir_name, dir_path))
                .collect::<Vec<_>>()
                .join("")
        };

        println!("{}\n{}\n{}\n{}\n{}", browser, root, directories, profiles, database_map);
    }
}
