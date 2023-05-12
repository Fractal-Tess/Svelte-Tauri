<script lang="ts">
  import { getTauriVersion, getVersion, getName } from '@tauri-apps/api/app';
  type Versions = {
    tauri: string;
    app: string;
    name: string;
  };

  const getVersions = async (): Promise<Versions> => {
    const [tauri, app, name] = await Promise.all([
      getName(),
      getTauriVersion(),
      getVersion()
    ]);
    return {
      tauri,
      app,
      name
    };
  };
</script>

<div
  class="form-control flex-1 items-center justify-center space-y-8 text-2xl font-bold">
  <h1 class="text-5xl text-primary">Versions</h1>
  <div>
    <ul class="form-control space-y-4">
      {#await getVersions() then versions}
        {#each Object.entries(versions) as [key, val]}
          <li>
            <span class="text-secondary"> {key}</span> -
            <span class="text-primary"> {val}</span>
          </li>
        {/each}
      {/await}
    </ul>
  </div>
</div>
