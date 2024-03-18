<template>
  <v-container class="fill-height">
    <v-row class="fill-height">
      <v-app-bar density="compact">
        <v-app-bar-title>
          <v-select
            :items="['GPT 3.5', 'GPT 4.0']"
            value="GPT 3.5"
            density="compact"
            single-line
            hide-details
            variant="underlined"
          />
        </v-app-bar-title>
      </v-app-bar>
      <div class="message-layout fill-height">
        <v-virtual-scroll :height="300" :items="messages">
          <template v-slot:default="{ item }">
            <div class="message">
              <div class="top">
                <v-avatar size="60" rounded="100%">
                  <v-img
                    src="https://tauri.by.simon.hyll.nu/icon.png"
                    cover
                  ></v-img>
                </v-avatar>
                <div class="text" v-html="md.render(item.text)"></div>
              </div>
              <div class="bottom">
                <v-btn size="x-small" variant="plain">
                  <v-icon>mdi-clipboard</v-icon>
                </v-btn>
                <v-btn size="x-small" variant="plain">
                  <v-icon>mdi-reload</v-icon>
                </v-btn>
                <v-btn size="x-small" variant="plain">
                  <v-icon>mdi-close</v-icon>
                </v-btn>
              </div>
            </div>
          </template>
        </v-virtual-scroll>
        <v-form
          ref="messageForm"
          style="width: 100%"
          @submit.capture.stop.prevent="sendMessage"
        >
          <v-textarea
            v-model="message"
            ref="messageInput"
            variant="outlined"
            append-inner-icon="mdi-comment"
            placeholder="Send me a message..."
            row-height="15"
            rows="1"
            auto-grow
            hide-details
            type="submit"
            @keydown.enter.exact.prevent.stop="handleEnter"
            @keyup.enter.shift.prevent.capture.stop=""
          ></v-textarea>
        </v-form>
      </div>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import markdownit from "markdown-it";
import { reactive, ref } from "vue";
const message = ref("");
const md = markdownit({ breaks: true });
const messages: any[] = reactive([]);
const messageForm = ref(null);
const handleEnter = async function () {
  await sendMessage();
};
//== sendMessage ==//
const sendMessage = async function () {
  messages.push({
    from: "user",
    text: message.value,
    time: new Date().toLocaleDateString(),
  });
  message.value = "";
  // TODO: Send the message to the backend
};
//== event: message ==//
// TODO: Set up an event listener
</script>

<style>
.message-layout {
  display: flex;
  flex-grow: 1;
  flex-direction: column;
  height: 100%;
}
.message-layout .v-virtual-scroll {
  overflow-x: auto;
  max-height: calc(100vh - 48px - 56px);
}
.message {
  display: inline-block;
}
.message .top {
  display: block;
  width: 100%;
}
.message .top .v-avatar {
  margin: 0 15px 5px 0;
}
.message .top .text {
  display: inline-block !important;
  max-width: calc(100% - 75px);
}
.message .bottom {
  display: block;
  width: 100%;
}
</style>
