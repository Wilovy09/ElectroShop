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

  async function deleteCategory(categoryId: number) {
    try {
      await categoriesRepository.deleteCategory(categoryId);
      categories.value?.forEach((category, index) => {
        if (category.id === categoryId) categories.value?.splice(index, 1);
      });
    } catch (e) {}
  }

  async function addCategory(
    categoryName: string
  ): Promise<{ id: number; name: string }> {
    const newCategory = await categoriesRepository.addCategories(categoryName);
    categories.value?.push(newCategory);
    return newCategory;
  }

  return { categories, getCategories, deleteCategory, addCategory };
});
