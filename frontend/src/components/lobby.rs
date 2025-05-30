use crate::routes::Route;
use crate::services::websocket::WebSocketService;
use dioxus::prelude::*;
use shared::{ClientMessage, GameInfo, ServerMessage};
use std::cell::RefCell;
use std::sync::Arc;

#[component]
pub fn Lobby() -> Element {
    let navigator = use_navigator();
    // let ws_service = use_context::<Option<Arc<RefCell<Option<WebSocketService>>>>>().unwrap();
    let ws_service_option: Option<Arc<RefCell<Option<WebSocketService>>>> = use_context();
    let ws_service = match ws_service_option {
        Some(context) => context,
        None => {
            // Panic with a message indicating the context was missing where expected.
            panic!("Failed to get WebSocket context in Lobby. Ensure App component provides a context of type Arc<RefCell<Option<WebSocketService>>>.");
        }
    };
    let mut player_name = use_signal(|| String::new());
    let mut game_name = use_signal(|| String::new());
    let available_games = use_signal(|| Vec::<GameInfo>::new());
    let mut selected_game = use_signal(|| None::<String>);
    let error_message = use_signal(|| None::<String>);
    let mut is_creating = use_signal(|| false);

    // Initialize WebSocket if not already done
    use_effect({
        let ws_service = ws_service.clone();
        move || {
            let ws_service = ws_service.clone();
            spawn(async move {
                let mut service = ws_service.borrow_mut();
                if service.is_none() {
                    let new_service = WebSocketService::new("ws://127.0.0.1:8080/api/ws").await;
                    *service = Some(new_service.expect("Failed to create WebSocketService"));
                }
            });
        }
    });

    // Handle WebSocket messages
    use_effect({
        let ws_service = ws_service.clone();
        let available_games = available_games.clone();
        let navigator = navigator.clone();
        let error_message = error_message.clone();
        move || {
            let ws_service = ws_service.clone();
            let mut available_games = available_games.clone();
            let navigator = navigator.clone();
            let mut error_message = error_message.clone();
            spawn(async move {
                if let Some(service) = ws_service.borrow().as_ref() {
                    while let Some(message) = service.receive_message() {
                        match message {
                            ServerMessage::GameCreated { .. }
                            | ServerMessage::GameJoined { .. } => {
                                navigator.push(Route::GameBoard {});
                            }
                            ServerMessage::AvailableGames(games) => {
                                available_games.set(games);
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

    let handle_create_game = {
        let ws_service = ws_service.clone();
        move |_| {
            to_owned![player_name, game_name, error_message];
            if player_name.read().is_empty() || game_name.read().is_empty() {
                error_message.set(Some(
                    "Please enter both player name and game name".to_string(),
                ));
                return;
            }

            if let Some(service) = ws_service.borrow().as_ref() {
                let msg = ClientMessage::CreateGame {
                    game_name: game_name.read().clone(),
                    player_name: player_name.read().clone(),
                };
                service.send_message(msg);
            }
        }
    };

    let handle_join_game = {
        let ws_service = ws_service.clone();
        move |_| {
            to_owned![player_name, selected_game, error_message];
            if player_name.read().is_empty() {
                error_message.set(Some("Please enter your name".to_string()));
                return;
            }

            let selected_game_value = selected_game.read().clone();
            if let Some(game_name) = selected_game_value {
                if let Some(service) = ws_service.borrow().as_ref() {
                    let msg = ClientMessage::JoinGame {
                        game_name,
                        player_name: player_name.read().clone(),
                    };
                    service.send_message(msg);
                }
            }
        }
    };

    let handle_refresh_games = {
        let ws_service = ws_service.clone();
        move |_| {
            if let Some(service) = ws_service.borrow().as_ref() {
                service.send_message(ClientMessage::GetAvailableGames);
            }
        }
    };

    // Clone the games data before using in rsx!
    let games_data = available_games.read().clone();
    let selected_game_name = selected_game.read().clone();

    rsx! {
        div {
            class: "lobby-container",
            div {
                class: "lobby-content",
                h1 {
                    class: "lobby-title",
                    "Game Lobby"
                }

                if let Some(error) = error_message.read().as_ref() {
                    div {
                        class: "error-message",
                        "{error}"
                    }
                }

                div {
                    class: "lobby-card",
                    div {
                        class: "form-group",
                        label {
                            class: "form-label",
                            "Your Name:"
                        }
                        input {
                            class: "form-input",
                            r#type: "text",
                            placeholder: "Enter your name",
                            value: "{player_name}",
                            oninput: move |evt| player_name.set(evt.value()),
                        }
                    }

                    div {
                        class: "tab-buttons",
                        button {
                            class: if *is_creating.read() { "tab-button tab-active" } else { "tab-button" },
                            onclick: move |_| is_creating.set(true),
                            "Create Game"
                        }
                        button {
                            class: if !*is_creating.read() { "tab-button tab-active" } else { "tab-button" },
                            onclick: move |_| is_creating.set(false),
                            "Join Game"
                        }
                    }

                    if *is_creating.read() {
                        div {
                            class: "create-game-section",
                            div {
                                class: "form-group",
                                label {
                                    class: "form-label",
                                    "Game Name:"
                                }
                                input {
                                    class: "form-input",
                                    r#type: "text",
                                    placeholder: "Enter game name",
                                    value: "{game_name}",
                                    oninput: move |evt| game_name.set(evt.value()),
                                }
                            }
                            button {
                                class: "create-button",
                                onclick: handle_create_game,
                                "Create Game"
                            }
                        }
                    } else {
                        div {
                            class: "join-game-section",
                            div {
                                class: "games-header",
                                h3 {
                                    class: "games-title",
                                    "Available Games"
                                }
                                button {
                                    class: "refresh-button",
                                    onclick: handle_refresh_games,
                                    "Refresh"
                                }
                            }

                            if games_data.is_empty() {
                                p {
                                    class: "no-games-message",
                                    "No games available. Create one!"
                                }
                            } else {
                                div {
                                    class: "games-list",
                                    for game in games_data {
                                        div {
                                            key: "{game.id}",
                                            class: if selected_game_name.as_ref() == Some(&game.name) {
                                                "game-item game-item-selected"
                                            } else {
                                                "game-item"
                                            },
                                            onclick: {
                                                let game_name = game.name.clone();
                                                move |_| selected_game.set(Some(game_name.clone()))
                                            },
                                            div {
                                                class: "game-info",
                                                span {
                                                    class: "game-name",
                                                    "{game.name}"
                                                }
                                                span {
                                                    class: "player-count",
                                                    "Players: {game.player_count}/2"
                                                }
                                            }
                                        }
                                    }
                                }

                                if selected_game_name.is_some() {
                                    button {
                                        class: "join-button",
                                        onclick: handle_join_game,
                                        "Join Selected Game"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
