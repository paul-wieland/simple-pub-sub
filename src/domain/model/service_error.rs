use std::error::Error;

pub enum ServiceError{
    ResourceExists(String),
    InternalServerError
}