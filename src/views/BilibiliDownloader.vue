<!--
 * @Author: SpenserCai
 * @Date: 2024-12-16 09:45:48
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-16 23:03:31
 * @Description: file content
-->
<template>
  <div class="flex flex-col items-center space-y-4 h-screen">
    <h1 class="text-4xl font-bold text-center">Bilibili Downloader</h1>
    <div class="flex w-full justify-center items-center space-x-2">
      <input v-model="bili_url" type="text" placeholder="Enter Bilibili URL" class="relative m-0 block flex-auto rounded-s border border-solid border-gray-300 bg-gray-50 bg-clip-padding px-3 py-2 text-base font-normal leading-[1.6] text-gray-900 outline-none transition duration-200 ease-in-out placeholder-gray-400 focus:z-[3] focus:border-primary focus:shadow-inset focus:outline-none motion-reduce:transition-none dark:border-zinc-600 dark:bg-zinc-700 dark:text-white dark:placeholder-gray-400 dark:autofill:shadow-autofill dark:focus:border-primary" />
      <button @click="get_real_url" class="text-gray-900 hover:text-white border border-zinc-800 hover:bg-zinc-900 focus:ring-4 focus:outline-none focus:ring-zinc-300 font-medium rounded-sm text-base px-3 py-2 text-center dark:border-zinc-600 dark:text-gray-400 dark:hover:text-white dark:hover:bg-zinc-600 dark:focus:ring-zinc-800">Get Real URL</button>
    </div>
    <div class="flex flex-col w-full h-[50%] mt-4">
      <textarea v-model="real_url" class="flex-grow block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-zinc-700 dark:border-zinc-600 dark:placeholder-zinc-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Output data..." readonly></textarea>
    </div>
  </div>
</template>
  
<script setup lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { ref } from 'vue'

  const bili_url = ref('');
  const real_url = ref('');

  async function get_real_url() {
    real_url.value = await invoke('get_bilibili_real_url', { url: bili_url.value });
  }
</script>
  
<style scoped>
  /* 功能一的样式 */
</style>