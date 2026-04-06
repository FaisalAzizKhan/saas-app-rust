use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub body: T,
}

#[derive(Serialize)]
pub struct ApiErrorResponse<String> {
    pub message: String,
}

 