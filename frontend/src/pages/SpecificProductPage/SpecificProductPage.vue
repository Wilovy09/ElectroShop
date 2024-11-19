<script setup lang="ts">
import { useRoute } from 'vue-router'
import { onMounted, ref } from 'vue'
import { apiRequest } from '../../services/api'
import { type Product  } from '../../entities/Product'
import { useCartStore } from '../../stores/useCartStore'

const cartStore = useCartStore()

const route = useRoute()
const productId = ref<number | null>(null)
const product = ref<Product | null>(null)

async function getProduct() {
  if (productId.value === null) return
  try {
    const response = await apiRequest(`/products/${productId.value}`, 'GET')

    if (response) {
      product.value = response
    } else {
      product.value = null
    }

    if (!product.value) {
      product.value = null
    }
  } catch (error) {
    console.error('Error fetching product:', error)
    product.value =  null
  }
}

function addToCart(product: Product) {
  cartStore.addToCart(product, 1) // Aquí 'product' es la prop recibida
}

onMounted(async () => {
  const idParam = Number(route.params.id)
  productId.value = Number.isNaN(idParam) ? null : idParam
  if (productId.value !== null) {
    await getProduct()
  }
})
</script>

<template>
  <div class="flex items-center justify-center">
    <!-- Contenido del producto -->
    <div v-if="product" class="grid max-w-5xl grid-cols-1 gap-6 p-6 md:grid-cols-2">
      <!-- Imagen del producto -->
      <div class="flex items-center justify-center">
        <img
          :src="product.image"
          alt="Product Image"
          class="h-auto w-full max-w-sm rounded-md object-cover shadow-md"
        />
      </div>

      <!-- Información del producto -->
      <div>
        <h1 class="mb-4 text-3xl font-bold text-gray-300">
          {{ product.name }}
        </h1>
        <p class="mb-4 text-lg text-gray-500">
          {{ product.description }}
        </p>
        <div class="mb-6 flex items-center space-x-4">
          <span class="text-2xl font-semibold text-green-600">${{ product.price }}</span>
          <span class="text-sm text-gray-500">En stock: {{ product.units }}</span>
        </div>
        <div class="flex gap-4">
          <button
            class="w-full rounded-md bg-black px-4 py-2 font-semibold text-white transition hover:bg-gray-950"
          >
            Comprar
          </button>
          <button
            @click="addToCart(product)"
            class="w-full rounded-md bg-yellow-500 px-4 py-2 font-semibold text-white transition hover:bg-yellow-600"
          >
            Agregar al carrito
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
