use clap::Parser;
use mastr::types::{ApiResponse, Unit};
use mastr::api::fetch_json_cached;

mod mastr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// City name
   #[arg(short, long)]
   city: String,
}

fn parse_json(data: String) -> () {
    let einheiten: Vec<Unit> = match serde_json::from_str::<ApiResponse>(&data) {
        Ok (api_result) => api_result.data,
        Err (error) => {
            println!("ERROR deserializing einheiten {}", error);
            Vec::new()
        },
    };

    let mut summe_solar: u32 = 0;
    let mut summe_solar_gross: f32 = 0.0;
    let mut summe_battery_gross: f32 = 0.0;
    let mut summe_others_gross: f32 = 0.0;
    let mut max_unit_gross_power: f32 = 0.0;
    let mut max_unit: Option<Unit> = None;

    for unit in einheiten {
        if unit.gross_power > max_unit_gross_power {
            max_unit_gross_power = unit.gross_power;
            max_unit = Some(unit.clone());
        }
        match unit.category {
            1 => {
                summe_solar_gross += unit.gross_power;
                summe_solar += 1;
            },
            8 => summe_battery_gross += unit.gross_power,
            _ => summe_others_gross += unit.gross_power,
            //_ => println!("WARNING: Unknown unit category {}:{}", unit.category, unit.energy_carrier),
        }
    }

    println!("🌞 Summe SolarAnlagen: {}", summe_solar);
    println!("🌞 Summe Solar Bruttoleistung: {}", summe_solar_gross);
    println!("🔋 Battery Bruttoleistung: {}", summe_battery_gross);
    println!("⌁ Andere Bruttoleistung: {}", summe_others_gross);
    println!("🔥 Maximalleistung: {} kW {:?}", max_unit_gross_power, max_unit);

}

fn main() {
    let args = Args::parse();

    let result = fetch_json_cached(args.city);

    match result {
        Some(body) => parse_json(body),
        None => println!("No result from MaStR API"),
    }
}