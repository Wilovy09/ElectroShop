use serde::Serialize;

#[derive(Serialize)]
pub struct Messages {
    pub message: String,
}
