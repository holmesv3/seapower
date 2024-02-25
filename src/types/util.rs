use std::str::FromStr;

use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::types::ship::ShipsByCountry;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct GameState {
    pub scale: GameScale,
    pub ships: ShipsByCountry,
}

impl GameState {
    pub fn is_set(&self) -> bool {
        match self.scale {
            GameScale::UNSET => false,
            _ => true,
        }
    }
}

#[derive(Params, PartialEq)]
pub struct CountryParams {
    pub country: Country,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Copy)]
pub enum Country {
    #[default]
    NONE,
    USA,
    GB,
    FR,
    CN,
    RUS,
    JPN,
    IT,
    GER,
}

impl FromStr for Country {
    type Err = crate::SPError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "" => Ok(Country::NONE),
            "usa" => Ok(Country::USA),
            "gb" => Ok(Country::GB),
            "fr" => Ok(Country::FR),
            "cn" => Ok(Country::CN),
            "rus" => Ok(Country::RUS),
            "jpn" => Ok(Country::JPN),
            "it" => Ok(Country::IT),
            "ger" => Ok(Country::GER),
            &_ => Err(crate::SPError::CatchAll),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub enum ShipType {
    #[default]
    DESTROYER,
    BATTLESHIP,
    CRUISER,
    SUBMARINE,
    CARRIER,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum GameScale {
    #[default]
    UNSET,
    SM,
    MD,
    LG,
}
