<!--
 * @Author: SpenserCai
 * @Date: 2024-12-16 09:45:48
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-16 15:16:06
 * @Description: file content
-->
<template>
  <div class="flex flex-col items-center space-y-4 h-screen">
    <h1 class="text-4xl font-bold text-center">Base64</h1>
    <div class="flex flex-col w-full h-[30%]">
      <label for="inputText" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Input</label>
      <textarea id="inputText" v-model="inputText" class="flex-grow block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-zinc-700 dark:border-zinc-600 dark:placeholder-zinc-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Input data..."></textarea>
    </div>
    
    <div class="flex flex-col w-full h-[30%] mt-4">
      <label for="outputText" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Output</label>
      <textarea id="outputText" v-model="outputText" class="flex-grow block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-zinc-700 dark:border-zinc-600 dark:placeholder-zinc-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Output data..." readonly></textarea>
    </div>
    <div class="flex space-x-2">
      <button type="button" @click="decode" class="text-gray-900 hover:text-white border border-zinc-800 hover:bg-zinc-900 focus:ring-4 focus:outline-none focus:ring-zinc-300 font-medium rounded-sm text-xs px-5 py-2.5 text-center me-2 mb-2 dark:border-zinc-600 dark:text-gray-400 dark:hover:text-white dark:hover:bg-zinc-600 dark:focus:ring-zinc-800">Decode</button>
      <button type="button" @click="encode" class="text-gray-900 hover:text-white border border-zinc-800 hover:bg-zinc-900 focus:ring-4 focus:outline-none focus:ring-zinc-300 font-medium rounded-sm text-xs px-5 py-2.5 text-center me-2 mb-2 dark:border-zinc-600 dark:text-gray-400 dark:hover:text-white dark:hover:bg-zinc-600 dark:focus:ring-zinc-800">Encode</button>
      <button type="button" @click="copy" class="text-gray-900 hover:text-white border border-zinc-800 hover:bg-zinc-900 focus:ring-4 focus:outline-none focus:ring-zinc-300 font-medium rounded-sm text-xs px-5 py-2.5 text-center me-2 mb-2 dark:border-zinc-600 dark:text-gray-400 dark:hover:text-white dark:hover:bg-zinc-600 dark:focus:ring-zinc-800">Copy</button>
      <button type="button" @click="clear" class="text-gray-900 hover:text-white border border-zinc-800 hover:bg-zinc-900 focus:ring-4 focus:outline-none focus:ring-zinc-300 font-medium rounded-sm text-xs px-5 py-2.5 text-center me-2 mb-2 dark:border-zinc-600 dark:text-gray-400 dark:hover:text-white dark:hover:bg-zinc-600 dark:focus:ring-zinc-800">Clear</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const inputText = ref('')
const outputText = ref('')

const encode = () => {
  outputText.value = btoa(inputText.value)
}

const decode = () => {
  outputText.value = atob(inputText.value)
}

const copy = () => {
  navigator.clipboard.writeText(outputText.value)
}

const clear = () => {
  inputText.value = ''
  outputText.value = ''
}
</script>

<style scoped>
  /* 功能一的样式 */
</style>