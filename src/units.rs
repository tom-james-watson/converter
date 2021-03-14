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
}
