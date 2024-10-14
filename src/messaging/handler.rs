use crate::messaging::message_types::MessageType;

pub struct MessageHandler;

impl MessageHandler {
    pub fn handle_message(message: MessageType) {
        match message {
            MessageType::PlayerMovement { player_id, position } => {
                println!("玩家 {} 移動到位置 {:?}", player_id, position);
                // 在這裡添加玩家位置更新的邏輯
            }
            MessageType::ChatMessage { player_id, content } => {
                println!("玩家 {} 發送聊天消息: {}", player_id, content);
                // 在這裡處理聊天消息
            }
            MessageType::SyncGameState => {
                println!("同步遊戲狀態");
                // 處理遊戲狀態同步
            }
            MessageType::PlayerConnected { player_id } => {
                println!("玩家 {} 已連接", player_id);
                // 處理玩家連接
            }
            MessageType::PlayerDisconnected { player_id } => {
                println!("玩家 {} 已斷開", player_id);
                // 處理玩家斷開
            }
        }
    }
}
