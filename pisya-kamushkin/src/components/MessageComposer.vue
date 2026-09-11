<script setup lang="ts">
  import {ref} from "vue";
  import { open } from "@tauri-apps/plugin-dialog";
  import { copyFile, mkdir, exists } from "@tauri-apps/plugin-fs";
  import { join, basename, extname, homeDir } from "@tauri-apps/api/path";
  import { convertFileSrc } from "@tauri-apps/api/core";

  const emit = defineEmits<{
    (e: "send", payload: { body: string; image: string | null }): void;
  }>();

  let closeTimeout: ReturnType<typeof setTimeout> | null = null;
  const draft = ref("");
  const pendingImage = ref<string | null>(null);
  const pendingImageSrc = ref<string | null>(null);

  async function resolveImagesDir(): Promise<string> {
    const home = await homeDir();
    return join(
      home,
      "RustroverProjects",
      "untitled",
      "pisya-kamushkin",
      "images"
    );
  }

  async function ensureImagesDir(): Promise<string> {
    const dir = await resolveImagesDir();
    const dirExists = await exists(dir);
    if (!dirExists) {
      await mkdir(dir, { recursive: true });
    }
    return dir;
  }

  async function generateUniqueFileName(originalPath: string): Promise<string> {
    const ext = await extname(originalPath);
    const base = await basename(originalPath, ext);
    const timestamp = Date.now();
    return `${base}_${timestamp}${ext}`;
  }

  async function handleFileSelect() {
    try {
      const file = await open({
        multiple: false,
        filters: [
          {
            name: "Images",
            extensions: ["png", "jpg", "jpeg", "webp", "gif", "bmp"]
          }
        ]
      });
      if (!file) return;

      const imagesDir = await ensureImagesDir();
      const fileName = await generateUniqueFileName(file as string);
      const destPath = await join(imagesDir, fileName);

      await copyFile(file as string, destPath);

      pendingImage.value = destPath;
      pendingImageSrc.value = convertFileSrc(destPath);

      submitMessage();
    } catch (err) {
      console.error("handleFileSelect failed:", err);
    }
  }

  function clearPendingImage() {
    pendingImage.value = null;
    pendingImageSrc.value = null;
  }

  function submitMessage() {
    const body = draft.value.trim();
    const hasImage = pendingImage.value !== null;

    if (!body && !hasImage) return;

    emit("send", {
      body,
      image: pendingImage.value
    });

    draft.value = "";
    clearPendingImage();
  }

  const emoji: string[] = ['😁','😀','🤡','💩'];
  const isOpen = ref(false);

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
    <div v-if="pendingImageSrc" class="image-preview">
      <img :src="pendingImageSrc" alt="preview" />
      <button
          type="button"
          class="image-preview-remove"
          @click="clearPendingImage"
      >×</button>
    </div>
    <div class="composer-row">
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
      <button type="button" class="ImgAdd" @click="handleFileSelect">📎</button>
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
  border-top: 1px solid #234344 ;
  background: #ff4344;
  flex-shrink: 0;
}
.composer-row{
  display: flex;
  gap: 10px;
  width: 100%;
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
.image-preview{
  position: relative;
  align-self: flex-start;
}
.image-preview img{
  max-height: 120px;
  max-width: 200px;
  border-radius: 8px;
  border: 2px solid #23422f;
  object-fit: cover;
}
.image-preview-remove{
  position: absolute;
  top: -8px;
  right: -8px;
  width: 26px;
  height: 26px;
  padding: 0 !important;
  border-radius: 50% !important;
  background: #ff4344 !important;
  color: #fff !important;
  font-size: 18px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 2px solid #fff !important;
  font-weight: 700 !important;
}
</style>
