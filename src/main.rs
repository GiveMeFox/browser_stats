use std::collections::{HashMap, HashSet};
use clap::{Parser};
use crate::browser::firefox::Firefox;
use diesel::{prelude::*, SqliteConnection};
use diesel::dsl::sql;
use diesel::sql_types::Text;
use url::Url;

pub mod browser {
    pub mod firefox;
}

pub mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 1)]
    browser: u8,

    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

fn get_links_from_databases(databases: &HashMap<String, String>) -> Vec<(String, Vec<String>)> {
    let mut links = Vec::new();

    for (dir_name, db_path) in databases {
        match SqliteConnection::establish(db_path) {
            Ok(mut conn) => {
                let query = sql::<(Text,)>("SELECT url FROM moz_places");
                match query.load::<(String,)>(&mut conn) {
                    Ok(rows) => {
                        let links_from_db = rows.iter().map(|(url,)| url.clone()).collect();
                        links.push((dir_name.clone(), links_from_db));
                    }
                    Err(err) => {
                        eprintln!("Error loading data from database {}: {}", db_path, err);
                        links.push((dir_name.clone(), Vec::new()));
                    }
                }
            }
            Err(err) => {
                eprintln!("Error connecting to database {}: {}", db_path, err);
            }
        }
    }

    links
}

fn extract_hostname(url_string: &str) -> Option<String> {
    if let Ok(url) = Url::parse(url_string) {
        if let Some(host) = url.host_str() {
            let mut hostname_parts: Vec<&str> = host.split('.').collect();
            let len = hostname_parts.len();
            if len > 1 && hostname_parts[len - 2] == "co" {
                // Handle .co.uk, .co.jp, etc.
                hostname_parts[len - 2] = hostname_parts[len - 3];
            }
            let hostname = hostname_parts.join(".");
            return Some(hostname);
        }
    }
    None
}

fn count_sites(profiles: &Vec<(String, Vec<String>)>) {
    let mut site_counts: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for (profile_name, links) in profiles {
        let mut domains: HashSet<String> = HashSet::new();

        for link in links {
            let domain;
            if let Some(d) = extract_hostname(link) {
                domain = d;
            } else {
                continue
            }

            // println!("domain: {}", domain);

            let domain_parts: Vec<&str> = domain.split('.').rev().take(2).collect();
            let domain_name = domain_parts.into_iter().rev().collect::<Vec<&str>>().join(".");

            domains.insert(domain_name.clone());

            *site_counts.entry(profile_name.clone()).or_default().entry(domain_name).or_default() += 1;
        }

        println!("Top 5 sites for profile '{}':", profile_name);
        if let Some(profile_counts) = site_counts.get(profile_name) {
            let mut site_counts_vec: Vec<_> = profile_counts
                .iter()
                .map(|(k, v)| (k, v))
                .collect();
            site_counts_vec.sort_by_key(|&(_, count)| count);
            site_counts_vec.reverse();
            for (site, count) in site_counts_vec.into_iter().take(5) {
                println!("{}: {}", site, count);
            }
        } else {
            println!("No site counts found for profile '{}'", profile_name);
        }
        println!();
    }
}

fn main() {
    let args = Args::parse();

    let firefox = match Firefox::new() {
        Ok(firefox) => firefox,
        Err(err) => panic!("{}", err)
    };

    // println!("{}", utils::supports_ansi());

    if args.verbose { firefox.print_info(&args); }

    // println!("{:?}", get_links_from_databases(&firefox.database_map));
    count_sites(&get_links_from_databases(&firefox.database_map));
}
