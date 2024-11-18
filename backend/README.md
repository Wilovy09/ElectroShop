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

## Notas Importantes

1. La autenticación utiliza tokens JWT que deben incluirse en el header `Authorization` con el formato `Bearer <token>`.
2. Los endpoints que requieren rol de Administrador devolverán un error 401 si el usuario no tiene los permisos necesarios.
3. El campo `deleted` en productos es opcional y puede ser null.
4. Los IDs son números enteros (i64).
5. Los precios se manejan como números de punto flotante (f64).

