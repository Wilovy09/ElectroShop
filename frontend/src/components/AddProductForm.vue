<script setup lang="ts">
import { ref, computed } from 'vue'
import handleError from '../helpers/handleError'
import { useCategoryStore } from '../stores/useCategoryStore'
import { fileToBase64 } from '../helpers/convertToBase64'
import { productsRepository } from '../repositories/productsRepository'
import { Product } from '../entities/Product'
import showSuccedSwal from '../helpers/succedSwal'

const categories = useCategoryStore().categories

const props = defineProps<{
  product: {
    id: number | null
    category_id: number | null
    name: string | null
    image: string | null
    description: string | null
    price: number | null
    units: number | null
    deleted: 'true' | 'false' | null
  }
}>()

const emit = defineEmits<{
  (e: 'update:showModal', value: boolean): void
  (e: 'save', value: Product): void
}>()

const formData = ref({
  category_id: props.product.category_id || null,
  name: props.product.name || '',
  description: props.product.description || '',
  price: props.product.price || null,
  units: props.product.units || null,
  image: props.product.image || null,
  deleted: props.product.deleted || null
})

const errors = ref({
  name: false,
  category_id: false,
  price: false,
  units: false
})

const fileInput = ref<HTMLInputElement | null>(null)

const isFormValid = computed(() => {
  return (
    formData.value.name &&
    formData.value.category_id &&
    formData.value.price &&
    formData.value.units &&
    Number.isInteger(formData.value.price) &&
    formData.value.price > 0 &&
    Number.isInteger(formData.value.units) &&
    formData.value.units >= 0 &&
    formData.value.image
  )
})

const handleImageUpload = async (event: Event) => {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    const file = input.files[0]

    // Validar si el archivo es una imagen
    if (!file.type.startsWith('image/')) {
      alert('Por favor, selecciona un archivo válido.')
      return
    }

    try {
      // Convertir archivo a Base64
      const base64 = await fileToBase64(file)
      formData.value.image = base64
    } catch (error) {
      console.error('Error al procesar la imagen:', error)
      alert('Ocurrió un error al cargar la imagen. Inténtalo de nuevo.')
    }
  }
}

const triggerFileInput = () => {
  fileInput.value?.click()
}

const handleNumberInput = (field: 'price' | 'units', value: string) => {
  const numValue = parseInt(value)
  if (Number.isInteger(numValue)) {
    formData.value[field] = numValue
    errors.value[field] = field === 'price' ? numValue <= 0 : numValue < 0
  } else {
    formData.value[field] = null
    errors.value[field] = true
  }
}

const validateForm = () => {
  errors.value.name = !formData.value.name
  errors.value.category_id = !formData.value.category_id
  errors.value.price =
    !formData.value.price || !Number.isInteger(formData.value.price) || formData.value.price <= 0
  errors.value.units =
    !formData.value.units || !Number.isInteger(formData.value.units) || formData.value.units < 0

  if (errors.value.price || errors.value.units) {
    handleError('El precio y las unidades deben ser números enteros positivos')
  }

  return !Object.values(errors.value).some(Boolean)
}

const isProductChanged = computed(() => {
  const originalProduct = {
    category_id: props.product.category_id,
    name: props.product.name,
    description: props.product.description,
    price: props.product.price,
    units: props.product.units,
    image: props.product.image,
    deleted: props.product.deleted
  }

  return JSON.stringify(originalProduct) !== JSON.stringify(formData.value)
})

const isLoading = ref(false)

