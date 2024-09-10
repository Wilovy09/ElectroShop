# API documentación

## POST `/user`

Este endpoint recibe un body tipo JSON.

```json
{
    "email": "...@gmail.com",
    "password": "Test12345."
}
```

- La contraseña debe tener minimo 8 caracteres, 1 letra mayusucula y un caracter especial.

### Ok

Regresa un `Ok` o sea un status 200.

```json
{
    "token": "<TOKEN-JWT>",
    "refresh": "<REFRESH-JWT>"
}
```

### Contraseña invalida

Regresa un `BadRequest` con un body JSON.

```json
{
    "error_code": 400,
    "message": "Password must be at least 8 characters long, contain at least one uppercase letter, and one special character".
}
```

### Error al hashear contraseña 

Regresa un `InternalServerError`

```json
{
    "error_code": 500,
    "Error hashing password: <ERROR>"
}
```

### Token invalido al momento de crearlo

Regresea un `Unauthorized`

```json
{
    "error_code": 401,
    "message": "Invalid token"
}
```

### Correo existente

Regresa un `BadRequest`

```json
{
    "error_code": 400,
    "message": "This email is already registered."
}
```

### Error al crear usuario

Regresa un `InternalServerError`

```json
{
    "error_code": 500,
    "message": "Error creating user"
}
```
