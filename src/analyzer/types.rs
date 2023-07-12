use crate::mastr::types::Unit;

#[derive(Debug)]
pub struct AnalyzerResult {
    pub unit_count: u32,

    pub balkonkraftwerke: Vec<Unit>,

    pub gross_power: f32,
}

impl AnalyzerResult {
    pub fn new() -> AnalyzerResult {
        Self {
            unit_count: 0,
            balkonkraftwerke: Vec::new(),
            gross_power: 0.0,
        }
    }
}
