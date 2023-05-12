<script lang="ts">
  import Router from 'svelte-spa-router';

  import Index from '$lib/router/routes/Index.svelte';
  import CallTauri from '$router/routes/CallTauri.svelte';
  import Versions from '$router/routes/Versions.svelte';

  const routes = {
    '/': Index,
    '/#call_tauri': CallTauri,
    '/#versions': Versions
  };
</script>

<Router {routes} />
