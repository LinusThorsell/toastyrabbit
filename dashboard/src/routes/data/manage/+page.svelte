<script lang="ts">
    import Input from "$lib/components/ui/input/input.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import { Label } from "$lib/components/ui/label/index.js";
    import * as Sheet from "$lib/components/ui/sheet/index.js";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import * as Select from "$lib/components/ui/select/index.js";

    const column_types = [
        { name: "Id", value: "Id", disabled: true },
        { name: "String", value: "String" },
        { name: "Number", value: "Number" },
        { name: "Boolean", value: "Boolean" },
        { name: "DateTime", value: "DateTime" },
    ];

    let table_name = $state("");
    let table_columns = $state([
        {
            name: "id",
            type: "Id",
            disabled: true,
        },
    ]);

    const createTable = () => {
        const columns: Record<string, string> = Object.fromEntries(
            table_columns
                .filter((c) => c.name.trim().length)
                .map((c) => [c.name, c.type] as const),
        );

        fetch("http://localhost:3000/database/table", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                table: table_name,
                columns: columns,
            }),
        })
            .then((res) => res.json())
            .then((res) => console.log(res));
    };

    const addColumn = () => {
        table_columns.push({ name: "", type: "String", disabled: false });
    };
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
