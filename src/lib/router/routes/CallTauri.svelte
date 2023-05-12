<script lang="ts">
  import { fade } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api';

  let hashInput = 'Hello world';
  let hashOutput = '';
  $: (async () => {
    hashOutput = await invoke('hash256sum', { hashInput });
  })();

  let message = '';

  async function callTauri() {
    message = await invoke('called_from_js');
  }
</script>

<div
  class="form-control h-full flex-1 items-center justify-center gap-y-8
  [&>section]:form-control [&>section]:items-center [&>section]:justify-center [&>section]:gap-y-4">
  <section class="">
    <button
      on:click={callTauri}
      class="btn-outline btn-primary btn-md btn font-extrabold"
      >Call Tauri</button>
    <div class="flex h-20 items-center">
      {#key message}
        <p
          class="whitespace-nowrap border-b-2 border-accent
    text-2xl"
          in:fade={{ duration: 300 }}>
          {message}
        </p>
      {/key}
    </div>
  </section>
  <section>
    <label class="input-group flex max-w-max">
      <span>Hash string</span>
      <input
        bind:value={hashInput}
        type="text"
        class="input-bordered input-secondary input focus:border-secondary focus:outline-none focus:ring-secondary" />
    </label>
    <p class="text-center text-lg">{hashOutput}</p>
  </section>
</div>
