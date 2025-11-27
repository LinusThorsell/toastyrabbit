<script lang="ts">
    import Input from "$lib/components/ui/input/input.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import { Label } from "$lib/components/ui/label/index.js";
    import * as Sheet from "$lib/components/ui/sheet/index.js";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import * as Select from "$lib/components/ui/select/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import ToastyRabbit, {
        type ColumnTypes,
        type TableInfo,
    } from "toastyrabbit";
    import { type TT } from "toastyrabbit/datatypes";
    import { onMount } from "svelte";

    const tr = new ToastyRabbit<TT>("http://localhost:3000/", fetch);

    const column_types: {
        name: string;
        value: ColumnTypes;
        disabled?: boolean;
    }[] = [
        { name: "Id", value: "Id", disabled: true },
        { name: "String", value: "String" },
        { name: "Number", value: "Number" },
        { name: "Boolean", value: "Boolean" },
        { name: "DateTime", value: "DateTime" },
    ];

    let table_name = $state("");
    let table_columns = $state<
        { name: string; type: ColumnTypes; disabled?: boolean }[]
    >([
        {
            name: "id",
            type: "Id",
            disabled: true,
        },
    ]);

    const createTable = () => {
        const columns: Record<string, ColumnTypes> = Object.fromEntries(
            table_columns
                .filter((c) => c.name.trim().length)
                .map((c) => [c.name, c.type]),
        );

        tr.createTable(table_name, columns);
        setTimeout(() => refreshTables(), 1000);
    };

    const addColumn = () => {
        table_columns.push({ name: "", type: "String", disabled: false });
    };

    const deleteTable = (table: TableInfo<TT>) => {
        tr.deleteTable(table.table);
        setTimeout(() => refreshTables(), 1000);
    };

    let tables = $state<TableInfo<TT>[]>();

    const refreshTables = async () => {
        tables = await tr.getTables();
    }

    onMount(async () => {
        await refreshTables();
    });
</script>

<Sheet.Root>
    <Sheet.Trigger>
        <Button>Create new table</Button>
    </Sheet.Trigger>
    <Sheet.Content>
        <Sheet.Header>
            <Sheet.Title>Create new table</Sheet.Title>
            <Sheet.Description>Fill in the form.</Sheet.Description>
        </Sheet.Header>

        <div class="px-4">
            <Label class="mb-2" for="table_name">Table name</Label>
            <Input id="table_name" bind:value={table_name} />

            <Separator class="my-4" />

            <Label class="mb-2">Columns</Label>

            {#each table_columns as column}
                <div class="flex items-center mb-2 gap-2">
                    <Input
                        disabled={column.disabled}
                        bind:value={column.name}
                    />
                    <Select.Root
                        type="single"
                        disabled={column.disabled}
                        bind:value={column.type}
                    >
                        <Select.Trigger class="w-[180px]"
                            >{column.type}</Select.Trigger
                        >
                        <Select.Content>
                            {#each column_types as column_type}
                                <Select.Item
                                    value={column_type.value}
                                    disabled={column_type.disabled}
                                >
                                    {column_type.name}
                                </Select.Item>
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>
            {/each}

            <Button onclick={addColumn} class="mt-2">Add column</Button>
        </div>

        <Sheet.Close class="mt-auto mb-4">
            <Separator class="my-4" />
            <div class="flex justify-between gap-2 px-4">
                <Button onclick={createTable} variant="default">Create</Button>
                <Button variant="secondary">Cancel</Button>
            </div>
        </Sheet.Close>
    </Sheet.Content>
</Sheet.Root>

{#each tables as table}
    <div>
        <Popover.Root>
            <Popover.Trigger>
                <Button>Delete table: {table.table}</Button>
            </Popover.Trigger>
            <Popover.Content>
                Are you sure you want to delete the table: {table.table}?

                <Popover.Close>
                    <Button
                        class="mb-2 mt-2"
                        variant="destructive"
                        onclick={() => deleteTable(table)}
                        >Yes, Delete table and all data</Button
                    >
                    <Button>Cancel</Button>
                </Popover.Close>
            </Popover.Content>
        </Popover.Root>
    </div>
{/each}
