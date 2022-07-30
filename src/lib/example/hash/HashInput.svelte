<script lang="ts">
  import { invoke } from '@tauri-apps/api';

  let hashInput = 'Hello world';
  let hashOutput: string = '';
  $: (async () => {
    hashOutput = await invoke('hash256sum', { hashInput });
  })();
</script>

<span class="text-center -mb-4 text-md">{hashOutput}</span>
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
