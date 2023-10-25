<script lang='ts'>
    import { get } from "svelte/store"
    import { ShipsByCountry, gameStateStore } from "$lib/objects/Misc";
    import ShipList from "$lib/components/ship/ShipList.svelte"
	import { is_empty } from "svelte/internal";
    
    let loaded_ships: ShipsByCountry = get(gameStateStore);
    let keys = Object.keys(loaded_ships.Hack)
    
    let selected: string = '';
    let countries: string[] = [];
    
    for (let index = 0; index < keys.length; index++) {
        const key: string = keys[index];;
        if (!is_empty(loaded_ships.Hack[key])) {
            countries.push(key);
        }
    } 
    
    function onChange() {
        console.log(selected);
    }
    
        
</script>
<div class="card p-4 py-8 m-3">
    <label class="label">
        <span>Country: </span>
        <select 
        class="select" 
        bind:value={selected}
        on:change={onChange}>
            {#each countries as country}
                <option value={country}>{country}</option>  
            {/each}
        </select>
    </label>
    {#if !is_empty(selected)}
        <ShipList bind:selected />
    {/if}

</div>
