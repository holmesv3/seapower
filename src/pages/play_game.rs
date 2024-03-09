use leptos::*;
use leptos_meta::*;

use std::str::FromStr;
use std::collections::HashMap;

use crate::types::ship::*;
use crate::types::gun::*;
use crate::types::util::{GameScale, GameState, Country, Side};

#[component]
pub fn PlayGame() -> impl IntoView {
    let selected = use_context::<RwSignal<bool>>().unwrap();
    let state = use_context::<RwSignal<GameState>>().unwrap();
    let ships_by_side = move || ShipsBySide::from_ships_by_country(&state().ships.clone());
    view! {
        <div class="base-div text-center">
            <GoodBadTabs ships=ships_by_side()/>
        </div>
    }
}

#[component]
pub fn GoodBadTabs(ships: ShipsBySide) -> impl IntoView {
    const SIDES: [Side; 2] = [Side::ALLIES, Side::AXIS];
    let allies = ships.good_guys.clone();
    let axis = ships.bad_guys.clone();
    
    let active_ship = create_rw_signal::<Option<RwSignal<Ship>>>(None);
    let target_ship = create_rw_signal::<Option<RwSignal<Ship>>>(None);
    
    
    // Set up which side is being played
    let (active_side, set_active_side) = create_signal(SIDES[0]);
    let active_side_str = move ||  active_side().to_string();
    let active_side_tabs = move || {
        SIDES
            .into_iter()
            .map(|side| 
                view! {
                    <button
                        class="tabLinks"
                        class:active=move || {
                            active_ship.set(None);
                            target_ship.set(None);
                            active_side() == side
                        }

                        on:click=move |_| set_active_side(side)
                    >
                        {side.to_string()}
                    </button>
                }
            ).collect_view()
    };
    // Setup which country is being played from the side
    let (_ally_hash, _axis_hash) = ships.get_hash();
    let (_ally_keys, _axis_keys) = ships.get_keys();
    
    let (ally_hash, _) = create_signal(_ally_hash);
    let (ally_keys, _) = create_signal(_ally_keys);
    let (axis_hash, _) = create_signal(_axis_hash);
    let (axis_keys, _) = create_signal(_axis_keys);
    
    
    view! {
        <h1 class="h1">"Select active faction"</h1>
        <div class="tab m-2">{active_side_tabs}</div>
        <br/>
        <h1 class="h1">"Select active ship"</h1>
        <Show when=move || active_side() == Side::ALLIES>
            <ShipsByCountryTableTabs
                hashmap=ally_hash.clone()
                keys=ally_keys.clone()
                active_ship=active_ship
            />
        </Show>
        <Show when=move || active_side() == Side::AXIS>
            <ShipsByCountryTableTabs
                hashmap=axis_hash.clone()
                keys=axis_keys.clone()
                active_ship=active_ship
            />
        </Show>
        // Active ship
        <div class="mx-auto green">
            <Show when=move || active_ship().is_some()>
                <h1 class="h1">"Current active ship"</h1>
                <ShipCard ship=active_ship/>
            </Show>

        </div>
        <br/>
        // Select target ship
        <Show when=move || (active_side() == Side::ALLIES) && active_ship().is_some()>
            <h1 class="h1">"Select target ship"</h1>
            <ShipsByCountryTableTabs
                hashmap=axis_hash.clone()
                keys=axis_keys.clone()
                active_ship=target_ship
            />
        </Show>
        <Show when=move || (active_side() == Side::AXIS) && active_ship().is_some()>
            <ShipsByCountryTableTabs
                hashmap=ally_hash.clone()
                keys=ally_keys.clone()
                active_ship=target_ship
            />
        </Show>

        // Show target ship
        <div class="mx-auto purple">
            <Show when=move || target_ship().is_some()>
                <h1 class="h1">"Current target ship"</h1>
                <ShipCard ship=target_ship/>
            </Show>

        </div>
        <br/>
        <Show when=move || active_ship().is_some() && target_ship().is_some()>
            <h1 class="h1">"Attack the target ship with the current active ship"</h1>
            <AttackButton a=active_ship t=target_ship/>
        </Show>
    }   
}

#[component]
fn ShipsByCountryTableTabs(
    hashmap: ReadSignal<HashMap::<Country, Vec<RwSignal<Ship>>>>,
    keys: ReadSignal<Vec<Country>>,
    active_ship: RwSignal<Option<RwSignal<Ship>>>
) -> impl IntoView {
    
    let init_active = keys()[0];
    let (get_active, set_active) = create_signal(init_active);
    let active_str = move || get_active().to_string();
    let tabs = keys()
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
).collect_view();

