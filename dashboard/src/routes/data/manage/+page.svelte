<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";

    const createTestTable = () => {
        console.log('create test table');

        fetch("http://localhost:3000/database/table", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                table: "payments",
                columns: {
                    id: "Id",
                    name: "String",
                    created_at: "DateTime",
                    amount: "Number",
                    is_paid: "Boolean"
                }
            })
        }).then(res => res.json()).then(res => console.log(res));
    }
</script>

Manage data

<Button onclick={createTestTable}>Create test table</Button>
