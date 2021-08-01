use crate::controllers::prelude::*;

#[put("/forgot_password")]
async fn controller(user: Json<UserEmail>, state: Data<AppState>) -> impl Responder {
    let user = user.into_inner();
    let db = state.as_ref().db.clone();

    let UserEmail { email } = user;

    let verify_email = db.send(VerifyEmail {email}).await;

    if verify_email.is_ok() {
        // TODO
        // - send a real email to the user
    }

    return HttpResponse::Ok().json(UserMessage::new("A reset token as been sent to your email".to_string()));


}