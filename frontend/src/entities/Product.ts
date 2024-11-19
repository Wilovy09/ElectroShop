export type Product = {
  id: number
  category_id: number
  name: string
  image: string
  description: string
  price: number
  units: number
  deleted: 'true' | 'false' | null
}
