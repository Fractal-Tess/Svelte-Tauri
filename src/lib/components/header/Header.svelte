<script lang="ts">
  import ThemeToggleIcon from '$lib/components/buttons/ThemeToggleIcon.svelte';
  import { theme } from '$lib/stores/theme';
  import { appWindow } from '@tauri-apps/api/window';
  import { faXmark, faWindowMinimize } from '@fortawesome/free-solid-svg-icons';
  import { faGithub } from '@fortawesome/free-brands-svg-icons';
  import Fa from 'svelte-fa';
  import HeaderNav from '$lib/components/nav/HeaderNav.svelte';
</script>

<div
  data-tauri-drag-region
  class="h-12 bg-base-100 shadow-lg items-center justify-between flex"
>
  <button on:click={theme.toggleTheme} class="hover:text-secondary ml-2">
    <ThemeToggleIcon class="w-8" />
  </button>

  <HeaderNav />

  <!-- TODO: Make a report for the on hover text effect being laggy -->
  <div class="flex [&>*]:px-2 h-full mr-2">
    <a
      target="_blank"
      href="https://github.com/Fractal-Tess/Svelte-Tauri"
      class="hover:text-secondary flex items-center"
      rel="noreferrer"
    >
      <Fa icon={faGithub} size="lg" />
    </a>
    <button on:click={appWindow.minimize} class="text-xl hover:text-secondary">
      <Fa icon={faWindowMinimize} />
    </button>
    <button on:click={appWindow.close} class="text-2xl hover:text-secondary">
      <Fa icon={faXmark} />
    </button>
  </div>
</div>
