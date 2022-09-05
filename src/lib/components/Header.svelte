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
    class="hover:text-secondary ml-2"
    aria-live="polite"
  >
    <ThemeToggleIcon cssClass={'w-8'} />
  </button>

  <div>
    <a
      target="_blank"
      href="https://github.com/Fractal-Tess/Svelte-Tauri"
      class="hover:text-secondary  inline-block"
    >
      <Fa icon={faGithub} size="lg" />
    </a>
    <button
      on:click={appWindow.minimize}
      aria-live="polite"
      class="p-2 text-xl hover:text-secondary"
    >
      <Fa icon={faWindowMinimize} />
    </button>
    <button
      on:click={appWindow.close}
      aria-live="polite"
      class="p-2 text-2xl hover:text-secondary"
    >
      <Fa icon={faXmark} />
    </button>
  </div>
</div>
