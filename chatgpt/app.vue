<template>
  <NuxtLayout />
</template>

<script>
import { attachConsole } from "@tauri-apps/plugin-log";
const detach = await attachConsole();
</script>

<style>
html,
body {
  overscroll-behavior: none;
  background-color: #111;
}
</style>
