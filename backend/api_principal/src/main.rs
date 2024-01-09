use actix_web::{App, HttpServer,HttpResponse,Responder,get};
use serde_json::json;



#[get("/")]
async fn home() -> impl Responder {
    let data = json!({
        "message": "hello, Actix"
    });
    return HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(home)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}