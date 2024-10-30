
pub enum ServiceError{
    ResourceExists,
    ResourceNotExists(String),
    InternalServerError
}