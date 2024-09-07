# ElectroShop

- React
- Typescript
- TailwindCSS
- AdonisJS API
- SQLite

## Tutorial React Router Dom

- [YT](https://youtu.be/-Xjy86xCNZg?si=WJoNdXHV8sZoOOam)

## Comandos

```sh
cargo install sqlx-cli --no-default-features --features sqlite
sqlx migrate add create_<ENTITIE>s_table
# Para esto ya tiene que estar creada la base de datos aunque sea vacia.
sqlx migrate run
cargo watch -x run
```

## TODO

- [ ] Verificar contraseña en endpoint de creación de usuarios (8 caracteres minimo, etc...).
- [ ] Agregar hasheo de contraseñas (se guardan hasheadas y al momento de que el usuario hace login se hashea y se compara con la db).
- [ ] Agregar la protección de rutas con JWT.
- [ ] Agregar permisos dependiendo el ROL.

