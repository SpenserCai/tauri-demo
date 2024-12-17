<!--
 * @Author: SpenserCai
 * @Date: 2024-12-17 09:56:32
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-17 11:31:08
 * @Description: file content
-->
<template>
  <div>
    <h1>Web</h1>
    <input type="text" v-model="url" />
    <button @click="openWeb">Open</button>
    <button @click="openWebview">Open Webview</button>
  </div>
</template>

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { ref } from 'vue';
// const webview = new WebviewWindow('my-label', {
//   url: 'https://www.baidu.com',
// });
// 
// webview.once('tauri://created', function () {
//  // webview successfully created
//  console.log('webview successfully created');
// });
// webview.once('tauri://error', function (e) {
//   // an error happened creating the webview
//   console.log('an error happened creating the webview:', e);
// });
const url = ref('');
const webview = ref<WebviewWindow | null>(null);

function openWeb() {
  openUrl(url.value);
}

function openWebview() {
  webview.value = new WebviewWindow('my-label', {
    url: url.value,
  });
  // 网页内居中显示
  webview.value.center();
  
  webview.value.once('tauri://created', function () {
    // webview successfully created
    console.log('webview successfully created');
  });
  webview.value.once('tauri://error', function (e) {
    // an error happened creating the webview
    console.log('an error happened creating the webview:', e);
  });
}
</script>