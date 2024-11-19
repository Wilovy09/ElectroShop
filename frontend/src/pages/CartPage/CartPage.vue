<script setup lang="ts">
import { useRouter } from 'vue-router'
import { sellDetails } from '../../entities/Sell'
import { sellHistoryRepository } from '../../repositories/sellHistoryRepository'
import { useCartStore } from '../../stores/useCartStore'
import { useUserStore } from '../../stores/useUserStore'
import showSuccedSwal from '../../helpers/succedSwal'
import handleError from '../../helpers/handleError'
import { ref } from 'vue'

const cartStore = useCartStore()

function handleRemove(productId: number) {
  cartStore.removeFromCart(productId)
}

function handleClear() {
  cartStore.clearCart()
}
const isLoading = ref(false)
async function proceedToCheckout() {
  isLoading.value = true
  const userId = useUserStore().userId ?? useUserStore().getUserId()
  if (!userId) useUserStore().logout(useRouter())
  const checkoutData: sellDetails = {
    user_id: userId,
    total_amount: cartStore.totalPrice,
    transactions: cartStore.cartItems.map((item) => ({
      product_name: item.product.name,
      id_product: item.product.id,
      quantity: item.quantity
    }))
  }
  try {
    await sellHistoryRepository.createSellPoint(checkoutData)
    handleClear()
    showSuccedSwal('Operación realizada con exito')
  } catch (e) {
    console.log(e)
    handleError(e)
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="min-h-screen bg-slate-900 p-6">
    <div class="mx-auto max-w-4xl">
      <!-- Header -->
      <div class="mb-6 flex items-center justify-between">
        <h1 class="text-2xl font-bold text-white">Carrito de Compras</h1>
        <button
          v-if="cartStore.cartItems.length > 0"
          @click="handleClear"
          class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-all hover:scale-95 hover:bg-blue-700"
        >
          Vaciar carrito
        </button>
      </div>

      <!-- Cart Items -->
      <div v-if="cartStore.cartItems.length" class="space-y-4">
        <div
          v-for="item in cartStore.cartItems"
          :key="item.product.id"
          class="overflow-hidden rounded-xl bg-slate-800 shadow-lg"
        >
          <div class="flex items-start gap-4 p-4">
            <!-- Product Image -->
            <div class="h-32 w-32 flex-shrink-0">
              <img
                :src="item.product.image"
                :alt="item.product.name"
                class="h-full w-full rounded-lg object-cover"
              />
            </div>

            <!-- Product Details -->
            <div class="min-w-0 flex-1">
              <!-- Added min-w-0 to ensure text truncation works -->
              <div class="flex items-start justify-between gap-4">
                <div class="min-w-0 flex-1">
                  <!-- Added min-w-0 and flex-1 for proper truncation -->
                  <h2 class="truncate text-xl font-semibold text-white">
                    {{ item.product.name }}
                  </h2>
                  <p class="mt-1 line-clamp-3 text-wrap text-sm text-zinc-400">
                    {{ item.product.description }}
                  </p>
                </div>
                <button
                  @click="handleRemove(item.product.id)"
                  class="flex-shrink-0 rounded-lg bg-red-500/10 px-3 py-1 text-sm font-medium text-red-500 transition-colors hover:bg-red-500/20"
                >
                  Eliminar
                </button>
              </div>

              <!-- Price Details -->
              <div class="mt-4 flex items-end justify-between">
                <div class="space-y-1">
                  <p class="text-sm text-zinc-400">
                    Cantidad: <span class="font-medium text-white">{{ item.quantity }}</span>
                  </p>
                  <p class="text-sm text-zinc-400">
                    Precio unitario:
                    <span class="font-medium text-white">${{ item.product.price.toFixed(2) }}</span>
                  </p>
                </div>
                <div class="text-right">
                  <p class="text-sm text-zinc-400">Subtotal</p>
                  <p class="text-lg font-semibold text-white">
                    ${{ (item.product.price * item.quantity).toFixed(2) }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Cart Summary -->
        <div class="mt-6 rounded-xl bg-slate-800 p-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-zinc-400">Total del carrito</p>
              <p class="text-2xl font-bold text-white">${{ cartStore.totalPrice.toFixed(2) }}</p>
            </div>
            <button
              @click="proceedToCheckout"
              class="rounded-lg bg-green-600 px-6 py-3 font-medium text-white transition-all hover:scale-95 hover:bg-green-700"
            >
              Proceder al pago
            </button>
          </div>
        </div>
      </div>

      <!-- Empty Cart Message -->
      <div
        v-else
        class="flex h-[200px] items-center justify-center rounded-xl bg-slate-800 text-zinc-400"
      >
        <p>El carrito está vacío.</p>
      </div>
    </div>
  </div>
</template>
