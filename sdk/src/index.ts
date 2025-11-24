export type TableMapBase = Record<string, any>;

export type TableRow<M extends TableMapBase> = M[keyof M]
export type TableName<M extends TableMapBase> = keyof M & string;
export type TableInfo<M extends TableMapBase> = {
  table: TableName<M>;
  columns: Record<string, string>;
};
export type ColumnTypes = "Id" | "String" | "Number" | "Boolean" | "DateTime";

export default class ToastyRabbit<M extends TableMapBase = TableMapBase> {
  url: string;
  fetch_fn: typeof fetch;

  constructor(url: string, fetch_fn?: typeof fetch) {
    if (!url.endsWith("/")) url += "/";
    if (url.endsWith("/")) url = url.slice(0, -1);
    this.url = url;
    this.fetch_fn = fetch_fn || fetch;
  }

  async get<K extends keyof M & string>(table: K, id: number): Promise<M[K]> {
    const res = await this.fetch_fn(`${this.url}/collection/${table}/get?` + new URLSearchParams({
      id: id.toString(),
    }));
    return await res.json();
  }

  async getPage<K extends keyof M & string>(table: K, page: number, per_page: number): Promise<M[K][]> {
    const res = await this.fetch_fn(`${this.url}/collection/${table}/page?` + new URLSearchParams({
      page: page.toString(),
      per_page: per_page.toString(),
    }));
    return await res.json();
  }

  async getFirst<K extends keyof M & string>(table: K): Promise<M[K]> {
    const res = await this.fetch_fn(`${this.url}/collection/${table}/first`);
    return await res.json();
  }

  async getAll<K extends keyof M & string>(table: K): Promise<M[K][]> {
    const res = await this.fetch_fn(`${this.url}/collection/${table}`);
    return await res.json();
  }

  async getTables(): Promise<TableInfo<M>[]> {
    const res = await this.fetch_fn(`${this.url}/database/table`);
    return await res.json();
  }

  async createTable(table_name: string, columns: Record<string, ColumnTypes>) {
    const res = await this.fetch_fn(`${this.url}/database/table`, {
      method: "POST",
      headers: {
          "Content-Type": "application/json",
      },
      body: JSON.stringify({
          table: table_name,
          columns: columns,
      }),
    });
    return await res.json();
  }
}
