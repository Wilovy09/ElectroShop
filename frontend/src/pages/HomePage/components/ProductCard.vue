<script setup lang="ts">
import { useCartStore } from "../../../stores/useCartStore";
import type { Product } from "../../../entities/Product";

// Recibe la prop del producto
defineProps<{ product: Product }>();

const cartStore = useCartStore();

function addToCart(product: Product) {
  cartStore.addToCart(product, 1); // Aquí 'product' es la prop recibida
}
</script>

<template>
  <div class="max-w-sm w-full rounded-xl shadow-lg bg-gray-800 text-white">
    <!-- Enlace al producto -->
    <a
      :href="`/product/${product.id}`"
      class="block max-h-48 overflow-hidden rounded-t-xl"
    >
      <img
        class="w-full h-48 object-cover"
        :src="product.image"
        :alt="product.name"
      />
    </a>

    <!-- Información del producto -->
    <div class="p-4">
      <h2 class="text-lg font-semibold">{{ product.name }}</h2>
      <p class="text-sm text-gray-400 line-clamp-2">
        {{ product.description }}
      </p>
      <p class="text-lg font-bold text-blue-400 mt-2">
        ${{ product.price.toFixed(2) }}
      </p>
    </div>

    <!-- Acciones -->
    <div
      class="flex max-[320px]:flex-col gap-x-2 justify-between items-center p-4 border-t border-gray-700"
    >
      <button
        class="px-4 py-2 max-[320px]:w-full bg-green-600 text-sm font-semibold rounded hover:bg-green-700 transition"
        @click="addToCart(product)"
      >
        Agregar al carrito
      </button>
      <a
        :href="`/product/${product.id}`"
        class="px-4 py-2 max-[320px]:w-full text-center bg-blue-600 text-sm font-semibold rounded hover:bg-blue-700 transition"
      >
        Ver detalles
      </a>
    </div>
  </div>
</template>
