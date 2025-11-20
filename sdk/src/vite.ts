import type { Plugin, PluginOption, ResolvedConfig } from "vite";
import fs from "node:fs/promises";
import path from "node:path";

const fetchTypeScriptDefinitions = async (): Promise<string> => {
  const request = await fetch("http://localhost:3000/development/typedefs");
  return await request.text();
}

const writeTypeScriptDefinitions = async (root: string): Promise<void> => {
  const content = await fetchTypeScriptDefinitions();
  const outFile = path.resolve(root, "src/toastyrabbit.generated.d.ts");

  await fs.writeFile(outFile, content, "utf8");
  console.log(`Wrote ${outFile}`);
}

export default function toastyrabbit(): PluginOption {
  let root = process.cwd();

  const plugin: Plugin = {
    name: "toastyrabbit",

    configResolved(config: ResolvedConfig) {
      root = config.root;
    },

    async buildStart() {
      await writeTypeScriptDefinitions(root);
    },

    async watchChange() {
      await writeTypeScriptDefinitions(root);
    },
  };

  return plugin;
}
