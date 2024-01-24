<script lang="ts">
  import { Input } from '$components/ui/input'
  import { Label } from '$components/ui/label'
  import { hash256sum } from '$lib/ipc'

  let hashInput = 'String to hash'
  let hashOutput = ''

  $: hash256sum(hashInput).then(hash => (hashOutput = hash))
</script>

<section class="flex items-center justify-center flex-col gap-8">
  <div class="flex flex-col w-full max-w-sm gap-1.5">
    <Label for="hash">Hash string</Label>
    <Input
      placeholder="String to hash"
      id="hash"
      bind:value={hashInput}
      type="text"
      class="input-bordered input-secondary input focus:border-secondary
    focus:outline-none focus:ring-secondary"
    />
  </div>
  <code class="border border-border rounded-md px-2 py-1 text-sm"
    >{hashOutput}</code
  >
</section>
