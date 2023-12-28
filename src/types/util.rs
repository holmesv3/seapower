use leptos::*;
use serde::Deserialize;
use crate::types::ship::*;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct GameState {
    pub scale: GameScale,
    pub ships: ShipsByCountry,
}
#[component]
pub fn GameStateComp(state: GameState) -> impl IntoView {
    view!{
        <div class="p-4 m-8 text-center">
            <GameScaleComp scale=state.scale/>
            <ShipsByCountryTableComp ships=state.ships />
        </div>
    }
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
#[component]
pub fn GameScaleComp(scale: GameScale) -> impl IntoView {
    let string = match scale {
        GameScale::SM => "Small",
        GameScale::MD => "Medium",
        GameScale::LG => "Large",
    };
    view!{
        <h2>Game Scale: {string}</h2> 
    }
}