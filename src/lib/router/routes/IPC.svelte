<script lang="ts">
  import { fade } from 'svelte/transition';
  import { hash256sum, helloTauri, storeSetKey, storeReadKey } from '$ipc';

  // Or you can also do this if you prefer

  // import * as ipc from '$ipc';
  // message = await ipc.helloTauri()

  // Calling the hash256 function any time the `hashInput` variable  changes
  let hashInput = 'Hello world';
  let hashOutput = '';
  $: (async () => {
    hashOutput = await hash256sum(hashInput);
  })();

  // Calling the `helloTauri` when we click the call tauri button
  let message = '';
  async function callTauri() {
    message = await helloTauri();
  }

  let storeMessage = '';
  let key = '';
  let val = '';
  async function setKeyVal() {
    await storeSetKey(key, val);
    storeMessage = `You have set the key '${key}' to be the value of '${val}''`;
  }

  async function readValFromKey() {
    const val = await storeReadKey(key);
    storeMessage = `Using the key '${key}', you have just retrieved the value of '${val}''`;
  }
</script>

<div
  class="form-control h-full flex-1 items-center justify-center gap-y-8
  [&>section]:form-control [&>section]:items-center [&>section]:justify-center [&>section]:gap-y-4">
  <section class="">
    <button
      on:click={callTauri}
      class="btn-outline btn-primary btn text-2xl capitalize"
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

  <div class="grid grid-cols-2 gap-4">
    <label class="input-group flex max-w-max">
      <span>Key</span>
      <input
        bind:value={key}
        type="text"
        class="input-bordered input-secondary input focus:border-secondary focus:outline-none focus:ring-secondary" />
    </label>
    <label class="input-group flex max-w-max">
      <span>Value</span>
      <input
        bind:value={val}
        type="text"
        class="input-bordered input-secondary input focus:border-secondary focus:outline-none focus:ring-secondary" />
    </label>

    <button on:click={setKeyVal} class="btn-outline btn-primary btn capitalize"
      >Set key</button>
    <button
      on:click={readValFromKey}
      class="btn-outline btn-primary btn capitalize">Read key</button>
    <p class="text-bold font-2xl col-span-full text-center text-primary">
      {storeMessage}
    </p>
  </div>
</div>
