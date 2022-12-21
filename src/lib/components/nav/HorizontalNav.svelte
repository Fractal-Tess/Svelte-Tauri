<script lang="ts">
  import type { NavLink } from '$types';
  import Fa from 'svelte-fa/src/fa.svelte';

  export let navLinks: NavLink[];
</script>

<nav>
  <ul class="flex space-x-4">
    {#each navLinks as navLink, i (i)}
      <li>
        <a
          href={navLink.target.href}
          target={navLink.target.newTab ? '_blank' : ''}
        >
          {#if navLink.content.icon}
            <Fa icon={navLink.content.icon} />
          {/if}
          {navLink.content.text}
        </a>
      </li>
    {/each}
  </ul>
</nav>
