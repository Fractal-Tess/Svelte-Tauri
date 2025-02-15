import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'
import { join } from 'node:path'

export default defineConfig({
  plugins: [tailwindcss(), svelte()],

  resolve: {
    alias: {
      $lib: join(__dirname, 'src/lib'),
      $commands: join(__dirname, 'src/ipc/commands.ts'),
      $events: join(__dirname, 'src/ipc/events.ts'),
      $components: join(__dirname, 'src/lib/components'),
      $router: join(__dirname, 'src/lib/router')
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
})
