<script setup lang="ts">
import ProductCard from './components/ProductCard.vue'
import { Product } from '../../entities/Product'
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { productsRepository } from '../../repositories/productsRepository'
import { useCategoryStore } from '../../stores/useCategoryStore'
import handleError from '../../helpers/handleError'
import AddProductCard from './components/AddProductCard.vue'
import AddProductForm from '../../components/AddProductForm.vue'
import { PencilSquareIcon, TrashIcon } from '@heroicons/vue/24/solid'
import { useUserStore } from '../../stores/useUserStore'
import showConfirmationSwal from '../../helpers/confirmationSwal'
import showSuccedSwal from '../../helpers/succedSwal'
import VLoader from '../../components/VLoader.vue'

type RouteParams = {
  categoryName: string
}
const isLoading = ref(true)

const isAdmin = useUserStore().userRole === 'Administrador'
const route = useRoute()
const routeName = useRoute().name
const showForm = ref<boolean>(false)

const params = route.params as unknown as RouteParams

const products = ref<Product[] | null>(null)
const selectedProduct = ref<{
  id: number | null
  category_id: number | null
  name: string | null
  image: string | null
  description: string | null
  price: number | null
  units: number | null
  deleted: 'true' | 'false' | null
}>({
  id: null,
  category_id: null,
  name: null,
  image: null,
  description: null,
  price: null,
  units: null,
  deleted: 'false'
})

async function getProducts() {
  isLoading.value =true
  try {
    if (routeName == 'Home') {
      products.value = await productsRepository.getAllProducts()
    } else {
      const categories = useCategoryStore().categories ?? (await useCategoryStore().getCategories())
      if (!categories) throw new Error('Esta página no existe')
      const categoryId = categories.find((category) => {
        return category.name == params.categoryName.replace('-', ' ')
      })?.id
      if (!categoryId) throw new Error('Esta página no existe')
      products.value = await productsRepository.getCategoryProducts(categoryId)
    }
    if (!isAdmin) {
      products.value = products.value.filter((product) => {
        return product.deleted !== 'true'
      })
      if (!products.value) throw new Error('No hay productos actualmente')
    }
  } catch (e) {
    handleError(e)
  }finally{isLoading.value = false}
}

function showFormModal(chosenProduct?: Product) {
  if (!chosenProduct)
    selectedProduct.value = {
      id: null,
      category_id: null,
      name: null,
      image: null,
      description: null,
      price: null,
      units: null,
      deleted: null
    }
  else selectedProduct.value = chosenProduct
  showForm.value = true
}
async function deleteProduct(productId: number) {
  try {
    const confirm = await showConfirmationSwal(
      'Eliminar producto',
      '¿Estás seguro que quieres eliminar este producto?'
    )
    if (confirm !== true) return
    isLoading.value = true
    const response = await productsRepository.deleteProduct(productId)
    products.value?.forEach((product, index) => {
      if (product.id === response.id) products.value?.splice(index, 1)
    })
    showSuccedSwal('Producto eliminado')
  } catch (e) {
    handleError(e)
  }finally{isLoading.value = false}
}

function handleAction(response: Product) {
  const isNew = !products.value?.some((product) => product.id === response.id)
  if (isNew) {
    products.value?.push(response)
  } else {
    products.value =
      products.value?.map((product) => (product.id === response.id ? response : product)) ??
      products.value
  }
}

onMounted(getProducts)
</script>

<template>
  <VLoader v-if="isLoading"/>
  <div v-else class="flex w-full justify-center">
    <div
      :class="[
        'grid w-full grid-cols-1 max-w-max gap-8 sm:grid-cols-2 xl:grid-cols-3',
      ]"
    >
      <div v-for="product in products" :key="product.id" class="relative">
        <ProductCard :product="product" />
        <div v-if="isAdmin" class="absolute right-0 top-0 flex gap-2 p-2">
          <button @click="showFormModal(product)" class="rounded-lg bg-slate-600 p-4 text-white">
            <PencilSquareIcon class="size-6" />
          </button>
          <button @click="deleteProduct(product.id)" class="rounded-lg bg-red-600 p-4 text-white">
            <TrashIcon class="size-6" />
          </button>
        </div>
      </div>
      <AddProductCard v-if="isAdmin" :class="products?.length ? '': ' min-w-80 min-[400px]:min-w-96'" @click="showFormModal()" />
      <AddProductForm
        v-if="showForm"
        @save="(value) => handleAction(value)"
        v-model:show-modal="showForm"
        :product="selectedProduct"
      />
    </div>
  </div>
</template>
