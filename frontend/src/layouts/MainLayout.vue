<script setup lang="ts">
import { ref, onMounted, onUnmounted, toRaw } from 'vue'
import {
  XMarkIcon,
  Bars3Icon,
  MagnifyingGlassIcon,
  ShoppingBagIcon,
  ShoppingCartIcon,
  HomeIcon,
  ListBulletIcon,
  ChevronDownIcon,
  PlusIcon
} from '@heroicons/vue/24/outline'
import { useUserStore } from '../stores/useUserStore'
import { useRouter } from 'vue-router'
import { MinusIcon } from '@heroicons/vue/24/solid'
import handleError from '../helpers/handleError'
import { showInputSwal } from '../helpers/inputSwal'
import showSuccedSwal from '../helpers/succedSwal'
import showConfirmationSwal from '../helpers/confirmationSwal'
import { useCategoryStore } from '../stores/useCategoryStore'

const router = useRouter()
const isAdmin = useUserStore().userRole === 'Administrador'

const shouldShowLogoutButton = ref(false)

const isSidebarOpen = ref(false)
const sidebarRef = ref<HTMLElement | null>(null)
const areCategoriesShown = ref(false)

const categories = ref<{ id: number; name: string }[] | null>(null)

async function getAllCategories() {
  const storeCategories =
    useCategoryStore().categories ?? (await useCategoryStore().getCategories())
  categories.value = storeCategories ? JSON.parse(JSON.stringify(toRaw(storeCategories))) : null
}

//agregar esto a la función donde se obtienen las categorias
function getCategoriesSectionHeight() {
  if (!categories.value) return 0
  return categories.value.length * 8 + 3.5 + (isAdmin ? 8 : 0)
}

async function showSwal() {
  try {
    const input = await showInputSwal('Nombre de la categoría')
    if (!input) return
    categories.value?.map((value) => {
      if (value.name === input) throw new Error('Ya existe esta categoría')
    })
    const response = await useCategoryStore().addCategory(input)
    categories.value?.push(response)
    showSuccedSwal('Categoría creada con exito')
  } catch (e) {
    handleError(e)
  }
}

async function showWarningSwal(categoryId: number) {
  try {
    const response = await showConfirmationSwal(
      'Eliminar categoría',
      'Todos los productos de esta categoría serán eliminados'
    )
    if (response !== true) return
    await useCategoryStore().deleteCategory(categoryId)
    if (categories.value)
      categories.value = categories.value.filter((value) => {
        if (value.id !== categoryId) return value
      })
    showSuccedSwal('Categoría eliminada con exito')
  } catch (e) {
    handleError(e)
  }
}

function toggleCategories() {
  areCategoriesShown.value = !areCategoriesShown.value
}

function toggleSidebar() {
  isSidebarOpen.value = !isSidebarOpen.value
}

const handleClickOutside = (event: MouseEvent) => {
  if (sidebarRef.value && !sidebarRef.value.contains(event.target as Node)) {
    isSidebarOpen.value = false
  }
}

