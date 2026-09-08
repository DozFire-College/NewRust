<script setup lang="ts">

//defineProps - спец конструкция vue которая сообщает:
// Этот компонент ожидает получения данных от родительского компонента
defineProps <{
  status: string;
}>();

</script>

<template>
  <header class="header">
    <div>
      <h1>Pisya Kamushkin</h1>
      <p>{{status}}</p>
    </div>

    <span class="badge">Локально </span>

  </header>
</template>

<style scoped>
  /*
  CSS Этого блока будет относиться только к текущему vue компоненту
  Например .header не повлияет на любой другой .header в коде вне этого компонента
   */
  .header{
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 18px 24px;
    border-bottom: 1px solid #292c34;
    background: #17191F;
    /* Управляет тем, может ли flex уменьшать элемент */
    flex-shrink: 0;
  }
  .header h1{
    margin: 0;
    font-size: 18px;
  }
  .header p{
    margin: 4px 0 0;
    color: #8f96a3
  }
  .badge{
    padding: 6px 10px;
    border: 1px solid #343842;
    border-radius: 6px;
    color: #afb5c0;
    background: #20232A;
    font-size: 12px;
  }
</style>