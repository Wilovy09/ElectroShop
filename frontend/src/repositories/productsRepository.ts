import { useRouter } from "vue-router";
import { Product } from "../entities/Product";
import { apiRequest } from "../services/api";
import { useUserStore } from "../stores/useUserStore";
import handleError from "../helpers/handleError";

export class productsRepository {
  public static async getAllProducts(): Promise<Product[]> {
    return await apiRequest("/products", "GET");
  }

  public static async getCategoryProducts(
    categoryId: number
  ): Promise<Product[]> {
    return await apiRequest(`/category/${categoryId}/products`, "GET");
  }

  public static async addProduct(productInformation: Omit<Product, "id">) {
    const token = useUserStore().token ?? useUserStore().getToken();
    if (!token) {
      handleError("Ha ocurrido un error de autenticación");
      useUserStore().logout(useRouter());
    }
    console.log(productInformation);
    return await apiRequest("/admin/product", "POST", productInformation, {
      Authorization: `Bearer ${token}`,
    });
  }

  public static async updateProduct(newDetails: Product) {
    const token = useUserStore().token ?? useUserStore().getToken();
    if (!token) {
      handleError("Ha ocurrido un error de autenticación");
      useUserStore().logout(useRouter());
    }
    console.log(newDetails);
    return await apiRequest(
      `/admin/products/${newDetails.id}`,
      "PUT",
      newDetails,
      {
        Authorization: `Bearer ${token}`,
      }
    );
  }

  public static async deleteProduct() {}
}
