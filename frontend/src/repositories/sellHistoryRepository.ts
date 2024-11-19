import { apiRequest } from "../services/api";
import { sellDetails } from "../entities/Sell";
import { useUserStore } from "../stores/useUserStore";
import { useRouter } from "vue-router";

export class sellHistoryRepository {
    public static async createSellPoint(sellDetails: sellDetails ){
    return await apiRequest("/sell", "POST", sellDetails )}

    public static async getSellHistory(){
        const userId= useUserStore().userId ?? useUserStore().getUserId()
        if(!userId){useUserStore().logout(useRouter())
            throw new Error("Error de autenticación")
        }
        return await apiRequest(`/sell/${userId}`, "GET")
    }

    public static async getAllSellHistory(){
        const token = useUserStore().token ?? useUserStore().getToken()
    if (!token) {useUserStore().logout(useRouter())
      throw new Error('Ha ocurrido un error de autenticación')
      
    }
    return await apiRequest("/admin/sell", "GET", undefined, {
      Authorization: `Bearer ${token}`
    } )
    }
}