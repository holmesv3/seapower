<script lang="ts">
    import { get } from "svelte/store"
    import type { Ship } from "$lib/objects/Ship"
    import { ShipsByCountry, gameStateStore } from "$lib/objects/Misc";
    import ShipCard from "$lib/components/ship/ShipCard.svelte";
	import { is_empty } from "svelte/internal";
    
    export let selected: string;
    let loaded_ships: ShipsByCountry = get(gameStateStore);
    
    let ships: Ship[] = loaded_ships.Hack[selected];
    
    class ListItem {
        constructor(
            public idx: number,
            public name: string
        ) {}
    }

    let ship_idx: number = 0;
    let ships_for_cntry: ListItem[] = [];
    for (let index = 0; index < ships.length; index++) {
        var ship = ships[index];
        ships_for_cntry.push(new ListItem(index, ship.name))
    }

</script>

{#if !is_empty(selected)}
<div class="card p-4 py-8 m-3">
    <label class="label">
        <span>Ship: </span>
        <select class="select" bind:value={ship_idx}>
            {#each ships_for_cntry as ship}
                <option value={ship.idx}>{ship.name}</option>  
            {/each}
        </select>
    </label>
    <ShipCard bind:ship={ships[ship_idx]} />
</div>
{/if}