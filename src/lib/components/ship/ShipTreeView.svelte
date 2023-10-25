<script lang='ts'>
    import { TreeView, TreeViewItem } from "@skeletonlabs/skeleton";
    import type { Ship } from "$lib/objects/Ship"
    export let ship: Ship;
</script>
<div class="card p-4 m-2">
	<TreeView>
		<input class="radio" type="radio" name="radio-direct" value="{ship.name}" />
		<TreeViewItem>
			{ship.name}
			<svelte:fragment slot="children">
				<TreeViewItem>
					{ship.val1}
				</TreeViewItem>
				<TreeViewItem>
					{ship.val2}
				</TreeViewItem>
			</svelte:fragment>
		</TreeViewItem>
	</TreeView>
</div>