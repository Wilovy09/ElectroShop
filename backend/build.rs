// ./build.rs << debes poner eso en ese archivo
use std::env::set_var;

fn main() {
    #[cfg(target_os = "windows")] {
        set_var("DATABASE_URL", "sqlite://$PWD\\temp\\sqlx.db")
    }

    #[cfg(target_os = "linux")] {
        set_var("DATABASE_URL", "sqlite://$PWD/temp/sqlx.db")
    }

    .expect("No se pudo colocar la variable de entorno");
}