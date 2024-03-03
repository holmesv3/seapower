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

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Copy, Hash, Eq)]
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
impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Country::NONE => "NONE".to_string(),
            Country::USA => "USA".to_string(),
            Country::GB => "GB".to_string(),
            Country::FR => "FR".to_string(),
            Country::CN => "CN".to_string(),
            Country::RUS => "RUS".to_string(),
            Country::JPN => "JPN".to_string(),
            Country::IT => "IT".to_string(),
            Country::GER => "GER".to_string(),
        }
    }
}
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Copy)]
pub enum Side {
    AXIS,
    ALLIES,
}
impl FromStr for Side {
    type Err = crate::SPError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "axis" => Ok(Side::AXIS),
            "allies" => Ok(Side::ALLIES),
            &_ => Err(crate::SPError::CatchAll),
        }
    }
}
impl ToString for Side {
    fn to_string(&self) -> String {
        match self {
            Side::AXIS => "Axis".to_string(),
            Side::ALLIES => "Allies".to_string(),
        }
    }
}


#[derive(Deserialize, Debug, Clone, Default, Copy)]
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
