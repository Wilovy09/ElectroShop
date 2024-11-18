<script setup lang="ts">
import ProductCard from "./components/ProductCard.vue";
</script>
<template>
  <div class="flex justify-center w-full">
    <div class="grid grid-cols-2 xl:grid-cols-3 max-w-max gap-8">
      <ProductCard />
      <ProductCard />
      <ProductCard />
      <ProductCard />
    </div>
  </div>
</template>
