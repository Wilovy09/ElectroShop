<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'
import VButton from '@/components/VButton.vue'
import VInput from '@/components/VInput.vue'

const auth = useAuthStore()
const router = useRouter()

const isCreatingAccount = ref(false)
const userEmail = ref('')
const userPassword = ref('')
const confirmPassword = ref('')

const form = computed(() => ({
  email: userEmail.value,
  password: userPassword.value
}))

async function loginUser() {
  try {
    await auth.login(form.value)
    router.push({ name: 'home' })
  } catch (error) {
    console.log(error)
  }
}

async function createUser() {
  if (userPassword.value !== confirmPassword.value) {
    console.log('Passwords don’t match')
    return
  }

  try {
    await auth.register(form.value)
  } catch (error) {
    console.log(error)
  }
}
</script>

<template>
  <main class="flex justify-center min-h-screen items-center">
    <form
      @submit.prevent="isCreatingAccount ? createUser() : loginUser()"
      class="p-8 gap-4 rounded-xl"
    >
      <h1 class="text-center font-bold text-xl">{{ isCreatingAccount ? 'Sign Up' : 'Login' }}</h1>
      <VInput v-model="userEmail" labelText="Email" type="email" />
      <VInput v-model="userPassword" labelText="Password" type="password" hasEye />
      <VInput
        v-if="isCreatingAccount"
        v-model="confirmPassword"
        labelText="Confirm Password"
        type="password"
      />
      <VButton backgroundColor="primary" class="mt-4">
        {{ isCreatingAccount ? 'Sign Up' : 'Login' }}
      </VButton>
      <p
        v-if="!isCreatingAccount"
        @click="isCreatingAccount = true"
        class="text-center mt-2 cursor-pointer hover:underline"
      >
        Don't have an account?
      </p>
      <p
        v-else
        @click="isCreatingAccount = false"
        class="text-center mt-2 cursor-pointer hover:underline"
      >
        I have an account
      </p>
    </form>
  </main>
</template>

<style scoped>
form {
  background: linear-gradient(145deg, #e6e6e6, #ffffff);
  box-shadow:
    20px 20px 60px #d9d9d9,
    -20px -20px 60px #ffffff;
}
</style>
