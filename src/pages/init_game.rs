use leptos::*;
use leptos_meta::*;

use std::str::FromStr;
use std::collections::HashMap;

use crate::types::ship::{Ship, ShipsByCountry};
use crate::types::util::{GameScale, GameState, Country};
use crate::types::default_state::DEFAULT;
use crate::components::buttons::*;

#[component]
pub fn InitGame() -> impl IntoView {
    let selected = use_context::<RwSignal<bool>>().unwrap();
    let state = use_context::<RwSignal<GameState>>().unwrap();
    
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Seapower - Start Game"/>
        <div class="text-center mx-auto p-4 space-y-4">
            <h1 class="h1">"Load default or upload save game"</h1>
            <div class="mx-auto grid grid-cols-2">
                <div class="m-4 mx-auto" on:click=move |_| { load_default() }>
                    <GreenBtn>"Load Default"</GreenBtn>
                </div>
                <div class="m-4 mx-auto" on:click=move |_| { load_save() }>
                    <PurpleBtn>"Upload Save"</PurpleBtn>
                </div>
            </div>
        </div>
        <div class="text-center mx-auto p-4 space-y-4">
            <Show when=move || selected.get() == true>
                <a class="mx-auto" href="/play_game">
                    <OrangeBtn>"Continue"</OrangeBtn>
                </a>
                <GameStateComp state=state/>
            </Show>
        </div>
    }
}
        
fn load_default() {
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    let set_state = use_context::<RwSignal<GameState>>().unwrap();
    let set_selected = use_context::<RwSignal<bool>>().unwrap();
    set_selected.set(false);
    let new_state = serde_json::from_str(DEFAULT).unwrap();
    set_state.set(new_state);
    set_selected.set(true);
}

fn load_save() {
    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    log::warn!("Haven't implemented loading from save yet");
    // let set_state = use_context::<RwSignal<GameState>>().unwrap();
    // let set_selected = use_context::<RwSignal<bool>>().unwrap();
    // set_selected.set(false);
}


#[component]
fn GameStateComp(state: RwSignal<GameState>) -> impl IntoView {

    let get_map = create_memo(move |_| state.get().ships.get_hash());
    let present_ships = create_memo(move |_| state.get().ships.get_keys());
    
    view! {
        <div class="p-4 m-8 text-center mx-auto">
            <GameScaleComp scale=state().scale/>
            <div>
                <ShipsByCountryTableTabs get_table=get_map get_keys=present_ships/>
            </div>
        </div>
    }
}

#[component]
fn GameScaleComp(scale: GameScale) -> impl IntoView {
    let string = match scale {
        GameScale::UNSET => "Unset",
        GameScale::SM => "Small",
        GameScale::MD => "Medium",
        GameScale::LG => "Large",
    };
    view! { <h2 class="mx-auto">Game Scale: {string}</h2> }
}

#[component]
fn ShipsByCountryTableTabs(
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
        let ships = &get_table()[&get_active()];
        init_ships_table(ships)};
    view! {
        <div class="tab">{tabs}</div>
        <div class="tabContent active">
            <div class="text-center">{active_str} Ships {table} <br/></div>
        </div>
    }
}

fn init_ships_table(ships: &Vec<Ship>) -> impl IntoView {
    let table_for_ships = ships
            .iter()
            .map(|ship| view! {
                <tr>
                    <td>{ship.name.clone()}</td>
                    <td>{ship.status}</td>
                    <td>{ship.turn_taken}</td>
                </tr>
            })
            .collect_view();
    view! {
        <table class="mx-auto">
            <tr>
                <th>Name</th>
                <th>Health</th>
                <th>Turn Taken</th>
            </tr>
            {table_for_ships}
        </table>
    } 
}


