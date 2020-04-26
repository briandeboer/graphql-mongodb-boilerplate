use actix_web::{HttpResponse, Responder};

pub async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub async fn pong() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

pub async fn readiness() -> impl Responder {
    let process = std::process::Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output();

    match process {
        Ok(_) => HttpResponse::Accepted(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
