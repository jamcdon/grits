use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/ping")]
async fn ping() -> impl Responder{
    println!("You have been pinged");
    HttpResponse::Ok().body("pong")
}
#[get("/")]
async fn index() -> impl Responder{
    HttpResponse::Ok().body("Grist Index")
}

// git commands

#[get("/init")]
async fn git_init() -> impl Responder{
    println!("git init");
    HttpResponse::Ok().body("repo init")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(ping)
                    .service(index)
                    .service(
                        web::scope("/git")
                        .service(git_init)
                    )
            )
    })
    .bind("0.0.0.0:7000")?
    .run()
    .await
}
