// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use trauso_lib::download_from_url;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if URL is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];

    // Run the download process
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(download_from_url(url));
}
