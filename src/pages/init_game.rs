use leptos::*;
use leptos_meta::*;

use crate::types::util::{GameState, GameStateComp};
#[component]
pub fn InitGame() -> impl IntoView {
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
    <Title text="Seapower - Start Game"/>
    <div class="base-div">
        <div class="top-bar text-center">"Top Bar"</div>
        <div class="mx-auto p-8 space-y-8">
            <h1 class="h1 text-center">"Load default or upload save game"</h1>
            <div class="mx-auto text-center">
                <button class="m-4 btn-green" hx-get="/init_game/load_default" hx-target="#hx-placeholder">Load Default</button>
                <button class="m-4 btn-purple">Upload Save</button>
            </div>
        </div>
        <div class="sub-div">
            <div id="hx-placeholder">
            </div>
        </div>
    </div> // Base div
            }
}

#[component]
pub fn LoadDefault() -> impl IntoView {
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    let mut state: GameState = serde_json::from_str(r#"
    {
        "scale": "SM",
        "ships": {
            "usa": [
                {
                    "name": "Ship 1",
                    "ship_class": "Destroyer",
                    "country": "USA",
                    "dv": 0,
                    "max_speed": 10,
                    "status": 100,
                    "gun_status": 100,
                    "hull_status": 100,
                    "torpedo_tubesize": 0,
                    "n_tube": 0,
                    "n_torpedo": 0,
                    "primary_gun": {
                        "gun_size": 5,
                        "n_gun": 20,
                        "shell_weight": 55,
                        "range": 17500,
                        "srm": 75,
                        "ammo": 72000,
                        "fired": false
                    },
                    "secondary_gun": {
                        "gun_size": 5,
                        "n_gun": 20,
                        "shell_weight": 55,
                        "range": 17500,
                        "srm": 75,
                        "ammo": 72000,
                        "fired": false
                    },
                    "tertiary_gun": {
                        "gun_size": 5,
                        "n_gun": 20,
                        "shell_weight": 55,
                        "range": 17500,
                        "srm": 75,
                        "ammo": 72000,
                        "fired": false
                    },
                    "catapults": 0,
                    "aircraft_on_board": 0,
                    "aircraft_in_air": 0,
                    "depth_charge": 0,
                    "anti_aircraft": 0,
                    "mines": 0,
                    "turn_taken": false
                },
                {
                    "name": "Ship 2",
                    "ship_class": "Destroyer",
                    "country": "USA",
                    "dv": 0,
                    "max_speed": 10,
                    "status": 100,
                    "gun_status": 100,
                    "hull_status": 100,
                    "torpedo_tubesize": 0,
                    "n_tube": 0,
                    "n_torpedo": 0,
                    "primary_gun": {
                        "gun_size": 5,
                        "n_gun": 20,
                        "shell_weight": 55,
                        "range": 17500,
                        "srm": 75,
                        "ammo": 72000,
                        "fired": false
                    },
                    "secondary_gun": {
                        "gun_size": 5,
                        "n_gun": 20,
                        "shell_weight": 55,
                        "range": 17500,
                        "srm": 75,
                        "ammo": 72000,
                        "fired": false
                    },
                    "catapults": 0,
                    "aircraft_on_board": 0,
                    "aircraft_in_air": 0,
                    "depth_charge": 0,
                    "anti_aircraft": 0,
                    "mines": 0,
                    "turn_taken": false
                }
            ],
            "gb": [
                {
                    "name": "Ship 9",
                    "ship_class": "Destroyer",
                    "country": "GB",
                    "dv": 0,
                    "max_speed": 10,
                    "status": 100,
                    "gun_status": 100,
                    "hull_status": 100,
                    "torpedo_tubesize": 0,
                    "n_tube": 0,
                    "n_torpedo": 0,
                    "primary_gun": {
                        "gun_size": 5,
                        "n_gun": 20,
                        "shell_weight": 55,
                        "range": 17500,
                        "srm": 75,
                        "ammo": 90000,
                        "fired": false
                    },
                    "catapults": 0,
                    "aircraft_on_board": 0,
                    "aircraft_in_air": 0,
                    "depth_charge": 0,
                    "anti_aircraft": 0,
                    "mines": 0,
                    "turn_taken": false
                }
            ],
            "fr": [],
            "cn": [],
            "rus": [],
            "jpn": [
                {
                    "name": "Ship 8",
                    "ship_class": "Destroyer",
                    "country": "JPN",
                    "dv": 0,
                    "max_speed": 10,
                    "status": 100,
                    "gun_status": 100,
                    "hull_status": 100,
                    "torpedo_tubesize": 0,
                    "n_tube": 0,
                    "n_torpedo": 0,
                    "primary_gun": {
                        "gun_size": 5,
                        "n_gun": 20,
                        "shell_weight": 55,
                        "range": 17500,
                        "srm": 75,
                        "ammo": 72000,
                        "fired": false
                    },
                    "secondary_gun": {
                        "gun_size": 5,
                        "n_gun": 20,
                        "shell_weight": 55,
                        "range": 17500,
                        "srm": 75,
                        "ammo": 72000,
                        "fired": false
                    },
                    "tertiary_gun": {
                        "gun_size": 5,
                        "n_gun": 20,
                        "shell_weight": 55,
                        "range": 17500,
                        "srm": 75,
                        "ammo": 72000,
                        "fired": false
                    },
                    "catapults": 0,
                    "aircraft_on_board": 0,
                    "aircraft_in_air": 0,
                    "depth_charge": 0,
                    "anti_aircraft": 0,
                    "mines": 0,
                    "turn_taken": false
                }
            ],
            "it": [],
            "ger": []
        }
    }"#).unwrap();
    log::info!("Loaded default state");
    view! {
        <GameStateComp state=state />
    }
}
