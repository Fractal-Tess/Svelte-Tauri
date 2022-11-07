<script lang="ts">
  import { active } from 'tinro';
  import type { NavLink } from '$types';
  import Fa from 'svelte-fa/src/fa.svelte';

  export let navLinks: NavLink[];
</script>

<nav>
  <ul class="flex space-x-4">
    {#each navLinks as navLink, i (i)}
      <li>
        <a
          use:active
          exact
          data-active-class="border-b-2"
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
