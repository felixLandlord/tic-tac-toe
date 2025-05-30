// use crate::routes::Route;
// use crate::services::websocket::WebSocketService;
// use dioxus::prelude::*;
// use shared::{CellState, ClientMessage, GameState, ServerMessage};
// use std::cell::RefCell;
// use std::sync::Arc;

// #[component]
// pub fn GameBoard() -> Element {
//     let navigator = use_navigator();
//     let ws_service = use_context::<Option<Arc<RefCell<Option<WebSocketService>>>>>().unwrap();

//     let game_state = use_signal(|| None::<GameState>);
//     let player_id = use_signal(|| None::<uuid::Uuid>);
//     let error_message = use_signal(|| None::<String>);

//     // Handle WebSocket messages
//     use_effect({
//         let ws_service = ws_service.clone();
//         let game_state = game_state.clone();
//         let error_message = error_message.clone();
//         move || {
//             let ws_service = ws_service.clone();
//             let mut game_state = game_state.clone();
//             let mut error_message = error_message.clone();
//             spawn(async move {
//                 if let Some(service) = ws_service.borrow().as_ref() {
//                     while let Some(message) = service.receive_message().await {
//                         match message {
//                             ServerMessage::GameState(state) => {
//                                 game_state.set(Some(state));
//                             }
//                             ServerMessage::Error(err) => {
//                                 error_message.set(Some(err));
//                             }
//                             _ => {}
//                         }
//                     }
//                 }
//             });
//         }
//     });

//     let handle_cell_click = move |row: usize, col: usize| {
//         let ws_service = ws_service.clone();
//         move |_| {
//             if let Some(service) = ws_service.borrow().as_ref() {
//                 let msg = ClientMessage::MakeMove { row, col };
//                 service.send_message(msg);
//             }
//         }
//     };

//     let handle_back_to_lobby = move |_| {
//         navigator.push(Route::Lobby {});
//     };

//     rsx! {
//         div {
//             class: "game-container",
//             div {
//                 class: "game-content",
//                 div {
//                     class: "game-header",
//                     h1 {
//                         class: "game-title",
//                         "Tic Tac Toe"
//                     }
//                     button {
//                         class: "back-button",
//                         onclick: handle_back_to_lobby,
//                         "Back to Lobby"
//                     }
//                 }

//                 if let Some(error) = error_message.read().as_ref() {
//                     div {
//                         class: "error-message",
//                         "{error}"
//                     }
//                 }

//                 if let Some(game) = game_state.read().as_ref() {
//                     div {
//                         class: "game-board-card",

//                         // Game info
//                         div {
//                             class: "game-info-section",
//                             h2 {
//                                 class: "game-name",
//                                 "Game: {game.name}"
//                             }
//                             div {
//                                 class: "players-info",
//                                 for player in &game.players {
//                                     div {
//                                         class: "player-info",
//                                         span {
//                                             class: if player.symbol == game.current_player && !game.game_over {
//                                                 "player-name current-player"
//                                             } else {
//                                                 "player-name"
//                                             },
//                                             "{player.name} ({player.symbol:?})"
//                                         }
//                                     }
//                                 }
//                             }
//                         }

//                         // Game status
//                         div {
//                             class: "game-status",
//                             if game.game_over {
//                                 if let Some(winner) = &game.winner {
//                                     p {
//                                         class: "winner-message",
//                                         "Winner: {winner:?}!"
//                                     }
//                                 } else {
//                                     p {
//                                         class: "draw-message",
//                                         "It's a Draw!"
//                                     }
//                                 }
//                             } else {
//                                 p {
//                                     class: "turn-message",
//                                     "Current Turn: {game.current_player:?}"
//                                 }
//                             }
//                         }

