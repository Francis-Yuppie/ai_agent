import { createRouter, createWebHistory } from 'vue-router'


const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    // {
    //   path: '/',
    //   name: 'home',
    //   component: LandingView,
    //   meta: { title: 'Omniface - Home' },
    // },
    // {
    //   path: '/view-demo/:id',
    //   name: 'view',
    //   component: View,
    //   meta: { title: 'Omniface - Home' },
    //   props: true
    // },
  ],
})

// router.beforeEach((to, from, next) => {
// document.title = to.meta.title || 'Omniface';
//   authMiddleware(to, from, next);
// });

export default router
