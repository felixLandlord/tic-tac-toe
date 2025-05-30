use crate::routes::Route;
use dioxus::{logger::tracing::info, prelude::*};
use web_sys::console;

#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();

    let handle_play = move |_| {
        info!("User clicked play");
        console::log_1(&"Play button clicked".into());
        navigator.push(Route::Lobby {});
        info!("Navigated to lobby");
        console::log_1(&"Navigation attempted".into());
    };

    rsx! {
        div {
            class: "home-container",
            div {
                class: "home-card",
                h1 {
                    class: "home-title",
                    "Tic Tac Toe Multiplayer"
                }
                p {
                    class: "home-description",
                    "Challenge your friends in this classic game!"
                }
                button {
                    class: "home-play-button",
                    onclick: handle_play,
                    "Play"
                }
            }
        }
    }
}
