<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import {
  XMarkIcon,
  Bars3Icon,
  MagnifyingGlassIcon,
  ShoppingBagIcon,
  ShoppingCartIcon,
  HomeIcon,
  ListBulletIcon,
  ChevronDownIcon,
  PlusIcon,
} from "@heroicons/vue/24/outline";
import { useUserStore } from "../stores/useUserStore";
import { useRouter } from "vue-router";
import { MinusIcon } from "@heroicons/vue/24/solid";

const router = useRouter();
const isAdmin = useUserStore().userRole == "Administrador";

const shouldShowLogoutButton = ref(false);

const isSidebarOpen = ref(false);
const sidebarRef = ref<HTMLElement | null>(null);
const areCategoriesShown = ref(false);

const categories = ref<{ id: string; name: string }[]>([
  { id: "1", name: "Electronica" },
  { id: "2", name: "Electronica" },
  { id: "3", name: "Electronica" },
  { id: "4", name: "Electronica" },
  { id: "5", name: "Electronica" },
]);

//agregar esto a la función donde se obtienen las categorias
function getCategoriesSectionHeight() {
  return categories.value.length * 8 + 3.5 + (isAdmin ? 8 : 0);
}

function toggleCategories() {
  areCategoriesShown.value = !areCategoriesShown.value;
}

function toggleSidebar() {
  isSidebarOpen.value = !isSidebarOpen.value;
}

const handleClickOutside = (event: MouseEvent) => {
  if (sidebarRef.value && !sidebarRef.value.contains(event.target as Node)) {
    isSidebarOpen.value = false;
  }
};

