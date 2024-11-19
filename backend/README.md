# Documentación API ElectroShop

Esta documentación describe los endpoints disponibles en la API de ElectroShop. El servidor base está localizado en `http://localhost:8080`.

## Autenticación

La API utiliza autenticación JWT (JSON Web Token). Los tokens tienen una duración de 525,600 minutos (365 días).

### POST /register

Registra un nuevo usuario en el sistema.

**Request Body:**
```json
{
    "email": "usuario@ejemplo.com",
    "password": "Contraseña123!"
}
```

**Respuesta Exitosa (200):**
```json
{
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Posibles Errores:**

- 400 Bad Request:
```json
{
    "message": "Password must be at least 8 characters long, contain at least one uppercase letter, and one special character."
}
```
```json
{
    "message": "The email is already registered"
}
```

- 500 Internal Server Error:
```json
{
    "message": "Error hashing password"
}
```
```json
{
    "message": "Error creating user"
}
```

### POST /login

Inicia sesión de un usuario existente.

**Request Body:**
```json
{
    "email": "usuario@ejemplo.com",
    "password": "Contraseña123!"
}
```

**Respuesta Exitosa (200):**
```json
{
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Posibles Errores:**

- 401 Unauthorized:
```json
{
    "message": "Invalid credentials"
}
```
```json
{
    "message": "Invalid token"
}
```

## Categorías

### POST /category

Crea una nueva categoría. Requiere rol de Administrador.

**Request Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
    "name": "Nombre Categoría"
}
```

**Respuesta Exitosa (200):**
```json
{
    "message": "Category created"
}
```

**Posibles Errores:**

- 401 Unauthorized:
```json
{
    "message": "Invalid role"
}
```

### GET /categories

Obtiene todas las categorías.

**Respuesta Exitosa (200):**
```json
[
    {
        "id": 1,
        "name": "Electrónicos"
    }
]
```

**Posibles Errores:**

- 404 Not Found:
```json
{
    "message": "Not categories"
}
```

### DELETE /categories/{id}

Elimina una categoría. Requiere rol de Administrador.

**Request Headers:**
```
Authorization: Bearer <token>
```

**Respuesta Exitosa (204):**
*No content*

**Posibles Errores:**

- 404 Not Found:
```json
{
    "message": "Category not found"
}
```

## Productos

### POST /product

Crea un nuevo producto. Requiere rol de Administrador.

**Request Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
    "category_id": 1,
    "name": "Nombre Producto",
    "image": "url_imagen",
    "description": "Descripción del producto",
    "price": 99.99,
    "units": 100,
    "deleted": null
}
```

**Respuesta Exitosa (200):**
```json
{
    "category_id": 1,
    "name": "Nombre Producto",
    "image": "url_imagen",
    "description": "Descripción del producto",
    "price": 99.99,
    "units": 100,
    "deleted": null
}
```

### GET /products

Obtiene todos los productos.

**Respuesta Exitosa (200):**
```json
[
    {
        "id": 1,
        "category_id": 1,
        "name": "Nombre Producto",
        "image": "url_imagen",
        "description": "Descripción del producto",
        "price": 99.99,
        "units": 100,
        "deleted": null
    }
]
```

### GET /products/{id}

Obtiene un producto específico por ID.

**Respuesta Exitosa (200):**
```json
{
    "id": 1,
    "category_id": 1,
    "name": "Nombre Producto",
    "image": "url_imagen",
    "description": "Descripción del producto",
    "price": 99.99,
    "units": 100,
    "deleted": null
}
```

### PUT /products/{id}

Actualiza un producto existente. Requiere rol de Administrador.

**Request Headers:**
```
Authorization: Bearer <token>
```

**Request Body:**
```json
{
    "category_id": 1,
    "name": "Nombre Actualizado",
    "image": "nueva_url_imagen",
    "description": "Nueva descripción",
    "price": 149.99,
    "units": 50,
    "deleted": null
}
```

