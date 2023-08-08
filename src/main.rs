use std::error::Error;
use analyzer::{solar_analyzer::parse_json, types::AnalyzerResult};
use clap::Parser;
use mastr::api::fetch_json_cached;
use output::plotter::draw_chart;

mod analyzer;
mod mastr;
mod output;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// City name
    #[arg(short, long)]
    city: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let city = &args.city;
    let api_result = fetch_json_cached(city.to_owned());

    let result = match api_result {
        Some(body) => parse_json(body),
        None => AnalyzerResult::new(),
    };

    println!("");
    println!("⛪ Solar Evaluation for {}", city);
    println!("");
    println!("🔆 Solar Units: {}", result.unit_count);
    println!("🔌 Balcony Units: {}", result.balcony_units.len());
    println!("💡 Sum Gross Power: {} kW", result.gross_power);
    println!("🚀 Max. Unit: {} kW", result.max_unit.unwrap().gross_power);

    draw_chart(result.power_added_by_day)?;

    Ok(())
}
