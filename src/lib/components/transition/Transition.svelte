<script>
  import { router } from 'tinro';
  import { fade } from 'svelte/transition';
</script>

{#key $router.path}
  <div in:fade={{ duration: 700 }} class="h-full w-full">
    <slot />
  </div>
{/key}
