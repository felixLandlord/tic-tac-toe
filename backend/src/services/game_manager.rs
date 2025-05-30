use shared::{GameInfo, GameState};
use std::collections::HashMap;
use uuid::Uuid;

pub struct GameManager {
    games: HashMap<Uuid, GameState>,
    game_names: HashMap<String, Uuid>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
            game_names: HashMap::new(),
        }
    }

    pub fn create_game(
        &mut self,
        game_name: String,
        player_name: String,
    ) -> Result<(Uuid, Uuid), String> {
        if self.game_names.contains_key(&game_name) {
            return Err("Game name already exists".to_string());
        }

        let mut game = GameState::new(game_name.clone());
        let player = game.add_player(player_name)?;
        let game_id = game.id;
        let player_id = player.id;

        self.games.insert(game_id, game);
        self.game_names.insert(game_name, game_id);

        Ok((game_id, player_id))
    }

    pub fn join_game(
        &mut self,
        game_name: String,
        player_name: String,
    ) -> Result<(Uuid, Uuid), String> {
        let game_id = self
            .game_names
            .get(&game_name)
            .ok_or("Game not found")?
            .clone();

        let game = self.games.get_mut(&game_id).ok_or("Game not found")?;

        let player = game.add_player(player_name)?;
        Ok((game_id, player.id))
    }

    pub fn make_move(
        &mut self,
        game_id: Uuid,
        row: usize,
        col: usize,
        player_id: Uuid,
    ) -> Result<(), String> {
        let game = self.games.get_mut(&game_id).ok_or("Game not found")?;

        game.make_move(row, col, player_id)
    }

    pub fn get_game(&self, game_id: Uuid) -> Option<&GameState> {
        self.games.get(&game_id)
    }

    pub fn get_available_games(&self) -> Vec<GameInfo> {
        self.games
            .values()
            .filter(|game| !game.is_full)
            .map(|game| GameInfo {
                id: game.id,
                name: game.name.clone(),
                player_count: game.players.len(),
                is_full: game.is_full,
            })
            .collect()
    }
}
