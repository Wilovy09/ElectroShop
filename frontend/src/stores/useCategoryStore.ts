import { defineStore } from "pinia";
import { ref } from "vue";
import { categoriesRepository } from "../repositories/categoriesRepository";

export const useCategoryStore = defineStore("category", () => {
  const categories = ref<{ id: number; name: string }[] | null>(null);
  async function getCategories() {
    try {
      categories.value = await categoriesRepository.getCategories();
      return categories.value;
    } catch (e) {
      categories.value = null;
      return categories.value;
    }
  }
  return { categories, getCategories };
});
