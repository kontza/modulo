import { defineStore } from 'pinia'

interface State {
  traces: string[]
}

export const useTracingStore = defineStore('tracing', {
  state: (): State => ({
    traces: [],
  }),
  getters: {
    getTraces: (state: State) => state.traces,
  },
  actions: {
    appendTrace(trace: string) {
      this.traces?.push(trace)
    },
  },
})
