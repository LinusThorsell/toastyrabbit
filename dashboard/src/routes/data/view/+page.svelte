<script lang="ts">
    import type { PageProps } from "./$types";
    import type { ColumnDef } from "@tanstack/table-core";

    import DataTable from "$lib/custom/DataTable.svelte";
    import * as Card from "$lib/components/ui/card/index.js";

    let { data }: PageProps = $props();

    const getColumnDefs = (columns: Record<string, string>): ColumnDef<Record<string, unknown>>[] => {
        const keys = Object.keys(columns);
        let defs = keys.map((key: string) => ({
            accessorKey: key,
            header: key,
        }));
        return defs;
    };
</script>

{#each data.tables as table}
    <Card.Root>
        <Card.Header>{table.table}</Card.Header>
        <Card.Content>
            <DataTable
                data={table.rows}
                columns={getColumnDefs(table.columns)}
            />
        </Card.Content>
    </Card.Root>
{/each}
