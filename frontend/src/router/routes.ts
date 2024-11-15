const routes = [
  {
    path: "/",
    name: "Home",
    component: () => import("../pages/Home.vue"),
  },
  {
    path: "/auth",
    name: "Authentication",
    component: () => import("../pages/AuthenticationPage/AuthPage.vue"),
  },
];

export { routes };
