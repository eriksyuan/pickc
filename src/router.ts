import {
  createRouter,
  createWebHistory,
  type RouteRecordRaw,
} from "vue-router";

import Home from "./pages/home.vue";
import Picker from "./pages/picker.vue";

const routes: RouteRecordRaw[] = [
  // 动态字段以冒号开始
  { path: "/", component: Home },
  { path: "/picker", component: Picker },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