//                         // Game board
//                         div {
//                             class: "board-container",
//                             for (row_idx, row) in game.board.iter().enumerate() {
//                                 for (col_idx, cell) in row.iter().enumerate() {
//                                     button {
//                                         key: "{row_idx}-{col_idx}",
//                                         class: if *cell != CellState::Empty || game.game_over {
//                                             "board-cell board-cell-disabled"
//                                         } else {
//                                             "board-cell"
//                                         },
//                                         disabled: *cell != CellState::Empty || game.game_over,
//                                         onclick: move |_| handle_cell_click(row_idx, col_idx),
//                                         match cell {
//                                             CellState::X => "X",
//                                             CellState::O => "O",
//                                             CellState::Empty => "",
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     div {
//                         class: "loading-card",
//                         p {
//                             class: "loading-message",
//                             "Loading game..."
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

use crate::routes::Route;
use crate::services::websocket::WebSocketService;
use dioxus::prelude::*;
use shared::{CellState, ClientMessage, GameState, ServerMessage};
use std::cell::RefCell;
use std::sync::Arc;

#[component]
pub fn GameBoard() -> Element {
    let navigator = use_navigator();
    let ws_service = use_context::<Option<Arc<RefCell<Option<WebSocketService>>>>>().unwrap();

    let game_state = use_signal(|| None::<GameState>);
    // let player_id = use_signal(|| None::<uuid::Uuid>);
    let error_message = use_signal(|| None::<String>);

    // Handle WebSocket messages
    use_effect({
        let ws_service = ws_service.clone();
        let game_state = game_state.clone();
        let error_message = error_message.clone();
        move || {
            let ws_service = ws_service.clone();
            let mut game_state = game_state.clone();
            let mut error_message = error_message.clone();
            spawn(async move {
                if let Some(service) = ws_service.borrow().as_ref() {
                    while let Some(message) = service.receive_message() {
                        match message {
                            ServerMessage::GameState(state) => {
                                game_state.set(Some(state));
                            }
                            ServerMessage::Error(err) => {
                                error_message.set(Some(err));
                            }
                            _ => {}
                        }
                    }
                }
            });
        }
    });

    // let handle_cell_click = move |row: usize, col: usize| {
    //     let ws_service = ws_service.clone();
    //     move |_: Event<MouseData>| {
    //         if let Some(service) = ws_service.borrow().as_ref() {
    //             let msg = ClientMessage::MakeMove { row, col };
    //             service.send_message(msg);
    //         }
    //     }
    // };

    let handle_back_to_lobby = move |_| {
        navigator.push(Route::Lobby {});
    };

    rsx! {
        div {
            class: "game-container",
            div {
                class: "game-content",
                div {
                    class: "game-header",
                    h1 {
                        class: "game-title",
                        "Tic Tac Toe"
                    }
                    button {
                        class: "back-button",
                        onclick: handle_back_to_lobby,
                        "Back to Lobby"
                    }
                }

                if let Some(error) = error_message.read().as_ref() {
                    div {
                        class: "error-message",
                        "{error}"
                    }
                }

                if let Some(game) = game_state.read().as_ref() {
                    div {
                        class: "game-board-card",

                        // Game info
                        div {
                            class: "game-info-section",
                            h2 {
                                class: "game-name",
                                "Game: {game.name}"
                            }
                            div {
                                class: "players-info",
                                for player in &game.players {
                                    div {
                                        class: "player-info",
                                        span {
                                            class: if player.symbol == game.current_player && !game.game_over {
                                                "player-name current-player"
                                            } else {
                                                "player-name"
                                            },
                                            "{player.name} ({player.symbol:?})"
                                        }
                                    }
                                }
                            }
                        }

                        // Game status
                        div {
                            class: "game-status",
                            if game.game_over {
                                if let Some(winner) = &game.winner {
                                    p {
                                        class: "winner-message",
                                        "Winner: {winner:?}!"
                                    }
                                } else {
                                    p {
                                        class: "draw-message",
                                        "It's a Draw!"
                                    }
                                }
                            } else {
                                p {
                                    class: "turn-message",
                                    "Current Turn: {game.current_player:?}"
                                }
                            }
                        }

                        // Game board
                        {
                            let ws_service_for_board = ws_service.clone();
                            rsx! {
                                div {
                                    class: "board-container",
                                    for (row_idx, row) in game.board.iter().enumerate() {
                                        for (col_idx, cell) in row.iter().enumerate() {
                                            button {
                                                key: "{row_idx}-{col_idx}",
                                                class: if *cell != CellState::Empty || game.game_over {
                                                    "board-cell board-cell-disabled"
                                                } else {
                                                    "board-cell"
                                                },
                                                disabled: *cell != CellState::Empty || game.game_over,
                                                onclick: {
                                                    let ws_service = ws_service_for_board.clone();
                                                    move |_| {
                                                        if let Some(service) = ws_service.borrow().as_ref() {
                                                            let msg = ClientMessage::MakeMove { row: row_idx, col: col_idx };
                                                            service.send_message(msg);
                                                        }
                                                    }
                                                },
                                                match cell {
                                                    CellState::X => "X",
                                                    CellState::O => "O",
                                                    CellState::Empty => "",
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "loading-card",
                        p {
                            class: "loading-message",
                            "Loading game..."
                        }
                    }
                }
            }
        }
    }
}
