use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;


pub struct Mailer {
    pub body: String,
    pub name: String,
    pub email: String,
    pub sender: String,
    pub subject: String,
    pub recipient: String,
    pub store_name: String,
}



#[derive(Debug, PartialEq)]
pub enum EmailTypes {
    Welcome,
    ForgotPassword,
}

pub fn email_body(email_type: EmailTypes, token: String) {
    if email_type == EmailTypes::Welcome {
        format!("Thank you for signing up, Please verify your email by clicking on this token {}", token);
    }
}


impl Mailer {
    pub fn new(name: String, email: String, body: String, subject: String,) -> Self {
        let store_name = env::var("STORE_NAME").expect("STORE_NAME is required");

        let recipient = format!("{} <{}>", name, email);

        let sender = format!("{} <{}>", store_name, store_name);

        Mailer {name, email, body, subject, recipient, store_name, sender}
    }


    pub fn send_email(self) {

    
        let email = Message::builder()
            .from(self.sender.parse().unwrap())
            .reply_to(self.recipient.parse().unwrap())
            .to(self.recipient.parse().unwrap())
            .subject(self.subject.as_str())
            .body(self.body).unwrap();



        let credentials = match (env::var("SMTP_USERNAME"), env::var("SMTP_PASSWORD")) {
            (Ok(username), Ok(password)) => (username, password),
            _ => {panic!("Environment variables SMTP_USERNAME and SMTP_PASSWORD are required")}
        };

        let creds = Credentials::new(credentials.0, credentials.1);


        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();



        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }

}

