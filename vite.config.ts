import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { resolve } from 'node:path';

export default defineConfig({
  plugins: [svelte()],

  resolve: {
    alias: {
      $styles: resolve('src/styles/app.postcss'),
      $lib: resolve('src/lib/'),
      $src: resolve('src/'),
      $assets:resolve('src/assets/')
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
