<script lang="ts">
  import ThemeToggleIcon from '$lib/components/buttons/ThemeToggleIcon.svelte';
  import { theme } from '$lib/stores/theme';
  import { appWindow } from '@tauri-apps/api/window';
  import { faXmark, faWindowMinimize } from '@fortawesome/free-solid-svg-icons';
  import { faGithub } from '@fortawesome/free-brands-svg-icons';
  import Fa from 'svelte-fa';
  import { navLinks } from '$data/NavLinks';
</script>

<div
  data-tauri-drag-region
  class="h-12 bg-base-100 shadow-lg items-center justify-between flex px-2"
>
  <button on:click={theme.toggleTheme} class="hover:text-secondary ml-2 grid">
    <ThemeToggleIcon theme={$theme} />
  </button>

  <nav>
    <ul class="flex space-x-4">
      {#each navLinks as navLink, i (i)}
        <li>
          <a
            href={navLink.target.href}
            target={navLink.target.newTab ? '_blank' : ''}
            class="hover:text-secondary transition-colors"
          >
            {#if navLink.content.icon}
              <Fa icon={navLink.content.icon} />
            {/if}
            {navLink.content.text}
          </a>
        </li>
      {/each}
    </ul>
  </nav>

  <div class="flex [&>*]:px-2 h-full  [&>*]:transition-all">
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
