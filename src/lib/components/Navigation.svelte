<script lang="ts">
    import { drawerStore } from "@skeletonlabs/skeleton";

    function drawerClose(): void {
		drawerStore.close();
	}
</script>

<nav class="list-nav p-4">
	<ul>
		<li><a href="/" on:click={drawerClose}>Homepage</a></li>
		<li><a href="/init_game" on:click={drawerClose}>Play Game</a></li>
	</ul>
</nav>