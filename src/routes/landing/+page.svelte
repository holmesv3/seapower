<script lang='ts'>
    import { get } from "svelte/store"
    import  { type GameState, gameStateStore } from "$lib/objects/GameState";
    import type { Ship, Hack, ShipsByCountry} from "$lib/objects/Ship";
    import ShipCard from "$lib/components/ship/ShipCard.svelte";
	import { is_empty } from "svelte/internal";
    import { AppBar, AppShell } from "@skeletonlabs/skeleton";
	import Fa from "svelte-fa"
	import {faFloppyDisk} from "@fortawesome/free-solid-svg-icons"
    import { modalStore, type ModalSettings } from "@skeletonlabs/skeleton";
    
    
    const modal: ModalSettings = 
    {
        type: 'confirm',
        // Data
        title: 'Save game state',
        body: 'Confirm to download a file which can be used to continue this game',
        // TRUE if confirm pressed, FALSE if cancel pressed
        response: (r: boolean) => downloadData(r),
    };
        
    // CLass for displaying the ships nicely
    class ListItem 
    {
        constructor(
            public idx: number,
            public name: string
        ) {}
    }
    
    // Determine which countries are available (ships not empty)
    let countries: string[] = [];
    let game_state: GameState = get(gameStateStore);
    let loaded_ships: ShipsByCountry = game_state.ships;
    let keys = Object.keys(loaded_ships)
    for (let index = 0; index < keys.length; index++) {
        const key: string = keys[index];;
        if (!is_empty((loaded_ships as any as Hack)[key])) {
            countries.push(key);
        }
    } 
    
    // 
    let ship_idx: number = 0;
    let ships_for_cntry: ListItem[] = [];
    let selected: string = keys[0];
    let ships: Ship[] = (loaded_ships as any as Hack)[selected];
    for (let index = 0; index < ships.length; index++) {
        let ship = ships[index];
        ships_for_cntry.push(new ListItem(index, ship.name));
    }
    
    function onChangeSelected() {
        // Reset variables 
        ship_idx = 0
        ships_for_cntry = [];
        
        // 
        ships = (loaded_ships as any as Hack)[selected];
        for (let index = 0; index < ships.length; index++) {
            let ship = ships[index];            
            ships_for_cntry.push(new ListItem(index, ship.name));
        }
    }
    
    function saveDataModal() {
        modalStore.trigger(modal);
    }
    
    function downloadData(r: boolean) {
        if (r) {
            let name = "seapower_save.json";
            let str = JSON.stringify(game_state);
            var a = document.createElement('a');
            document.body.append(a);
            a.download = name;
            a.href = URL.createObjectURL(new Blob([str]));
            a.click();
            a.remove();
        }
	}
    
</script>

<AppShell>

	<svelte:fragment slot="header">
		<AppBar>
			<svelte:fragment slot="lead">
                <div class="flex items-center">
					
				</div>
            </svelte:fragment>
			<svelte:fragment slot="trail">
				<button class="btn-icon-md" on:click={saveDataModal}>
					<Fa icon={faFloppyDisk} />
                </button>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	
    <div class="card p-4 py-8 m-3">
        <!-- List of countries -->
        <label class="label">
            <span>Country: </span>
            <select class="select" bind:value={selected} on:change={onChangeSelected}>
                {#each countries as country}
                    <option value={country}>{country}</option>  
                {/each}
            </select>
        </label>
    <!-- List of ships for this country -->
    <div class="card p-4 py-8 m-3">
        <label class="label">
            <span>Ship: </span>
            <select class="select" bind:value={ship_idx}>
                    {#each ships_for_cntry as ship}
                        <option value={ship.idx}>{ship.name}</option>  
                    {/each}
                </select>
            </label>
            <!-- 'Card' of info for selected ship -->
            <ShipCard bind:ship={ships[ship_idx]} />
        </div>
        
    </div>


</AppShell> 