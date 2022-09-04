<script lang="ts">
  import CallTauri from '$lib/components/CallTauri.svelte';
  import HashInput from '$lib/components/HashInput.svelte';
</script>

<div class="flex flex-col gap-8 items-center font-bold drop-shadow-ft">
  <h1 class="text-4xl mt-16">Welcome</h1>
  <h2 class="text-xl">
    This is a <span class="bg-secondary text-secondary-content rounded-md p-1"
      >Svelte Tauri</span
    > Template
  </h2>

  <CallTauri />
  <HashInput />
</div>
