import Role from '#models/role'
import type { HttpContext } from '@adonisjs/core/http'

export default class RolesController {
  async getAll({ response }: HttpContext) {
    const roles = await Role.all()
    return response.ok({ roles })
  }

  async create({ request, response }: HttpContext) {
    const { nameRole } = request.only(['nameRole'])
    try {
      const role = new Role()
      role.nameRole = nameRole
      await role.save()
      return response.created({ message: 'Role created successfully', role })
    } catch (e) {
      return response.badRequest({ message: 'Failed to create role', e })
    }
  }

  async delete({ params, response }: HttpContext) {
    const roleId = params.id
    try {
      const role = await Role.findOrFail(roleId)
      await role.delete()
      return response.ok({ message: 'Role deleted successfully' })
    } catch (e) {
      return response.notFound({ message: 'Role not found', e })
    }
  }
}
