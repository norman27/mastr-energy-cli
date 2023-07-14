use std::collections::BTreeMap;

use crate::mastr::types::Unit;

#[derive(Debug)]
pub struct AnalyzerResult {
    pub unit_count: u32,
    pub balcony_units: Vec<Unit>,
    pub gross_power: f32,
    pub power_added_by_day: BTreeMap<i64, PowerAdded>, // maps timestamp -> PowerAdded
    pub max_unit: Option<Unit>,
}

// @TODO does this need to be a struct or can it be some kind of `type`?
#[derive(Debug)]
pub struct PowerAdded {
    pub added_units: u16,
    pub added_gross_power: f32,
}

impl AnalyzerResult {
    pub fn new() -> AnalyzerResult {
        Self {
            unit_count: 0,
            balcony_units: Vec::new(),
            gross_power: 0.0,
            power_added_by_day: BTreeMap::new(),
            max_unit: None,
        }
    }
}
