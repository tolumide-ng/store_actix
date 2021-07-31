use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserData {
    #[validate(length(min=1, message="First name must have atleast one character"))]
    pub first_name: String,
    #[validate(length(min=1, message="Last name must have atleast one character"))]
    pub last_name: String,
    #[validate(email(message="Please provide a valid email"))]
    pub email: String,
    pub password: String
}

fn validate_password(password: String) {}


#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserLogin {
    #[validate(email(message="Please provide a valid email"))]
    pub email: String,
    pub password: String,
}