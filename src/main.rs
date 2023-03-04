use std::sync::{Arc, Mutex};

use clap::Parser;
use colored::Colorize;
use hashbrown::HashMap;

use util::*;

mod util;

static IP_REGEX: &str = r"(\d{1,3}\.){3}\d{1,3}";

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The top amount of ips to display
    #[clap(short, long, default_value = "5")]
    top: usize,

    /// The file which contains the logs
    #[clap(short, long, required = true)]
    file: String,

    /// The output file, if none specified the date will be used
    #[clap(short, long, default_value = "%Y-%m-%d-%H-%M-%S.toip")]
    output: String,
}


fn main() {
    let args = Args::parse();
    let ips = Arc::new(Mutex::new(HashMap::new()));

    let buffer = io::get_file_buffer(&args.file);
    io::fill_ips(&ips, buffer);

    let ips = ips.lock().unwrap();
    println!("{}", format!("Number of unique IPs: {}", ips.len()).bright_green());

    let top_ips = iter::get_top_ips(&ips, args.top);

    println!("{}", format!("Country origin from top IP: {}", web::get_country(&top_ips[0].0)).bright_red());
    println!("{}", format!("Count per IP (top {}):", args.top).bright_blue());
    println!("{}", "Count\tIP".bright_green());

    io::ip_writer(&args.output, &top_ips, |data| println!("{}", data));
}