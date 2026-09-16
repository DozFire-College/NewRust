<script setup lang="ts">

import type { User } from "./types/user";

// Импорт 2 функций из vue
// onMounted - запускает код после появления компонента
// ref -  создает быстрые перемещения
import { onMounted, ref } from "vue";

import Database from "@tauri-apps/plugin-sql";

import AppHeader from "./components/AppHeader.vue";

import MessageList from "./components/MessageList.vue";

import MessageComposer from "./components/MessageComposer.vue";

import ChatSidebar from "./components/ChatSidebar.vue";

import type { Chat } from "./types/chats";

import type { Message } from "./types/message";

const oleg: User = {
  id: 1,
  name: "Олег",
};

const kirill: User = {
  id: 2,
  name: "Кирилл",
};

const users: User[] =[
  oleg,
  kirill,
];

const currentUser = ref<User>(oleg);

function selectUser(user: User){
  currentUser.value = user;
}

// Создаем структуру одного сообщения

// Список сообщений, которые vue отображет в диалоге на экране
const messages = ref<Message[]>([]);

const chats = ref<Chat[]>([]);

const activeChat = ref<Chat | null>(null);

const activeChatId = ref(1);

const isEditingChatInfo = ref(false);
const editingTitle = ref("");
const editingSubtitle = ref("");

// Статус подключения к бд
const status = ref("Подключение...")

// Здесь будет подключение к бд (честно), но пока тут null
let db: Database | null = null;

async function loadChats(){
  if (!db) return;

  chats.value = await db.select<Chat[]>(
    "SELECT id, title, subtitle FROM chats ORDER BY id ASC",
  );

  if (chats.value.length > 0){
    await selectChat(chats.value[0]);
  }
}

async function selectChat(chat: Chat){
  activeChat.value = chat;

  activeChatId.value = chat.id;

  await loadMessages(chat.id);
}

async function loadMessages(chatId: number){
  if (!db) return;

  messages.value = await db.select<Message[]>(
    "SELECT id, author, body, created_at, type, attachment FROM messages WHERE chat_id = $1 ORDER BY id ASC",
      [chatId],
  );
}

async function sendMessage(body: string, attachment: string | null = null){
  if (!db) return;

  if (!activeChat.value) return;

  const messageType = attachment ? 'image' : 'text';

  await db.execute(
    `
       INSERT INTO messages (
            chat_id,
            author,
            body,
            type,
            attachment
       )
       VALUES ($1, $2, $3, $4, $5)
    `,
      [
          activeChat.value.id,
          currentUser.value.name,
          body,
          messageType,
          attachment,
      ],
  );
  await loadMessages(activeChat.value.id)
}

async function updateChat(chatId: number, title: string, subtitle: string){
  if (!db) return;

  await db.execute(
    "UPDATE chats SET title = $1, subtitle = $2 WHERE id = $3",
    [title, subtitle, chatId],
  );

  const chatIndex = chats.value.findIndex(c => c.id === chatId);
  if (chatIndex !== -1){
    chats.value[chatIndex] = { ...chats.value[chatIndex], title, subtitle };
  }

  if (activeChat.value && activeChat.value.id === chatId){
    activeChat.value = { ...activeChat.value, title, subtitle };
  }
}

function startEditChatInfo(){
  if (!activeChat.value) return;
  editingTitle.value = activeChat.value.title;
  editingSubtitle.value = activeChat.value.subtitle;
  isEditingChatInfo.value = true;
}

function cancelEditChatInfo(){
  isEditingChatInfo.value = false;
}

async function saveEditChatInfo(){
  if (!activeChat.value) return;
  const newTitle = editingTitle.value.trim() || activeChat.value.title;
  const newSubtitle = editingSubtitle.value.trim();
  await updateChat(activeChat.value.id, newTitle, newSubtitle);
  isEditingChatInfo.value = false;
}

// VUE выполнит код ниже, когда интерфейс программы уже загрузится
onMounted(async()=>{
  try{
    // Открываем бд
    db = await Database.load("sqlite:messenger.db");

    // Загружаем из базы старые сообщения
    await loadChats();

    // Показываем успешеное состоние
    status.value = "История сохраняется локально";
  }catch (error){
    console.error(error);

    status.value = "Ошибка подключения к базе";
  }
});

</script>

