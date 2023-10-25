<script lang="ts">
	import { Drawer, AppShell, AppBar, drawerStore } from "@skeletonlabs/skeleton";
	import Navigation from "$lib/components/Navigation.svelte";

	function drawerOpen(): void { 
		drawerStore.open({});
	}
</script>
<Drawer>
	<Navigation />
</Drawer>

<AppShell slotSidebarLeft="bg-surface-500/5 w-0 lg:w-56">
	<svelte:fragment slot="header">
		<AppBar>
			<svelte:fragment slot="lead">
				<div class="flex items-center">
					<button class="lg:hidden btn btn-sm mr-4" on:click={drawerOpen}>
						<span>
							<svg viewBox="0 0 100 80" class="fill-token w-4 h-4">
								<rect width="100" height="20" />
								<rect y="30" width="100" height="20" />
								<rect y="60" width="100" height="20" />
							</svg>
						</span>
					</button>
					<strong class="text-xl uppercase">Seapower</strong>
				</div>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	
	<svelte:fragment slot="sidebarLeft">
		<!-- Insert the list: -->
		<Navigation />
		<!-- --- -->
	</svelte:fragment>
	<!-- Router Slot -->
	<div class="container mx-auto p-8 space-y-8">
		<h1 class="h1">Hello Skeleton</h1>
		<p>Lorem ipsum dolor sit amet consectetur adipisicing elit.</p>
		<section>
			<a class="btn variant-filled-primary" href="https://kit.svelte.dev/">SvelteKit</a>
			<a class="btn variant-filled-secondary" href="https://tailwindcss.com/">Tailwind</a>
			<a class="btn variant-filled-tertiary" href="https://github.com/">GitHub</a>
		</section>
	</div>
		
	<!-- ---- / ---- -->
	<!-- <svelte:fragment slot="pageFooter">Page Footer</svelte:fragment> -->
	<!-- (footer) -->
		
</AppShell> 
