<script lang="ts">
	import type { Ship } from "$lib/objects/Ship";
	import ShipCard from "./ShipCard.svelte";

    export let ships: Ship[];
    console.log("From the ships selector", ships);
    let ship_idx: number = 0;
    class thing {
        constructor(
            public idx: number,
            public name: string
        ) {}
    }
    let things: thing[] = [];
    for (let index = 0; index < ships.length; index++) {
        const ship = ships[index];
        things.push(new thing(index, ship.name))
    }
</script>

<div class="card p-4 py-8 m-3">
    <label class="label">
        <span>Ship: </span>
        <select class="select" bind:value={ship_idx}>
            {#each things as thing}
                <option value={thing.idx}>{thing.name}</option>  
            {/each}
        </select>
    </label>
    <ShipCard bind:ship={ships[ship_idx]} />
</div>