let table = move || {
    let ships = &hashmap()[&get_active()];
    play_ships_table(ships.clone(), active_ship)};
    view! {
        <div class="tab">{tabs}</div>
        <div class="tabContent active">
            <div class="text-center">{active_str} Ships {table} <br/></div>
        </div>
    }
}
fn play_ships_table(ships: Vec<RwSignal<Ship>>, active_ship: RwSignal<Option<RwSignal<Ship>>>) -> impl IntoView {
    let table_for_ships = ships
    .into_iter()
    .map(|ship| {
        view! {
            <tr
                class:unavail=move || ship().turn_taken
                on:click=move |_| {
                    active_ship.set(None);
                    active_ship.set(Some(ship));
                }
            >

                <td>{move || ship().name.clone()}</td>
                <td>{move || ship().status}</td>
                <td>{move || ship().turn_taken}</td>
            </tr>
        }
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

#[component]
fn ShipCard(ship: RwSignal<Option<RwSignal<Ship>>>) -> impl IntoView {
    let ship = move || ship().unwrap()();
    view! {
        <div class="card p-4 m-2">{ship().name}</div>

        <div class="card p-4 m-6 shadow-xl" data-popup=ship().name>
            <div class="arrow bg-surface-100-800-token"></div>
            <div class="grid grid-cols-3 gap-2 m-1">
                <p>ship_class: {ship().ship_class}</p>
                <p>DV: {ship().dv}</p>
                <p>max_speed: {ship().max_speed}</p>
                <p>status: {ship().status}</p>
                <p>gun_status: {ship().gun_status}</p>
                <p>hull_status: {ship().hull_status}</p>
                <p>torpedo_tubesize: {ship().torpedo_tubesize}</p>
                <p>n_tube: {ship().n_tube}</p>
                <p>n_torpedo: {ship().n_torpedo}</p>
                <p>catapults: {ship().catapults}</p>
                <p>aircraft_on_board: {ship().aircraft_on_board}</p>
                <p>aircraft_in_air: {ship().aircraft_in_air}</p>
                <p>depth_charge: {ship().depth_charge}</p>
                <p>anti_aircraft: {ship().anti_aircraft}</p>
                <p>mines: {ship().mines}</p>
            </div>

            <section class="grid grid-cols-3 gap-4 m-1">
                <div class="card border-rounded">
                    <header class="card-header">Primary Gun</header>
                    <hr/>
                    <GunCard gun={ship().primary_gun}/>
                    <div class="p-4"></div>
                </div>
                <Show when=move || ship().secondary_gun.is_some()>
                    <div class="card">
                        <header class="card-header">Secondary Gun</header>
                        <hr/>
                        <GunCard gun={ship().secondary_gun.unwrap()}/>
                        <div class="p-4"></div>
                    </div>
                </Show>
                <Show when=move || ship().tertiary_gun.is_some()>
                    <div class="card">
                        <header class="card-header">Tertiary Gun</header>
                        <hr/>
                        <GunCard gun={ship().tertiary_gun.unwrap()}/>
                        <div class="p-4"></div>
                    </div>
                </Show>
            </section>
        </div>
    }
}
#[component]
fn GunCard(gun: Gun) -> impl IntoView {
    view!(
        <section class="grid grid-cols-1 md:grid-cols-3 gap-1">
            <p>gun_size: {gun.gun_size}</p>
            <p>n_gun: {gun.n_gun}</p>
            <p>shell_weight: {gun.shell_weight}</p>
            <p>range: {gun.range}</p>
            <p>SRM: {gun.srm}</p>
            <p>ammo: {gun.ammo}</p>
        </section>
    )
}

#[component]
fn AttackButton(a: RwSignal<Option<RwSignal<Ship>>>, t: RwSignal<Option<RwSignal<Ship>>>) -> impl IntoView {
   
    let l_a = a().unwrap();
    let l_t = t().unwrap();
    
    let unavail = move || l_a().turn_taken;
    
    let unavail_btn = move || view! { <button class="btn gray">{l_a().name} already fired</button> };
    
    let fire_button = move || view! {
        <button
            class="btn red"
            on:click=move |_| {
                log::info!("HERE I AM");
                log::info!("T: {}", l_t.with(| s | s.status));
                log::info!("A: {}", l_a.with(| s | s.turn_taken));
                log::info!("UNAVIL: {}", unavail());
                l_t.update(|s| s.status -= 1.0);
                l_a.update(|s| s.turn_taken = true);
                log::info!("T: {}", l_t.with(| s | s.status));
                log::info!("A: {}", l_a.with(| s | s.turn_taken));
                log::info!("UNAVIL: {}", unavail());
                t.set(None);
            }
        >
            "FIRE"
        </button>
    };
    
    
    view! {
        <div class="m-4">
            <Show when=move || !unavail() fallback=unavail_btn>
                {fire_button()}
            </Show>
        </div>
    }
        
}