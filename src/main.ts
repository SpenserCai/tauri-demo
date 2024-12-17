/*
 * @Author: SpenserCai
 * @Date: 2024-12-13 16:39:18
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-16 10:09:02
 * @Description: file content
 */
import { createApp } from "vue";
import App from "./App.vue";
import "./style.css"; // 引入 Tailwind CSS
import router from './router';

createApp(App).use(router).mount("#app");
