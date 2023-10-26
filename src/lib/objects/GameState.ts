import {ShipsByCountry} from "./Misc";
import type { Writable } from 'svelte/store';
import { localStorageStore } from '@skeletonlabs/skeleton';
export enum GameScale {
    SM,
    MD,
    LG
}
export class GameState {
    constructor(
        public scale: GameScale,
        public ships: ShipsByCountry, 
    ) {}
}
const _def = new GameState(GameScale.MD, new ShipsByCountry([]));
export const gameStateStore: Writable<GameState> = localStorageStore('game_state', _def);
