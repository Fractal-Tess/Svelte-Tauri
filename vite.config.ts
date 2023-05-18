import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { join } from 'node:path';

export default defineConfig({
  plugins: [svelte()],

  resolve: {
    alias: {
      $lib: join(__dirname, 'src/lib'),
      $ipc: join(__dirname, 'src/lib/ipc.ts'),
      $components: join(__dirname, 'src/lib/components'),
      $assets: join(__dirname, 'src/assets'),
      $router: join(__dirname, 'src/lib/router'),
      $layout: join(__dirname, 'src/lib/layout'),
      $data: join(__dirname, 'src/lib/data'),
      $types: join(__dirname, 'src/lib/types')
    }
  },

  clearScreen: false,

  publicDir: 'static',

  server: {
    strictPort: true,
    port: 5173
  },

  envPrefix: ['VITE_', 'TAURI_'],

  build: {
    outDir: 'build',
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG
  }
});
