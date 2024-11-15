<script setup lang="ts">
import { ref, computed } from "vue";
import { useRouter } from "vue-router";
import handleError from "../../helpers/handleError";
import { UserCircleIcon } from "@heroicons/vue/24/solid";
import StarBackground from "./components/StarBackground.vue";
import PasswordDetails from "./components/PasswordDetails.vue";

const router = useRouter();
// const userStore = useUserStore();

const authAction = ref<"login" | "register">("login");
const email = ref("");
const password = ref("");
const repeatedPassword = ref("");

const isLoading = ref(false);
const needsEmail = ref(false);
const needsPassword = ref(false);
const wrongRepeatedPassword = ref(false);
const hasTriedToRegister = ref(false);

const lengthRegex = /.{8,}/;
const uppercaseRegex = /[A-Z]/;
const specialCharRegex = /[!@#$%^&*(),.?:{}|<>]/;

const isLengthValid = computed(() => lengthRegex.test(password.value));
const isUppercaseValid = computed(() => uppercaseRegex.test(password.value));
const isSpecialCharValid = computed(() =>
  specialCharRegex.test(password.value)
);
const isValidPassword = computed(
  () =>
    isLengthValid.value && isUppercaseValid.value && isSpecialCharValid.value
);

// Form submission handler
const handleSubmit = async () => {
  let valid = true;

  if (!email.value) {
    needsEmail.value = true;
    valid = false;
  }

  if (!password.value) {
    needsPassword.value = true;
    valid = false;
  }
  if (!email.value.includes("@") && email.value) {
    needsEmail.value = true;
    valid = false;
    handleError("Correo no valido");
  }
  if (authAction.value === "register") {
    hasTriedToRegister.value = true;
    if (
      repeatedPassword.value !== password.value ||
      (password.value && !repeatedPassword.value)
    ) {
      wrongRepeatedPassword.value = true;
      handleError("La contraseña no coincide");
      valid = false;
    }
    if (!repeatedPassword.value) {
      wrongRepeatedPassword.value = true;
      valid = false;
    }
    if (!lengthRegex.test(password.value)) {
      needsPassword.value = true;
      handleError("La contraseña debe contener al menos 8 caracteres");
      valid = false;
    }
    if (!uppercaseRegex.test(password.value)) {
      needsPassword.value = true;
      handleError("La contraseña debe contener al menos una letra mayúscula");
      valid = false;
    }
    if (!specialCharRegex.test(password.value)) {
      needsPassword.value = true;
      handleError("La contraseña debe contener al menos un carácter especial");
      valid = false;
    }
  }

  if (!valid) return;

  try {
    isLoading.value = true;
    if (authAction.value === "login") {
      //   await userStore.login(email.value, password.value);
      console.log("login");
    } else {
      //   await userStore.register(email.value, password.value);
      console.log("register");
    }
    // router.push("/");
  } catch (e) {
    handleError(e);
  } finally {
    isLoading.value = false;
  }
};
</script>
<template>
  <div
    class="text-zinc-400 flex justify-center max-sm:items-end sm:items-center bg-gradient-to-tr from-gray-800 to-zinc-950 w-screen h-screen relative overflow-hidden"
  >
    <div
      class="bg-slate-800 w-full max-w-md p-8 rounded-xl z-10 transition-all duration-300 ease-in-out"
    >
      <UserCircleIcon
        class="w-32 h-32 text-white mx-auto -mt-24 rounded-full bg-slate-800"
      />

      <div class="text-center">
        <h1 class="text-white -mt-4 mb-4 font-mono text-5xl font-bold">
          Electroshop
        </h1>
      </div>

      <!-- Auth Toggle Buttons -->
      <div class="w-full mt-2 mb-4">
        <button
          @click="authAction = 'login'"
          :class="[
            'w-1/2 p-1 relative overflow-hidden duration-300',
            authAction === 'login' ? 'text-white' : 'hover:text-white',
          ]"
        >
          Iniciar Sesión
          <span
            :class="[
              'absolute bottom-0 left-0 w-full h-0.5 rounded-full bg-white transform origin-right transition-transform duration-300',
              authAction === 'login' ? 'scale-x-100' : 'scale-x-0',
            ]"
          ></span>
        </button>
        <button
          @click="authAction = 'register'"
          :class="[
            'w-1/2 p-1 relative overflow-hidden duration-300',
            authAction === 'register' ? 'text-white' : 'hover:text-white',
          ]"
        >
          Registrarse
          <span
            :class="[
              'absolute bottom-0 left-0 w-full h-0.5 rounded-full bg-white transform origin-left transition-transform duration-300',
              authAction === 'register' ? 'scale-x-100' : 'scale-x-0',
            ]"
          ></span>
        </button>
      </div>

      <!-- Auth Form -->
      <form @submit.prevent="handleSubmit" class="flex flex-col justify-center">
        <div class="mb-2">
          <label
            for="email"
            :class="['block mb-1', needsEmail ? 'text-red-500' : '']"
          >
            Correo
          </label>
          <input
            id="email"
            v-model="email"
            :class="[
              'w-full text-white rounded-md border p-1.5 outline-none bg-slate-600',
              needsEmail ? 'border-red-600' : 'border-transparent',
            ]"
            type="text"
            @input="needsEmail = false"
          />
        </div>

        <div :class="authAction === 'register' ? 'mt-4' : 'mt-4 mb-8'">
          <label
            for="password"
            :class="['block mb-1', needsPassword ? 'text-red-500' : '']"
          >
            Contraseña
          </label>
          <input
            id="password"
            v-model="password"
            :class="[
              'w-full text-white rounded-md border p-1.5 outline-none bg-slate-600',
              needsPassword ? 'border-red-600' : 'border-transparent',
            ]"
            type="password"
            @input="needsPassword = false"
          />
        </div>

        <!-- Password Info Hover -->
        <PasswordDetails
          v-if="authAction === 'register'"
          :isValidPassword="isValidPassword"
          :hasTriedToRegister="hasTriedToRegister"
          :isLengthValid="isLengthValid"
          :isUppercaseValid="isUppercaseValid"
          :isSpecialCharValid="isSpecialCharValid"
        />

        <!-- Repeat Password Field -->
        <div
          :class="[
            'transition-all duration-300 ease-in-out overflow-hidden',
            authAction === 'register' ? 'max-h-20  mb-8 ' : 'max-h-0 ',
          ]"
        >
          <label
            for="repeatPassword"
            :class="['block mb-1', wrongRepeatedPassword ? 'text-red-500' : '']"
          >
            Repetir contraseña
          </label>
          <input
            id="repeatPassword"
            v-model="repeatedPassword"
            :class="[
              'w-full text-white rounded-md border p-1.5 outline-none bg-slate-600',
              wrongRepeatedPassword ? 'border-red-600' : 'border-transparent',
            ]"
            type="password"
            @input="wrongRepeatedPassword = false"
          />
        </div>

        <!-- Submit Button -->
        <div v-if="isLoading" class="w-10 h-10 mx-auto"></div>
        <input
          v-else
          class="bg-zinc-950 text-white hover:text-zinc-400 hover:scale-95 duration-150 ease-in-out w-40 mx-auto cursor-pointer rounded-xl py-2"
          type="submit"
          value="Continuar"
        />
      </form>
    </div>

    <!-- Stars Background -->
    <StarBackground />
  </div>
</template>
