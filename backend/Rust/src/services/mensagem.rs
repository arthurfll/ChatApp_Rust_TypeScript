use actix_web::{
    get,post,delete,
    HttpResponse, Responder
};


#[get("/conversa")]
pub async fn get_mensagens_by_conversa() -> impl Responder {
    HttpResponse::Ok().body("mensagens dessa conversa")
}

// criar mensagem
// apagar mensagem  
