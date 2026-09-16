<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { convertFileSrc } from "@tauri-apps/api/core";

const emit = defineEmits<{
  send: [body:string, attachment: string | null];
}>();

const draft = ref("");
const attachment = ref<string | null>(null);
const attachmentPreview = ref<string | null>(null);

async function selectImage() {
const file = await open({
  multiple: false,

  filters: [
    {
      name: "Image",
      extensions: [
        "png",
        "jpg",
        "jpeg",
        "webp",
        "gif"
      ]
    }
  ]
});


if (!file || typeof file !== "string") {
  return;
}
  const savePath = await invoke<string>(
      "save_attachment",
      {
        source:file
      }
  );
  attachment.value = savePath;
  attachmentPreview.value = convertFileSrc(savePath);
}

function clearAttachment() {
  attachment.value = null;
  attachmentPreview.value = null;
}

function submitMessage(){
  const body = draft.value.trim();

  if(!body && !attachment.value) return;

  emit("send", body, attachment.value);

  draft.value = "";
  clearAttachment();
}
</script>

<template>
  <form
      class="composer"
      @submit.prevent="submitMessage"
  >
    <div v-if="attachmentPreview" class="attachment-preview">
      <img :src="attachmentPreview" alt="Preview" />
      <button type="button" class="remove-attachment" @click="clearAttachment">
        ✕
      </button>
    </div>
    <div class="composer-row">
      <input
          v-model="draft"
          type="text"
          placeholder="Ну пиши уже че нить"
          autocomplete="off"
      />
      <button type="button"
      class="image-button"
      @click="selectImage">
        📎
      </button>
      <button type="submit">Отправить</button>
    </div>
  </form>
</template>

<style scoped>

.composer{
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 15px 20px;
  border-top: 1px solid #252830;
  background: #17191f;
  flex-shrink: 0;
}

.composer-row{
  display: flex;
  gap: 10px;
  width: 100%;
}

.attachment-preview{
  position: relative;
  align-self: flex-start;
}

.attachment-preview img{
  max-width: 150px;
  max-height: 150px;
  border-radius: 8px;
  object-fit: cover;
  display: block;
}

.remove-attachment{
  position: absolute;
  top: -8px;
  right: -8px;
  width: 24px;
  height: 24px;
  border-radius: 50% !important;
  padding: 0 !important;
  background: #ff4444 !important;
  font-size: 12px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.composer input{
  flex: 1;
  min-width: 0;
  padding: 11px 13px;
  border: 1px solid #343842;
  border-radius: 7px;
  outline: none;
  color: #f2f3f5;
  background: #20232a;
  font: inherit;
}
.composer input:focus{
  border-color: #4f7fea;
}

.composer button{
  padding: 0 18px;
  border: none;
  border-radius: 7px;
  cursor: pointer;
  color: white;
  background: #386be0;
  font: inherit;
  font-weight: 600;
}
.image-button{
  padding: 0 18px;
  border: none;
  border-radius: 7px;
  cursor: pointer;
  color: white;
  background: #386be0;
  font: inherit;
  font-weight: 600;
}
</style>