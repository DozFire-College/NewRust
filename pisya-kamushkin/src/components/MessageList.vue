<script setup lang="ts">
import {
  nextTick, // позволяет дождаться момента когда vue обновит html
    onMounted,
    useTemplateRef, // даёт возможность получить ссылку на html-элемент из template
    watch, // позволяет следить за изменением выбранных данных
} from "vue";
import MessageBubble from "./MessageBubble.vue";
import type { Message} from "../types/message.ts";

const props = defineProps< {
messages: Message[];
currentUserName: string;

}>();

const bottomAnchor = useTemplateRef<HTMLDivElement>("bottom-anchor");

async function scrollToBottom(){
  /* Нужно дождаться обновления DOM  */
  await nextTick();

  bottomAnchor.value?.scrollIntoView({
    behavior: "smooth",
    block: "end"
  });
}

function getMessageCount(){
  return props.messages.length;

}
watch(
  getMessageCount,
    scrollToBottom
    );

onMounted(scrollToBottom);
</script>

<template>
  <div class="message-list-root">
    <div class="messages">
      <div class="messages-inner">
        <div
            v-if="messages.length === 0"
            class="empty"
        >
          <strong>Здесь пока пусто</strong>
          <span>Напишите первое сообщение</span>
        </div>
        <MessageBubble
            v-for="message in messages"
            :key="message.id"
            :message="message"
            :is-own="message.author === currentUserName"
        />
        <div
        ref="bottomAnchor"
        class="bottomAnchor"
        aria-hidden="true">
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.message-list-root{
  flex: 1 1 auto;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.bottomAnchor{
  height: 1px;
  flex-shrink: 0;
}
.messages{
  flex: 1 1 auto;
  min-height: 0;
  overflow-y: auto;
  padding: 24px;
}
.messages-inner{
  min-height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
  gap: 10px;
}
.empty{
  margin: auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
  text-align: center;
  color: #313443;
}
</style>