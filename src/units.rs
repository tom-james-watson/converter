pub mod length;
pub mod mass;

pub struct UnitType {
    pub name: String,
    pub units: Vec<Unit>,
}

pub struct Unit {
    pub name: String,
    pub abbreviation: String,
    pub factor: f64,
}

impl Unit {
    pub fn convert(&self, value: f64, to: &Unit) -> f64 {
        value * self.factor / to.factor
    }

    pub fn convert_as_string(&self, value: f64, to: &Unit) -> String {
        String::from(
            format!("{:.10}", self.convert(value, to))
                .trim_end_matches('0')
                .trim_end_matches('.'),
        )
    }

    pub fn get_title(&self) -> String {
        format!("{} ({})", self.name, self.abbreviation)
    }
}
