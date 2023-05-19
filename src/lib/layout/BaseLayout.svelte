<script lang="ts">
  import { fade } from 'svelte/transition';

  import Header from '$layout/BaseHeader.svelte';
</script>

<div
  class="form-control min-h-screen bg-fixed heropattern-topography-black/10 dark:heropattern-topography-white/10">
  <Header />
  <main class="form-control flex-1" in:fade={{ delay: 300, duration: 1000 }}>
    <slot />
  </main>
</div>
