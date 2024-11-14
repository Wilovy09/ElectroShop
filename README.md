# ElectroShop

## Backend

> [!NOTE] > [Notas sobre actix](https://github.com/Wilovy09/Actix-Desde-0)

### Instalar Cargo watch

```sh
cargo install cargo-watch
```

```sh
cargo watch -x run
```

### Instalar SQLX

```sh
cargo install sqlx-cli --no-default-features --features sqlite
```

#### Comandos

##### Crear db

```sh
# A veces hay que crearla manualmente en base a la ruta que dimos en el `.env`
sqlx database create
```

##### Borrar db

```sh
sqlx database drop
```

##### Crear migraciones

```sh
sqlx migrate add create_TABLE-NAME_table
```

##### Correr migraciones

```sh
sqlx migrate run
```

### Querys

#### Crear roles

```sql
INSERT INTO Role (name_role) VALUES ('Administrador'), ('Cliente');
```

## Frontend
