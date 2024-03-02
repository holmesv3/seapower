use leptos::*;
use leptos_meta::*;

use std::str::FromStr;
use std::collections::HashMap;

use crate::types::ship::*;
use crate::types::util::{GameScale, GameState, Country, Side};
use crate::types::default_state::DEFAULT;

#[component]
pub fn PlayGame() -> impl IntoView {
    let selected = use_context::<RwSignal<bool>>().unwrap();
    let state = use_context::<RwSignal<GameState>>().unwrap();
    let ships_by_side = move || ShipsBySide::from_ships_by_country(&state().ships);
    view!(<div class="base-div text-center">"P deez nuts"</div>)
}

#[component]
pub fn GoodBadTabs() {
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
            <div class="text-center">
                {active_str} Ships 
                {table}
                <br/>
            </div>
        </div>
    }
}

pub fn init_ships_table(ships: &Vec<Ship>) -> impl IntoView {
    let table_for_ships = ships
            .iter()
            .map(|ship| view!{
                <tr>
                    <td>{ship.name.clone()}</td>
                    <td>{ship.status}</td>
                    <td>{ship.turn_taken}</td>
                </tr>
            })
            .collect_view();
    view!{
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