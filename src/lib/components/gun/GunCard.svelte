<script lang="ts">
    import type { Gun } from "$lib/objects/Gun";
    export let gun: Gun;
    
</script>

<section class="grid grid-cols-1 md:grid-cols-3 gap-1">
    <p>gun_size: {gun.gun_size}</p>
    <p>n_gun: {gun.n_gun}</p>
    <p>shell_weight: {gun.shell_weight}</p>
    <p>range: {gun.range}</p>
    <p>SRM: {gun.SRM}</p>
    <p>ammo: {gun.ammo}</p>
</section>
