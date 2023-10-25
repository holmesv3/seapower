<script lang="ts">
	import { Drawer, AppShell, AppBar, drawerStore } from "@skeletonlabs/skeleton";
	import Navigation from "$lib/components/Navigation.svelte";
	import {Github} from "@inqling/svelte-icons/simple-icons"	
	
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
			<svelte:fragment slot="trail">
				<a class="btn-icon-sm"  href="https://github.com/holmesv3/seapower" >
					<Github />
				</a>
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
		<h1 class="h1">Welcome to Seapower</h1>
		<p>
			This is the home page, which isn't much.
		</p>
		<p>
			Eventually I'll probably have some instructions or something here
		</p>
		<p>
			Select 'Play Game' on the left to start the game setup 
		</p>
		<p>
			(spoiler alert, it probably won't work right)
		</p>

	</div>
		
		
</AppShell> 
