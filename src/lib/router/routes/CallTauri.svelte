<script lang="ts">
  import { fade } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api';

  let message = '';

  async function callTauri() {
    message = await invoke('called_from_js');
  }
</script>

<div class="h-full flex flex-col items-center justify-center">
  <button
    on:click={callTauri}
    class="font-extrabold btn btn-outline btn-primary btn-md">Call Tauri</button
  >
  <div class="h-20 flex items-center">
    {#key message}
      <p
        class="border-b-2 border-accent text-2xl
    whitespace-nowrap"
        in:fade={{ duration: 300 }}
      >
        {message}
      </p>
    {/key}
  </div>
</div>