<template>
  <main class="app">
    <AppHeader
        :status="status"
        :users="users"
        :current-user="currentUser"
        @select="selectUser"
    />
    <div class="workspace">
      <ChatSidebar
          :chats="chats"
          :active-chat-id="activeChatId"
          @select="selectChat"
      />
      <section class="chat">
        <template v-if="activeChat">
          <div class="chat-info">
            <template v-if="!isEditingChatInfo">
              <div class="chat-info__content">
                <h2>{{ activeChat.title }}</h2>
                <p>{{ activeChat.subtitle }}</p>
              </div>
              <button
                type="button"
                class="chat-info__edit-btn"
                @click="startEditChatInfo"
                title="Изменить название чата"
              >
                ✎
              </button>
            </template>
            <template v-else>
              <div class="chat-info__edit-form">
                <input
                  v-model="editingTitle"
                  type="text"
                  class="chat-info__input chat-info__input--title"
                  placeholder="Название чата"
                  maxlength="50"
                />
                <input
                  v-model="editingSubtitle"
                  type="text"
                  class="chat-info__input chat-info__input--subtitle"
                  placeholder="Описание чата"
                  maxlength="100"
                />
                <div class="chat-info__edit-actions">
                  <button
                    type="button"
                    class="chat-info__btn chat-info__btn--save"
                    @click="saveEditChatInfo"
                  >
                    Сохранить
                  </button>
                  <button
                    type="button"
                    class="chat-info__btn chat-info__btn--cancel"
                    @click="cancelEditChatInfo"
                  >
                    Отмена
                  </button>
                </div>
              </div>
            </template>
          </div>
          <MessageList
              :messages="messages"
              :current-user-name="currentUser.name"
          />
          <MessageComposer @send="(body, attachment) => sendMessage(body, attachment)" />
        </template>
      </section>
    </div>
  </main>
</template>

<style scoped>
/* Все элементы будут использовать одну модель размеров */
:global(*){
  box-sizing: border-box;
}

:global(html){
  background: #111318;
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

  color: #f2f3f5;

  background: #111318;
}

.workspace{
  flex: 1;
  min-height: 0;
  display: flex;
  overflow: hidden;
}

.app{
  height: 100vh;
  display: flex;
  flex-direction: column;
  /*
      Запретит всему app прокручиваться
      Разрешим прокрутку только для MessageList
  */
  overflow: hidden;
}

.chat{
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden; /* Потому что chat целиком не должен прокручиваться, только MessageList внутри него */
}

.chat-info{
  padding: 20px 24px;
  border-bottom: 1px solid #252830;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.chat-info__content{
  flex: 1;
  min-width: 0;
}

.chat-info h2{
  margin: 0;
  font-size: 16px;
}

.chat-info p{
  margin: 5px 0 0;
  color: #858c98;
  font-size: 13px;
}

.chat-info__edit-btn{
  flex-shrink: 0;
  width: 36px;
  height: 36px;
  border: 1px solid #252830;
  border-radius: 8px;
  background: #20232a;
  color: #f2f3f5;
  cursor: pointer;
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s ease, border-color 0.15s ease;
}

.chat-info__edit-btn:hover{
  background: #292c34;
  border-color: #3a3f4b;
}

.chat-info__edit-form{
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.chat-info__input{
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #252830;
  border-radius: 8px;
  background: #111318;
  color: #f2f3f5;
  font: inherit;
  outline: none;
  transition: border-color 0.15s ease;
}

.chat-info__input:focus{
  border-color: #5b5fc7;
}

.chat-info__input--title{
  font-size: 16px;
  font-weight: 600;
}

.chat-info__input--subtitle{
  font-size: 13px;
  color: #858c98;
}

.chat-info__edit-actions{
  display: flex;
  gap: 8px;
  margin-top: 4px;
}

.chat-info__btn{
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font: inherit;
  font-size: 13px;
  font-weight: 500;
  transition: background 0.15s ease;
}

.chat-info__btn--save{
  background: #5b5fc7;
  color: #fff;
}

.chat-info__btn--save:hover{
  background: #6c70d9;
}

.chat-info__btn--cancel{
  background: #20232a;
  color: #f2f3f5;
  border: 1px solid #252830;
}

.chat-info__btn--cancel:hover{
  background: #292c34;
}

</style>










