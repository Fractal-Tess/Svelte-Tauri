<script lang="ts">
  import Router from '$router/Router.svelte'
  import { ModeWatcher, mode } from 'mode-watcher'
  import Header from '$lib/components/Header.svelte'
  import { Toaster } from 'svelte-sonner'
</script>

<ModeWatcher defaultMode="dark" />
<Toaster theme={$mode} />

<div class="relative flex flex-col h-screen" data-vaul-drawer-wrapper id="page">
  <Header />
  <main class="flex-1">
    <Router />
  </main>
</div>
