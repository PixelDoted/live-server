use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use clap::Parser;
use colored::Colorize;
use warp::{filters::path::FullPath, http::StatusCode, Filter};

mod cli;

#[tokio::main]
async fn main() {
    let mut args = cli::Args::parse();
    if args.ip.to_lowercase() == "localhost" {
        args.ip = "127.0.0.1".to_string();
    }

    let files_path = args.path.unwrap_or(env::current_dir().unwrap());
    let addr = SocketAddr::new(IpAddr::from_str(&args.ip).unwrap(), args.port);
    println!("{}    {:?}", "Path".cyan(), files_path);
    println!("{} `{}`", "Address".cyan(), addr.to_string());
    println!();

    let route = warp::get()
        .and(warp::path::full())
        .map(move |param: FullPath| {
            let requested_path = if param.as_str() == "/" {
                "index.html"
            } else {
                &param.as_str()[1..]
            };

            let mut path = files_path.clone();
            path.push(requested_path);

            println!("{} \"{}\"", "Requested".blue(), requested_path);
            let Ok(file) = std::fs::read(&path) else {
                println!("{} \"{}\"", "Not Found".bright_red(), requested_path);

                return warp::http::Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(b"File not found.".to_vec())
                    .unwrap();
            };

            let mime = mime_guess::from_path(&path)
                .first()
                .unwrap_or(mime_guess::mime::APPLICATION_OCTET_STREAM);

            println!("{}        \"{}\"", "OK".bright_green(), requested_path);
            warp::http::Response::builder()
                .header("content-type", &mime.to_string())
                .body(file)
                .unwrap()
        });

    warp::serve(route).run(addr).await;
}
