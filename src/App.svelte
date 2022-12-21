<script lang="ts">
  import { theme } from '$lib/stores/theme';
  import Router from '$lib/router/Router.svelte';
  import BaseLayout from '$lib/layout/BaseLayout.svelte';

  $: {
    document.documentElement.setAttribute('data-theme', $theme);
    document.documentElement.classList.value = $theme;
  }
</script>

<BaseLayout>
  <Router />
</BaseLayout>
