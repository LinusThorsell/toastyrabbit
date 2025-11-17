import type { PageLoad } from "./$types";
import { PUBLIC_API_URL } from "$env/static/public";

import ToastyRabbit from "toastyrabbit";
import { type ToastyTables } from "toastyrabbit/datatypes"

type TableDef = {
    table: string;
    columns: Record<string, string>;
    rows: Record<string, unknown>[];
};

export const load: PageLoad = async ({ fetch }) => {
    const tr = new ToastyRabbit<ToastyTables>("http://localhost:3000/");

    const users = await tr.getAll("users");

    const tablesRes = await fetch(`${PUBLIC_API_URL}/database/table`);
    const baseTables = (await tablesRes.json()) as Array<
        Omit<TableDef, "rows">
    >;

    const tables: TableDef[] = await Promise.all(
        baseTables.map(async (table) => {
            const rowsRes = await fetch(
                `${PUBLIC_API_URL}/collection/${table.table}`,
            );
            const rows = (await rowsRes.json()) as Record<string, unknown>[];
            return {
                ...table,
                rows,
            };
        }),
    );

    return {
        tables,
    };
};

