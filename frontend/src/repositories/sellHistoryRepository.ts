import { apiRequest } from "../services/api";
import { sellDetails } from "../entities/Sell";

export class sellHistoryRepository {
    public static async createSellPoint(sellDetails: sellDetails ){
    return await apiRequest("/sell", "POST", sellDetails )}
}