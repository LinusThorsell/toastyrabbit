<script lang="ts">
    import type { PageProps } from "./$types";

    let { data }: PageProps = $props();
</script>

{#each data.tables as table}
    <div class="mt-2">
        <span class="font-semibold">{table.table}</span>
        <ul class="ml-4 list-disc">
            {#each Object.entries(table.columns) as [column, type]}
                <li>{column}: <span class="text-slate-400">{type}</span></li>
            {/each}
        </ul>
    </div>
{/each}
