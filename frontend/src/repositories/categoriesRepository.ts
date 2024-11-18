import { useRouter } from "vue-router";
import handleError from "../helpers/handleError";
import { apiRequest } from "../services/api";
import { useUserStore } from "../stores/useUserStore";

export class categoriesRepository {
  public static async getCategories(): Promise<{ id: number; name: string }[]> {
    return await apiRequest("/categories", "GET");
  }

  public static async deleteCategory(categoryId: number) {
    const token = useUserStore().token ?? useUserStore().getToken();
    if (!token) {
      handleError("Ha ocurrido un error de autenticación");
      useUserStore().logout(useRouter());
    }
    await apiRequest(`/admin/categories/${categoryId}`, "DELETE", undefined, {
      Authorization: `Bearer ${token}`,
    });
  }

  public static async addCategories(categoryName: string) {
    const token = useUserStore().token ?? useUserStore().getToken();
    if (!token) {
      handleError("Ha ocurrido un error de autenticación");
      useUserStore().logout(useRouter());
    }
    return await apiRequest(
      "/admin/category",
      "POST",
      { name: categoryName },
      { Authorization: `Bearer ${token}` }
    );
  }
}
