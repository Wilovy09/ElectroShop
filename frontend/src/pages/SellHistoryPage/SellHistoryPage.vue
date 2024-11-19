<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { sellHistoryRepository } from '../../repositories/sellHistoryRepository';
import handleError from '../../helpers/handleError';
import VLoader from '../../components/VLoader.vue';
import { useUserStore } from '../../stores/useUserStore';

// Definir el tipo de los datos de la venta
interface Sale {
  total_amount: number;
  sell_date: string;
  product_name: string;
  id_sell: number;
  id_product: number;
  quantity: number;
}

const isAdmin = useUserStore().userRole === 'Administrador'


const isLoading = ref(false)

// Datos de ventas
const sales = ref<Sale[] | null>(null);

async function getHistoryElements() {
    isLoading.value = true
    try{
        console.log(isAdmin)
        if(isAdmin)sales.value = await sellHistoryRepository.getAllSellHistory()
        else
      sales.value = await sellHistoryRepository.getSellHistory()
    }catch(e){
        handleError(e)
    }finally{
        isLoading.value = false
    }
    
}

// Método para formatear la fecha
const formatDate = (date: string): string => {
  const d = new Date(date);
  return d.toLocaleDateString('es-MX', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  });
};

// Método para ver detalles de una venta
onMounted(getHistoryElements)
</script>
<template>
    <VLoader v-if="isLoading"/>
  
    <!-- Verificar si hay ventas -->
    <div v-else-if="sales && sales.length === 0" class="text-center text-white">
      <p>No hay ventas disponibles.</p>
    </div>

    <div v-else class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
      <!-- Tarjetas de venta -->
      <div
        v-for="sale in sales"
        :key="sale.id_sell"
        class="bg-slate-800 text-white p-4 rounded-xl shadow-md hover:shadow-lg transition-shadow duration-300"
      >
        <h3 class="text-xl font-semibold mb-2">{{ sale.product_name }}</h3>
        <p class="text-md mb-1"><strong>Fecha de venta:</strong> {{ formatDate(sale.sell_date) }}</p>
        <p class="text-md mb-1"><strong>Cantidad:</strong> {{ sale.quantity }}</p>
        <p class="text-md mb-1"><strong>Total:</strong> ${{ sale.total_amount.toFixed(2) }}</p>
        <div class="mt-4 flex justify-end">
            <RouterLink :to="{ name: 'product', params: { id: sale.id_product } }">
          <button
            
            
            class="text-zinc-400 hover:text-white transition-colors duration-300"
          >
            Ver detalles
          </button></RouterLink>
        </div>
      </div>
    </div>
  
</template>