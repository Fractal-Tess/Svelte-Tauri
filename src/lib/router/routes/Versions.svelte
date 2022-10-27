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

<h3>Versions</h3>
<div>
  {#await getVersions() then versions}
    {#each Object.entries(versions) as [key, val], i (i)}
      <div>
        {key} - {val}
      </div>
    {/each}
  {/await}
</div>
