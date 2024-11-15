<script setup lang="ts">
import { ref } from "vue";
import { InformationCircleIcon } from "@heroicons/vue/24/solid";

// Props
defineProps<{
  isValidPassword: boolean;
  hasTriedToRegister: boolean;
  isLengthValid: boolean;
  isUppercaseValid: boolean;
  isSpecialCharValid: boolean;
}>();

// Local state
const isInfoOpen = ref(false);
const timeoutLeaveRef = ref<number | null>(null);
const timeoutEnterRef = ref<number | null>(null);

// Methods
const handleInfoMouseEnter = () => {
  if (timeoutLeaveRef.value) clearTimeout(timeoutLeaveRef.value);
  timeoutEnterRef.value = window.setTimeout(() => {
    isInfoOpen.value = true;
  }, 500);
};

const handleInfoMouseLeave = () => {
  if (timeoutEnterRef.value) clearTimeout(timeoutEnterRef.value);
  timeoutLeaveRef.value = window.setTimeout(() => {
    isInfoOpen.value = false;
  }, 150);
};
</script>
<template>
  <div class="relative w-max inline-block mt-1 mb-3 text-zinc-400">
    <div
      class="flex w-max items-center gap-1 cursor-pointer"
      @click="isInfoOpen = true"
      @mouseenter="handleInfoMouseEnter"
      @mouseleave="handleInfoMouseLeave"
    >
      <InformationCircleIcon
        :class="[
          'size-4',
          hasTriedToRegister && !isValidPassword
            ? 'text-red-500'
            : 'text-zinc-400',
        ]"
      />
      <p
        :class="[
          'text-xs',
          hasTriedToRegister && !isValidPassword
            ? 'text-red-500'
            : 'text-zinc-400',
        ]"
      >
        Características de la contraseña
      </p>
    </div>
    <div
      class="absolute z-10 p-2 bg-slate-700 rounded shadow-lg transition-all duration-300 ease-in-out w-max"
      :class="[
        isInfoOpen ? 'opacity-100 translate-x-0' : 'opacity-0 -translate-x-2',
        isInfoOpen ? 'visible' : 'invisible',
      ]"
      style="top: 0; left: calc(100% + 10px)"
      @mouseenter="handleInfoMouseEnter"
      @mouseleave="handleInfoMouseLeave"
    >
      <ul class="list-disc">
        <li
          :class="[
            'ml-5 text-xs mr-2.5',
            isLengthValid || !hasTriedToRegister
              ? 'text-zinc-400'
              : 'text-red-500',
          ]"
        >
          Al menos 8 caracteres
        </li>
        <li
          :class="[
            'ml-5 text-xs mr-2.5',
            isUppercaseValid || !hasTriedToRegister
              ? 'text-zinc-400'
              : 'text-red-500',
          ]"
        >
          Una letra en mayúscula
        </li>
        <li
          :class="[
            'ml-5 text-xs mr-2.5',
            isSpecialCharValid || !hasTriedToRegister
              ? 'text-zinc-400'
              : 'text-red-500',
          ]"
        >
          Un carácter especial
        </li>
      </ul>
    </div>
  </div>
</template>
