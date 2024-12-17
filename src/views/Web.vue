<!--
 * @Author: SpenserCai
 * @Date: 2024-12-17 09:56:32
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-17 10:20:25
 * @Description: file content
-->
<template>
  <div>
    <h1>Web</h1>
    <button @click="openWeb">Open</button>
  </div>
</template>

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener';
function openWeb() {
  openUrl('https://www.baidu.com');
}
</script>