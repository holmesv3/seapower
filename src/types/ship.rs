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


// class Side:

//     _all: list[Ship]

//     @staticmethod
//     def from_ships(ships: list[Ship]):
//         raise NotImplementedError

//     @staticmethod
//     def from_ships_by_country(ships: ShipsByCountry):
//         raise NotImplementedError

//     def get_all(self) -> list[Ship]:
//         return self._all;

//     def check_all(self) -> list[Ship]:
//         """Return all ships which have actions remaning"""
//         out = []
//         ships = self.get_all()
//         for ship in ships:
//             if ship.status:
//                 out.append(ship)

//         return out

// class GoodGuys(Side):
//     usa: list[Ship] = []
//     gb: list[Ship] = []
//     fr: list[Ship] = []
//     cn: list[Ship] = []
//     rus: list[Ship] = []

//     @staticmethod
//     def from_ships(ships: list[Ship]):
//         gg = GoodGuys()
//         for ship in ships:
//             match ship.country:
//                 case Country.USA:
//                     gg.usa.append(ship)
//                 case Country.GB:
//                     gg.gb.append(ship)
//                 case Country.FR:
//                     gg.fr.append(ship)
//                 case Country.CN:
//                     gg.cn.append(ship)
//                 case Country.RUS:
//                     gg.rus.append(ship)
//         gg._all = gg.usa + gg.gb + gg.fr + gg.cn + gg.rus
//         return gg

//     @staticmethod
//     def from_ships_by_country(ships: ShipsByCountry):
//         gg = GoodGuys()
//         gg.usa = ships.usa
//         gg.gb = ships.gb
//         gg.fr = ships.fr
//         gg.cn = ships.cn
//         gg.rus = ships.rus
//         gg._all = gg.usa + gg.gb + gg.fr + gg.cn + gg.rus
//         return gg

// class BadGuys(Side):
//     jpn: list[Ship] = []
//     it: list[Ship] = []
//     ger: list[Ship] = []

//     @staticmethod
//     def from_ships(ships: list[Ship]):
//         bg = BadGuys()
//         for ship in ships:
//             match ship.country:
//                 case Country.JPN:
//                     bg.jpn.append(ship)
//                 case Country.IT:
//                     bg.it.append(ship)
//                 case Country.GER:
//                     bg.ger.append(ship)
//         bg._all = bg.jpn + bg.it + bg.ger
//         return bg

//     @staticmethod
//     def from_ships_by_country(ships: ShipsByCountry):
//         bg = BadGuys()
//         bg.jpn = ships.jpn
//         bg.it = ships.it
//         bg.ger = ships.ger
//         bg._all = bg.jpn + bg.it + bg.ger
//         return bg

// class ShipsBySide:
//     good_guys: GoodGuys
//     bad_guys: BadGuys

//     @staticmethod
//     def from_ships(ships: list[Ship]):
//         sbs = ShipsBySide()
//         sbs.good_guys = GoodGuys(ships)
//         sbs.bad_guys = BadGuys(ships)
//         return sbs

//     @staticmethod
//     def from_ships_by_country(ships: ShipsByCountry):
//         sbs = ShipsBySide()
//         sbs.good_guys = GoodGuys(ships)
//         sbs.bad_guys = BadGuys(ships)
//         return sbs
