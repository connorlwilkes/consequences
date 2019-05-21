use actix_web::middleware::identity::Identity;
use actix_web::HttpResponse;

pub fn check(id: Identity) -> HttpResponse {
    if let Some(user) = id.identity() {
        HttpResponse::Ok().body(user)
    } else {
        HttpResponse::Ok().finish()
    }
}