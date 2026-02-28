<script lang="ts">
  import { commands } from '$lib/ipc'
  import { Input } from '$components/ui/input'
  import { Label } from '$components/ui/label'
  import { Button } from '$components/ui/button'
  import { toast } from 'svelte-sonner'

  let key = $state('')
  let val = $state('')
  let enableSetKey = $state(false)
  let enableReadKey = $state(false)

  $effect(() => {
    if (key.length > 0 && val.length > 0) {
      enableSetKey = true
    }
  })
  $effect(() => {
    if (key.length > 0) {
      enableReadKey = true
    }
  })

  async function setValueWithKey() {
    await commands.storeSetKey(key, val)
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
    const val = await commands.storeReadKey(key)
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
  <div class="grid gap-4 grid-cols-2 w-full">
    <div class="flex-1 flex-col gap-y-2 flex">
      <Label for="key">Key</Label>
      <Input id="key" bind:value={key} />
    </div>

    <div class="flex-1 flex-col gap-y-2 flex">
      <Label for="val">Value</Label>
      <Input id="val" bind:value={val} />
    </div>
    <Button disabled={!enableSetKey} onclick={setValueWithKey}>Set</Button>
    <Button disabled={!enableReadKey} onclick={readValFromKey}>Read</Button>
  </div>
</div>
