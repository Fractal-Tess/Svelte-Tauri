<script lang="ts">
  import svelte_logo from '$assets/svelte_logo.svg';
  import tauri_logo from '$assets/tauri_logo.svg';
</script>

<div
  class="flex flex-col space-y-6 items-center justify-center font-bold drop-shadow-ft h-full"
>
  <h1 class="text-6xl">Welcome</h1>
  <h2 class="text-3xl flex items-center">
    This is a &#160
    <span>
      <img src={svelte_logo} alt="svelte logo" class="h-12" />
    </span>
    &#160-&#160
    <span>
      <img src={tauri_logo} alt="svelte logo" class="h-12" />
    </span>
    &#160 Template
  </h2>
</div>
