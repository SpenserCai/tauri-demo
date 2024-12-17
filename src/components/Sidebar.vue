<!--
 * @Author: SpenserCai
 * @Date: 2024-12-16 09:46:51
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-17 09:58:19
 * @Description: file content
-->
<template>
  <aside id="default-sidebar" class="fixed top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0" aria-label="Sidebar">
    <div class="h-full px-3 py-4 overflow-y-auto bg-zinc-50 dark:bg-zinc-800">
      <ul class="space-y-2 font-medium">
        <SidebarItem v-for="item in menuItems" :key="item.name" :item="item" />
      </ul>
    </div>
</aside>
</template>

<script lang="ts" setup>
import {  onMounted } from "vue";
import { initTWE } from "tw-elements";
import SidebarItem from './SidebarItem.vue';

onMounted(() => {
  initTWE({ });
});

interface MenuItem {
  name: string;
  path?: string;
  children?: MenuItem[];
}

const menuItems: MenuItem[] = [
  {
    name: '首页',
    path: '/',
  },
  {
    name: 'Base64',
    path: '/base64',
  },
  {
    name: 'Bilibili Downloader',
    path: '/bilibili-downloader',
  },
  {
    name: 'Web',
    path: '/web',
  },
];
</script>

<style scoped>
.sidebar {
  width: 200px;
  /* 其他样式 */
}
</style>