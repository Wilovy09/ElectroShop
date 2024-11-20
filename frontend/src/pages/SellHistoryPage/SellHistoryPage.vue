<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { sellHistoryRepository } from '../../repositories/sellHistoryRepository'
import handleError from '../../helpers/handleError'
import VLoader from '../../components/VLoader.vue'
import { useUserStore } from '../../stores/useUserStore'
import router from '../../router'

// Definir el tipo de los datos de la venta
interface Sale {
  total_amount: number
  sell_date: string
  product_name: string
  id_sell: number
  id_product: number
  quantity: number
}

// Interfaz para ventas agrupadas
interface GroupedSale {
  id_sell: number
  sell_date: string
  products: Sale[]
  total: number
}

const isAdmin = useUserStore().userRole === 'Administrador'
const isLoading = ref(false)
const groupedSales = ref<GroupedSale[]>([])

async function getHistoryElements() {
  isLoading.value = true
  try {
    // Obtener las ventas y asegurar el tipo
    const sales: Sale[] = isAdmin
      ? await sellHistoryRepository.getAllSellHistory()
      : await sellHistoryRepository.getSellHistory()

    // Crear un mapa para agrupar por id_sell
    const groupedMap = sales.reduce((acc: Map<number, GroupedSale>, sale: Sale) => {
      if (!acc.has(sale.id_sell)) {
        acc.set(sale.id_sell, {
          id_sell: sale.id_sell,
          sell_date: sale.sell_date,
          products: [],
          total: 0
        })
      }

      const group = acc.get(sale.id_sell)!
      group.products.push(sale)
      group.total += sale.total_amount

      return acc
    }, new Map<number, GroupedSale>())

    // Convertir el mapa a array y ordenar por fecha
    const groupedArray: GroupedSale[] = Array.from(groupedMap.values())
    groupedSales.value = groupedArray.sort(
      (a: GroupedSale, b: GroupedSale) =>
        new Date(b.sell_date).getTime() - new Date(a.sell_date).getTime()
    )
  } catch (e) {
    handleError(e)
  } finally {
    isLoading.value = false
  }
}

// Método para formatear la fecha completa
const formatFullDate = (date: string): string => {
  const d = new Date(date)
  return d.toLocaleDateString('es-MX', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    hour12: true
  })
}

onMounted(getHistoryElements)
</script>

<template>
  <VLoader v-if="isLoading" />

  <div v-else-if="!groupedSales.length" class="text-center text-white">
    <p>No hay ventas disponibles.</p>
  </div>

  <div v-else class="h-[calc(100vh-100px)] overflow-y-auto pr-4">
    <div class="space-y-6">
      <!-- Grupo de ventas -->
      <div
        v-for="group in groupedSales"
        :key="group.id_sell"
        class="rounded-xl bg-slate-800 p-6 text-white shadow-md transition-shadow duration-300 hover:shadow-lg"
      >
        <div class="mb-4 flex items-center justify-between border-b border-slate-700 pb-4">
          <h3 class="text-xl font-semibold">
            {{ formatFullDate(group.sell_date) }}
          </h3>
        </div>

        <!-- Productos en la venta -->
        <div class="space-y-4">
          <div
            v-for="product in group.products"
            :key="`${group.id_sell}-${product.id_product}`"
            @click="router.push({ name: 'product', params: { id: product.id_product } })"
            class="flex cursor-pointer items-center justify-between rounded-lg bg-slate-700 p-4"
          >
            <div class="flex-1">
              <h4 class="font-medium">{{ product.product_name }}</h4>
              <p class="text-sm text-zinc-400">Cantidad: {{ product.quantity }}</p>
            </div>
            <p class="ml-4 whitespace-nowrap">${{ product.total_amount.toFixed(2) }}</p>
          </div>
        </div>

        <!-- Total de la venta -->
        <div class="mt-4 flex items-center justify-between border-t border-slate-700 pt-4">
          <span class="text-lg font-semibold">Total</span>
          <span class="text-lg">${{ group.total.toFixed(2) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
