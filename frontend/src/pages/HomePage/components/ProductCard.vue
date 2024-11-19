<script setup lang="ts">
import { useCartStore } from '../../../stores/useCartStore'
import type { Product } from '../../../entities/Product'

// Recibe la prop del producto
defineProps<{ product: Product }>()

const cartStore = useCartStore()

function addToCart(product: Product) {
  cartStore.addToCart(product, 1) // Aquí 'product' es la prop recibida
}
</script>

<template>
  <div class="w-full max-w-sm rounded-xl bg-gray-800 text-white shadow-lg">
    <!-- Enlace al producto -->
    <a :href="`/product/${product.id}`" class="block max-h-48 overflow-hidden rounded-t-xl">
      <img class="h-48 w-full object-cover" :src="product.image" :alt="product.name" />
    </a>

    <!-- Información del producto -->
    <div class="p-4">
      <h2 class="text-lg font-semibold">{{ product.name }}</h2>
      <p class="line-clamp-2 text-sm text-gray-400">
        {{ product.description }}
      </p>
      <p class="mt-2 text-lg font-bold text-blue-400">${{ product.price.toFixed(2) }}</p>
    </div>

    <!-- Acciones -->
    <div
      class="flex items-center justify-between gap-x-2 border-t border-gray-700 p-4 max-[320px]:flex-col"
    >
      <button
        class="rounded bg-green-600 px-4 py-2 text-sm font-semibold transition hover:bg-green-700 max-[320px]:w-full"
        @click="addToCart(product)"
      >
        Agregar al carrito
      </button>
      <a
        :href="`/product/${product.id}`"
        class="rounded bg-blue-600 px-4 py-2 text-center text-sm font-semibold transition hover:bg-blue-700 max-[320px]:w-full"
      >
        Ver detalles
      </a>
    </div>
  </div>
</template>
