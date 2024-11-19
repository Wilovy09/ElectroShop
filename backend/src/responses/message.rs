use serde::Serialize;

#[derive(Serialize)]
pub struct Messages {
    pub message: String,
}

// https://developer.mozilla.org/es/docs/Web/HTTP/Status
#[derive(Serialize)]
pub struct ErrorMessages {
    pub message: String,
}
