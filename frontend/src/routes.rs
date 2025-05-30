use crate::components::{game_board::GameBoard, home::Home, lobby::Lobby};

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/lobby")]
    Lobby {},
    #[route("/gameboard")]
    GameBoard {},
}
