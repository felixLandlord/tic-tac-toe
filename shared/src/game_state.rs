use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum CellState {
    Empty,
    X,
    O,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub id: Uuid,
    pub name: String,
    pub board: [[CellState; 3]; 3],
    pub current_player: CellState,
    pub players: Vec<Player>,
    pub winner: Option<CellState>,
    pub is_full: bool,
    pub game_over: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub symbol: CellState,
}

impl GameState {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            board: [[CellState::Empty; 3]; 3],
            current_player: CellState::X,
            players: Vec::new(),
            winner: None,
            is_full: false,
            game_over: false,
        }
    }

    pub fn add_player(&mut self, player_name: String) -> Result<Player, String> {
        if self.players.len() >= 2 {
            return Err("Game is full".to_string());
        }

        let symbol = if self.players.is_empty() {
            CellState::X
        } else {
            CellState::O
        };

        let player = Player {
            id: Uuid::new_v4(),
            name: player_name,
            symbol,
        };

        self.players.push(player.clone());
        self.is_full = self.players.len() == 2;

        Ok(player)
    }

    pub fn make_move(&mut self, row: usize, col: usize, player_id: Uuid) -> Result<(), String> {
        if self.game_over {
            return Err("Game is over".to_string());
        }

        if self.board[row][col] != CellState::Empty {
            return Err("Cell is already occupied".to_string());
        }

        let player = self
            .players
            .iter()
            .find(|p| p.id == player_id)
            .ok_or("Player not found")?;

        if player.symbol != self.current_player {
            return Err("Not your turn".to_string());
        }

        self.board[row][col] = self.current_player.clone();

        if self.check_winner() {
            self.winner = Some(self.current_player.clone());
            self.game_over = true;
        } else if self.is_board_full() {
            self.game_over = true;
        } else {
            self.current_player = match self.current_player {
                CellState::X => CellState::O,
                CellState::O => CellState::X,
                CellState::Empty => CellState::X,
            };
        }

        Ok(())
    }

    fn check_winner(&self) -> bool {
        // Check rows, columns, and diagonals
        for i in 0..3 {
            if self.board[i][0] != CellState::Empty
                && self.board[i][0] == self.board[i][1]
                && self.board[i][1] == self.board[i][2]
            {
                return true;
            }
            if self.board[0][i] != CellState::Empty
                && self.board[0][i] == self.board[1][i]
                && self.board[1][i] == self.board[2][i]
            {
                return true;
            }
        }

        // Diagonals
        if self.board[0][0] != CellState::Empty
            && self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
        {
            return true;
        }

        if self.board[0][2] != CellState::Empty
            && self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
        {
            return true;
        }

        false
    }

    fn is_board_full(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|cell| *cell != CellState::Empty))
    }
}
