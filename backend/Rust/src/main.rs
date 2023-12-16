use actix_web::{HttpServer,App};

pub mod database;
pub mod middlewares;
pub mod models;
pub mod services;

pub use database::database::connect_db;
pub use services::{
    mensagem::get_mensagens_by_conversa,
    conversa::get_conversas_by_usuario,
};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let client = connect_db().await.unwrap();
    HttpServer::new(|| {
        App::new()
        // conversas
            .service(get_conversas_by_usuario)
        // mensagens
            .service(get_mensagens_by_conversa)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
