import vuetify, { transformAssetUrls } from "vite-plugin-vuetify";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  // (optional) Enable the Nuxt devtools
  devtools: { enabled: false },
  // Enable SSG
  ssr: false,
  build: {
    transpile: ["vuetify"],
  },
  modules: [
    (_options, nuxt) => {
      nuxt.hooks.hook("vite:extendConfig", (config) => {
        // @ts-expect-error
        config.plugins.push(vuetify({ autoImport: true }));
      });
    },
  ],
  vite: {
    vue: {
      template: {
        transformAssetUrls,
      },
    },
    // Better support for Tauri CLI output
    clearScreen: false,
    server: {
      // Tauri requires a consistent port
      strictPort: true,
      // Enables the development server to be discoverable by other devices for mobile development
      hmr: {
        // Use websocket for mobile hot reloading
        protocol: "ws",
        // Make sure it's available on the network
        host: "0.0.0.0",
        // Use a specific port for hmr
        port: 5183,
      },
    },
  },
});
