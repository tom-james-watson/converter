use crate::units::{Unit, UnitType};

pub fn init() -> UnitType {
    UnitType {
        name: String::from("Mass"),
        units: vec![
            Unit {
                name: String::from("Grams"),
                abbreviation: String::from("g"),
                factor: 0.001,
            },
            Unit {
                name: String::from("Kilograms"),
                abbreviation: String::from("kg"),
                factor: 1.0,
            },
        ],
    }
}
