use std::collections::HashMap;
use actix_web::{HttpRequest, Responder};
use serde::{Serialize, Deserialize};

// #[derive(Serialize)]
pub struct ResponseBody {
    pub status_code: u32,
    pub message: Option<String>,
    pub data: Option<Box<dyn ResponseTrait>>,
}

pub trait ResponseTrait {
    // fn generate_body(&self) {}
}

impl ResponseBody {
    pub fn new(status_code: u32, message: String, data: Box<dyn ResponseTrait>) -> Self {
        ResponseBody {status_code, message: Some(message), data: Some(data)}
    }
}


#[derive(Serialize)]
pub struct SuccessResponse<T: ResponseTrait> {
    data: T,
    status_code: u32,
}

// impl<T> SuccessResponse<T> where T: ResponseTrait {
//     pub fn new(status_code: u32, data: T) -> Self {
//         SuccessResponse {status_code, data}
//     }
// }

