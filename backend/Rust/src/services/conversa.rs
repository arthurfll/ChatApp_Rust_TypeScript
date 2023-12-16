use actix_web::{
    get,post,delete,
    HttpResponse, Responder
};


#[get("/usuario")]
pub async fn get_conversas_by_usuario() -> impl Responder {
    HttpResponse::Ok().body("conversas desse usuario")
}

// criar conversa
// apagar conversa
