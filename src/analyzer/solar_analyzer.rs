use mastr::types::{ApiResponse, Unit};

use crate::{analyzer::types::AnalyzerResult, mastr};

pub(crate) fn parse_json(data: String) {
    let mut result = AnalyzerResult::new();

    let einheiten: Vec<Unit> = match serde_json::from_str::<ApiResponse>(&data) {
        Ok(api_result) => api_result.data,
        Err(error) => {
            println!("ERROR deserializing einheiten {}", error);
            Vec::new()
        }
    };

    let mut max_unit_gross_power: f32 = 0.0;
    let mut max_unit: Option<Unit> = None;

    let active_solar_iter = einheiten
        .iter()
        .filter(|unit| unit.is_active() && unit.is_solar());

    for unit in active_solar_iter {
        if unit.gross_power > max_unit_gross_power {
            max_unit_gross_power = unit.gross_power;
            max_unit = Some(unit.clone());
        }

        if unit.is_balkonkraftwerk() {
            result.balkonkraftwerke.push(unit.clone());
        }

        // totals
        result.gross_power += unit.gross_power;
        result.unit_count += 1;
    }

    println!("🌞 Summe SolarAnlagen: {}", result.unit_count);
    println!("🌞 Summe Solar Bruttoleistung: {}", result.gross_power);
    println!(
        "🔥 Maximalleistung: {} kW {:?}",
        max_unit_gross_power, max_unit
    );
    //println!("{:?}", result.balkonkraftwerke);
}
