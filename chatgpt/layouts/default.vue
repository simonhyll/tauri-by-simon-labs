<template>
  <v-app theme="dark">
    <v-navigation-drawer theme="dark" permanent rail expand-on-hover>
      <template #prepend>
        <v-list nav density="compact">
          <v-list-item
            prepend-icon="mdi-plus"
            title="New conversation"
            value="inbox"
          ></v-list-item>
        </v-list>
      </template>
      <v-list nav :items="items" density="compact" />
      <template #append>
        <v-list nav density="compact">
          <v-list-item
            @click="showTokenSelector = !showTokenSelector"
            prepend-icon="mdi-account"
            title="Account"
          ></v-list-item>
        </v-list>
      </template>
    </v-navigation-drawer>
    <v-main>
      <NuxtPage />
    </v-main>
    <v-dialog v-model="showTokenSelector" max-width="10cm" persistent>
      <v-card v-if="showTokenSelector">
        <v-card-title>API Token</v-card-title>
        <v-card-text>
          <v-text-field
            v-model="token"
            variant="solo-filled"
            type="password"
            single-line
            hide-details
            placeholder="ChatGPT API token..."
            @keydown.enter.prevent="saveToken"
          ></v-text-field>
        </v-card-text>
        <v-card-actions>
          <v-btn
            color="error"
            variant="text"
            @click="
              showTokenSelector = false;
              token = '';
            "
          >
            Close
          </v-btn>
          <v-spacer></v-spacer>
          <v-btn color="success" variant="text" @click="saveToken">
            Save
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-app>
</template>

<script setup lang="ts">
import { ref } from "vue";
const showTokenSelector = ref(false);
const token = ref("");
const items = [
  { type: "subheader", title: "Conversations" },
  {
    title: "Item #1",
    value: 1,
  },
];
//== saveToken ==//
const saveToken = async function () {
  // TODO: Send the token to the backend
  // Cleanup after saving
  token.value = "";
  showTokenSelector.value = false;
};
</script>

<style>
html,
body {
  overscroll-behavior: none;
  background-color: #111;
}
</style>
