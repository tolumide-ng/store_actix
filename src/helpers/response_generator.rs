

use std::collections::HashMap;
use actix_web::{HttpRequest, Responder};
use serde::{Serialize, Deserialize};

// #[derive(Serialize, Queryable, Deserialize, Debug)]
pub struct ResponseBody {
    pub status_code: u32,
    pub message: Option<String>,
    pub data: Option<Box<dyn ResponseTrait>>,
}



pub trait ResponseTrait {
    fn generate_body(&self) {}
}



impl ResponseBody {
    pub fn new(status_code: u32, message: String, data: Option<Box<dyn ResponseTrait>>) -> Self {
        ResponseBody {status_code, message: Some(message), data}
    }
}


#[derive(Serialize)]
pub struct SuccessResponse<T: ResponseTrait> {
    data: T,
    status_code: u32,
}




#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    pub message: &'a str,
    pub status: u32,
}


impl<'a> ErrorResponse<'a> {
    pub fn new (status: u32, message: &'a str) -> Self {
        ErrorResponse {
            status, message
        }
    }
}
