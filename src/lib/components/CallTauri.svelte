<script lang="ts">
  import { fade } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api';

  let message = '';

  async function callTauri() {
    message = await invoke('called_from_js');
  }
</script>

<button
  on:click={callTauri}
  class=" font-extrabold btn btn-outline btn-secondary btn-md"
  >Call Tauri</button
>
<div class="relative">
  {#key message}
    <p
      class="absolute outline-secondary border-b-2 border-secondary text-2xl
     whitespace-nowrap
      -translate-x-1/2 -translate-y-1/2"
      in:fade={{ duration: 300 }}
    >
      {message}
    </p>
  {/key}
</div>
