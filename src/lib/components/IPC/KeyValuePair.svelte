<script lang="ts">
  import { storeReadKey, storeSetKey } from '$lib/ipc'
  import { Input } from '$components/ui/input'
  import { Label } from '$components/ui/label'
  import { Button } from '$components/ui/button'
  import { toast } from 'svelte-sonner'

  let key = ''
  let val = ''

  async function setValueWithKey() {
    await storeSetKey(key, val)
    let id = (Math.random() + 1).toString(36).substring(16)
    toast(`You have set the key '${key}' to be the value of '${val}'`, {
      id,
      action: {
        label: 'X',
        onClick: () => {
          toast.dismiss(id)
        }
      }
    })
  }

  async function readValFromKey() {
    const val = await storeReadKey(key)
    let id = (Math.random() + 1).toString(36).substring(16)
    toast(`You have read the key '${key}' to be the value of '${val}'`, {
      id,
      action: {
        label: 'X',
        onClick: () => {
          toast.dismiss(id)
        }
      }
    })
  }
</script>

<div class="flex flex-col gap-4 items-center justify-center">
  <div class="flex gap-4">
    <div class="flex-1 flex-col gap-y-2 flex">
      <Label for="key">Key</Label>
      <Input id="key" bind:value={key} />
    </div>

    <div class="flex-1 flex-col gap-y-2 flex">
      <Label for="val">Value</Label>
      <Input id="val" bind:value={val} />
    </div>
  </div>
  <div class="flex w-full gap-4">
    <Button class="w-full bg-primary" on:click={setValueWithKey}>Set</Button>
    <Button class="w-full" on:click={readValFromKey}>Read</Button>
  </div>
</div>
