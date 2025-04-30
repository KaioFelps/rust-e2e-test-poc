import react from '@vitejs/plugin-react';
import laravel from 'laravel-vite-plugin';
import tailwind from "@tailwindcss/vite"
import type { UserConfig } from 'vite';

import tsconfig from "./tsconfig.json";
import path from 'path';

export const tsconfigPathAliases = Object.fromEntries(
  Object.entries(tsconfig.compilerOptions.paths).map(([key, values]) => {
    let value = values[0];
    if (key.endsWith("/*")) {
      key = key.slice(0, -2);
      value = value.slice(0, -2);
    }

    const nodeModulesPrefix = "node_modules/";
    if (value.startsWith(nodeModulesPrefix)) {
      value = value.replace(nodeModulesPrefix, "");
    } else {
      value = path.join(__dirname, value);
    }

    return [key, value];
  }),
);


export default {
  plugins: [    
      tailwind(),
      laravel({
          input: 'www/app.tsx',
          buildDirectory: 'bundle',
          ssrOutputDirectory: "dist/ssr",
          //ssr: "www/ssr.tsx",
      }),
      react(),
  ],
  publicDir: "/public",
  server: {
      watch: {
          ignored: ["*"]
      }
  },
  resolve: {
    alias: tsconfigPathAliases,
  },
} satisfies UserConfig;
