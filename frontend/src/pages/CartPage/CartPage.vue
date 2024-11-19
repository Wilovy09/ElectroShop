<script setup lang="ts">
import { useCartStore } from '../../stores/useCartStore'

const cartStore = useCartStore()

function handleRemove(productId: number) {
  cartStore.removeFromCart(productId)
}

function handleClear() {
  cartStore.clearCart()
}
</script>

<template>
  <div class="p-4">
    <div class="my-4 flex justify-between">
      <h1 class="text-2xl font-bold text-white">Carrito</h1>
      <button
        v-if="cartStore.cartItems.length > 0"
        @click="handleClear"
        class="rounded bg-blue-500 px-4 py-2 text-white hover:bg-blue-600"
      >
        Vaciar carrito
      </button>
    </div>

    <div v-if="cartStore.cartItems.length">
      <ul class="space-y-4">
        <li
          v-for="item in cartStore.cartItems"
          :key="item.product.id"
          class="flex items-center justify-between rounded border p-2"
        >
          <div>
            <h2 class="text-lg font-semibold text-white">
              {{ item.product.name }}
            </h2>
            <p class="text-sm text-gray-500">
              {{ item.product.description }}
            </p>
            <div class="mt-2 flex justify-between">
              <div class="grid">
                <p class="text-sm text-white">Cantidad: {{ item.quantity }}</p>
                <p class="text-sm font-semibold text-gray-500">
                  Sub-total: ${{ (item.product.price * item.quantity).toFixed(2) }}
                </p>
              </div>
              <button
                @click="handleRemove(item.product.id)"
                class="rounded bg-red-500 px-4 py-2 text-white hover:bg-red-600"
              >
                Quitar
              </button>
            </div>
          </div>
        </li>
      </ul>

      <div class="mt-4 text-right">
        <p class="text-lg font-bold text-white">Total: ${{ cartStore.totalPrice.toFixed(2) }}</p>
      </div>
    </div>

    <p v-else class="text-gray-500">El carrito está vacío.</p>
  </div>
</template>
