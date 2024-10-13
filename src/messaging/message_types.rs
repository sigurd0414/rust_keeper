use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    PlayerMovement { player_id: String, position: (i32, i32) },
    ChatMessage { player_id: String, content: String },
    SyncGameState,
    PlayerConnected { player_id: String },
    PlayerDisconnected { player_id: String },
}
