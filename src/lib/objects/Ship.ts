import type { Gun } from "./Gun";

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
        public primary_gun: Gun,
        public secondary_gun: Gun,
        public tertiary_gun: Gun,
        public catapults: number,
        public aircraft_on_board: number,
        public aircraft_in_air: number,
        public depth_charge: number,
        public anti_aircraft: number,
        public mines: number,
        public turn_taken: boolean = false,
    ) {}

    public static deserialize(str: string): Ship {
        const ship: ShipType = JSON.parse(str);
        return new Ship(
            ship.name,
            ship.ship_class,
            ship.country,
            ship.DV,
            ship.max_speed,
            ship.status,
            ship.gun_status,
            ship.hull_status,
            ship.torpedo_tubesize,
            ship.n_tube,
            ship.n_torpedo,
            ship.primary_gun,
            ship.secondary_gun,
            ship.tertiary_gun,
            ship.catapults,
            ship.aircraft_on_board,
            ship.aircraft_in_air,
            ship.depth_charge,
            ship.anti_aircraft,
            ship.mines,
            ship.turn_taken,
        )
    }
    public static fromInterface(int: ShipType): Ship {
        return new Ship(
            int.name,
            int.ship_class,
            int.country,
            int.DV,
            int.max_speed,
            int.status,
            int.gun_status,
            int.hull_status,
            int.torpedo_tubesize,
            int.n_tube,
            int.n_torpedo,
            int.primary_gun,
            int.secondary_gun,
            int.tertiary_gun,
            int.catapults,
            int.aircraft_on_board,
            int.aircraft_in_air,
            int.depth_charge,
            int.anti_aircraft,
            int.mines,
            int.turn_taken,
        )
    }
    public toInterface(): ShipType {
        return {
            name: this.name,
            ship_class: this.ship_class,
            country: this.country,
            DV: this.DV,
            max_speed: this.max_speed,
            status: this.status,
            gun_status: this.gun_status,
            hull_status: this.hull_status,
            torpedo_tubesize: this.torpedo_tubesize,
            n_tube: this.n_tube,
            n_torpedo: this.n_torpedo,
            primary_gun: this.primary_gun,
            secondary_gun: this.secondary_gun,
            tertiary_gun: this.tertiary_gun,
            catapults: this.catapults,
            aircraft_on_board: this.aircraft_on_board,
            aircraft_in_air: this.aircraft_in_air,
            depth_charge: this.depth_charge,
            anti_aircraft: this.anti_aircraft,
            mines: this.mines,
            turn_taken: this.turn_taken,
        }
    }
    public attack(range: number, target: Ship) {
        let dmg = 1 / range;
        target.status -= dmg;
    }
}

export interface ShipType {
    name: string,
    ship_class: string;
    country: string;
    DV: number;
    max_speed: number;
    status: number;
    gun_status: number;
    hull_status: number;
    torpedo_tubesize: number;
    n_tube: number;
    n_torpedo: number;
    primary_gun: Gun;
    secondary_gun: Gun;
    tertiary_gun: Gun;
    catapults: number;
    aircraft_on_board: number;
    aircraft_in_air: number;
    depth_charge: number;
    anti_aircraft: number;
    mines: number;
    turn_taken: boolean;
}

