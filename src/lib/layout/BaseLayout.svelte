<script lang="ts">
  import { fade } from 'svelte/transition';

  import Header from '$lib/components/header/Header.svelte';
  import Footer from '$lib/components/footer/Footer.svelte';
</script>

<div
  class="font-sans bg-base-100 text-base-content h-screen flex flex-col overflow-y-auto overflow-x-hidden"
>
  <Header />
  <main class="flex-1" in:fade={{ delay: 300, duration: 1000 }}>
    <slot />
  </main>
  <Footer />
</div>
