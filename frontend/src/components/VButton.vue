<script setup lang="ts">
defineProps<{
  backgroundColor?: 'primary' | 'secondary' | 'danger'
}>()
</script>
<template>
  <button
    class="font-normal disabled:opacity-50 disabled:cursor-not-allowed w-full h-11 rounded-lg duration-300 p-2 transition-all text-sm text-center"
    :class="{
      'text-white bg-blue-500 hover:bg-blue-700': backgroundColor === 'primary',
      'bg-gray-100 text-black hover:enabled:bg-gray-300': backgroundColor === 'secondary',
      'bg-red-500 text-white hover:enabled:bg-red-700': backgroundColor === 'danger'
    }"
  >
    <slot></slot>
  </button>
</template>
