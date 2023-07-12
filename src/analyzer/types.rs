#[derive(Debug)]
pub struct AnalyzerResult {
    pub unit_count: u32,

    pub gross_power: f32,
}

impl AnalyzerResult {
    pub fn new() -> AnalyzerResult {
        Self {
            unit_count: 0,
            gross_power: 0.0,
        }
    }
}
