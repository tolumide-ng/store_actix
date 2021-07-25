use crate::errors::response_error;
use std::result;

pub type Result<T, E = response_error::Error> = result::Result<T, E>;