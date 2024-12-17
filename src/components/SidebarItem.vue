<!--
 * @Author: SpenserCai
 * @Date: 2024-12-16 09:52:28
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-16 23:39:20
 * @Description: file content
-->
<script lang="ts" setup>
import { defineProps } from 'vue';

interface MenuItem {
  name: string;
  path?: string;
  children?: MenuItem[];
}

defineProps<{
  item: MenuItem;
}>();
</script>

<template>
  <li>
    <div>
      <router-link class="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-zinc-100 dark:hover:bg-zinc-700 group" v-if="item.path" :to="item.path">
        <span class="ms-3">{{ item.name }}</span>
      </router-link>
      <a href="#" class="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-zinc-100 dark:hover:bg-zinc-700 group" v-else>
        <span class="ms-3">{{ item.name }}</span>
      </a>
    </div>
    <ul v-if="item.children && item.children.length">
      <SidebarItem v-for="child in item.children" :key="child.name" :item="child" />
    </ul>
  </li>
</template>


<style scoped>
li {
  list-style: none;
  /* 其他样式 */
}
</style>