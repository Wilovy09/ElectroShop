const routes = [
  {
    path: '/auth',
    name: 'Authentication',
    component: () => import('../pages/AuthenticationPage/AuthPage.vue')
  },
  {
    path: '/',
    name: 'Layout',
    meta: { requiresAuth: true },
    component: () => import('../layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        name: 'Home',
        component: () => import('../pages/HomePage/Home.vue'),
        meta: { key: 'home' }
      },
      {
        path: '/product/:id',
        name: 'product',
        component: () => import('../pages/SpecificProductPage/SpecificProductPage.vue')
      },
      {
        path: '/cart',
        name: 'cart',
        component: () => import('../pages/CartPage/CartPage.vue')
      },
      {
        path: '/:categoryName/products',
        name: 'CategoryProducts',
        component: () => import('../pages/HomePage/Home.vue'),
        meta: { key: 'category-products' }
      },
      {
        path: '/historial-de-compras',
        name: 'SellHistory',
        component: () => import('../pages/SellHistoryPage/SellHistoryPage.vue')
      }
    ]
  }
]

export { routes }
