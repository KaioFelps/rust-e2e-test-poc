import react from '@vitejs/plugin-react';
import laravel from 'laravel-vite-plugin';
import tailwind from "@tailwindcss/vite"
import type { UserConfig } from 'vite';

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
  }
} satisfies UserConfig;
