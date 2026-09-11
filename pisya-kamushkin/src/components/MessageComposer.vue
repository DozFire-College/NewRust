<script setup lang="ts">
  import {ref} from "vue";
  import { open } from "@tauri-apps/plugin-dialog";

  //define emits сообщает движку vue какие из событий данный компонент
  // имеет право рассылать
  const emit = defineEmits<{
    send: [body:string]
  }>();
  let closeTimeout: ReturnType<typeof setTimeout> | null = null;
  // Текст, который пользователь воодит
  const draft = ref("");
  const imagePath = ref<string | null>(null);
  async function sendFile (){
    const file = await open({
      multiple: false,
      filters: [
        {
          name: "Images",

          extensions: [
            "png",
            "jpg",
            "jpeg",
            "webp"
          ]
        }
      ]
    });
    if (!file) return;

    imagePath.value = file as string;


    emit("send", `${file}`);
  }
  function submitMessage(){
    const body = draft.value.trim()

    if(!body) return;
    emit("send", body);

    draft.value = ""
  }
  const emoji: string[] = ['😁','😀','🤡','💩'];

  const isOpen = ref(false)

  function addEmoji(emojiChar: string) {
    draft.value += emojiChar;
  }
  function openEmojiList() {

    if (closeTimeout) {
      clearTimeout(closeTimeout);
      closeTimeout = null;
    }
    isOpen.value = true;
  }

  function CloseEmojiList() {

    closeTimeout = setTimeout(() => {
      isOpen.value = false;
      closeTimeout = null;
    }, 500);
}
</script>

<template>
  <form
      class="composer"
      @submit.prevent="submitMessage()"
  >
    <input
        v-model="draft"
        type="text"
        placeholder="Ну пиши уже че нить"
        autocomplete="off"
    />
    <div class="emoji-container" @mouseenter="openEmojiList" @mouseleave="CloseEmojiList">
      <button type="button" class="emoji" @click="isOpen = true" >🙂</button>
      <div v-if="isOpen" class="emoji-list">
        <button
            v-for="(emojiChar, index) in emoji"
            :key="index"
            type="button"
            class="emoji-item"
            @click="addEmoji(emojiChar)"
        >
          {{ emojiChar }}
        </button>
      </div>
    </div>
    <button class="ImgAdd" @click="sendFile">📎</button>

    <button type="submit">Отправить</button>
  </form>
</template>

<style scoped>
.composer{
  display: flex;
  gap: 10px;
  padding: 15px 20px;
  border-top: 1px solid #234344 ;
  background: #ff4344;
  flex-shrink: 0;
}
.composer input{
  flex: 1;
  min-width: 0;
  padding: 11px 13px;
  border: 1px solid #135334;
  border-radius: 7px;
  outline: none;
  color: #ffffff;
  background: #23422f;
  font: inherit;
}
.composer input:focus{
  border-color: #4f3f ;

}
.composer button{
  padding: 0 18px;
  border: none;
  border-radius: 7px;
  cursor: pointer;
  color: #000000;
  background: #F3FF33;
  font: inherit;
  font-weight: 600;
}
.emoji-container {
  position: relative;
  display: inline-block;
}
.emoji{
  padding: 18px;
  border: none;
  border-radius: 7px;
  cursor: pointer;
  color: #000000;
  background: #F3FF33;
  font: inherit;
  font-weight: 600;
}
.emoji-list {
  position: absolute;
  bottom: calc(100% + 5px);
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 5px;
  padding: 8px;
  background: #23422f;
  border: 1px solid #135334;
  border-radius: 7px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
  z-index: 1000;
}
.emoji-item {
  padding: 8px 10px;
  font-size: 20px;
  cursor: pointer;
  transition: transform 0.2s, background-color 0.2s;
}
.emoji-item:hover {
  transform: scale(1.2);
  background-color: #2a4d38;
}
.ImgAdd{
  padding: 18px;
  border: none;
  border-radius: 7px;
  cursor: pointer;
  color: #000000;
  background: #F3FF33;
  font: inherit;
  font-weight: 600;
}
</style>