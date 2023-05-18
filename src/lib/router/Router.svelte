<script lang="ts">
  import Router from 'svelte-spa-router';

  import Index from '$lib/router/routes/Index.svelte';
  import IPC from '$lib/router/routes/IPC.svelte';
  import Versions from '$router/routes/Versions.svelte';

  const routes = {
    '/': Index,
    '/#ipc': IPC,
    '/#versions': Versions
  };
</script>

<Router {routes} />
