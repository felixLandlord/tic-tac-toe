use crate::services::game_manager::GameManager;
use actix::{Actor, ActorContext, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use shared::{ClientMessage, ServerMessage};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    game_manager: web::Data<Arc<Mutex<GameManager>>>,
) -> Result<HttpResponse> {
    let websocket = GameWebSocket::new(game_manager.get_ref().clone());
    ws::start(websocket, &req, stream)
}

pub struct GameWebSocket {
    game_manager: Arc<Mutex<GameManager>>,
    game_id: Option<Uuid>,
    player_id: Option<Uuid>,
}

impl GameWebSocket {
    pub fn new(game_manager: Arc<Mutex<GameManager>>) -> Self {
        Self {
            game_manager,
            game_id: None,
            player_id: None,
        }
    }

    fn handle_client_message(&mut self, msg: ClientMessage, ctx: &mut ws::WebsocketContext<Self>) {
        match msg {
            ClientMessage::CreateGame {
                game_name,
                player_name,
            } => {
                let mut manager = self.game_manager.try_lock().unwrap();
                match manager.create_game(game_name, player_name) {
                    Ok((game_id, player_id)) => {
                        self.game_id = Some(game_id);
                        self.player_id = Some(player_id);

                        let response = ServerMessage::GameCreated { game_id, player_id };
                        ctx.text(serde_json::to_string(&response).unwrap());

                        if let Some(game) = manager.get_game(game_id) {
                            let game_state = ServerMessage::GameState(game.clone());
                            ctx.text(serde_json::to_string(&game_state).unwrap());
                        }
                    }
                    Err(e) => {
                        let response = ServerMessage::Error(e);
                        ctx.text(serde_json::to_string(&response).unwrap());
                    }
                }
            }
            ClientMessage::JoinGame {
                game_name,
                player_name,
            } => {
                let mut manager = self.game_manager.try_lock().unwrap();
                match manager.join_game(game_name, player_name) {
                    Ok((game_id, player_id)) => {
                        self.game_id = Some(game_id);
                        self.player_id = Some(player_id);

                        let response = ServerMessage::GameJoined { game_id, player_id };
                        ctx.text(serde_json::to_string(&response).unwrap());

                        if let Some(game) = manager.get_game(game_id) {
                            let game_state = ServerMessage::GameState(game.clone());
                            ctx.text(serde_json::to_string(&game_state).unwrap());
                        }
                    }
                    Err(e) => {
                        let response = ServerMessage::Error(e);
                        ctx.text(serde_json::to_string(&response).unwrap());
                    }
                }
            }
            ClientMessage::MakeMove { row, col } => {
                if let (Some(game_id), Some(player_id)) = (self.game_id, self.player_id) {
                    let mut manager = self.game_manager.try_lock().unwrap();
                    match manager.make_move(game_id, row, col, player_id) {
                        Ok(()) => {
                            if let Some(game) = manager.get_game(game_id) {
                                let game_state = ServerMessage::GameState(game.clone());
                                ctx.text(serde_json::to_string(&game_state).unwrap());
                            }
                        }
                        Err(e) => {
                            let response = ServerMessage::Error(e);
                            ctx.text(serde_json::to_string(&response).unwrap());
                        }
                    }
                }
            }
            ClientMessage::GetAvailableGames => {
                let manager = self.game_manager.try_lock().unwrap();
                let games = manager.get_available_games();
                let response = ServerMessage::AvailableGames(games);
                ctx.text(serde_json::to_string(&response).unwrap());
            }
        }
    }
}

impl Actor for GameWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    self.handle_client_message(client_msg, ctx);
                }
            }
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => {}
        }
    }
}
