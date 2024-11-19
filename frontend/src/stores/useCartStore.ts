import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import type { Product } from '../entities/Product'

export const useCartStore = defineStore('cart', () => {
  const savedCart = localStorage.getItem('cart')
  const cartItems = ref<Array<{ product: Product; quantity: number }>>(
    savedCart && savedCart !== 'undefined' ? JSON.parse(savedCart) : []
  )

  const totalItems = computed(() => cartItems.value.reduce((sum, item) => sum + item.quantity, 0))

  const totalPrice = computed(() =>
    cartItems.value.reduce((sum, item) => sum + item.product.price * item.quantity, 0)
  )

  function addToCart(product: Product, quantity = 1) {
    const existingItem = cartItems.value.find((item) => item.product.id === product.id)
    if (existingItem) {
      existingItem.quantity += quantity
    } else {
      cartItems.value.push({ product, quantity })
    }
  }

  function removeFromCart(productId: number) {
    cartItems.value = cartItems.value.filter((item) => item.product.id !== productId)
  }

  function clearCart() {
    cartItems.value = []
  }

  watch(
    cartItems,
    (newCart) => {
      localStorage.setItem('cart', JSON.stringify(newCart))
    },
    { deep: true }
  )

  return {
    cartItems,
    totalItems,
    totalPrice,
    addToCart,
    removeFromCart,
    clearCart
  }
})
