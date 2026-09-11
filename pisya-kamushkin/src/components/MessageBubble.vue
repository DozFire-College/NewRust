<script setup lang="ts">
import { computed, ref } from "vue";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { Message } from "../types/message.ts";

const props = defineProps<{
  message: Message;
  isOwn: boolean;
}>();

const imageLoaded = ref(true);

const imageSrc = computed(() => {
  if (!props.message.image) return null;
  return convertFileSrc(props.message.image);
});

const hasBody = computed(() => {
  return props.message.body && props.message.body.trim().length > 0;
});

function onImageError() {
  imageLoaded.value = false;
}
</script>

<template>
  <article
      class="message"
      :class="{
    'message--own': isOwn,
    'message--other': !isOwn,
      }"
  >
    <img
        v-if="imageSrc && imageLoaded"
        :src="imageSrc"
        alt="image"
        class="message-image"
        @error="onImageError"
    />
    <p v-if="hasBody"> {{message.body}}</p>
    <footer>
            <span>
              {{message.author}}
            </span>
      <span>
              |
            </span>
      <span>
              {{message.created_at}}
            </span>
    </footer>
  </article>
</template>

<style scoped>
.message{
  max-width: 70%;
  margin: 0;
  padding: 10px 12px;
  border-radius: 10px;
  align-self: flex-end;
}
.message--own{
  align-self: flex-start;
  background: #252830;
}
.message-image{
  max-width: 100%;
  max-height: 300px;
  border-radius: 8px;
  margin-bottom: 8px;
  object-fit: contain;
  display: block;
}
.message p{
  margin: 0;
  line-height: 1.45;
  overflow-wrap:anywhere;
  color: #fff;
}
.message footer{
  display: flex;
  justify-content: flex-end;
  gap: 5px;
  margin-top: 6px;
  color: #b5bbc7;
  font-size: 10px;
}
</style>
