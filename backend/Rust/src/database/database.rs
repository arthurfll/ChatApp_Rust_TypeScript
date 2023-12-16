use mongodb::{Client, options::ClientOptions};

pub async fn connect_db() -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse("mongodb+srv://root:root@cluster0.3j27vtb.mongodb.net/?retryWrites=true&w=majority")
        .await?;
    client_options
        .app_name = Some("My App".to_string());
    let client = Client::with_options(client_options)?;
    println!("conectado ao banco de dados");
    Ok(client)
}