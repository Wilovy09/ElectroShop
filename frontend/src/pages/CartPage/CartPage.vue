<script setup lang="ts">
import { useCartStore } from "../../stores/useCartStore";

const cartStore = useCartStore();

function handleRemove(productId: number) {
    cartStore.removeFromCart(productId);
}

function handleClear() {
    cartStore.clearCart();
}
</script>

<template>
    <div class="p-4">
        <div class="flex justify-between my-4">
            <h1 class="text-2xl font-bold text-white">Carrito</h1>
            <button
                v-if="cartStore.cartItems.length > 0"
                @click="handleClear"
                class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            >
                Vaciar carrito
            </button>
        </div>

        <div v-if="cartStore.cartItems.length">
            <ul class="space-y-4">
                <li
                    v-for="item in cartStore.cartItems"
                    :key="item.product.id"
                    class="flex justify-between items-center p-2 border rounded"
                >
                    <div>
                        <h2 class="text-lg font-semibold text-white">
                            {{ item.product.name }}
                        </h2>
                        <p class="text-sm text-gray-500">
                            {{ item.product.description }}
                        </p>
                        <div class="flex justify-between mt-2">
                            <div class="grid">
                                <p class="text-sm text-white">
                                    Cantidad: {{ item.quantity }}
                                </p>
                                <p class="text-sm font-semibold text-gray-500">
                                    Sub-total: ${{
                                        (
                                            item.product.price * item.quantity
                                        ).toFixed(2)
                                    }}
                                </p>
                            </div>
                            <button
                                @click="handleRemove(item.product.id)"
                                class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
                            >
                                Quitar
                            </button>
                        </div>
                    </div>
                </li>
            </ul>

            <div class="mt-4 text-right">
                <p class="text-lg font-bold text-white">
                    Total: ${{ cartStore.totalPrice.toFixed(2) }}
                </p>
            </div>
        </div>

        <p v-else class="text-gray-500">El carrito está vacío.</p>
    </div>
</template>
