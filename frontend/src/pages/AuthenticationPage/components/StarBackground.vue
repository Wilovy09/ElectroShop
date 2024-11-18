<script setup lang="ts">
const getStarStyle = () => {
  const size = Math.random() * 2;
  return {
    top: `${Math.random() * 100}%`,
    left: `${Math.random() * 100}%`,
    width: `${size}px`,
    height: `${size}px`,
    animationDelay: `${Math.random() * 5}s`,
  };
};
</script>
<template>
  <div
    v-for="n in 200"
    :key="n"
    class="absolute z-0 bg-white rounded-full twinkleAnimation"
    :style="getStarStyle()"
  />
</template>
