<script setup lang="ts">
// Импорт 2 функций из vue
// on Mounted - запускает код после появления компонента
//ref - создаёт быстрые перемещения
import { onMounted, ref } from "vue";
import type {User} from "./types/user";
import {Message} from "./types/message.ts";
import Database from "@tauri-apps/plugin-sql";
import AppHeader from "./components/AppHeader.vue";
import MessageComposer from "./components/MessageComposer.vue";
import MessageList from "./components/MessageList.vue";
//Создаем структуру одного сообщения

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

async function sendMessage (body: string){
  if (!db) return;

  await db.execute(
      "INSERT INTO messages (author, body) VALUES ($1, $2)",
      [
          currentUser.value.name,
          body,
      ],
  );
  await loadMessages()
}

//Список сообщений, которые vue отображает в диалоге на экране

const messages = ref<Message[]>([]);

//статус подключения
const status = ref("Подключение..");

// Здесь будет подключение к бд (честно), но пока null

let db: Database | null = null;

// Асинхронная функция загрузки сообщений из SQL
async function loadMessages(){
  // Если база ещё не подключена, прерываем выполнение
  if (!db) return;
  // читаем данные из таблицы message
  messages.value = await db.select<Message[]>(
    "SELECT id, author, body, created_at FROM messages ORDER BY id ASC",
  );
}

// Функция отправки нового сообщения

  // После отправки очищаем поле ввода

  //Обновляем историю сообщений в чате

// Vue выполнит код ниже, когда интерфейс программы уже загрузится
onMounted(async ()=> {
  try{
    //Открываем базу данных
    db = await Database.load("sqlite:messenger.db");

    // Загружаем из базы старые сообщения
    await loadMessages();

    // /Показываем успешное состояние
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
        <!-- Vue создаёт article для каждого сообщения из базы -->

    </section>
  </main>

</template>

<style scoped>
/* все элементы будут использовать одну модель размера */

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
  /* Запрещает всему app прокручиваться
    разрешим прокрутку только для messageList
  */
  overflow: hidden;
}


.chat{
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden; /* Потому что чат не должен прокручиваться, только MessageList*/

}
.chat-info{
  padding: 20px 24px;
  border-bottom: 1px solid #000000;
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
.messages{
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 24px;
}

.empty{
  margin: auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
  text-align: center;
  color: #313443;

}
.message{
  align-self: flex-end;
  max-width: 70%;
  margin: 0;
  padding: 10px 12px;
  border-radius: 10px;
  background: #f34242;
}
.message p{
  margin: 0;
  line-height: 1.45;
  overflow-wrap:anywhere;
}
.message footer{
  display: flex;
  justify-content: flex-end;
  gap: 5px;
  margin-top: 6px;
  color: #413431;
  font-size: 10px;
}

</style>