use leptos::*;
use leptos_meta::*;

use std::str::FromStr;
use std::collections::HashMap;

use crate::types::ship::{Ship, ShipsByCountry};
use crate::types::util::{GameScale, GameState, Country};
use crate::types::default_state::DEFAULT;

#[component]
pub fn InitGame() -> impl IntoView {
    let selected = use_context::<RwSignal<bool>>().unwrap();
    let state = use_context::<RwSignal<GameState>>().unwrap();
    
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Seapower - Start Game"/>
        <div class="base-div mx-auto p-8 space-y-8">
            <h1 class="h1 text-center">"Load default or upload save game"</h1>
            <div class="mx-auto text-center">
                <button class="m-4 btn-green" on:click=move |_| { load_default() }>
                    "Load Default"
                </button>
                <button class="m-4 btn-purple" on:click=move |_| { load_save() }>
                    "Upload Save"
                </button>
            </div>
            <Show when=move || selected.get() == true>
                <a class="btn-orange" href="/play_game">
                    "Continue"
                </a>
                <GameStateComp state=state/>
                </Show>
                </div>
            }
        }
        
pub fn load_default() {
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    let set_state = use_context::<RwSignal<GameState>>().unwrap();
    let set_selected = use_context::<RwSignal<bool>>().unwrap();
    set_selected.set(false);
    let new_state = serde_json::from_str(DEFAULT).unwrap();
    set_state.set(new_state);
    set_selected.set(true);
}

pub fn load_save() {
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    log::warn!("Haven't implemented loading from save yet");
    let set_state = use_context::<RwSignal<GameState>>().unwrap();
    let set_selected = use_context::<RwSignal<bool>>().unwrap();
    set_selected.set(false);
}


#[component]
pub fn GameStateComp(state: RwSignal<GameState>) -> impl IntoView {

    let get_map = create_memo(move |_| state.get().ships.get_hash());
    let present_ships = create_memo(move |_| state.get().ships.get_keys());
    
    view! {
        <div class="p-4 m-8 text-center mx-auto">
            <GameScaleComp scale=state().scale/>
            <div>
                // <ShipsByCountryTableComp table_map={hash.clone()}/>
                <ShipsByCountryTableTabs get_table=get_map get_keys=present_ships/>
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

#[component]
pub fn ShipsByCountryTableComp(table_map: HashMap::<Country, Vec<Ship>>) -> impl IntoView {
    table_map.iter().map(|(k, v)| {
        view! {
            <div class="text-center">
                {k.to_string()} Ships <table class="mx-auto">
                    <tr>
                        <th>Name</th>
                        <th>Health</th>
                        <th>Turn Taken</th>
                    </tr>

                    {v.clone().into_iter().map(|ship| ship_table_row(ship)).collect_view()}

                </table> <br/>
            </div>
        }
    }).collect_view()
    
}

#[component]
pub fn ShipsByCountryTableTabs(
    get_table: Memo<HashMap::<Country, Vec<Ship>>>,
    get_keys: Memo<Vec<Country>>
) -> impl IntoView {
        
    let init_active = move || get_keys()[0];
    let (get_active, set_active) = create_signal(init_active());
    let active_str = move || get_active().to_string();
    let tabs = move || {
        get_keys()
            .into_iter()
            .map(|country| {
                view! {
                    <button
                        class="tabLinks"
                        class:active=move || get_active() == country
                        on:click=move |_| set_active(country)
                    >
                        {country.to_string()}
                    </button>
                }
            }
            ).collect_view()
    };
    
    let table = move || {
        get_table()[&get_active()]
            .iter()
            .map(|ship| ship_table_row(ship.clone()))
            .collect_view()
    };
    view! {
        <div class="tab">{tabs}</div>
        <div class="tabContent active">
            <div class="text-center">
                {active_str} Ships <table class="mx-auto">
                    <tr>
                        <th>Name</th>
                        <th>Health</th>
                        <th>Turn Taken</th>
                    </tr>
                    {table}
                </table> <br/>
            </div>
        </div>
    }
    
}

pub fn ship_table_row(ship: Ship) -> impl IntoView {
    view! {
        <tr>
            <td>{ship.name}</td>
            <td>{ship.status}</td>
            <td>{ship.turn_taken}</td>
        </tr>
    }
}

