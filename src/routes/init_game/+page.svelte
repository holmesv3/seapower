<script lang="ts">
    import { FileDropzone, AppBar } from '@skeletonlabs/skeleton';
    import data from "$lib/files/shipmanifest.json";
    import type { ShipsByCountry } from "$lib/objects/Misc"
    import { is_empty } from "svelte/internal";
    import CountryShipPagination from "$lib/components/ship/ShipPagination.svelte" 
    import { TabGroup, Tab } from "@skeletonlabs/skeleton";
    import US from "$lib/components/flags/US.svelte";
    import GB from "$lib/components/flags/GB.svelte";
    import FR from "$lib/components/flags/FR.svelte";
    import CN from "$lib/components/flags/CN.svelte";
    import RU from "$lib/components/flags/RU.svelte";
    import JP from "$lib/components/flags/JP.svelte";
    import IT from "$lib/components/flags/IT.svelte";
    import DE from "$lib/components/flags/DE.svelte";
    import { GameState, GameScale, gameStateStore } from '$lib/objects/GameState';

    let input: FileList;

    let scale: GameScale;
    let tab_set = 0;
    let cs_ships: ShipsByCountry = [];

    function onDefault() {
        scale = data.scale;
        cs_ships = data.ships as any as ShipsByCountry;
        tab_set = 0;
        
    }
    
    async function onLoad() {
        let file = input.item(0);
        let state_str = await file?.text()
        if (state_str) {
            let state: GameState = JSON.parse(state_str);
            scale = state.scale;
            cs_ships = state.ships;
            gameStateStore.set(state);  
            tab_set = 0
        }
    }

    function storeState() {
        let scale = GameScale.MD;
        let state = new GameState(scale, cs_ships);
        gameStateStore.set(state);
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
            <FileDropzone name="files" bind:files={input} on:change={onLoad}/>
        </div>
    </div>
</div>

{#if !is_empty(cs_ships)}
<div class="container mx-auto p-8 space-y-8">
    <AppBar>
        <svelte:fragment slot="lead">
            <h1 class="h2">Loaded ships</h1>
        </svelte:fragment>
        <svelte:fragment slot="trail">
            <a class="btn variant-filled-primary" href="/landing" on:click={storeState}>Continue</a>
        </svelte:fragment>
    </AppBar>
    
<TabGroup>
    {#if !is_empty(cs_ships.USA)}
        <Tab bind:group={tab_set} name="tab1" value={1}><US /> USA</Tab>     <!-- US -->
    {/if}
    {#if !is_empty(cs_ships.GB)}
        <Tab bind:group={tab_set} name="tab2" value={2}><GB /> GB</Tab>      <!-- GB --> 
    {/if}
    {#if !is_empty(cs_ships.FR)}
        <Tab bind:group={tab_set} name="tab3" value={3}><FR /> FR</Tab>      <!-- FR -->
    {/if}
    {#if !is_empty(cs_ships.CN)}
        <Tab bind:group={tab_set} name="tab4" value={4}><CN /> CN</Tab>      <!-- CN -->
    {/if}
    {#if !is_empty(cs_ships.RUS)}
        <Tab bind:group={tab_set} name="tab5" value={5}><RU /> RUS</Tab>     <!-- RU -->
    {/if}
    {#if !is_empty(cs_ships.JPN)}
        <Tab bind:group={tab_set} name="tab6" value={6}><JP /> JPN</Tab>     <!-- JP -->
    {/if}
    {#if !is_empty(cs_ships.IT)}
        <Tab bind:group={tab_set} name="tab7" value={7}><IT /> IT</Tab>      <!-- IT -->
    {/if}
    {#if !is_empty(cs_ships.GER)}
        <Tab bind:group={tab_set} name="tab8" value={8}><DE /> GER</Tab>     <!-- DE -->
    {/if}
	<!-- Tab Panels --->
	<svelte:fragment slot="panel">
		{#if tab_set === 0}
			<div>Select Country to display available cs_ships...</div>
        {:else if tab_set === 1}
            <CountryShipPagination bind:source={cs_ships.USA} />
        {:else if tab_set === 2}
            <CountryShipPagination bind:source={cs_ships.GB} />
        {:else if tab_set === 3}
            <CountryShipPagination bind:source={cs_ships.FR} />
        {:else if tab_set === 4}
            <CountryShipPagination bind:source={cs_ships.CN} />
        {:else if tab_set === 5}
            <CountryShipPagination bind:source={cs_ships.RUS} />
        {:else if tab_set === 6}
            <CountryShipPagination bind:source={cs_ships.JPN} />
        {:else if tab_set === 7}
            <CountryShipPagination bind:source={cs_ships.IT} />
        {:else if tab_set === 8}
            <CountryShipPagination bind:source={cs_ships.GER} />
		{/if}
	</svelte:fragment>
</TabGroup>
</div>
{:else}
<div class="container mx-auto p-8 space-y-8 placegolder"></div>

{/if}