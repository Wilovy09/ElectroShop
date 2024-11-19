import { defineStore } from 'pinia'
import { AuthRepository } from '../repositories/authRepository'
import { ref } from 'vue'
import { jwtDecode, JwtPayload } from 'jwt-decode'
import { Router } from 'vue-router'

type userInfo = {
  user_id: string
  user_role: 'Administrador' | 'Cliente'
}

const authRepository = AuthRepository

export const useUserStore = defineStore('user', () => {
  const token = ref<string | null>(getToken())
  const userId = ref<string | null>(getUserId())
  const userRole = ref<'Administrador' | 'Cliente' | null>(getUserRole())

  async function login(email: string, password: string) {
    const response = await authRepository.login(email, password)
    token.value = response.token
    localStorage.setItem('token', JSON.stringify(token.value))
    setUser()
  }

  async function register(email: string, password: string) {
    const response = await authRepository.register(email, password)
    token.value = response.token
    localStorage.setItem('token', JSON.stringify(token.value))
    setUser()
  }

  function getToken() {
    const token = localStorage.getItem('token')
    return token ? JSON.parse(token) : null
  }

  async function setUser() {
    const jwt = jwtDecode(token.value!) as JwtPayload & userInfo
    userId.value = jwt.user_id
    userRole.value = jwt.user_role
    localStorage.setItem('userId', JSON.stringify(jwt.user_id))
    localStorage.setItem('userRole', JSON.stringify(jwt.user_role))
  }

  function getUserRole() {
    const role = localStorage.getItem('userRole')
    return role ? JSON.parse(role) : null
  }
  function getUserId() {
    const userId = localStorage.getItem('userId')
    return userId ? JSON.parse(userId) : null
  }
  function logout(router: Router) {
    token.value = null
    userId.value = null
    userRole.value = null
    localStorage.removeItem('token')
    localStorage.removeItem('userId')
    localStorage.removeItem('userRole')
    router.push({ name: 'Authentication' })
  }

  return {
    token,
    userId,
    userRole,
    login,
    register,
    logout,
    getToken,
    getUserId,
    getUserRole
  }
})
