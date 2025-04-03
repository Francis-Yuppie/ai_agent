<script setup>
import { ref } from "vue";

import AppMenuItem from "./AppMenuItem.vue";

const model = ref([
  {
    label: "Home",
    items: [
    { label: "Dashboard", icon: "pi pi-fw pi-home", to: "/dashboard" },
    { label: "Canisters", icon: "pi pi-fw pi-box", to: "/canisters" },
    { label: "Threats", icon: "pi pi-fw pi-exclamation-triangle", to: "/threats" },
    { label: "Reports", icon: "pi pi-fw pi-file", to: "/reports" },
    { label: "History", icon: "pi pi-fw pi-clock", to: "/history" },
    ],
  },
  {
    label: "Settings",
    items: [{ label: "Settings", icon: "pi pi-fw pi-cog", to: "/settings" }],
  },
  //account
    {
        label: "Account",
        items: [
        { label: "Profile", icon: "pi pi-fw pi-user", to: "/me" },
        { label: "Logout", icon: "pi pi-fw pi-sign-out", to: "/logout" },
        ],
    },

  {
    separator: true,
  },
  {
    label: "Help",
    items: [{ label: "Help", icon: "pi pi-fw pi-question", to: "/help" }],
  },
]);
</script>

<template>
  <ul class="layout-menu">
    <template v-for="(item, i) in model" :key="item">
      <app-menu-item
        v-if="!item.separator"
        :item="item"
        :index="i"
      ></app-menu-item>
      <li v-if="item.separator" class="menu-separator"></li>
    </template>
  </ul>
</template>

<style lang="scss" scoped></style>
