use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command()]
pub struct Args {
    pub path: Option<PathBuf>,

    #[arg(short, long, default_value_t = String::from("localhost"))]
    pub ip: String,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
