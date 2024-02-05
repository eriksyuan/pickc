import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import router from "./router";


import '@unocss/reset/sanitize/sanitize.css'
import '@unocss/reset/sanitize/assets.css'
// main.ts
import 'virtual:uno.css'
createApp(App).use(router).mount("#app");
