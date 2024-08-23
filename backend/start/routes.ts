import AuthController from '#controllers/auth_controller'
import router from '@adonisjs/core/services/router'
import { middleware } from './kernel.js'
import RolesController from '#controllers/roles_controller'

router.get('/', async () => {
  return {
    healthCheck: 'ok',
  }
})

router.post('/register', [AuthController, 'register']).as('auth.register')
router.post('/login', [AuthController, 'login']).as('auth.login')
router.delete('/logout', [AuthController, 'logout']).as('auth.logout').use(middleware.auth())
router.get('/me', [AuthController, 'me']).as('auth.me')

router.post('/roles', [RolesController, 'create']).as('roles.create').use(middleware.auth())
router.delete('/roles/:id', [RolesController, 'delete']).as('roles.delete').use(middleware.auth())
