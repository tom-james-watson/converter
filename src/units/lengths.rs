use crate::units::{Unit, UnitType};

pub fn init() -> UnitType {
    UnitType {
        name: String::from("Lengths"),
        units: vec![
            Unit {
                name: String::from("Centimetre"),
                abbreviation: String::from("cm"),
                factor: 0.01,
            },
            Unit {
                name: String::from("Metre"),
                abbreviation: String::from("m"),
                factor: 1.0,
            },
            Unit {
                name: String::from("Kilometre"),
                abbreviation: String::from("km"),
                factor: 1000.0,
            },
        ],
    }
}
