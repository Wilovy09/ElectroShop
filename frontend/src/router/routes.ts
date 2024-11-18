const routes = [
  {
    path: "/auth",
    name: "Authentication",
    component: () => import("../pages/AuthenticationPage/AuthPage.vue"),
  },
  {
    path: "/",
    name: "Layout",
    meta: { requiresAuth: true },
    component: () => import("../layouts/MainLayout.vue"),
    children: [
      {
        path: "",
        name: "Home",
        component: () => import("../pages/HomePage/Home.vue"),
      },
      {
        path: "/product/:id",
        name: "product",
        component: () =>
          import("../pages/EspecificProductPage/SpecificProductPage.vue"),
      },
    ],
  },
];

export { routes };
