/*
 * @Author: SpenserCai
 * @Date: 2024-12-15 12:40:37
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-15 12:58:56
 * @Description: file content
 */
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
    "./node_modules/tw-elements/js/**/*.js"
  ],
  theme: {
    extend: {},
  },
  darkMode: "class",
  plugins: [require("tw-elements/plugin.cjs")]
}

