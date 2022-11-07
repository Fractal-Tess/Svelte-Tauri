import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { join } from 'node:path';

export default defineConfig({
  plugins: [svelte()],

  resolve: {
    alias: {
      $lib: join(__dirname, 'src/lib'),

      $components: join(__dirname, 'src/lib/components'),

      $assets: join(__dirname, 'src/assets'),

      $router: join(__dirname, 'src/lib/router'),

      $layout: join(__dirname, 'src/lib/layout'),

      $data: join(__dirname, 'src/lib/data'),

      $styles: join(__dirname, 'src/styles/app.postcss'),

      $types: join(__dirname, 'src/lib/types')
    }
  },

  publicDir: false,

  clearScreen: false,

  server: {
    strictPort: true,
    port: 3000
  },

  envPrefix: ['VITE_', 'TAURI_'],

  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG
  }
});
