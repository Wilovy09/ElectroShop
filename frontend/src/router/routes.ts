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
    //component: ()=> import ("../layouts/MainLayout.vue")
    children: [
      {
        path: "",
        name: "Home",
        component: () => import("../pages/Home.vue"),
      },
    ],
  },
];

export { routes };