**Respuesta Exitosa (200):**
```json
{
    "id": 1,
    "category_id": 1,
    "name": "Nombre Actualizado",
    "image": "nueva_url_imagen",
    "description": "Nueva descripción",
    "price": 149.99,
    "units": 50,
    "deleted": null
}
```

### DELETE /products/{id}

Elimina un producto. Requiere rol de Administrador.

**Request Headers:**
```
Authorization: Bearer <token>
```

**Respuesta Exitosa (200):**
```json
{
    "id": 1,
    "category_id": 1,
    "name": "Nombre Producto",
    "image": "url_imagen",
    "description": "Descripción del producto",
    "price": 99.99,
    "units": 100,
    "deleted": null
}
```

### GET /category/{id}/products

Obtiene todos los productos de una categoría específica.

**Respuesta Exitosa (200):**
```json
[
    {
        "id": 1,
        "category_id": 1,
        "name": "Nombre Producto",
        "image": "url_imagen",
        "description": "Descripción del producto",
        "price": 99.99,
        "units": 100,
        "deleted": null
    }
]
```

**Posibles Errores:**

- 404 Not Found:
```json
{
    "message": "No products found in this category"
}
```

## Historial de ventas

### POST /sell

Crea una nueva venta con sus transacciones asociadas.

**Request Body:**
```json
{
    "user_id": 1,
    "total_amount": 299.99,
    "transactions": [
        {
            "product_name": "Smartphone XYZ",
            "id_product": 1,
            "quantity": 2
        },
        {
            "product_name": "Auriculares ABC",
            "id_product": 3,
            "quantity": 1
        }
    ]
}
```

**Respuesta Exitosa (204):**
*No content*

**Posibles Errores:**

- 500 Internal Server Error:
```json
{
    "message": "Failed to create sell"
}
```
```json
{
    "message": "Failed to create transaction"
}
```

### GET /sell/{user_id}

Obtiene todas las transacciones de venta de un usuario específico.

**Parámetros URL:**
- `user_id`: ID del usuario (i64)

**Respuesta Exitosa (200):**
```json
[
    {
        "total_amount": 299.99,
        "sell_date": "2024-01-01T12:00:00",
        "product_name": "Smartphone XYZ",
        "id_sell": 1,
        "id_product": 1,
        "quantity": 2
    },
    {
        "total_amount": 299.99,
        "sell_date": "2024-01-01T12:00:00",
        "product_name": "Auriculares ABC",
        "id_sell": 1,
        "id_product": 3,
        "quantity": 1
    }
]
```

**Posibles Errores:**

- 500 Internal Server Error:
```json
{
    "message": "Failed to fetch sell transactions"
}
```

### GET /sell

Obtiene todas las transacciones de venta del sistema. Requiere rol de Administrador.

**Request Headers:**
```
Authorization: Bearer <token>
```

**Respuesta Exitosa (200):**
```json
[
    {
        "total_amount": 299.99,
        "sell_date": "2024-01-01T12:00:00",
        "product_name": "Smartphone XYZ",
        "id_sell": 1,
        "id_product": 1,
        "quantity": 2
    },
    {
        "total_amount": 149.99,
        "sell_date": "2024-01-02T15:30:00",
        "product_name": "Tablet ABC",
        "id_sell": 2,
        "id_product": 4,
        "quantity": 1
    }
]
```

**Posibles Errores:**

- 401 Unauthorized:
```json
{
    "message": "Invalid role"
}
```

- 500 Internal Server Error:
```json
{
    "message": "Failed to fetch sell transactions"
}
```

## Notas Importantes

1. La autenticación utiliza tokens JWT que deben incluirse en el header `Authorization` con el formato `Bearer <token>`.
2. Los endpoints que requieren rol de Administrador devolverán un error 401 si el usuario no tiene los permisos necesarios.
3. El campo `deleted` en productos es opcional y puede ser null.
4. Los IDs son números enteros (i64).
5. Los precios se manejan como números de punto flotante (f64).

