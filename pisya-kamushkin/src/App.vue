<script setup lang="ts">
import { onMounted, ref } from "vue";
import type {User} from "./types/user";
import {Message} from "./types/message.ts";
import Database from "@tauri-apps/plugin-sql";
import AppHeader from "./components/AppHeader.vue";
import MessageComposer from "./components/MessageComposer.vue";
import MessageList from "./components/MessageList.vue";

const oleg: User = {
  id: 1,
  name: "Oleg",
};
const kirill: User = {
  id: 2,
  name: "Кирилл"
};

const users: User[] = [
    oleg,
    kirill,
];

const currentUser = ref<User>(oleg);

function  selectUser(user: User){
  currentUser.value = user;
}

async function sendMessage (payload: { body: string; image: string | null }){
  if (!db) return;

  await db.execute(
      "INSERT INTO messages (author, body, image) VALUES ($1, $2, $3)",
      [
          currentUser.value.name,
          payload.body,
          payload.image,
      ],
  );
  await loadMessages()
}

const messages = ref<Message[]>([]);
const status = ref("Подключение..");
let db: Database | null = null;

async function loadMessages(){
  if (!db) return;
  messages.value = await db.select<Message[]>(
    "SELECT id, author, body, COALESCE(image, NULL) as image, created_at FROM messages ORDER BY id ASC",
  );
}

onMounted(async ()=> {
  try{
    db = await Database.load("sqlite:messenger.db");
    await loadMessages();
    status.value = "История сохраняется локально";
  }catch (error){
    console.error(error);
    status.value = "Ошибка подключения к базе";
  }
});

</script>

<template>

  <main class="App">
    <AppHeader :status="status"
    :users="users"
    :current-user="currentUser"
    @select="selectUser"
    />
    <section class="chat">
      <div class="chat-info">
        <h2>Первый чат</h2>
        <p>Первый локальный мессенджер</p>
      </div>
        <MessageList :messages="messages"
        :current-user-name="currentUser.name"/>
        <MessageComposer @send="sendMessage"/>

    </section>
  </main>

</template>

<style scoped>
:global(*){
  box-sizing: border-box;
}
:global(html){
  background: #ffffff;
  color-scheme: dark;
}
:global(body){
  margin: 0;

  font-family:
      Inter,
      system-ui,
      -apple-system,
      BlinkMacSystemFont,
      "Segoe UI",
      sans-serif;

  color: #000000;

  background: #837b7b;
}
.App{
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}


.chat{
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.chat > :deep(*) {
  min-width: 0;
}
.chat-info{
  padding: 20px 24px;
  border-bottom: 1px solid #000000;
  flex-shrink: 0;
}
.chat-info h2{
  margin: 0;
  font-size: 16px;
}
.chat-info p{
  margin: 5px 0 0;
  color: #045a9f;
  font-size: 13px;
}
</style>
