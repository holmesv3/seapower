<script lang="ts">
import type { ShipsByCountry } from "$lib/objects/Misc"
import { is_empty } from "svelte/internal";
import CountryShipPagination from "./CountryShipPagination.svelte";"./CountryShipPagination.svelte";
import { TabGroup, Tab } from "@skeletonlabs/skeleton";
import US from "$lib/components/flags/US.svelte";
import GB from "$lib/components/flags/GB.svelte";
import FR from "$lib/components/flags/FR.svelte";
import CN from "$lib/components/flags/CN.svelte";
import RU from "$lib/components/flags/RU.svelte";
import JP from "$lib/components/flags/JP.svelte";
import IT from "$lib/components/flags/IT.svelte";
import DE from "$lib/components/flags/DE.svelte";


export let ships: ShipsByCountry;

let tabSet: number = 0;
</script>

<TabGroup>
    {#if !is_empty(ships.USA)}
        <Tab bind:group={tabSet} name="tab1" value={1}><US /> USA</Tab>     <!-- US -->
    {/if}
    {#if !is_empty(ships.GB)}
        <Tab bind:group={tabSet} name="tab2" value={2}><GB /> GB</Tab>      <!-- GB --> 
    {/if}
    {#if !is_empty(ships.FR)}
        <Tab bind:group={tabSet} name="tab3" value={3}><FR /> FR</Tab>      <!-- FR -->
    {/if}
    {#if !is_empty(ships.CN)}
        <Tab bind:group={tabSet} name="tab4" value={4}><CN /> CN</Tab>      <!-- CN -->
    {/if}
    {#if !is_empty(ships.RUS)}
        <Tab bind:group={tabSet} name="tab5" value={5}><RU /> RUS</Tab>     <!-- RU -->
    {/if}
    {#if !is_empty(ships.JPN)}
        <Tab bind:group={tabSet} name="tab6" value={6}><JP /> JPN</Tab>     <!-- JP -->
    {/if}
    {#if !is_empty(ships.IT)}
        <Tab bind:group={tabSet} name="tab7" value={7}><IT /> IT</Tab>      <!-- IT -->
    {/if}
    {#if !is_empty(ships.GER)}
        <Tab bind:group={tabSet} name="tab8" value={8}><DE /> GER</Tab>     <!-- DE -->
    {/if}
	<!-- Tab Panels --->
	<svelte:fragment slot="panel">
		{#if tabSet === 0}
			<div>Select Country to display available ships...</div>
        {:else if tabSet === 1}
            <CountryShipPagination bind:source={ships.USA} />
        {:else if tabSet === 2}
            <CountryShipPagination bind:source={ships.GB} />
        {:else if tabSet === 3}
            <CountryShipPagination bind:source={ships.FR} />
        {:else if tabSet === 4}
            <CountryShipPagination bind:source={ships.CN} />
        {:else if tabSet === 5}
            <CountryShipPagination bind:source={ships.RUS} />
        {:else if tabSet === 6}
            <CountryShipPagination bind:source={ships.JPN} />
        {:else if tabSet === 7}
            <CountryShipPagination bind:source={ships.IT} />
        {:else if tabSet === 8}
            <CountryShipPagination bind:source={ships.GER} />
		{/if}
	</svelte:fragment>
</TabGroup>
			