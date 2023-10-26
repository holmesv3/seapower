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

import type { Writable } from 'svelte/store';
import { TreeView, localStorageStore } from '@skeletonlabs/skeleton';


export interface Hack {
    [id: string]: Ship[];
}

import type { Ship } from "$lib/objects/Ship"

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


export interface ShipsByCountryType {
    [id: string]: Ship[];
}