onMounted(() => {
  document.addEventListener("mousedown", handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener("mousedown", handleClickOutside);
});
</script>
<template>
  <div class="flex flex-grow-0 w-screen h-screen">
    <!-- Overlay for small screens -->
    <div
      v-if="isSidebarOpen"
      class="fixed inset-0 backdrop-blur-sm z-20 md:hidden duration-300 transition-all"
      @click="isSidebarOpen = false"
    ></div>

    <!-- Sidebar -->
    <div
      ref="sidebarRef"
      :class="[
        'fixed inset-y-0 left-0 z-30 min-w-64 overflow-hidden bg-slate-800 transform transition-transform duration-300 ease-in-out shadow-xl shadow-zinc-950/50',
        isSidebarOpen ? 'translate-x-0' : '-translate-x-full',
        'md:relative md:translate-x-0',
      ]"
    >
      <!-- Franja simulando la continuación de la navbar -->
      <div
        class="h-16 bg-slate-900 absol flex items-center justify-between px-4"
      >
        <h2 class="text-xl font-semibold text-white">ElectroShop</h2>
        <button
          @click="toggleSidebar"
          class="md:hidden text-zinc-400 hover:text-white"
        >
          <XMarkIcon class="size-6" />
        </button>
      </div>

      <div class="text-zinc-400">
        <!-- Add your sidebar content here -->
        <div class="flex flex-col">
          <RouterLink
            :to="{ name: 'Home' }"
            class="w-full flex items-center border-y border-slate-600 p-4 text-left hover:text-white hover:border-white transition-all duration-300"
          >
            <HomeIcon class="size-6 mr-2" />
            <p>Inicio</p>
          </RouterLink>
          <RouterLink
            :to="{ name: '' }"
            class="w-full flex items-center border-y border-slate-600 p-4 text-left hover:text-white hover:border-white transition-all duration-300"
          >
            <ShoppingCartIcon class="size-6 mr-2" />
            <p>Carrito</p>
          </RouterLink>
          <RouterLink
            :to="{ name: '' }"
            class="w-full flex items-center border-y border-slate-600 p-4 text-left hover:text-white hover:border-white transition-all duration-300"
          >
            <ShoppingBagIcon class="size-6 mr-2" />
            <p>Compras</p>
          </RouterLink>
          <div
            class="w-full border-y border-slate-600 hover:border-white transition-all duration-300 ease-in-out"
          >
            <button
              @click="toggleCategories()"
              class="w-full flex justify-between items-center p-4 text-left hover:text-white"
            >
              <div class="flex items-center">
                <ListBulletIcon class="size-6 mr-2" />
                <p>Categorías</p>
              </div>
              <ChevronDownIcon
                :class="[
                  'size-4 transition-all duration-300',
                  areCategoriesShown ? 'rotate-180' : '',
                ]"
              />
            </button>
            <div
              :class="[
                'transition-all duration-300 ease-in overflow-hidden',
                areCategoriesShown ? `h-full` : 'h-0',
              ]"
              :style="{
                maxHeight: areCategoriesShown
                  ? `${getCategoriesSectionHeight() * 4}px`
                  : '0px',
              }"
            >
              <ul class="list-disc flex-col flex ml-8 mb-3.5 text-sm">
                <div v-for="category in categories" :key="category.id">
                  <button
                    @click="console.log('seleccionado')"
                    :disabled="!areCategoriesShown"
                    class="hover:text-white w-fit pl-6 py-1 my-0.5"
                  >
                    <li :class="isAdmin ? 'w-28 mr-3' : 'w-full'">
                      <div class="flex items-center justify-between text-left">
                        <p>{{ category.name }}</p>
                      </div>
                    </li></button
                  ><button
                    v-if="isAdmin"
                    :disabled="!areCategoriesShown"
                    @click="console.log('eliminado')"
                    class="bg-white/10 mr-2 p-0.5 backdrop-blur-md rounded-full"
                  >
                    <MinusIcon class="size-4" />
                  </button>
                </div>
                <button
                  v-if="isAdmin"
                  :disabled="!areCategoriesShown"
                  class="hover:text-white bg-slate-950 rounded-lg w-fit pl-6 pr-2 py-1 my-0.5"
                >
                  <li>
                    <div
                      class="w-36 flex items-center justify-between text-left"
                    >
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
          class="w-full rounded-t-md bg-zinc-950 absolute bottom-0 flex justify-center py-2"
          @mouseover="shouldShowLogoutButton = true"
          @mouseleave="shouldShowLogoutButton = false"
        >
          <ChevronDownIcon class="size-4 rotate-180 text-white" />
        </div>
        <div
          :class="[
            'w-full bg-zinc-950 rounded-t-md absolute bottom-0 transition-all duration-300',
            shouldShowLogoutButton ? '' : 'translate-y-16',
          ]"
          @mouseover="shouldShowLogoutButton = true"
          @mouseleave="shouldShowLogoutButton = false"
        >
          <button
            @click="useUserStore().logout(router)"
            class="w-full py-5 active:text-zinc-400 text-white hover:scale-95 transition-all duration-150"
          >
            Cerrar sesión
          </button>
        </div>
      </div>
    </div>

    <!-- Main content -->
    <div class="flex flex-col flex-grow">
      <!-- Navbar -->
      <nav class="bg-slate-900 min-h-16 px-4 flex items-center justify-between">
        <button
          @click="toggleSidebar"
          class="text-zinc-400 hover:text-white md:hidden"
        >
          <Bars3Icon class="size-6" />
        </button>
        <div class="flex justify-center flex-grow">
          <div class="relative w-3/5">
            <input
              type="text"
              placeholder="Search..."
              class="w-full px-4 py-2 pr-10 rounded-md bg-slate-600 text-white placeholder-zinc-400 focus:outline-none ring-slate-600 ring-1 focus:ring-zinc-400"
            />
            <button
              class="absolute right-2 top-1/2 transform -translate-y-1/2 text-zinc-400 hover:text-white"
            >
              <MagnifyingGlassIcon class="size-5" />
            </button>
          </div>
        </div>
      </nav>

      <!-- Page content -->
      <main
        class="flex-grow p-4 overflow-y-auto bg-gradient-to-tr from-gray-800 to-zinc-950"
      >
        <router-view />
      </main>
    </div>
  </div>
</template>
