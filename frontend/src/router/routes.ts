export const routes = [
  {
    path: '/auth',
    name: 'auth',
    component: () => import('../views/AuthPage/AuthPage.vue')
  },
  {
    path: '/',
    name: 'main-layout',
    component: () => import('../layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        name: 'home',
        component: () => import('../views/Home/HomePage.vue')
      }
    ]
  }
]
