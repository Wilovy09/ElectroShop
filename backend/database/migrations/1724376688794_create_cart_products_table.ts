import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'cart_products'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table.primary(['cart_id', 'product_id'])

      table.integer('cart_id').unsigned().references('id').inTable('carts').onDelete('CASCADE')
      table
        .integer('product_id')
        .unsigned()
        .references('id')
        .inTable('products')
        .onDelete('CASCADE')
      table.integer('quantity').notNullable().comment('Cantidad de producto en el carrito')

      table.timestamp('created_at')
      table.timestamp('updated_at')
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}
