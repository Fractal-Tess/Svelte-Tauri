<script lang="ts">
  import { active } from 'tinro';
</script>

<nav class="h-full">
  <ul class="flex space-x-4 h-full items-center [&>li>a]:p-2">
    <li>
      <a use:active exact data-active-class="border-b-2" href="/">Home</a>
    </li>
    <li>
      <a use:active exact data-active-class="border-b-2" href="/call-tauri"
        >CallTauri</a
      >
    </li>
    <li>
      <a use:active exact data-active-class="border-b-2" href="/hash-string"
        >HashString</a
      >
    </li>
    <li>
      <a use:active exact data-active-class="border-b-2" href="/versions"
        >Versions</a
      >
    </li>
  </ul>
</nav>
