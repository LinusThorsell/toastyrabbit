export type TableMapBase = Record<string, any>;

export type TableRow<M extends TableMapBase> = M[keyof M]
export type TableName<M extends TableMapBase> = keyof M & string;
export type TableInfo<M extends TableMapBase> = {
  table: TableName<M>;
  columns: Record<string, string>;
};

export default class ToastyRabbit<M extends TableMapBase = TableMapBase> {
  url: string;

  constructor(url: string) {
    if (!url.endsWith("/")) url += "/";
    if (url.endsWith("/")) url = url.slice(0, -1);
    this.url = url;
  }

  async get<K extends keyof M & string>(table: K, id: number): Promise<M[K]> {
    const res = await fetch(`${this.url}/collection/${table}/get?` + new URLSearchParams({
      id: id.toString(),
    }));
    return await res.json();
  }

  async getPage<K extends keyof M & string>(table: K, page: number, per_page: number): Promise<M[K][]> {
    const res = await fetch(`${this.url}/collection/${table}/page?` + new URLSearchParams({
      page: page.toString(),
      per_page: per_page.toString(),
    }));
    return await res.json();
  }

  async getFirst<K extends keyof M & string>(table: K): Promise<M[K]> {
    const res = await fetch(`${this.url}/collection/${table}/first`);
    return await res.json();
  }

  async getAll<K extends keyof M & string>(table: K): Promise<M[K][]> {
    const res = await fetch(`${this.url}/collection/${table}`);
    return await res.json();
  }

  async getTables(): Promise<TableInfo<M>[]> {
    const res = await fetch(`${this.url}/database/table`);
    return await res.json();
  }
}
