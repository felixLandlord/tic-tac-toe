use crate::GameState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    CreateGame {
        game_name: String,
        player_name: String,
    },
    JoinGame {
        game_name: String,
        player_name: String,
    },
    MakeMove {
        row: usize,
        col: usize,
    },
    GetAvailableGames,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    GameCreated { game_id: Uuid, player_id: Uuid },
    GameJoined { game_id: Uuid, player_id: Uuid },
    GameState(GameState),
    AvailableGames(Vec<GameInfo>),
    Error(String),
    PlayerConnected { player_name: String },
    PlayerDisconnected { player_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub id: Uuid,
    pub name: String,
    pub player_count: usize,
    pub is_full: bool,
}
