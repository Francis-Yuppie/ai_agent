import AppLayout from '@/layout/AppLayout.vue';
import { createRouter, createWebHistory } from 'vue-router';

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/dashboard',
            component: AppLayout,
            children: [
                {
                    path: '/dashboard',
                    name: 'dashboard',
                    component: () => import('@/views/Dashboard.vue')
                },
                {
                    path: '/threats',
                    name: 'threats',
                    component: () => import('@/views/pages/ThreatsView.vue')
                },
                {
                    path: '/canisters',
                    name: 'canisters',
                    component: () => import('@/views/pages/CanistersView.vue')
                },
                {
                    path: '/reports',
                    name: 'reports',
                    component: () => import('@/views/pages/ReportsView.vue')
                },
                {
                    path: '/history',
                    name: 'history',
                    component: () => import('@/views/pages/HistoryView.vue')
                },
                {
                    path: '/settings',
                    name: 'settings',
                    component: () => import('@/views/pages/SettingsView.vue')
                },
                {
                    path: '/help',
                    name: 'help',
                    component: () => import('@/views/pages/HelpView.vue')
                },
                {
                    path: '/me',
                    name: 'me',
                    component: () => import('@/views/pages/MeView.vue')
                }

            ]
        },
        {
            path: '/',
            name: 'landing',
            component: () => import('@/views/pages/Landing.vue')
        },

        {
            path: '/auth/login',
            name: 'login',
            component: () => import('@/views/pages/auth/Login.vue')
        },
        {
            path: '/:pathMatch(.*)*',
            name: 'notFound',
            component: () => import('@/views/pages/NotFound.vue')
        },
        {
            path: '/auth/access',
            name: 'accessDenied',
            component: () => import('@/views/pages/auth/Access.vue')
        },
        {
            path: '/auth/error',
            name: 'error',
            component: () => import('@/views/pages/auth/Error.vue')
        }
    ]
});

export default router;
