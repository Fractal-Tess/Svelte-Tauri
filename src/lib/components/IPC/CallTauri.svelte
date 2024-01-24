<script lang="ts">
  import { Button } from '$components/ui/button'
  import { helloTauri } from '$lib/ipc'
  import { cn } from '$lib/utils'

  let message = ''
  async function callTauri() {
    message = await helloTauri()
  }
</script>

<Button variant="outline" class="px-8 py-6" on:click={callTauri}
  >Call Tauri</Button
>

<p
  class={cn(
    'text-2xl font-bold border-b-2 border-border transition-opacity min-h-10 duration-500',
    message ? 'opacity-100' : 'opacity-0'
  )}
>
  {message}
</p>
