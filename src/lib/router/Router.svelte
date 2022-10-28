<script lang="ts">
  import { Route } from 'tinro';

  import Index from '$lib/router/routes/Index.svelte';
  import CallTauri from '$router/routes/CallTauri.svelte';
  import HashString from '$router/routes/HashString.svelte';
  import Versions from '$router/routes/Versions.svelte';
</script>

<!-- The routes give off an error: Here is a fix -> https://github.com/AlexxNB/tinro/pull/121/commits/d2251ffed630aac6e76e71856204ead5dd2f6661 -->
<Route path="/"><Index /></Route>
<Route path="/call-tauri"><CallTauri /></Route>
<Route path="/hash-string"><HashString /></Route>
<Route path="/versions"><Versions /></Route>
