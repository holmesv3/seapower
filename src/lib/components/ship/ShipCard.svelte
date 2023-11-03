<script lang='ts'>
    import type { Ship } from "$lib/objects/Ship"
    import { fire } from "$lib/objects/Gun"
    import GunCard from "$lib/components/gun/GunCard.svelte";
    import { modalStore, type ModalSettings } from "@skeletonlabs/skeleton";

    export let ship: Ship;
    
    const modal: ModalSettings = {
        type: 'confirm',
        // Data
        title: 'Please Confirm',
        body: 'Are you sure you wish to proceed?',
        // TRUE if confirm pressed, FALSE if cancel pressed
        response: (r: boolean) => onClickFireConfirm(r),
    };
    
    function onClickFireModal() {
        modalStore.trigger(modal);
        console.log(modal.response);
    }
    function onClickFireConfirm(r: boolean) {
        if (r) {
            if (ship.primary_gun)
                fire(ship.primary_gun);
            if (ship.secondary_gun)
                fire(ship.secondary_gun);
            if (ship.tertiary_gun)
                fire(ship.tertiary_gun);
            ship.turn_taken = true;
        }
    }
    
    
</script>

<div class="card p-4 m-2">
    <div>        
    {#if !ship.turn_taken}
        <button class="variant-filled-error card h3 text-center p-4" on:click={onClickFireModal}>
            FIRE
        </button>
    {:else}
        <button class="variant-glass-warning card h3 text-center p-4">
            Already fired
        </button>    
    {/if}
    </div>
    <div class="arrow bg-surface-100-800-token" />
    <div class="grid grid-cols-1 md:grid-cols-3 gap-2 m-1">
        <p>ship_class: {ship.ship_class}</p>
        <p>DV: {ship.DV}</p>
        <p>max_speed: {ship.max_speed}</p>
        <p>status: {ship.status}</p>
        <p>gun_status: {ship.gun_status}</p>
        <p>hull_status: {ship.hull_status}</p>
        <p>torpedo_tubesize: {ship.torpedo_tubesize}</p>
        <p>n_tube: {ship.n_tube}</p>
        <p>n_torpedo: {ship.n_torpedo}</p>
        <p>catapults: {ship.catapults}</p>
        <p>aircraft_on_board: {ship.aircraft_on_board}</p>
        <p>aircraft_in_air: {ship.aircraft_in_air}</p>
        <p>depth_charge: {ship.depth_charge}</p>
        <p>anti_aircraft: {ship.anti_aircraft}</p>
        <p>mines: {ship.mines}</p>
    </div>

    <section class="grid grid-cols-1 md:grid-cols-3 gap-4 m-1">
        <div class="variant-glass-primary card">
            <header class="card-header">Primary Gun</header>
            <hr />
            <div class="p-4">
                <GunCard bind:gun={ship.primary_gun}/>
            </div>
        </div>
        {#if ship.secondary_gun}
            <div class="variant-glass-secondary card">
                <header class="card-header">Secondary Gun</header>
                <hr />
                <div class="p-4">
                    <GunCard bind:gun={ship.secondary_gun}/>
                </div>
            </div>
        {/if}
        {#if ship.tertiary_gun}
            <div class="variant-glass-tertiary card">
                <header class="card-header">Tertiary Gun</header>
                <hr />
                <div class="p-4">
                    <GunCard bind:gun={ship.tertiary_gun}/>
                </div>
            </div>
        {/if}
    </section>
    
    <a class="btn variant-filled-primary" href="/init_game">DEBUG</a>

</div>
