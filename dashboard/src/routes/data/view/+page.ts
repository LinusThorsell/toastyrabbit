import type { PageLoad } from "./$types";

import ToastyRabbit from "toastyrabbit";
import { type TableRow } from "toastyrabbit";
import { type TT } from "toastyrabbit/datatypes"

type TableWithDataDef = {
    table: string;
    columns: Record<string, string>;
    rows: TableRow<TT>[];
};

export const load: PageLoad = async ({ fetch }) => {
    const tr = new ToastyRabbit<TT>("http://localhost:3000/");

    tr.getFirst("users").then((rows) => {
        console.log(rows);
    });

    tr.get("users", 2).then((rows) => {
        console.log(rows);
    });

    tr.getPage("users", 2, 3).then((rows) => {
        console.log(rows[0]);
    });

    const baseTables = await tr.getTables();

    const tables: TableWithDataDef[] = await Promise.all(
        baseTables.map(async (table) => {
            const rows = await tr.getAll(table.table);
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

