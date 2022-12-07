<script setup lang="ts">
  import { invoke } from '@tauri-apps/api'
  import { listen } from '@tauri-apps/api/event'
  import { useTracingStore } from '@/stores/trace'

  defineProps<{ msg: string }>()

  function trigger() {
    invoke('greet', { name: 'foo' })
  }

  const tracingStore = useTracingStore()
  const unlisten = await listen('TRACE', (evt: any) => {
    const msg = JSON.parse(evt.payload.message)
    tracingStore.appendTrace(msg.fields?.payload)
  })
  let traces = ref('Nothing to see here. Move along.')
  tracingStore.$subscribe((mutation, state) => {
    traces.value = state.traces.join('\n')
  })
  const allTraces = computed(() => traces.value)
</script>

<template>
  <div class="card">
    <button type="button" @click="trigger">Click Me!</button><br />
    <textarea id="traces" name="traces" rows="25" cols="80" readonly>{{
      allTraces
    }}</textarea>
  </div>
</template>

<style scoped>
  .read-the-docs {
    color: #888;
  }
  textarea {
    font-family: monospace;
  }
</style>
