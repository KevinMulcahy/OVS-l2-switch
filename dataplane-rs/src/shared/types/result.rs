// src/shared/types/result.rs
use crate::shared::types::error::AppError;

pub type AppResult<T> = Result<T, AppError>;
