<script setup lang="ts">
import { ref, computed } from "vue";
import { useRouter } from "vue-router";
import handleError from "../../helpers/handleError";
import { UserCircleIcon } from "@heroicons/vue/24/solid";
import StarBackground from "./components/StarBackground.vue";
import PasswordDetails from "./components/PasswordDetails.vue";
import { useUserStore } from "../../stores/useUserStore";
import Loader from "./components/Loader.vue";

const router = useRouter();
const userStore = useUserStore();

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

function validateEmail(email: string) {
  if (!email) return "Ingrese un correo";
  if (!email.includes("@")) return "Correo no válido";
  return null;
}

function validatePassword(password: string) {
  if (!password) return "Ingrese una contraseña";

  return null;
}

function validatePasswordOnRegister(password: string) {
  if (!lengthRegex.test(password))
    return "La contraseña debe contener al menos 8 caracteres";
  if (!uppercaseRegex.test(password))
    return "La contraseña debe contener al menos una letra mayúscula";
  if (!specialCharRegex.test(password))
    return "La contraseña debe contener al menos un carácter especial";
  return null;
}

function validateRepeatedPassword(password: string, repeatedPassword: string) {
  if (!repeatedPassword) return "Debe repetir la contraseña";
  if (password !== repeatedPassword) return "La contraseña no coincide";
  return null;
}

const handleSubmit = async () => {
  const errors = ref<{
    email?: string;
    password?: string;
    repeatedPassword?: string;
  }>({});

  const emailError = validateEmail(email.value);
  if (emailError) errors.value.email = emailError;

  const passwordError = validatePassword(password.value);
  if (passwordError) errors.value.password = passwordError;

  if (authAction.value === "register") {
    const passwordErrorRegister = validatePasswordOnRegister(password.value);
    const repeatedPasswordError = validateRepeatedPassword(
      password.value,
      repeatedPassword.value
    );
    if (passwordErrorRegister) errors.value.password = passwordErrorRegister;
    if (repeatedPasswordError)
      errors.value.repeatedPassword = repeatedPasswordError;
  }

  if (Object.keys(errors.value).length > 0) {
    needsEmail.value = Boolean(errors.value.email);
    needsPassword.value = Boolean(errors.value.password);
    wrongRepeatedPassword.value = Boolean(errors.value.repeatedPassword);

    Object.values(errors.value).forEach(function (message) {
      console.log(message);
      handleError(message);
    });
    return;
  }

  try {
    isLoading.value = true;
    if (authAction.value === "login") {
      await userStore.login(email.value, password.value);
    } else {
      await userStore.register(email.value, password.value);
    }
    router.push("/");
  } catch (e) {
    console.log(e);
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

        <PasswordDetails
          v-if="authAction === 'register'"
          :isValidPassword="isValidPassword"
          :hasTriedToRegister="hasTriedToRegister"
          :isLengthValid="isLengthValid"
          :isUppercaseValid="isUppercaseValid"
          :isSpecialCharValid="isSpecialCharValid"
        />

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
            :disabled="authAction !== 'register'"
            :class="[
              'w-full text-white rounded-md border p-1.5 outline-none bg-slate-600',
              wrongRepeatedPassword ? 'border-red-600' : 'border-transparent',
            ]"
            type="password"
            @input="wrongRepeatedPassword = false"
          />
        </div>

        <div v-if="isLoading" class="w-10 h-10 mx-auto"><Loader /></div>
        <input
          v-else
          class="bg-zinc-950 text-white hover:text-zinc-400 hover:scale-95 duration-150 ease-in-out w-40 mx-auto cursor-pointer rounded-xl py-2"
          type="submit"
          value="Continuar"
        />
      </form>
    </div>

    <StarBackground />
  </div>
</template>
