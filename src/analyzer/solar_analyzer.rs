use mastr::types::{ApiResponse, Unit};

use crate::{analyzer::types::AnalyzerResult, mastr};

pub(crate) fn parse_json(data: String) -> AnalyzerResult {
    let mut result = AnalyzerResult::new();

    let units: Vec<Unit> = match serde_json::from_str::<ApiResponse>(&data) {
        Ok(api_result) => api_result.data,
        Err(error) => {
            println!("ERROR deserializing units {}", error);
            Vec::new()
        }
    };

    let mut max_unit_gross_power: f32 = 0.0;
    let mut max_unit: Option<Unit> = None;

    let active_solar_iter = units
        .iter()
        .filter(|unit| unit.is_active() && unit.is_solar());

    for unit in active_solar_iter {
        // find max power source
        if unit.gross_power > max_unit_gross_power {
            max_unit_gross_power = unit.gross_power;
            max_unit = Some(unit.clone());
        }

        // balcony
        if unit.is_balcony() {
            result.balcony_units.push(unit.clone());
        }

        // add daily stats
        result.power_added_by_day
            .entry(unit.start_daystamp())
            .and_modify(|p_by_day| p_by_day.1 += unit.gross_power)
            .and_modify(|p_by_day| p_by_day.0 += 1)
            .or_insert((1, unit.gross_power));

        // totals
        result.gross_power += unit.gross_power;
        result.unit_count += 1;
    }

    result.max_unit = max_unit;

    return result;
}
