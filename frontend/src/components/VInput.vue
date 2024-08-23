<script setup lang="ts">
import { EyeIcon, EyeSlashIcon } from '@heroicons/vue/24/outline'
import { ref } from 'vue'

const props = defineProps<{
  modelValue: string | null
  labelText: string
  isDisabled?: boolean
  type: 'text' | 'email' | 'password'
  hasEye?: boolean
}>()
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const isPassword = ref(props.type === 'password')
const showEye = props.hasEye && props.type === 'password'

const togglePasswordVisibility = () => {
  isPassword.value = !isPassword.value
}
</script>

<template>
  <div class="mt-3 relative">
    <label class="text-base">{{ labelText }}</label>
    <div class="flex">
      <input
        :type="isPassword ? 'password' : 'text'"
        :value="modelValue"
        @input="$emit('update:modelValue', ($event.target as any).value)"
        v-bind="$attrs"
        :disabled="isDisabled"
        :class="{ 'text-base': isDisabled }"
        class=" rounded-lg w-full p-2 pr-10"
      />
      <button v-if="showEye" type="button" @click="togglePasswordVisibility" class="-ml-9">
        <EyeSlashIcon v-if="isPassword" class="w-6" />
        <EyeIcon v-else class="w-6" />
      </button>
    </div>
  </div>
</template>

<style scoped></style>
