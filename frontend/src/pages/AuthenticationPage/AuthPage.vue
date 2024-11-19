<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import handleError from '../../helpers/handleError'
import { UserCircleIcon } from '@heroicons/vue/24/solid'
import StarBackground from './components/StarBackground.vue'
import PasswordDetails from './components/PasswordDetails.vue'
import { useUserStore } from '../../stores/useUserStore'
import Loader from './components/Loader.vue'

const router = useRouter()
const userStore = useUserStore()

const authAction = ref<'login' | 'register'>('login')
const email = ref('')
const password = ref('')
const repeatedPassword = ref('')

const isLoading = ref(false)
const needsEmail = ref(false)
const needsPassword = ref(false)
const wrongRepeatedPassword = ref(false)
const hasTriedToRegister = ref(false)

const lengthRegex = /.{8,}/
const uppercaseRegex = /[A-Z]/
const specialCharRegex = /[!@#$%^&*(),.?:{}|<>]/

const isLengthValid = computed(() => lengthRegex.test(password.value))
const isUppercaseValid = computed(() => uppercaseRegex.test(password.value))
const isSpecialCharValid = computed(() => specialCharRegex.test(password.value))
const isValidPassword = computed(
  () => isLengthValid.value && isUppercaseValid.value && isSpecialCharValid.value
)

function validateEmail(email: string) {
  if (!email) return 'Ingrese un correo'
  if (!email.includes('@')) return 'Correo no válido'
  return null
}

function validatePassword(password: string) {
  if (!password) return 'Ingrese una contraseña'

  return null
}

function validatePasswordOnRegister(password: string) {
  if (!lengthRegex.test(password)) return 'La contraseña debe contener al menos 8 caracteres'
  if (!uppercaseRegex.test(password))
    return 'La contraseña debe contener al menos una letra mayúscula'
  if (!specialCharRegex.test(password))
    return 'La contraseña debe contener al menos un carácter especial'
  return null
}

function validateRepeatedPassword(password: string, repeatedPassword: string) {
  if (!repeatedPassword) return 'Debe repetir la contraseña'
  if (password !== repeatedPassword) return 'La contraseña no coincide'
  return null
}

const handleSubmit = async () => {
  const errors = ref<{
    email?: string
    password?: string
    repeatedPassword?: string
  }>({})

  const emailError = validateEmail(email.value)
  if (emailError) errors.value.email = emailError

  const passwordError = validatePassword(password.value)
  if (passwordError) errors.value.password = passwordError

  if (authAction.value === 'register') {
    const passwordErrorRegister = validatePasswordOnRegister(password.value)
    const repeatedPasswordError = validateRepeatedPassword(password.value, repeatedPassword.value)
    if (passwordErrorRegister) errors.value.password = passwordErrorRegister
    if (repeatedPasswordError) errors.value.repeatedPassword = repeatedPasswordError
  }

  if (Object.keys(errors.value).length > 0) {
    needsEmail.value = Boolean(errors.value.email)
    needsPassword.value = Boolean(errors.value.password)
    wrongRepeatedPassword.value = Boolean(errors.value.repeatedPassword)

    Object.values(errors.value).forEach(function (message) {
      handleError(message)
    })
    return
  }

  try {
    isLoading.value = true
    if (authAction.value === 'login') {
      await userStore.login(email.value, password.value)
    } else {
      await userStore.register(email.value, password.value)
    }
    router.push('/')
  } catch (e) {
    handleError(e)
  } finally {
    isLoading.value = false
  }
}
</script>
<template>
  <div
    class="relative flex h-screen w-screen justify-center overflow-hidden bg-gradient-to-tr from-gray-800 to-zinc-950 text-zinc-400 max-sm:items-end sm:items-center"
  >
    <div
      class="z-10 w-full max-w-md rounded-xl bg-slate-800 p-8 transition-all duration-300 ease-in-out"
    >
      <UserCircleIcon class="mx-auto -mt-24 h-32 w-32 rounded-full bg-slate-800 text-white" />

      <div class="text-center">
        <h1 class="-mt-4 mb-4 font-mono text-5xl font-bold text-white">Electroshop</h1>
      </div>

      <div class="mb-4 mt-2 w-full">
        <button
          @click="authAction = 'login'"
          :class="[
            'relative w-1/2 overflow-hidden p-1 duration-300',
            authAction === 'login' ? 'text-white' : 'hover:text-white'
          ]"
        >
          Iniciar Sesión
          <span
            :class="[
              'absolute bottom-0 left-0 h-0.5 w-full origin-right transform rounded-full bg-white transition-transform duration-300',
              authAction === 'login' ? 'scale-x-100' : 'scale-x-0'
            ]"
          ></span>
        </button>
        <button
          @click="authAction = 'register'"
          :class="[
            'relative w-1/2 overflow-hidden p-1 duration-300',
            authAction === 'register' ? 'text-white' : 'hover:text-white'
          ]"
        >
          Registrarse
          <span
            :class="[
              'absolute bottom-0 left-0 h-0.5 w-full origin-left transform rounded-full bg-white transition-transform duration-300',
              authAction === 'register' ? 'scale-x-100' : 'scale-x-0'
            ]"
          ></span>
        </button>
      </div>

      <form @submit.prevent="handleSubmit" class="flex flex-col justify-center">
        <div class="mb-2">
          <label for="email" :class="['mb-1 block', needsEmail ? 'text-red-500' : '']">
            Correo
          </label>
          <input
            id="email"
            v-model="email"
            :class="[
              'w-full rounded-md border bg-slate-600 p-1.5 text-white outline-none',
              needsEmail ? 'border-red-600' : 'border-transparent'
            ]"
            type="text"
            @input="needsEmail = false"
          />
        </div>

        <div :class="authAction === 'register' ? 'mt-4' : 'mb-8 mt-4'">
          <label for="password" :class="['mb-1 block', needsPassword ? 'text-red-500' : '']">
            Contraseña
          </label>
          <input
            id="password"
            v-model="password"
            :class="[
              'w-full rounded-md border bg-slate-600 p-1.5 text-white outline-none',
              needsPassword ? 'border-red-600' : 'border-transparent'
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
            'overflow-hidden transition-all duration-300 ease-in-out',
            authAction === 'register' ? 'mb-8 max-h-20' : 'max-h-0'
          ]"
        >
          <label
            for="repeatPassword"
            :class="['mb-1 block', wrongRepeatedPassword ? 'text-red-500' : '']"
          >
            Repetir contraseña
          </label>
          <input
            id="repeatPassword"
            v-model="repeatedPassword"
            :disabled="authAction !== 'register'"
            :class="[
              'w-full rounded-md border bg-slate-600 p-1.5 text-white outline-none',
              wrongRepeatedPassword ? 'border-red-600' : 'border-transparent'
            ]"
            type="password"
            @input="wrongRepeatedPassword = false"
          />
        </div>

        <div v-if="isLoading" class="mx-auto h-10 w-10"><Loader /></div>
        <input
          v-else
          class="mx-auto w-40 cursor-pointer rounded-xl bg-zinc-950 py-2 text-white duration-150 ease-in-out hover:scale-95 hover:text-zinc-400"
          type="submit"
          value="Continuar"
        />
      </form>
    </div>

    <StarBackground />
  </div>
</template>
