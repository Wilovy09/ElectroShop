<script setup lang="ts">
import { PlusIcon } from '@heroicons/vue/24/outline' // Importa el ícono
</script>

<template>
  <div class="relative w-full max-w-sm cursor-pointer rounded-xl bg-gray-800 text-white shadow-lg">
    <div class="h-48 w-full"></div>
    <div class="h-36 w-full"></div>

    <PlusIcon class="absolute inset-0 m-auto h-16 w-16" />
  </div>
</template>