onMounted(async () => {
  await getAllCategories()
  document.addEventListener('mousedown', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('mousedown', handleClickOutside)
})
</script>
<template>
  <div class="flex h-screen w-screen flex-grow-0">
    <!-- Overlay for lgall screens -->
    <div
      v-if="isSidebarOpen"
      class="fixed inset-0 z-20 backdrop-blur-lg transition-all duration-300 lg:hidden"
      @click="isSidebarOpen = false"
    ></div>

    <!-- Sidebar -->
    <div
      ref="sidebarRef"
      :class="[
        'fixed inset-y-0 left-0 z-30 min-w-64 transform overflow-hidden bg-slate-800 shadow-xl shadow-zinc-950/50 transition-transform duration-300 ease-in-out',
        isSidebarOpen ? 'translate-x-0' : '-translate-x-full',
        'lg:relative lg:translate-x-0'
      ]"
    >
      <!-- Franja simulando la continuación de la navbar -->
      <div class="absol flex h-16 items-center justify-between bg-slate-900 px-4">
        <h2 class="text-xl font-semibold text-white">ElectroShop</h2>
        <button @click="toggleSidebar" class="text-zinc-400 hover:text-white lg:hidden">
          <XMarkIcon class="size-6" />
        </button>
      </div>

      <div class="text-zinc-400">
        <!-- Add your sidebar content here -->
        <div class="flex flex-col">
          <RouterLink
            :to="{ name: 'Home' }"
            class="flex w-full items-center border-y border-slate-600 p-4 text-left transition-all duration-300 hover:border-white hover:text-white"
          >
            <HomeIcon class="mr-2 size-6" />
            <p>Inicio</p>
          </RouterLink>
          <RouterLink
            :to="{ name: 'cart' }"
            class="flex w-full items-center border-y border-slate-600 p-4 text-left transition-all duration-300 hover:border-white hover:text-white"
          >
            <ShoppingCartIcon class="mr-2 size-6" />
            <p>Carrito</p>
          </RouterLink>
          <RouterLink
            :to="{ name: 'SellHistory' }"
            class="flex w-full items-center border-y border-slate-600 p-4 text-left transition-all duration-300 hover:border-white hover:text-white"
          >
            <ShoppingBagIcon class="mr-2 size-6" />
            <p>Compras</p>
          </RouterLink>
          <div
            v-if="categories?.length || isAdmin"
            class="w-full border-y border-slate-600 transition-all duration-300 ease-in-out hover:border-white"
          >
            <button
              @click="toggleCategories()"
              class="flex w-full items-center justify-between p-4 text-left hover:text-white"
            >
              <div class="flex items-center">
                <ListBulletIcon class="mr-2 size-6" />
                <p>Categorías</p>
              </div>
              <ChevronDownIcon
                :class="[
                  'size-4 transition-all duration-300',
                  areCategoriesShown ? 'rotate-180' : ''
                ]"
              />
            </button>
            <div
              :class="[
                'overflow-hidden transition-all duration-300 ease-in',
                areCategoriesShown ? `h-full` : 'h-0'
              ]"
              :style="{
                maxHeight: areCategoriesShown ? `${getCategoriesSectionHeight() * 4}px` : '0px'
              }"
            >
              <ul class="mb-3.5 ml-8 flex list-disc flex-col text-sm">
                <div v-for="category in categories" :key="category.id">
                  <RouterLink
                    :to="{
                      name: 'CategoryProducts',
                      params: { categoryName: category.name.replace(' ', '-') }
                    }"
                  >
                    <button
                      :disabled="!areCategoriesShown"
                      :class="['my-0.5 py-1 pl-6 hover:text-white', isAdmin ? 'w-fit' : 'w-full']"
                    >
                      <li :class="isAdmin ? 'mr-3 w-28' : 'w-full'">
                        <div class="flex items-center justify-between text-left">
                          <p>{{ category.name }}</p>
                        </div>
                      </li>
                    </button></RouterLink
                  ><button
                    v-if="isAdmin"
                    :disabled="!areCategoriesShown"
                    @click="showWarningSwal(category.id)"
                    class="rounded-full bg-white/10 p-0.5 backdrop-blur-md"
                  >
                    <MinusIcon class="size-4" />
                  </button>
                </div>
                <button
                  v-if="isAdmin"
                  @click="showSwal()"
                  :disabled="!areCategoriesShown"
                  class="my-0.5 w-fit rounded-lg bg-slate-950 py-1 pl-6 pr-2 hover:text-white"
                >
                  <li>
                    <div class="flex w-36 items-center justify-between text-left">
                      <p>Agregar</p>
                      <PlusIcon class="size-5" />
                    </div>
                  </li>
                </button>
              </ul>
            </div>
          </div>
        </div>
        <div
          class="absolute bottom-0 flex w-full justify-center rounded-t-md bg-zinc-950 py-2"
          @mouseover="shouldShowLogoutButton = true"
          @mouseleave="shouldShowLogoutButton = false"
        >
          <ChevronDownIcon class="size-4 rotate-180 text-white" />
        </div>
        <div
          :class="[
            'absolute bottom-0 w-full rounded-t-md bg-zinc-950 transition-all duration-300',
            shouldShowLogoutButton ? '' : 'translate-y-16'
          ]"
          @mouseover="shouldShowLogoutButton = true"
          @mouseleave="shouldShowLogoutButton = false"
        >
          <button
            @click="useUserStore().logout(router)"
            class="w-full py-5 text-white transition-all duration-150 hover:scale-95 active:text-zinc-400"
          >
            Cerrar sesión
          </button>
        </div>
      </div>
    </div>

    <!-- Main content -->
    <div class="flex flex-grow flex-col">
      <!-- Navbar -->
      <nav class="flex min-h-16 items-center justify-between bg-slate-900 px-4">
        <button @click="toggleSidebar" class="text-zinc-400 hover:text-white lg:hidden">
          <Bars3Icon class="size-6" />
        </button>
        <div class="flex flex-grow justify-center">
          <div class="relative w-3/5">
            <input
              type="text"
              placeholder="Search..."
              class="w-full rounded-md bg-slate-600 px-4 py-2 pr-10 text-white placeholder-zinc-400 ring-1 ring-slate-600 focus:outline-none focus:ring-zinc-400"
            />
            <button
              class="absolute right-2 top-1/2 -translate-y-1/2 transform text-zinc-400 hover:text-white"
            >
              <MagnifyingGlassIcon class="size-5" />
            </button>
          </div>
        </div>
      </nav>

      <!-- Page content -->
      <main class="flex-grow overflow-y-auto bg-gradient-to-tr from-gray-800 to-zinc-950 p-4">
        <router-view :key="$route.name + JSON.stringify($route.params)" />
      </main>
    </div>
  </div>
</template>
