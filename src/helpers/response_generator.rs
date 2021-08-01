
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
pub struct SuccessResponse {
    // data: Box<dyn ResponseTrait>,
    // IMplement serializer, and deserializer for this 
    // check out serde's documentation on how to implement serializer and deserializer for structs
    status_code: u32,
}




#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: u32,
}


impl ErrorResponse {
    pub fn new (status: u32, message: String) -> Self {
        ErrorResponse {
            status, message
        }
    }

    pub fn server_error() -> Self {
        ErrorResponse::new(500, "Internal Server Error, Please try again later".to_owned())
    }


    pub fn auth_error<'a>(error_text: Option<&'a str>) -> Self {
        let mut msg = String::from("Authentication Error");

        match error_text {
            Some(text) => {
                msg.push_str(text);
            }
            None => {}
        }

        return ErrorResponse::new(401,  msg);
    }
}


