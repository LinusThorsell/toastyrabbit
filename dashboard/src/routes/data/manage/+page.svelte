<script lang="ts">
    import Input from "$lib/components/ui/input/input.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import { Label } from "$lib/components/ui/label/index.js";
    import * as Sheet from "$lib/components/ui/sheet/index.js";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import * as Select from "$lib/components/ui/select/index.js";

    const createTestTable = () => {
        console.log("create test table");

        fetch("http://localhost:3000/database/table", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                table: "payments",
                columns: {
                    id: "Id",
                    name: "String",
                    created_at: "DateTime",
                    amount: "Number",
                    is_paid: "Boolean",
                },
            }),
        })
            .then((res) => res.json())
            .then((res) => console.log(res));
    };

    let table_name = $state("");
</script>

Manage data

<Button onclick={createTestTable}>Create test table</Button>

<Input bind:value={table_name} />

<Sheet.Root open={true}>
    <Sheet.Trigger>
        <Button>Open</Button>
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

            <div class="flex items-center gap-2">
                <Input disabled={true} value="id" />
                <Select.Root type="single" disabled={true}>
                    <Select.Trigger class="w-[180px]">Primary key</Select.Trigger>
                </Select.Root>
            </div>

            <Button class="mt-4">Add column</Button>
        </div>

        <Sheet.Close class="mt-auto mb-4">
            <Separator class="my-4" />
            <div class="flex justify-between gap-2 px-4">
                <Button variant="default">Create</Button>
                <Button variant="secondary">Cancel</Button>
            </div>
        </Sheet.Close>
    </Sheet.Content>
</Sheet.Root>
