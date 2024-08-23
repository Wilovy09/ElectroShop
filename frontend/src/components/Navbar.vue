<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { ArrowLeftStartOnRectangleIcon } from '@heroicons/vue/24/outline'

const auth = useAuthStore()
const router = useRouter()
const isMenuOpen = ref(false)

function toggleMenu() {
  isMenuOpen.value = !isMenuOpen.value
}

async function logout() {
  await auth.logout()
  router.push({ name: 'auth' })
}
</script>

<template>
  <nav class="bg-blue-700 p-2">
    <div class="container mx-auto flex items-center justify-between flex-wrap">
      <div class="flex items-center flex-shrink-0 text-white mr-6">
        <span class="font-semibold text-xl tracking-tight">ElectroShop</span>
      </div>
      <div class="block lg:hidden">
        <button
          class="flex items-center px-3 py-2 border rounded text-gray-300 border-gray-400 hover:text-white hover:border-white"
          @click="toggleMenu"
        >
          <svg class="fill-current h-3 w-3" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <title>Menu</title>
            <path d="M0 3h20v2H0V3zm0 5h20v2H0V8zm0 5h20v2H0v-2z" />
          </svg>
        </button>
      </div>
      <div
        class="w-full block flex-grow lg:flex lg:items-center lg:w-auto"
        :class="{ hidden: !isMenuOpen }"
      >
        <div class="text-sm lg:flex-grow">
          <a href="#" class="block mt-4 lg:inline-block lg:mt-0 text-gray-300 hover:text-white mr-4"
            >Inicio</a
          >
          <a href="#" class="block mt-4 lg:inline-block lg:mt-0 text-gray-300 hover:text-white mr-4"
            >Servicios</a
          >
          <a href="#" class="block mt-4 lg:inline-block lg:mt-0 text-gray-300 hover:text-white"
            >Contacto</a
          >
        </div>
        <div>
          <ArrowLeftStartOnRectangleIcon
            @click="logout()"
            class="w-8 text-white cursor-pointer hover:text-black"
          />
        </div>
      </div>
    </div>
  </nav>
</template>

<style scoped></style>
