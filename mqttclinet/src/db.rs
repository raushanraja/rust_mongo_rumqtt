use mongodb::{options::ClientOptions, Client};
use std::error::Error;

async fn get_client() -> Result<Client, Box<dyn Error>> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.credential = Some(
        mongodb::options::Credential::builder()
            .username("root".to_string())
            .password("password".to_string())
            .build(),
    );
    let client = Client::with_options(client_options)?;
    Ok(client)
}

pub async fn list_collection_names() -> Result<Vec<String>, Box<dyn Error>> {
    let client = get_client().await?;
    let db = client.database("admin");
    let collections = db.list_collection_names(None).await?;
    println!("Collections = {:?}", collections);
    Ok(collections)
}
