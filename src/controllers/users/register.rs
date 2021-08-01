use crate::controllers::prelude::*;


// TODO
//  - Send a real email to the user on signup

#[post("/register")]
async fn controller(user: Json<UserData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    let UserData {first_name, last_name, email, password} = user;

    match db.send(VerifyEmail {email: email.clone()}).await {
        Ok(user) => {
            match user {
                Ok(_) => {
                    let msg = ErrorResponse::new(409, "Email already exists".to_owned());
                    return HttpResponse::Conflict().json(msg)
                }
                _ => {}
            }
        }
        Err(_err) => {
            return HttpResponse::InternalServerError().json(ErrorResponse::server_error())
        }
    }


    let hash = LocalHasher::generate_hash(&password);
    
    match db.send(CreateUser {first_name, last_name, email,  hash}).await {
        Ok(Ok(message)) => HttpResponse::Created().json(message),
        _ => return HttpResponse::InternalServerError().json(ErrorResponse::server_error())
    }
}
