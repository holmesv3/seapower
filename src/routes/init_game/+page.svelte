<script lang="ts">
    import { is_empty } from 'svelte/internal';
    import { FileDropzone, AppBar } from '@skeletonlabs/skeleton';

    import data from "$lib/files/shipmanifest2.json";
    import { Ship, type ShipType } from "$lib/objects/Ship";

	import CountryShipTabulated from '$lib/components/ship_by_country/CountryShipTabulated.svelte';
	import { ShipsByCountry, gameStateStore } from '$lib/objects/Misc';

    let input: FileList;

    let ships: Array<Ship> = [];
    let cs_ships: ShipsByCountry;

    function onDefault (): void {
        loadShips(data);
    }
      
    function loadShips(data: any): void {
        ships = []; // clear prior entries
        let keys = Object.keys(data.ships);
        for (let index = 0; index < keys.length; index++) {
            ships.push(Ship.fromInterface(data.ships[keys[index]] as ShipType));
        }
        cs_ships = new ShipsByCountry(ships);
        
    }

    function saveShips() {
        gameStateStore.set(cs_ships);
    }

</script>

<div class="grid grid-cols-1 md:grid-cols-3 m-2 p-8">
    <div class="card mx-auto p-4 py-4 m-4 text-center items-center">
        <div class="mx-auto p-2">
            Use default configuration 
        </div>
        <button on:click={onDefault}>
            <div class="card card-hover variant-glass-primary p-4 py-8 mb-4 mx-auto border-2 border-dashed opacity-75 rounded-container-token">
                <p class="font-bold">
                    (start new game)    
                </p>
            </div>

        </button>
    </div>
    <h1 class="h2 justify-center items-center flex">or</h1>
    <div class="card mx-auto p-8 m-4 text-center items-center">
        Use prior save
        <div class="card card-hover">
            <FileDropzone name="files" bind:files={input}/>
        </div>
    </div>
</div>

{#if !is_empty(ships)}
<div class="container mx-auto p-8 space-y-8">
    <AppBar>
        <svelte:fragment slot="lead">
            <h1 class="h2">Loaded ships</h1>
        </svelte:fragment>
        <svelte:fragment slot="trail">
            <a class="btn variant-filled-primary" href="/landing" on:click={saveShips}>Continue</a>
        </svelte:fragment>
    </AppBar>
    <CountryShipTabulated bind:ships={cs_ships}/>
</div>
{:else}
<div class="container mx-auto p-8 space-y-8 placegolder"></div>

{/if}