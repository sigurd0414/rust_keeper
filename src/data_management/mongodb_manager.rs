use mongodb::{Client, options::ClientOptions, bson::doc, Database};
use std::sync::Arc;

pub struct MongoDBManager {
    pub database: Arc<Database>,
}

impl MongoDBManager {
    pub async fn new(uri: &str, db_name: &str) -> Self {
        let mut client_options = ClientOptions::parse(uri).await.unwrap();
        client_options.app_name = Some("GameApp".to_string());
        let client = Client::with_options(client_options).unwrap();
        let database = client.database(db_name);

        Self {
            database: Arc::new(database),
        }
    }

    pub async fn save_player_inventory(&self, player_id: &str, items: Vec<String>) {
        let collection = self.database.collection("player_inventories");
        let filter = doc! { "player_id": player_id };
        let update = doc! { "$set": { "items": items } };
        let _ = collection.update_one(filter, update, None).await.unwrap();
    }

    pub async fn get_player_inventory(&self, player_id: &str) -> Option<Vec<String>> {
        let collection = self.database.collection("player_inventories");
        let filter = doc! { "player_id": player_id };
        if let Some(result) = collection.find_one(filter, None).await.unwrap() {
            let items: Vec<String> = result.get_array("items")
                .unwrap()
                .iter()
                .map(|b| b.as_str().unwrap().to_string())
                .collect();
            return Some(items);
        }
        None
    }
}
