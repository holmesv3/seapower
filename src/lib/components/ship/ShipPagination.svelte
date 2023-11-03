<script lang="ts">
	import type { Ship } from "$lib/objects/Ship";
    import { Table, Paginator, tableMapperValues } from "@skeletonlabs/skeleton";

    // PaginatorSettings
    export let source: Ship[];
    let objs = tableMapperValues(source, ['name', 'country', 'status']);
    let headers: string[] = [
        'Name', 
        'Country', 
        'Status'];

    let page = {
        offset: 0,
        limit: 5,
        size: objs.length,
        amounts: [1,2,5,10],
    };

</script>

<Table interactive={true} source={{head: headers, 
    body: objs.slice(
        page.offset * page.limit,             
        page.offset * page.limit + page.limit)}}
/>
<Paginator 
    bind:settings={page} 
    showFirstLastButtons="{true}" 
    showPreviousNextButtons="{true}">
</Paginator>