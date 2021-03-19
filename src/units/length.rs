use crate::units::{Unit, UnitType};

pub fn init() -> UnitType {
    UnitType {
        name: String::from("Length"),
        units: vec![
            Unit {
                name: String::from("Centimetres"),
                abbreviation: String::from("cm"),
                factor: 0.01,
            },
            Unit {
                name: String::from("Metres"),
                abbreviation: String::from("m"),
                factor: 1.0,
            },
            Unit {
                name: String::from("Kilometres"),
                abbreviation: String::from("km"),
                factor: 1000.0,
            },
        ],
    }
}
