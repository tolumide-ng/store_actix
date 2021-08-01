use crate::controllers::prelude::*;


#[post("/login")]
async fn controller(user: Json<UserLogin>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    let UserLogin {email, password} = user;

    //  PLEASE REFACTOR THIS MATCH HELL
    match db.send(VerifyEmail {email: email}).await {
        Ok(user) => {
            match user {
                Ok(the_user) => {

                    let User {first_name, last_name, email, hash, .. } = the_user;


                    if LocalHasher::verify_hash(hash, password) {
                        let user_token = token::UserToken::generate_token(email.to_owned(), 40, "store_recs".to_string());

                        match user_token {
                            Ok(token) => {
                                return HttpResponse::Ok().json(UserAuth::new(first_name, last_name, email, token))
                            }
                            Err(_e) => {}
                        }

                    } else {
                        return HttpResponse::Unauthorized().json(ErrorResponse::auth_error(Some(": Email or Password is incorrect")))
                    }
                }
                _ => {
                    return HttpResponse::Unauthorized().json(ErrorResponse::auth_error(Some(": Email or Password is incorrect")))
                }
            }
        }
        _ => {
            return HttpResponse::InternalServerError().json(ErrorResponse::server_error())
        }
    }



    let msg = UserMessage {message: "This is a placeholder text".to_string()};
    HttpResponse::Ok().json(msg)
}