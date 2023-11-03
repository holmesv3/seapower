import { append, is_empty } from "svelte/internal";
import * as gun from "./Gun";


export function attack(ship: Ship, range: number): void {
    let dmg = 1 / range;
    if (ship.primary_gun)
        gun.fire(ship.primary_gun);
    if (ship.secondary_gun)
        gun.fire(ship.secondary_gun);
    if (ship.tertiary_gun)
        gun.fire(ship.tertiary_gun);
    ship.turn_taken = true;
}

export function reset_ship(ship: Ship): void {
    if (ship.primary_gun)
        gun.reset(ship.primary_gun);
    if (ship.secondary_gun)
        gun.reset(ship.secondary_gun);
    if (ship.tertiary_gun)
        gun.reset(ship.tertiary_gun);
}

export function check_status(ship: Ship): boolean { 
    return ship.turn_taken;
}
export class Ship {
    constructor(
        public name: string,
        public ship_class: string,
        public country: string,
        public DV: number,
        public max_speed: number,
        public status: number,
        public gun_status: number,
        public hull_status: number,
        public torpedo_tubesize: number,
        public n_tube: number,
        public n_torpedo: number,
        public primary_gun: gun.Gun,
        public secondary_gun: gun.Gun,
        public tertiary_gun: gun.Gun,
        public catapults: number,
        public aircraft_on_board: number,
        public aircraft_in_air: number,
        public depth_charge: number,
        public anti_aircraft: number,
        public mines: number,
        public turn_taken: boolean = false,
    ) {}
}

// Possible countries
//      USA = "United States"
//      GB = "Great Britain"
//      FR = "France"
//      CN = "Canada"
//      RUS = "Russia"
//      JPN = "Japan"
//      IT = "Italy"
//      GER = "Germany"

// Possible ships types
//      DESTROYER = "Destroyer"
//      BATTLESHIP = "Battleship"
//      CRUISER = "Cruiser"
//      SUBMARINE = "Submarine"
//      CARRIER = "Carrier"

export interface Hack {
    [id: string]: Ship[];
}

export class ShipsByCountry {
    
    public USA: Ship[] = [];
    public GB: Ship[] = [];
    public FR: Ship[] = [];
    public CN: Ship[] = [];
    public RUS: Ship[] = [];
    public JPN: Ship[] = [];
    public IT: Ship[] = [];
    public GER: Ship[] = [];
    
    constructor(ships: Ship[]) {
        for (let index = 0; index < ships.length; index++) {
            let ship = ships[index];
            let country = ship.country;
            switch (country) {
                case 'USA':
                    this.USA.push(ship);
                    break;
                case 'GB':
                    this.GB.push(ship);
                    break;
                case 'FR':
                    this.FR.push(ship);
                    break;
                case 'CN':
                    this.CN.push(ship);
                    break;
                case 'RUS':
                    this.RUS.push(ship);
                    break;
                case 'JPN':
                    this.JPN.push(ship);
                    break;
                case 'IT':
                    this.IT.push(ship);
                    break;
                case 'GER':
                    this.GER.push(ship);
                    break;
                default:
                    console.log('Cannot find proper country for ', ship.name);
                    break;
            }
        }
    } 
}

export class GoodGuys {
    
    public countries: Hack = {};
    private _all_ships: Ship[] = [];
    
    constructor(ships: ShipsByCountry) {
        
        if (!is_empty(ships.USA)) {
            this.countries['USA'] = ships.USA;
            this._all_ships.concat(ships.USA);
        }
        if (!is_empty(ships.GB)) {
            this.countries['GB'] = ships.GB;
            this._all_ships.concat(ships.GB);
        }
        if (!is_empty(ships.FR)) {
            this.countries['FR'] = ships.FR;
            this._all_ships.concat(ships.FR);
        }
        if (!is_empty(ships.CN)) {
            this.countries['CN'] = ships.CN;
            this._all_ships.concat(ships.CN);
        }
        if (!is_empty(ships.RUS)) {
            this.countries['RUS'] = ships.RUS;
            this._all_ships.concat(ships.RUS);
        }
    }
    
    public get_all(): Ship[] {
        return this._all_ships;
    }
    
    public check_all(): Ship[] {
        let ret_ships: Ship[] = [];
        for (const ship of this._all_ships) {
            if (check_status(ship))
                ret_ships.push(ship);
        }
        return ret_ships;
    }
}

export class BadGuys {
    
    public countries: Hack = {};
    private _all_ships: Ship[] = [];
    
    constructor(ships: ShipsByCountry) {
        if (!is_empty(ships.JPN)) {
            this.countries['JPN'] = ships.JPN;
            this._all_ships.concat(ships.JPN);
        }
        if (!is_empty(ships.IT)) {
            this.countries['IT'] = ships.IT;
            this._all_ships.concat(ships.IT);
        }
        if (!is_empty(ships.GER)) {
            this.countries['GER'] = ships.GER;
            this._all_ships.concat(ships.GER);
        }
    }
    
    public get_all(): Ship[] {
        return this._all_ships;
    }
    
    public check_all(): Ship[] {
        let ret_ships: Ship[] = [];
        for (const ship of this._all_ships) {
            if (check_status(ship))
                ret_ships.push(ship);
        }
        return ret_ships;
    }
}

export class ShipsBySide {
    public good_guys: GoodGuys;
    public bad_guys: BadGuys;
    constructor(
        ships_by_country: ShipsByCountry
    ) {
        this.good_guys = new GoodGuys(ships_by_country);
        this.bad_guys = new BadGuys(ships_by_country);
    }
}