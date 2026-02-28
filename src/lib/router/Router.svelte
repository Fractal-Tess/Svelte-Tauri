<script lang="ts">
  import Router from 'svelte-spa-router'

  import Home from '$lib/router/routes/Home.svelte'
  import IPC from '$lib/router/routes/IPC.svelte'
  import Versions from '$router/routes/Versions.svelte'

  const routes = {
    '/': Home,
    '/#ipc': IPC,
    '/#versions': Versions
  }
</script>

<Router {routes} />
