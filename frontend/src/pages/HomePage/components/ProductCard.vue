<script setup lang="ts">
import { fakeProduct } from "../../../entities/Product";
</script>
<template>
    <a
        :href="`/product/${fakeProduct.id}`"
        class="max-w-96 h-fit rounded-xl shadow-lg shadow-white/5"
    >
        <div class="w-full max-h-48 overflow-hidden">
            <img
                class="h-full object-cover rounded-t-lg"
                :src="fakeProduct.image"
                alt=""
            />
        </div>
        <div
            class="flex max-sm:flex-col justify-between bg-slate-800 rounded-b-lg text-white"
        >
            <div class="m-2 mt-0">
                <h2 class="text-base">{{ fakeProduct.name }}</h2>
                <p class="text-sm line-clamp-5 max-sm:line-clamp-3 text-left">
                    <span class="block text-stone-500 text-xs">{{
                        fakeProduct.category_id
                    }}</span>
                    {{ fakeProduct.description }}
                </p>
            </div>
            <div
                class="flex flex-col min-w-28 sm:gap-4 max-sm:m-0 max-sm:mt-1 m-2"
            >
                <button
                    class="text-sm h-8 sm:h-12 px-2 bg-zinc-950 sm:rounded-lg"
                >
                    Comprar
                </button>
                <button
                    class="text-sm h-8 sm:h-12 px-2 bg-slate-600 sm:rounded-lg max-sm:rounded-b-lg"
                >
                    Agregar al carrito
                </button>
            </div>
        </div>
    </a>
</template>
