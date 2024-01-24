<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window'
  import { faXmark, faWindowMinimize } from '@fortawesome/free-solid-svg-icons'
  import { faGithub } from '@fortawesome/free-brands-svg-icons'
  import Fa from 'svelte-fa'
  import { link } from 'svelte-spa-router'
  import { Button } from '$components/ui/button'
  import { Sun, Moon } from 'lucide-svelte'
  import { toggleMode } from 'mode-watcher'
  import { cn } from '$lib/utils'

  import { location } from 'svelte-spa-router'

  // You can structure your links however you'd like, but I like to keep them in an array of objects
  type Link = {
    label: string
    href: string
  }
  const links: Link[] = [
    {
      label: 'Home',
      href: '/'
    },
    {
      label: 'IPC',
      href: '/#IPC'
    },
    {
      label: 'Versions',
      href: '/#versions'
    }
  ]
</script>

<header
  data-tauri-drag-region
  class="flex h-14 items-center justify-between bg-base-100 shadow-lg sticky top-0 z-50 border-b
   border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
>
  <Button on:click={toggleMode} size="icon" variant="outline" class="ml-2">
    <Sun
      class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
    />
    <Moon
      class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
    />
    <span class="sr-only">Toggle theme</span>
  </Button>

  <nav
    class="flex space-x-4 text-xl font-bold select-none"
    on:dragstart|preventDefault
  >
    {#each links as { href, label }}
      <a
        use:link
        {href}
        class={cn(
          'transition-colors hover:text-foreground/80',
          href === $location ? '' : 'text-foreground/60'
        )}>{label}</a
      >
    {/each}
  </nav>

  <div class="flex h-full [&>*]:px-2 [&>*]:transition-all">
    <a
      on:dragstart|preventDefault
      target="_blank"
      href="https://github.com/Fractal-Tess/Svelte-Tauri"
      class="flex items-center hover:text-secondary"
      rel="noreferrer noopener"
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
</header>
