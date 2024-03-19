<template>
  <v-container class="fill-height" style="max-width: 21cm; padding-bottom: 0">
    <v-row class="fill-height">
      <v-navigation-drawer theme="dark" permanent rail expand-on-hover>
        <template #prepend>
          <v-list nav density="compact">
            <v-list-item
              prepend-icon="mdi-plus"
              title="New discussion"
              @click="createDiscussion"
            ></v-list-item>
          </v-list>
        </template>
        <v-list nav density="compact" v-model="discussion">
          <v-list-item
            v-for="n in discussions"
            :key="n.value"
            :title="n.title"
            @click="setDiscussion(n)"
          ></v-list-item>
        </v-list>
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
      <v-app-bar density="compact">
        <v-app-bar-title>
          <v-select
            :items="['gpt-3.5-turbo', 'gpt4']"
            v-model="model"
            density="compact"
            single-line
            hide-details
            variant="solo-filled"
          />
        </v-app-bar-title>
      </v-app-bar>
      <div class="message-layout fill-height">
        <span v-if="messages.length === 0" class="empty-message"
          >Ask me anything!</span
        >
        <v-virtual-scroll :items="messages">
          <template v-slot:default="{ item }">
            <div class="message">
              <div class="top">
                <template v-if="item.role === 'user'">
                  <v-avatar size="40" rounded="100%" color="white">
                    <v-icon>mdi-account</v-icon>
                  </v-avatar>
                </template>
                <template v-else>
                  <v-avatar size="40" rounded="100%" color="white">
                    <v-img
                      src="https://tauri.by.simon.hyll.nu/icon.png"
                      cover
                    ></v-img>
                  </v-avatar>
                </template>
                <div class="text" v-html="md.render(item.content)"></div>
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
          v-if="discussion"
          ref="messageForm"
          style="width: 100%"
          @submit.capture.stop.prevent="sendMessage"
        >
          <v-textarea
            v-model="message"
            ref="messageInput"
            variant="outlined"
            append-inner-icon="mdi-comment"
            placeholder="Send a message..."
            row-height="15"
            rows="1"
            auto-grow
            hide-details
            single-line
            type="submit"
            @keydown.enter.exact.prevent.stop="handleEnter"
            @keyup.enter.shift.prevent.capture.stop=""
          ></v-textarea>
        </v-form>
      </div>
    </v-row>
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
  </v-container>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import markdownit from "markdown-it";
import { error, info } from "@tauri-apps/plugin-log";
import { reactive, ref, onBeforeUnmount } from "vue";

interface Message {
  role: string;
  content: string;
}

const message = ref("");
const model = ref("gpt-3.5-turbo");
const discussion = ref(null);
const md = markdownit({ breaks: true });
const messages: Message[] = ref([]);
const messageForm = ref(null);
const showTokenSelector = ref(false);
const token = ref("");
const discussions = ref([]);
for (const id of await invoke("get_discussions")) {
  discussions.value.push({
    title: `Discussion ${id}`,
    value: id,
  });
}

const handleEnter = async function () {
  await sendMessage();
};

//== createDiscussion ==//
const createDiscussion = async function () {
  try {
    const discussionID = await invoke("create_discussion");
    discussions.value = [];
    for (const id of await invoke("get_discussions")) {
      discussions.value.push({
        title: `Discussion ${id}`,
        value: id,
      });
    }
  } catch (error) {
    error(error);
  }
};

//== sendMessage ==//
const sendMessage = async function () {
  messages.value.push({
    role: "user",
    content: message.value,
  });
  try {
    await invoke("prompt", {
      discussion: discussion.value,
      model: model.value,
      message: message.value,
    });
    message.value = "";
  } catch (error) {
    error(error);
  }
};

//== sendMessage ==//
const setDiscussion = async function (id: any) {
  discussion.value = id.value;
  try {
    messages.value = [];
    messages.value = await invoke("get_messages", {
      discussion: discussion.value,
    });
    message.value = "";
  } catch (error) {
    error(error);
  }
};

//== event: message ==//
const unlisten = await listen("message", (event: any) => {
  info("received event:", event);
  messages.value.push(event.payload);
});
onBeforeUnmount(() => {
  unlisten();
});

//== saveToken ==//
const saveToken = async function () {
  // TODO: Send the token to the backend
  try {
    await invoke("connect", { token: token.value });
  } catch (error) {
    error(error);
  }
  // Cleanup after saving
  token.value = "";
  showTokenSelector.value = false;
};
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
  margin-bottom: 15px;
}
.message .top {
  display: flex;
  width: 100%;
  flex-direction: row;
}
.message .top .v-avatar {
  margin: 5px 15px 5px 5px;
}
.message .top .text {
  display: inline-block !important;
  flex-grow: 1;
}
.message .bottom {
  display: block;
  width: 100%;
  margin-left: 50px;
}
.v-toolbar-title {
  margin: 0px !important;
}
.empty-message {
  text-align: center;
  font-size: large;
  color: gray;
  margin-top: 1cm;
}
</style>
