use leptos::*;
use leptos_meta::*;

use crate::types::ship::{Ship, ShipsByCountry};
use crate::types::util::{GameScale, GameState};

#[component]
pub fn InitGame() -> impl IntoView {
    let selected = use_context::<ReadSignal<bool>>().unwrap();
    let state = use_context::<ReadSignal<GameState>>().unwrap();
    
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Seapower - Start Game"/>
        <div class="base-div mx-auto p-8 space-y-8">
            <h1 class="h1 text-center">"Load default or upload save game"</h1>
            <div class="mx-auto text-center">
                <button class="m-4 btn-green" on:click=move |_| { load_default() }>
                    Load Default
                </button>
                <button
                    class="m-4 btn-purple"
                >
                    Upload Save
                </button>
            </div>
            <Show when={move || selected.get() == true}
                fallback=|| view!{ "REEEEEEEEEEEEEEE" }>
                    <GameStateComp state={state}/>
            </Show>
        </div>
    }
}

pub fn load_default() {
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    let new_state = serde_json::from_str(
        r#"
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
            ]
        }
    }"#,
    )
    .unwrap();
    let set_state =
        use_context::<WriteSignal<GameState>>().unwrap();
    set_state(new_state);
    let set_selected = use_context::<WriteSignal<bool>>().unwrap();
    set_selected(true);
    
    log::info!("Loaded default state");
}
#[component]
pub fn LoadSave() -> impl IntoView {
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    log::warn!("Haven't implemented loading from save yet");
    view! { <p>"Haven't implemented loading from save yet"</p> }
}

#[component]
pub fn ShipsByCountryTableComp(table_map: std::collections::HashMap::<String, Vec<Ship>>) -> impl IntoView {
    let keys: Vec<&String> = table_map.keys().collect();
    let selected = keys[0];
    let ships = table_map[selected].clone();
    
    fn table_row(ship: Ship) -> impl IntoView {
        view! {
            <tr>
                <td>{ship.name}</td>
                <td>{ship.status}</td>
                <td>{ship.turn_taken}</td>
            </tr>
        }
    }

    view! {
        <div class="text-center">
            {selected} Ships 
            <table class="mx-auto">
                <tr>
                    <th>Name</th>
                    <th>Health</th>
                    <th>Turn Taken</th>
                </tr>
            {ships.into_iter()
            .map(|ship| table_row(ship))
            .collect_view()
            }
            </table>
        </div>
    }
}

#[component]
pub fn GameStateComp(state: ReadSignal<GameState>) -> impl IntoView {
    let ships = state().ships.clone();
    let mut hash = std::collections::HashMap::new();
    if ships.usa.is_some() {
        hash.insert("USA".to_string(), ships.usa.unwrap().clone());
    }
    if ships.gb.is_some() {
        hash.insert("GB".to_string(), ships.gb.unwrap().clone());
    }
    if ships.fr.is_some() {
        hash.insert("FR".to_string(), ships.fr.unwrap().clone());
    }
    if ships.cn.is_some() {
        hash.insert("CN".to_string(), ships.cn.unwrap().clone());
    }
    if ships.rus.is_some() {
        hash.insert("RUS".to_string(), ships.rus.unwrap().clone());
    }
    if ships.jpn.is_some() {
        hash.insert("JPN".to_string(), ships.jpn.unwrap().clone());
    }
    if ships.it.is_some() {
        hash.insert("IT".to_string(), ships.it.unwrap().clone());
    }
    if ships.ger.is_some() {
        hash.insert("GER".to_string(), ships.ger.unwrap().clone());
    }   
    view! {
        <div class="p-4 m-8 text-center mx-auto">
            <GameScaleComp scale=state().scale/>
            <div>
                <ShipsByCountryTableComp table_map={hash}/>
            </div>
        </div>
    }
}

#[component]
pub fn GameScaleComp(scale: GameScale) -> impl IntoView {
    let string = match scale {
        GameScale::UNSET => "Unset",
        GameScale::SM => "Small",
        GameScale::MD => "Medium",
        GameScale::LG => "Large",
    };
    view! { <h2 class="mx-auto">Game Scale: {string}</h2> }
}
