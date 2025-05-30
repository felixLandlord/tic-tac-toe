// use dioxus::{logger::tracing::info, prelude::*};
// mod components;
// mod routes;
// mod services;
// mod utils;

// use crate::routes::Route;
// use services::websocket::WebSocketService;
// use std::cell::RefCell;
// use std::sync::Arc;
// use web_sys::console;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/main.css");

// fn main() {
//     console_error_panic_hook::set_once();
//     dioxus::launch(App);
// }

// // #[component]
// // fn App() -> Element {
// //     use_context_provider(|| None::<WebSocketService>);

// //     rsx! {
// //         document::Link { rel: "icon", href: FAVICON }
// //         document::Link { rel: "stylesheet", href: MAIN_CSS }
// //         Router::<Route> {}
// //     }
// // }

// #[component]
// fn App() -> Element {
//     // Use use_signal instead of use_ref for reactive state
//     // let mut websocket_service = use_signal(|| None::<WebSocketService>);

//     // Create the Arc<RefCell<Option<WebSocketService>>> immediately
//     let ws_service_context =
//         use_context_provider(|| Arc::new(RefCell::new(None::<WebSocketService>)));

//     use_effect(move || {
//         // Clone the Arc before moving into the async block
//         let ws_service_context = ws_service_context.clone();
//         spawn(async move {
//             info!("Initializing WebSocket service");
//             print!("Initializing WebSocket service");
//             console::log_1(&"Initializing WebSocket...".into());

//             let new_service = WebSocketService::new("ws://127.0.0.1:8080/api/ws").await;
//             *ws_service_context.borrow_mut() = Some(new_service);
//             info!("WebSocket service initialized");
//             println!("WebSocket service initialized");
//             console::log_1(&"WebSocket initialized".into());
//         });
//     });

//     // Fix: use_future doesn't take closure parameters in newer Dioxus versions
//     // Use use_resource or spawn a future manually
//     // use_effect(move || {
//     //     spawn(async move {
//     //         let ws = WebSocketService::new("ws://127.0.0.1:8080/api/ws").await;
//     //         websocket_service.set(Some(ws));
//     //     });
//     // });

//     // Provide the context - simplified approach
//     // use_context_provider(|| websocket_service);

//     // let ws_service_context = Arc::new(RefCell::new(websocket_service.get()));
//     // use_context_provider(|| Some(ws_service_context));

//     rsx! {
//         document::Link { rel: "icon", href: FAVICON }
//         document::Link { rel: "stylesheet", href: MAIN_CSS }
//         // Router::<Route> {}
//         ContextProvider {
//             value: ws_service.read().clone(),
//             Router::<Route> {}
//         }
//     }
// }

use dioxus::{logger::tracing::Level, prelude::*};
mod components;
mod routes;
mod services;
mod utils;
use crate::routes::Route;
use services::websocket::WebSocketService;
use std::cell::RefCell;
use std::sync::Arc;
extern crate console_error_panic_hook;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

// fn main() {
//     dioxus::logger::init(Level::INFO).expect("failed to init logger");
//     console_error_panic_hook::set_once();
//     dioxus::launch(App);
// }
fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    let config = dioxus_web::Config::new();
    dioxus_web::launch::launch_cfg(App, config);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        WebSocketProvider {}
    }
}

#[component]
fn WebSocketProvider() -> Element {
    // Create and provide the WebSocket context
    let ws_service = use_signal(|| Arc::new(RefCell::new(None::<WebSocketService>)));

    // Initialize WebSocket connection
    use_effect({
        let mut ws_service = ws_service.clone();
        move || {
            spawn(async move {
                match WebSocketService::new("ws://127.0.0.1:8080/api/ws").await {
                    Ok(service) => {
                        // Fix: Use write() instead of read() to get mutable access
                        *ws_service.write().borrow_mut() = Some(service);
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize WebSocket: {:?}", e);
                    }
                }
            });
        }
    });

    // Provide the context to all children
    // Fix: Use read() to get the current value for context
    use_context_provider(|| ws_service.read().clone());

    rsx! {
        Router::<Route> {}
    }
}

// Better approach - cleaner separation of concerns
#[component]
fn WebSocketProviderAlternative() -> Element {
    // Provide context at the top level
    use_context_provider(|| Arc::new(RefCell::new(None::<WebSocketService>)));

    rsx! {
        WebSocketInitializer {}
        Router::<Route> {}
    }
}

#[component]
fn WebSocketInitializer() -> Element {
    // Get the context
    let ws_service_context = use_context::<Arc<RefCell<Option<WebSocketService>>>>();

    use_effect({
        let ws_service_context = ws_service_context.clone();
        move || {
            let ws_service_context = ws_service_context.clone();
            spawn(async move {
                match WebSocketService::new("ws://127.0.0.1:8080/api/ws").await {
                    Ok(service) => {
                        *ws_service_context.borrow_mut() = Some(service);
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize WebSocket: {:?}", e);
                    }
                }
            });
        }
    });

    // This component doesn't render anything visible
    rsx! { div { style: "display: none;" } }
}

// Even better approach - using a more idiomatic pattern
#[component]
fn WebSocketProviderBest() -> Element {
    // Use Signal directly for the WebSocket service
    let ws_service: Signal<Option<WebSocketService>> = use_signal(|| None);

    // Initialize WebSocket connection
    use_effect({
        let mut ws_service = ws_service.clone();
        move || {
            spawn(async move {
                match WebSocketService::new("ws://127.0.0.1:8080/api/ws").await {
                    Ok(service) => {
                        ws_service.set(Some(service));
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize WebSocket: {:?}", e);
                    }
                }
            });
        }
    });

    // Provide the signal as context
    use_context_provider(|| ws_service);

    rsx! {
        Router::<Route> {}
    }
}
