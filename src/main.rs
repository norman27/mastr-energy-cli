use analyzer::solar_analyzer::parse_json;
use clap::Parser;
use mastr::api::fetch_json_cached;

mod analyzer;
mod mastr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// City name
    #[arg(short, long)]
    city: String,
}

fn main() {
    let args = Args::parse();

    let result = fetch_json_cached(args.city);

    match result {
        Some(body) => parse_json(body),
        None => println!("No result from MaStR API"),
    }
}
