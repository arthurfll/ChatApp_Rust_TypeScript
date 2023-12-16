use serde::{Serialize,Deserialize};


#[derive(Deserialize,Serialize)]
pub struct Conversa {
    pub id : String,
    pub titulo: String,
    pub id_remetente: String,
    pub id_destinatario : String,
}