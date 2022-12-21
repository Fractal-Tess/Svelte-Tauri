<script lang="ts">
  import { Route, router } from 'tinro';
  router.mode.hash();

  import Index from '$lib/router/routes/Index.svelte';
  import CallTauri from '$router/routes/CallTauri.svelte';
  import HashString from '$router/routes/HashString.svelte';
  import Versions from '$router/routes/Versions.svelte';
  import Transition from '$lib/components/transition/Transition.svelte';
</script>

<!-- Fix for linting errors: ->
   https://github.com/AlexxNB/tinro/pull/121/commits/d2251ffed630aac6e76e71856204ead5dd2f6661 -->
<Transition>
  <Route path="/"><Index /></Route>
  <Route path="/call-tauri"><CallTauri /></Route>
  <Route path="/hash-string"><HashString /></Route>
  <Route path="/versions"><Versions /></Route>
</Transition>
