<script setup lang="ts">
  import {ref} from "vue";

  //define emits сообщает движку vue какие из событий данный компонент
  // имеет право рассылать
  const emit = defineEmits<{
    send: [body:string]
  }>();

  // Текст, который пользователь воодит
  const draft = ref("");

  function submitMessage(){
    const body = draft.value.trim()

    if(!body) return;
    emit("send", body);

    draft.value = ""


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
</style>