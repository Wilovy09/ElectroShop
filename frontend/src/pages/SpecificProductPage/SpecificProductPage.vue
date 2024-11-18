<script setup lang="ts">
import { useRoute } from "vue-router";
import { onMounted, ref } from "vue";
import { apiRequest } from "../../services/api";
import {
    type Product,
    fakesProducts as productFallback,
} from "../../entities/Product";
import { useCartStore } from "../../stores/useCartStore";

const cartStore = useCartStore();

const route = useRoute();
const productId = ref<number | null>(null);
const product = ref<Product | null>(null);

async function getProduct() {
    if (productId.value === null) return;
    try {
        const response = await apiRequest(
            `/products/${productId.value}`,
            "GET",
        );

        if (response) {
            product.value = response;
        } else {
            product.value =
                productFallback.find((p) => p.id === productId.value) ?? null;
        }

        if (!product.value) {
            product.value = null;
        }
    } catch (error) {
        console.error("Error fetching product:", error);
        product.value =
            productFallback.find((p) => p.id === productId.value) ?? null;
    }
}

function addToCart(product: Product) {
    cartStore.addToCart(product, 1); // Aquí 'product' es la prop recibida
}

onMounted(async () => {
    const idParam = Number(route.params.id);
    productId.value = Number.isNaN(idParam) ? null : idParam;
    if (productId.value !== null) {
        await getProduct();
    }
});
</script>

<template>
    <div class="flex items-center justify-center">
        <!-- Contenido del producto -->
        <div
            v-if="product"
            class="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-5xl p-6"
        >
            <!-- Imagen del producto -->
            <div class="flex items-center justify-center">
                <img
                    :src="product.image"
                    alt="Product Image"
                    class="w-full max-w-sm h-auto object-cover rounded-md shadow-md"
                />
            </div>

            <!-- Información del producto -->
            <div>
                <h1 class="text-3xl font-bold mb-4 text-gray-300">
                    {{ product.name }}
                </h1>
                <p class="text-lg text-gray-500 mb-4">
                    {{ product.description }}
                </p>
                <div class="flex items-center space-x-4 mb-6">
                    <span class="text-2xl font-semibold text-green-600"
                        >${{ product.price }}</span
                    >
                    <span class="text-sm text-gray-500"
                        >En stock: {{ product.units }}</span
                    >
                </div>
                <div class="flex gap-4">
                    <button
                        class="w-full px-4 py-2 bg-black text-white rounded-md hover:bg-gray-950 transition font-semibold"
                    >
                        Comprar
                    </button>
                    <button
                        @click="addToCart(product)"
                        class="w-full px-4 py-2 bg-yellow-500 text-white rounded-md hover:bg-yellow-600 transition font-semibold"
                    >
                        Agregar al carrito
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
