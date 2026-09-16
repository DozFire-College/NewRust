<script setup lang="ts">

import type { Message } from "../types/message";
import { computed } from "vue";
import { convertFileSrc } from "@tauri-apps/api/core";

const props = defineProps<{
  message: Message;
  isOwn: boolean;
}>();

const attachmentSrc = computed(() => {
  if (props.message.attachment) {
    return convertFileSrc(props.message.attachment);
  }
  return null;
});

const isImage = computed(() => {
  if (!props.message.attachment) return false;
  const ext = props.message.attachment.split('.').pop()?.toLowerCase();
  return ['png', 'jpg', 'jpeg', 'gif', 'webp'].includes(ext || '');
});
</script>

<template>
  <article
      class="message"
      :class="{
        'message--own': isOwn,
        'message--other': !isOwn,
      }"
  >
    <div v-if="isImage && attachmentSrc" class="attachment-image">
      <img :src="attachmentSrc" alt="Attachment" />
    </div>
    <p v-if="message.body">
      {{message.body}}
    </p>
    <footer>
            <span>
              {{ message.author}}
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
}
.message--own{
  align-self: flex-end;
  background: #386be0;
}
.message--other{
  align-self: flex-start;
  background: #252830;
}

.attachment-image{
  margin-bottom: 8px;
}

.attachment-image img{
  max-width: 100%;
  max-height: 300px;
  border-radius: 8px;
  display: block;
  object-fit: contain;
}

.message p{
  margin: 0;
  line-height: 1.45;
  overflow-wrap: anywhere;
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