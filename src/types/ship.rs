use leptos::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use crate::types::gun::Gun;
use crate::types::util::Country;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct Ship {
    pub name: String,
    pub ship_class: String,
    pub country: Country,
    pub dv: f32,
    pub max_speed: f32,
    pub status: f32,
    pub gun_status: f32,
    pub hull_status: f32,
    pub torpedo_tubesize: f32,
    pub n_tube: f32,
    pub n_torpedo: f32,
    pub primary_gun: Gun,
    pub secondary_gun: Option<Gun>,
    pub tertiary_gun: Option<Gun>,
    pub catapults: f32,
    pub aircraft_on_board: f32,
    pub aircraft_in_air: f32,
    pub depth_charge: f32,
    pub anti_aircraft: f32,
    pub mines: f32,
    pub turn_taken: bool,
}

impl Ship {
    pub fn attack(&mut self, range: f32) {
        let _ = 1. / range;
        if !self.primary_gun.is_fired() {
            self.primary_gun.fire();
        }
        if self.secondary_gun.is_some() {
            self.secondary_gun.as_mut().unwrap().fire();
        }
        if self.tertiary_gun.is_some() {
            self.tertiary_gun.as_mut().unwrap().fire();
        }
        self.turn_taken = true;
    }

    pub fn reset(&mut self) {
        self.primary_gun.reset();
        if self.secondary_gun.is_some() {
            self.secondary_gun.as_mut().unwrap().reset();
        }
        if self.tertiary_gun.is_some() {
            self.tertiary_gun.as_mut().unwrap().reset();
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ShipsByCountry {
    pub usa: Option<Vec<Ship>>,
    pub gb: Option<Vec<Ship>>,
    pub fr: Option<Vec<Ship>>,
    pub cn: Option<Vec<Ship>>,
    pub rus: Option<Vec<Ship>>,
    pub jpn: Option<Vec<Ship>>,
    pub it: Option<Vec<Ship>>,
    pub ger: Option<Vec<Ship>>,
}

impl ShipsByCountry {
    pub fn get_hash(&self) -> HashMap<Country, Vec<Ship>> {
        let mut hash = HashMap::new();
        if self.usa.is_some() {
            hash.insert(Country::USA, self.usa.as_ref().unwrap().clone());
        }
        if self.gb.is_some() {
            hash.insert(Country::GB, self.gb.as_ref().unwrap().clone());
        }
        if self.fr.is_some() {
            hash.insert(Country::FR, self.fr.as_ref().unwrap().clone());
        }
        if self.cn.is_some() {
            hash.insert(Country::CN, self.cn.as_ref().unwrap().clone());
        }
        if self.rus.is_some() {
            hash.insert(Country::RUS, self.rus.as_ref().unwrap().clone());
        }
        if self.jpn.is_some() {
            hash.insert(Country::JPN, self.jpn.as_ref().unwrap().clone());
        }
        if self.it.is_some() {
            hash.insert(Country::IT, self.it.as_ref().unwrap().clone());
        }
        if self.ger.is_some() {
            hash.insert(Country::GER, self.ger.as_ref().unwrap().clone());
        }
        hash
    }
    pub fn get_keys(&self) -> Vec<Country> {
        let mut present_ships = vec![];
        if self.usa.is_some() {
            present_ships.push(Country::USA);
        }
        if self.gb.is_some() {
            present_ships.push(Country::GB);
        }
        if self.fr.is_some() {
            present_ships.push(Country::FR);
        }
        if self.cn.is_some() {
            present_ships.push(Country::CN);
        }
        if self.rus.is_some() {
            present_ships.push(Country::RUS);
        }
        if self.jpn.is_some() {
            present_ships.push(Country::JPN);
        }
        if self.it.is_some() {
            present_ships.push(Country::IT);
        }
        if self.ger.is_some() {
            present_ships.push(Country::GER);
        }
        present_ships
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GoodGuys {   
    pub usa: Option<Vec<RwSignal<Ship>>>,
    pub gb: Option<Vec<RwSignal<Ship>>>,
    pub fr: Option<Vec<RwSignal<Ship>>>,
    pub cn: Option<Vec<RwSignal<Ship>>>,
    pub rus: Option<Vec<RwSignal<Ship>>>,
}
impl GoodGuys {
    pub fn from_ships_by_country(ships: &ShipsByCountry) -> Self {
        Self {
            usa: make_vec_ship_sig(ships.usa.clone()),
            gb: make_vec_ship_sig(ships.gb.clone()),
            fr: make_vec_ship_sig(ships.fr.clone()),
            cn: make_vec_ship_sig(ships.cn.clone()),
            rus: make_vec_ship_sig(ships.rus.clone()),
        }
    }
    pub fn get_hash(&self) -> HashMap<Country, Vec<RwSignal<Ship>>> {
        let mut good_hash = HashMap::new();
        if self.usa.is_some() {
            good_hash.insert(Country::USA, self.usa.as_ref().unwrap().clone());
        }
        if self.gb.is_some() {
            good_hash.insert(Country::GB, self.gb.as_ref().unwrap().clone());
        }
        if self.fr.is_some() {
            good_hash.insert(Country::FR, self.fr.as_ref().unwrap().clone());
        }
        if self.cn.is_some() {
            good_hash.insert(Country::CN, self.cn.as_ref().unwrap().clone());
        }
        if self.rus.is_some() {
            good_hash.insert(Country::RUS, self.rus.as_ref().unwrap().clone());
        }
        good_hash
    }
    pub fn get_keys(&self) -> Vec<Country> {
        let mut good_present_ships = vec![];
        if self.usa.is_some() {
            good_present_ships.push(Country::USA);
        }
        if self.gb.is_some() {
            good_present_ships.push(Country::GB);
        }
        if self.fr.is_some() {
            good_present_ships.push(Country::FR);
        }
        if self.cn.is_some() {
            good_present_ships.push(Country::CN);
        }
        if self.rus.is_some() {
            good_present_ships.push(Country::RUS);
        }
        good_present_ships
    }
    pub fn get_all_ships(&self) -> Vec<RwSignal<Ship>> {
        let mut good_present_ships = vec![];
        if self.usa.is_some() {
            for ship in self.usa.as_ref().unwrap().iter()
            {
                good_present_ships.push(ship.clone());
            };
        }
        if self.gb.is_some() {
            for ship in self.gb.as_ref().unwrap().iter()
            {
                good_present_ships.push(ship.clone());
            };
        }
        if self.fr.is_some() {
            for ship in self.fr.as_ref().unwrap().iter()
            {
                good_present_ships.push(ship.clone());
            };
        }
        if self.cn.is_some() {
            for ship in self.cn.as_ref().unwrap().iter()
            {
                good_present_ships.push(ship.clone());
            };
        }
        if self.rus.is_some() {
            for ship in self.rus.as_ref().unwrap().iter()
            {
                good_present_ships.push(ship.clone());
            };
        }
        good_present_ships
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BadGuys{
    pub jpn: Option<Vec<RwSignal<Ship>>>,
    pub it: Option<Vec<RwSignal<Ship>>>,
    pub ger: Option<Vec<RwSignal<Ship>>>,
}

fn make_vec_ship_sig(ships: Option<Vec<Ship>>) -> Option<Vec<RwSignal<Ship>>> {
    if ships.is_some()
    {
        return Some(ships.unwrap().into_iter().map(|ship| create_rw_signal(ship)).collect())
    }
    None
}

impl BadGuys {
    
    pub fn from_ships_by_country(ships: &ShipsByCountry) -> Self {
        Self {
            jpn: make_vec_ship_sig(ships.jpn.clone()),
            it: make_vec_ship_sig(ships.it.clone()),
            ger: make_vec_ship_sig(ships.ger.clone()),
        }    
    }
    pub fn get_hash(&self) -> HashMap<Country, Vec<RwSignal<Ship>>> {
        let mut bad_hash = HashMap::new();
        if self.jpn.is_some() {
            bad_hash.insert(Country::JPN, self.jpn.as_ref().unwrap().clone());
        }
        if self.it.is_some() {
            bad_hash.insert(Country::IT, self.it.as_ref().unwrap().clone());
        }
        if self.ger.is_some() {
            bad_hash.insert(Country::GER, self.ger.as_ref().unwrap().clone());
        }
        bad_hash
    }
    pub fn get_keys(&self) -> Vec<Country> {
        let mut bad_present_ships = vec![];
        if self.jpn.is_some() {
            bad_present_ships.push(Country::JPN);
        }
        if self.it.is_some() {
            bad_present_ships.push(Country::IT);
        }
        if self.ger.is_some() {
            bad_present_ships.push(Country::GER);
        }
        bad_present_ships
    }
    pub fn get_all_ships(&self) -> Vec<RwSignal<Ship>> {
        let mut bad_present_ships = vec![];
        if self.jpn.is_some() {
            for ship in self.jpn.as_ref().unwrap().iter()
            {
                bad_present_ships.push(ship.clone());
            }
        }
        if self.it.is_some() {
            for ship in self.it.as_ref().unwrap().iter()
            {
                bad_present_ships.push(ship.clone());
            }
        }
        if self.ger.is_some() {
            for ship in self.ger.as_ref().unwrap().iter()
            {
                bad_present_ships.push(ship.clone());
            }
        }
        bad_present_ships
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShipsBySide {
    pub good_guys: GoodGuys,
    pub bad_guys: BadGuys,
}

impl ShipsBySide {
    pub fn from_ships_by_country(ships: &ShipsByCountry) -> Self {
        Self {
            good_guys: GoodGuys::from_ships_by_country(ships),
            bad_guys: BadGuys::from_ships_by_country(ships),
        }
    }
    
    pub fn get_hash(&self) -> (HashMap<Country, Vec<RwSignal<Ship>>>, HashMap<Country, Vec<RwSignal<Ship>>>) {
        (self.good_guys.get_hash(), self.bad_guys.get_hash())
    }
    pub fn get_keys(&self) -> (Vec<Country>, Vec<Country>) {
        (self.good_guys.get_keys(), self.bad_guys.get_keys())
    }
}
