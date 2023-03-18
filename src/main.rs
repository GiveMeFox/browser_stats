use clap::{Parser};

pub mod browser {
    pub mod firefox;
}
pub mod utils;

use crate::browser::firefox::Firefox;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 1)]
    browser: u8,

    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let firefox = Firefox::new();

    if args.verbose { firefox.print_info(&args); }
}