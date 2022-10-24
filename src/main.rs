use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

// structs etc
//
// this is what i used for the base https://medium.com/swlh/build-your-first-rest-api-using-rust-language-and-actix-framework-8f827175b30f

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

#[get("/init/{name}")]
async fn git_init(repo: web::Path<String>) -> impl Responder{
    //println!("{}", repo.to_string());
    let repo_name = String::from(repo.as_str());
    println!("git init for: {}", repo_name);
    HttpResponse::Ok().body("repo init")
}

#[get("/verify/{name}")]
//async fn git_verify(repo: web::Path<String>) -> impl Responder {
async fn git_verify(repo: web::Path<String>) -> HttpResponse {
    let mut repo_name = String::from(repo.as_str());
    println!("this will eventually check for git repo: {}", repo_name);
    //let output: String = ("{} doesnt exist this is scaffolding code", repo_name.as_str());
    repo_name.push_str(" doest exist this is scaffolding code");
    //HttpResponse::Ok().body(output.to_string())
    HttpResponse::Ok().body(repo_name)
}

//end git commands

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
                        .service(git_verify)
                    )
            )
    })
    .bind("0.0.0.0:7000")?
    .run()
    .await
}
