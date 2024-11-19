<script setup lang="ts">
import { PlusIcon } from "@heroicons/vue/24/outline"; // Importa el ícono
</script>

<template>
  <div
    class="max-w-sm w-full rounded-xl shadow-lg bg-gray-800 text-white relative cursor-pointer"
  >
    <div class="h-48 w-full"></div>
    <div class="h-36 w-full"></div>

    <PlusIcon class="h-16 w-16 absolute inset-0 m-auto" />
  </div>
</template>
