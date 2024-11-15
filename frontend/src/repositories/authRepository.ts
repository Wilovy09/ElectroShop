import { apiRequest } from "../services/api";

export class AuthRepository {
  public static async login(email: string, password: string) {
    return await apiRequest("/login", "POST", { email, password });
  }

  public static async register(email: string, password: string) {
    return await apiRequest("/user", "POST", { email, password });
  }
}
