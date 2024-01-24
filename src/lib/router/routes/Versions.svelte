<script lang="ts">
  import { getTauriVersion, getVersion, getName } from '@tauri-apps/api/app'
  type Versions = {
    tauri: string
    app: string
    name: string
  }

  const getVersions = async (): Promise<Versions> => {
    const [name, tauri, app] = await Promise.all([
      getName(),
      getTauriVersion(),
      getVersion()
    ])
    return {
      tauri,
      app,
      name
    }
  }
</script>

<section class="h-full flex flex-col items-center justify-center gap-10">
  <h1 class="text-5xl font-extrabold italic">Versions</h1>
  <ul class="flex flex-col gap-2">
    {#await getVersions() then versions}
      {#each Object.entries(versions) as [key, val]}
        <li class="text-xl font-bold">
          {key} - {val}
        </li>
      {/each}
    {/await}
  </ul>
</section>
