<!--
 * @Author: SpenserCai
 * @Date: 2024-12-13 16:39:18
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-16 13:52:25
 * @Description: file content
-->
<script lang="ts" setup>
import Sidebar from './components/Sidebar.vue';

</script>

<template>
  <div class="app-container dark">
    <Sidebar />
    <div class="p-4 sm:ml-64 bg-zinc-750">
      <router-view />
    </div>
  </div>
</template>

<style scoped>

</style>