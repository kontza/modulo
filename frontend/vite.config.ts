import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {
  const env = loadEnv(mode, process.cwd() + '/..', '')
  return {
    server: {
      strictPort: true,
      port: env.SERVER_PORT,
    },
    plugins: [vue()],
  }
})