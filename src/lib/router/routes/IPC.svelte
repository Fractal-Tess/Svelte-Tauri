<script lang="ts">
  import CallTauri from '$lib/components/IPC/CallTauri.svelte'
  import HashString from '$lib/components/IPC/HashString.svelte'
  import KeyValuePair from '$lib/components/IPC/KeyValuePair.svelte'
</script>

<div class="h-full flex flex-col items-center justify-center gap-y-8">
  <CallTauri />
  <HashString />
  <KeyValuePair />
</div>
