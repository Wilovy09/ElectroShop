import { ref } from 'vue'
import { defineStore } from 'pinia'
import { api } from '@/helpers/apiRequest'

export const useAuthStore = defineStore('auth', () => {
  const user = ref(null)
  const token = ref(localStorage.getItem('token'))

  function authentication(result: any) {
    token.value = result.token
    if (token.value) localStorage.setItem('token', token.value)
  }

  async function login(payload: any) {
    const result = await api('POST', '/login', payload)
    authentication(result)
    setUserID()
    return result
  }

  async function register(payload: any) {
    const result = await api('POST', '/register', payload)
    authentication(result)
    return result
  }

  async function logout() {
    await api('DELETE', '/logut')
    clearUser()
  }

  async function setUserID() {
    const result = await api('GET', '/me')
    const userId = result.user.id
    localStorage.setItem('userId', userId)
    return userId
  }

  async function getUserID() {
    return localStorage.getItem('userId')
  }

  function clearUser() {
    token.value = null
    user.value = null
    localStorage.removeItem('userId')
    localStorage.removeItem('token')
  }

  return { user, token, login, register, logout, getUserID }
})
