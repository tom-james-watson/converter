use crate::units::{Unit, UnitType};

pub fn init() -> UnitType {
    UnitType {
        name: String::from("Mass"),
        units: vec![
            Unit {
                name: String::from("Gram"),
                abbreviation: String::from("g"),
                factor: 0.01,
            },
            Unit {
                name: String::from("Kilogram"),
                abbreviation: String::from("kg"),
                factor: 1.0,
            },
        ],
    }
}
