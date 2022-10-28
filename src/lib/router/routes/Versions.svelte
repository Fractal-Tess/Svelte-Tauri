<script lang="ts">
  import { getTauriVersion, getVersion, getName } from '@tauri-apps/api/app';
  type Versions = {
    tauri: string;
    app: string;
    name: string;
  };

  const getVersions = async (): Promise<Versions> => {
    const [tauri, app, name] = await Promise.all([
      getTauriVersion(),
      getVersion(),
      getName()
    ]);
    return {
      tauri,
      app,
      name
    };
  };
</script>

<div
  class="h-full flex flex-col space-y-8 items-center justify-center text-xl font-bold"
>
  <h1 class="text-2xl">Versions</h1>
  <div>
    <ul class="flex flex-col space-y-4">
      {#await getVersions() then versions}
        {#each Object.entries(versions) as [key, val], i (i)}
          <li>
            {key} - {val}
          </li>
        {/each}
      {/await}
    </ul>
  </div>
</div>
