import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { resolve } from 'node:path';

export default defineConfig({
  // Svelte
  plugins: [svelte()],

  // Inject style variables
  css: {
    preprocessorOptions: {
      scss: {
        additionalData: '@use "src/styles/variables.scss" as *;',
      },
    },
  },

  resolve: {
    alias: {
      $styles: resolve('src/styles/app.scss'),
      $lib: resolve('src/lib'),
      $src: resolve('src/'),
    },
  },
  publicDir: 'static',

  clearScreen: false,
  server: {
    strictPort: true,
    port: 3000,
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
