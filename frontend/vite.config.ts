import { defineConfig, loadEnv } from "vite";

// https://vite.dev/config/
export default defineConfig(({
  mode,
}) => {
  const env = loadEnv(mode, process.cwd(), '');
  return {
    define: {
      'process.env': env
    },
  };
})
