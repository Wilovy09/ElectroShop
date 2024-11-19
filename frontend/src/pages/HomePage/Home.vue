<script setup lang="ts">
import ProductCard from "./components/ProductCard.vue";
import { fakesProducts, Product } from "../../entities/Product";
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { productsRepository } from "../../repositories/productsRepository";
import { useCategoryStore } from "../../stores/useCategoryStore";
import handleError from "../../helpers/handleError";
import AddProductCard from "./components/addProductCard.vue";
import AddProductForm from "../../components/AddProductForm.vue";
import { PencilSquareIcon } from "@heroicons/vue/24/solid";
import { useUserStore } from "../../stores/useUserStore";

const isAdmin = useUserStore().userRole === "Administrador";
const route = useRoute();
const routeName = useRoute().name;
const showForm = ref<boolean>(false);

const products = ref<Product[] | null>(fakesProducts);
const selectedProduct = ref<{
  id: number | null;
  category_id: number | null;
  name: string | null;
  image: string | null;
  description: string | null;
  price: number | null;
  units: number | null;
  deleted: "true" | "false" | null;
}>({
  id: null,
  category_id: null,
  name: null,
  image: null,
  description: null,
  price: null,
  units: null,
  deleted: "false",
});

async function getProducts() {
  try {
    if (routeName == "Home") {
      products.value = await productsRepository.getAllProducts();
    } else {
      const categories =
        useCategoryStore().categories ??
        (await useCategoryStore().getCategories());
      if (!categories) throw new Error("Esta página no existe");
      const categoryId = categories.find((category) => {
        category.name == route.params.categoryName;
      })?.id;
      if (!categoryId) throw new Error("Esta página no existe");
      products.value = await productsRepository.getCategoryProducts(categoryId);
    }
    if (!isAdmin) {
      products.value = products.value.filter((product) => {
        product.deleted !== "true";
      });
      if (!products.value) throw new Error("No hay productos actualmente");
    }
  } catch (e) {
    handleError(e);
  }
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
      deleted: null,
    };
  else selectedProduct.value = chosenProduct;
  showForm.value = true;
}

onMounted(getProducts);
</script>

<template>
  <div class="flex justify-center w-full">
    <div
      :class="[
        'grid w-full grid-cols-2 xl:grid-cols-3 gap-8',
        products?.length ? 'max-w-max' : '',
      ]"
    >
      <div v-for="product in products" :key="product.id" class="relative">
        <ProductCard :product="product" />
        <button
          @click="showFormModal(product)"
          class="absolute top-0 right-0 bg-gray-800 text-white p-4 rounded-lg"
        >
          <PencilSquareIcon class="size-6" />
        </button>
      </div>
      <AddProductCard v-if="isAdmin" @click="showFormModal()" />
      <AddProductForm
        v-if="showForm"
        v-model:show-modal="showForm"
        :product="selectedProduct"
      />
    </div>
  </div>
</template>
