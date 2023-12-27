use serde::Deserialize;

use crate::types::ship::ShipsByCountry;


#[derive(Deserialize, Debug, Clone, Default)]
pub struct GameState {
    pub scale: GameScale,
    pub ships: ShipsByCountry, 
}

#[derive(Deserialize, Debug, Clone, Default)]
pub enum Country {
    #[default]
    USA,
    GB,
    FR,
    CN,
    RUS,
    JPN,
    IT,
    GER,
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

#[derive(Deserialize, Debug, Clone, Default)]
pub enum GameScale {
    #[default]
    SM,
    MD,
    LG,
}