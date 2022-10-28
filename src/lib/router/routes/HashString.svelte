<script lang="ts">
  import { invoke } from '@tauri-apps/api';

  let hashInput = 'Hello world';
  let hashOutput = '';
  $: (async () => {
    hashOutput = await invoke('hash256sum', { hashInput });
  })();
</script>

<div class="h-full flex flex-col space-y-8 items-center justify-center">
  <div class="form-control">
    <label class="input-group">
      <span>Hash string</span>
      <input
        bind:value={hashInput}
        type="text"
        class="input input-bordered input-secondary focus:ring-secondary focus:border-secondary focus:outline-none"
      />
    </label>
  </div>
  <p class="text-center text-lg">{hashOutput}</p>
</div>
