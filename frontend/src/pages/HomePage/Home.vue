<script setup lang="ts">
import ProductCard from "./components/ProductCard.vue";
import { fakesProducts } from "../../entities/Product";
</script>

<template>
    <div class="flex justify-center w-full">
        <div class="grid grid-cols-2 xl:grid-cols-3 max-w-max gap-8">
            <ProductCard
                v-for="product in fakesProducts"
                :key="product.id"
                :product="product"
            />
        </div>
    </div>
</template>
