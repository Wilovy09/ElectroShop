<script setup lang="ts">
import { useRoute } from 'vue-router'
import { onMounted, ref } from 'vue'
import { type Product } from '../../entities/Product'
import { useCartStore } from '../../stores/useCartStore'
import { productsRepository } from '../../repositories/productsRepository'
import handleError from '../../helpers/handleError'
import VLoader from '../../components/VLoader.vue'
import { useUserStore } from '../../stores/useUserStore'
import showSuccedSwal from '../../helpers/succedSwal'
import router from '../../router'
import showConfirmationSwal from '../../helpers/confirmationSwal'
import AddProductForm from '../../components/AddProductForm.vue'
import { PencilSquareIcon, TrashIcon } from '@heroicons/vue/24/solid'

const cartStore = useCartStore()

const isAdmin = useUserStore().userRole === 'Administrador'

const route = useRoute()
const productId = ref<number | null>(null)
const product = ref<Product | null>(null)
const isLoading = ref(false)
const error = ref<string | null>(null)
const showForm = ref<boolean>(false)

async function getProduct() {
  if (productId.value === null) return

  isLoading.value = true
  error.value = null

  try {
    const response = await productsRepository.getSpecificProduct(productId.value)

    if (response) {
      product.value = response
    } else {
      product.value = null
      error.value = 'Producto no encontrado'
    }
  } catch (e) {
    handleError(e)
  } finally {
    isLoading.value = false
  }
}

function addToCart(product: Product) {
  cartStore.addToCart(product, 1)
}

function buyNow(product: Product) {
  cartStore.addToCart(product, 1)
  router.push({ name: 'cart' })
}

async function deleteProduct(productId: number) {
  try {
    const confirm = await showConfirmationSwal(
      'Eliminar producto',
      '¿Estás seguro que quieres eliminar este producto?'
    )
    if (confirm !== true) return
    isLoading.value = true
    await productsRepository.deleteProduct(productId)
    showSuccedSwal('Producto eliminado')
    await router.push({ name: 'Home' })
  } catch (e) {
    handleError(e)
  } finally {
    isLoading.value = false
  }
}
function handleSave(newValue: Product) {
  product.value = newValue
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
  <!-- isLoading State -->
  <VLoader v-if="isLoading" />
  <div v-else class="flex h-full w-full items-center justify-center">
    <!-- Error State -->
    <div v-if="error" class="text-center">
      <div
        class="relative rounded border border-red-600 bg-red-900/50 px-4 py-3 text-red-300"
        role="alert"
      >
        <strong class="font-bold">Error: </strong>
        <span class="block sm:inline">{{ error }}</span>
      </div>
    </div>

    <!-- Product Content -->
    <div v-else-if="product">
      <div
        class="relative grid max-w-5xl grid-cols-1 gap-8 rounded-xl bg-slate-800 p-8 shadow-lg md:grid-cols-2"
      >
        <div v-if="isAdmin" class="absolute right-0 top-0 flex gap-2 p-2">
          <button @click="showForm = true" class="rounded-lg bg-slate-600 p-4 text-white">
            <PencilSquareIcon class="size-6" />
          </button>
          <button @click="deleteProduct(product.id)" class="rounded-lg bg-red-600 p-4 text-white">
            <TrashIcon class="size-6" />
          </button>
        </div>
        <!-- Product Image -->
        <div class="flex items-center justify-center">
          <img
            :src="product.image"
            :alt="product.name"
            class="h-auto w-full max-w-md rounded-lg object-cover shadow-2xl transition-transform"
          />
        </div>

        <!-- Product Information -->
        <div class="max-w-full space-y-6 text-zinc-400">
          <div>
            <h1 class="mb-4 text-4xl font-bold text-white">
              {{ product.name }}
            </h1>
            <p
              class="mb-4 line-clamp-4 inline-block h-fit w-full max-w-full text-pretty text-lg text-zinc-400"
            >
              {{ product.description }}
            </p>
          </div>

          <div class="flex items-center justify-between">
            <span class="text-3xl font-bold text-green-500">${{ product.price.toFixed(2) }}</span>
            <span class="rounded-full bg-slate-700 px-3 py-1 text-sm text-zinc-400">
              En stock: {{ product.units }}
            </span>
          </div>

          <div class="flex flex-col space-y-4 sm:flex-row sm:space-x-4 sm:space-y-0">
            <button
              @click="buyNow(product)"
              class="flex-1 rounded-md bg-zinc-950 px-6 py-3 font-semibold text-white transition-all hover:bg-zinc-800 focus:outline-none focus:ring-2 focus:ring-zinc-600 focus:ring-offset-2"
            >
              Comprar ahora
            </button>
            <button
              @click="addToCart(product)"
              class="flex-1 rounded-md bg-yellow-500 px-6 py-3 font-semibold text-white transition-all hover:bg-yellow-600 focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-offset-2"
            >
              Agregar al carrito
            </button>
          </div>

          <div class="border-t border-slate-700 pt-4">
            <div class="text-sm text-zinc-400">
              <p><strong>Categoría:</strong> {{ product.category_id }}</p>
            </div>
          </div>
        </div>
      </div>
      <AddProductForm
        v-if="showForm"
        @save="(value) => handleSave(value)"
        v-model:show-modal="showForm"
        :product="product"
      />
    </div>

    <!-- No Product State -->
    <div v-else class="text-center">
      <p class="text-zinc-400">No se encontró ningún producto</p>
    </div>
  </div>
</template>
s
