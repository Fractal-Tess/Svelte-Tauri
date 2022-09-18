<script lang="ts">
  import ThemeToggleIcon from '$lib/components/ThemeToggleIcon.svelte';
  import { theme } from '$lib/stores/theme';
  import { appWindow } from '@tauri-apps/api/window';
  import { faXmark, faWindowMinimize } from '@fortawesome/free-solid-svg-icons';
  import { faGithub } from '@fortawesome/free-brands-svg-icons';
  import Fa from 'svelte-fa';
</script>

<div
  data-tauri-drag-region
  class="h-12 bg-base-100 shadow-lg items-center justify-between flex"
>
  <button
    on:click={theme.toggleTheme}
    class="hover:text-primary ml-2"
    aria-live="polite"
  >
    <ThemeToggleIcon class="w-8" />
  </button>

  <!-- TODO: Investigate why hovering over the icons is laggy -->
  <div class="flex [&>*]:px-2 h-full">
    <a
      target="_blank"
      href="https://github.com/Fractal-Tess/Svelte-Tauri"
      class="hover:text-primary flex items-center"
    >
      <Fa icon={faGithub} size="lg" />
    </a>
    <button
      on:click={appWindow.minimize}
      aria-live="polite"
      class="text-xl hover:text-primary"
    >
      <Fa icon={faWindowMinimize} />
    </button>
    <button
      on:click={appWindow.close}
      aria-live="polite"
      class="text-2xl hover:text-primary"
    >
      <Fa icon={faXmark} />
    </button>
  </div>
</div>
