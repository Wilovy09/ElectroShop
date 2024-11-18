import { createRouter, createWebHistory } from "vue-router";
import { routes } from "./routes";
import { useUserStore } from "../stores/useUserStore";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: routes,
});

router.beforeEach((to, from, next) => {
  const toRouteName = to.name?.toString();

  const token = useUserStore().token ?? useUserStore().getToken();
  if (to.meta.requiresAuth && !token) {
    if (toRouteName !== "Authentication") {
      return next({ name: "Authentication" });
    }
  }
  next();
});

export default router;
