/*
 * @Author: SpenserCai
 * @Date: 2024-12-16 09:41:49
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-17 09:57:45
 * @Description: file content
 */
import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  {
    path: '/',
    component: () => import('../views/Home.vue'),
  },
  {
    path: '/base64',
    component: () => import('../views/Base64.vue'),
    children: [
      {
        path: 'subfeature1',
        component: () => import('../views/SubFeature1.vue'),
      },
      // 更多子功能路由
    ],
  },
  {
    path: '/bilibili-downloader',
    component: () => import('../views/BilibiliDownloader.vue'),
  },
  // 更多功能路由
  {
    path: '/web',
    component: () => import('../views/Web.vue'),
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;