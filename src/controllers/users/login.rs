use crate::controllers::prelude::*;


// TODOS:
//  - Implement with redis to save all issued tokens
//  - Log all failed and successful login requests
#[post("/login")]
async fn controller(user: Json<UserLogin>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    
    let UserLogin {email, password} = user;
    
    let verify_email = db.send(VerifyEmail {email}).await;
    
    
    if verify_email.is_err() {
        // executes if there was a server error in checking the request
        return HttpResponse::InternalServerError().json(ErrorResponse::server_error())
    }
    
    let user = verify_email.unwrap();

    let bad_auth = HttpResponse::Unauthorized().json(ErrorResponse::auth_error(Some(": Email or Password is incorrect")));

    if user.is_err() {
        return bad_auth
    }

    let the_user = user.unwrap();

    let User {first_name, last_name, email, hash, .. } = the_user;

    let correct_password = LocalHasher::verify_hash(hash, password);

    if !correct_password {
        return bad_auth
    }


    // generate token for the user
    let token = token::UserToken::generate_token(email.to_owned(), 40, "store_recs".to_string());

    if token.is_err() {
        return bad_auth
    }

    let the_token = token.unwrap();

    return HttpResponse::Ok().json(UserAuth::new(first_name, last_name, email, the_token))

}