async function handleSubmit() {
  isLoading.value = true
  if (!validateForm() || !isProductChanged) {
    return
  }
  const validFormData: Omit<Product, 'id'> = {
    category_id: formData.value.category_id!,
    name: formData.value.name!,
    description: formData.value.description || '',
    price: formData.value.price!,
    units: formData.value.units!,
    image: formData.value.image!,
    deleted: formData.value.deleted
  }

  // Verificación adicional de seguridad
  if (
    !validFormData.category_id ||
    !validFormData.name ||
    validFormData.price === null ||
    validFormData.units === null ||
    validFormData.image === null
  ) {
    handleError('Por favor, complete todos los campos requeridos')
    return
  }

  const response = ref<Product | null>(null)
  try {
    if (!props.product.id) {
      response.value = await productsRepository.addProduct({
        ...validFormData
      })
    } else {
      response.value = await productsRepository.updateProduct({
        id: props.product.id,
        ...validFormData
      })
    }

    if (response.value) {
      showSuccedSwal('Acción realizada con exito')
      emit('save', response.value)
      emit('update:showModal', false)
    }
  } catch (e) {
    handleError(e)
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div
    @click="emit('update:showModal', false)"
    class="modal-backdrop fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
  >
    <div class="flex max-h-[90vh] w-4/5 flex-col rounded-xl bg-slate-800 text-zinc-400" @click.stop>
      <!-- Header fijo -->
      <div class="border-b border-slate-700 p-6">
        <h2 class="text-center text-2xl font-bold text-white">
          {{ props.product.id ? 'Editar Producto' : 'Nuevo Producto' }}
        </h2>
      </div>

      <!-- Contenido scrolleable -->
      <div class="overflow-y-auto p-6">
        <form @submit.prevent="handleSubmit" class="space-y-4">
          <div class="space-y-4 lg:flex lg:space-x-6 lg:space-y-0">
            <!-- Imagen -->
            <div class="lg:w-1/3">
              <div
                @click="triggerFileInput"
                class="flex aspect-square h-full w-full cursor-pointer items-center justify-center rounded-lg border-2 border-dashed transition-colors duration-300 hover:border-white"
                :class="[formData.image ? 'border-green-500' : 'border-zinc-400']"
              >
                <input
                  ref="fileInput"
                  type="file"
                  class="hidden"
                  accept="image/*"
                  @change="handleImageUpload"
                />
                <div v-if="!formData.image" class="text-center">
                  <p>Haga clic para subir una imagen</p>
                </div>
                <img
                  v-else
                  :src="formData.image"
                  alt="Preview"
                  class="h-full w-full rounded-lg object-contain"
                />
              </div>
            </div>

            <!-- Formulario -->
            <div class="space-y-4 lg:w-2/3">
              <!-- Categoría -->
              <div>
                <label class="mb-1 block" :class="[errors.category_id ? 'text-red-500' : '']">
                  Categoría
                </label>
                <select
                  v-model="formData.category_id"
                  :class="[
                    'w-full rounded-md border bg-slate-600 p-1.5 text-white outline-none',
                    errors.category_id ? 'border-red-600' : 'border-transparent'
                  ]"
                  @input="errors.category_id = false"
                >
                  <option :value="null">Seleccione una categoría</option>
                  <option v-for="category in categories" :key="category.id" :value="category.id">
                    {{ category.name }}
                  </option>
                </select>
              </div>

              <!-- Nombre -->
              <div>
                <label class="mb-1 block" :class="[errors.name ? 'text-red-500' : '']">
                  Nombre
                </label>
                <input
                  v-model="formData.name"
                  type="text"
                  :class="[
                    'w-full rounded-md border bg-slate-600 p-1.5 text-white outline-none',
                    errors.name ? 'border-red-600' : 'border-transparent'
                  ]"
                  @input="errors.name = false"
                />
              </div>

              <!-- Descripción -->
              <div>
                <label class="mb-1 block">Descripción</label>
                <textarea
                  v-model="formData.description"
                  class="w-full resize-none rounded-md border border-transparent bg-slate-600 p-1.5 text-white outline-none"
                  style="min-height: 100px; max-height: 200px"
                ></textarea>
              </div>

              <div class="flex space-x-4">
                <!-- Precio -->
                <div class="w-1/2">
                  <label class="mb-1 block" :class="[errors.price ? 'text-red-500' : '']">
                    Precio
                  </label>
                  <input
                    :value="formData.price"
                    type="number"
                    min="1"
                    step="1"
                    :class="[
                      'w-full rounded-md border bg-slate-600 p-1.5 text-white outline-none',
                      errors.price ? 'border-red-600' : 'border-transparent'
                    ]"
                    @input="(e) => handleNumberInput('price', (e.target as HTMLInputElement).value)"
                  />
                </div>

                <!-- Unidades -->
                <div class="w-1/2">
                  <label class="mb-1 block" :class="[errors.units ? 'text-red-500' : '']">
                    Unidades
                  </label>
                  <input
                    :value="formData.units"
                    type="number"
                    min="0"
                    step="1"
                    :class="[
                      'w-full rounded-md border bg-slate-600 p-1.5 text-white outline-none',
                      errors.units ? 'border-red-600' : 'border-transparent'
                    ]"
                    @input="(e) => handleNumberInput('units', (e.target as HTMLInputElement).value)"
                  />
                </div>
              </div>
            </div>
          </div>
        </form>
      </div>

      <!-- Footer fijo -->
      <div class="border-t border-slate-700 p-6">
        <div class="flex justify-end space-x-4">
          <button
            type="button"
            @click="emit('update:showModal', false)"
            class="rounded-lg px-4 py-2 transition-colors duration-300 hover:bg-slate-700"
          >
            Cancelar
          </button>
          <button
            @click="handleSubmit"
            :disabled="!isFormValid || !isProductChanged || isLoading"
            class="rounded-lg bg-zinc-950 px-4 py-2 text-white transition-all duration-300 hover:scale-95"
            :class="{
              'cursor-not-allowed opacity-50': !isFormValid || !isProductChanged || isLoading
            }"
          >
            Guardar
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
