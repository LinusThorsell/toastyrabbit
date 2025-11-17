export type TableMapBase = Record<string, any>;

export default class ToastyRabbit<M extends TableMapBase = TableMapBase> {
  url: string;

  constructor(url: string) {
    if (!url.endsWith("/")) url += "/";
    if (url.endsWith("/")) url = url.slice(0, -1);
    this.url = url;
  }

  async getAll<K extends keyof M & string>(table: K): Promise<M[K][]> {
    const res = await fetch(`${this.url}/collection/${table}`);
    return await res.json();
  }
}
