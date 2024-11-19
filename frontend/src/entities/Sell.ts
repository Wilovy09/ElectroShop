export type sellDetails = {
  user_id: number
  total_amount: number
  transactions: {
    product_name: string
    id_product: number
    quantity: number
  }[]
}
