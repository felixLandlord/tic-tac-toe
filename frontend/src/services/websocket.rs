// use shared::{ClientMessage, ServerMessage};
// use std::collections::VecDeque;
// use std::sync::{Arc, Mutex};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use web_sys::{CloseEvent, ErrorEvent, MessageEvent, WebSocket};

// #[derive(Clone)]
// pub struct WebSocketService {
//     websocket: WebSocket,
//     message_queue: Arc<Mutex<VecDeque<ServerMessage>>>,
// }

// impl WebSocketService {
//     pub async fn new(url: &str) -> Self {
//         let websocket = WebSocket::new(url).expect("Failed to create WebSocket");
//         let message_queue = Arc::new(Mutex::new(VecDeque::new()));

//         // Set up message handling
//         let queue_clone = message_queue.clone();
//         let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
//             if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
//                 let message_str = txt.as_string().unwrap();
//                 if let Ok(server_msg) = serde_json::from_str::<ServerMessage>(&message_str) {
//                     if let Ok(mut queue) = queue_clone.lock() {
//                         queue.push_back(server_msg);
//                     }
//                 }
//             }
//         }) as Box<dyn FnMut(MessageEvent)>);

//         websocket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
//         onmessage_callback.forget();

//         // Set up error handling
//         let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
//             web_sys::console::log_1(&"WebSocket error".into());
//         }) as Box<dyn FnMut(ErrorEvent)>);

//         websocket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
//         onerror_callback.forget();

//         // Set up close handling
//         let onclose_callback = Closure::wrap(Box::new(move |e: CloseEvent| {
//             web_sys::console::log_1(&"WebSocket closed".into());
//         }) as Box<dyn FnMut(CloseEvent)>);

//         websocket.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
//         onclose_callback.forget();

//         Self {
//             websocket,
//             message_queue,
//         }
//     }

//     pub fn send_message(&self, message: ClientMessage) {
//         if let Ok(json) = serde_json::to_string(&message) {
//             let _ = self.websocket.send_with_str(&json);
//         }
//     }

//     pub async fn receive_message(&self) -> Option<ServerMessage> {
//         if let Ok(mut queue) = self.message_queue.lock() {
//             queue.pop_front()
//         } else {
//             None
//         }
//     }
// }

use shared::{ClientMessage, ServerMessage};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CloseEvent, ErrorEvent, MessageEvent, WebSocket};

#[derive(Clone)]
pub struct WebSocketService {
    websocket: WebSocket,
    message_queue: Arc<Mutex<VecDeque<ServerMessage>>>,
}

impl WebSocketService {
    // Fix: Return Result<Self, JsValue> to handle WebSocket creation errors
    pub async fn new(url: &str) -> Result<Self, JsValue> {
        let websocket = WebSocket::new(url)?; // Remove .expect(), use ? operator
        let message_queue = Arc::new(Mutex::new(VecDeque::new()));

        // Set up message handling
        let queue_clone = message_queue.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                let message_str = txt.as_string().unwrap();
                if let Ok(server_msg) = serde_json::from_str::<ServerMessage>(&message_str) {
                    if let Ok(mut queue) = queue_clone.lock() {
                        queue.push_back(server_msg);
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        websocket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        // Set up error handling
        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            web_sys::console::log_1(&"WebSocket error".into());
        }) as Box<dyn FnMut(ErrorEvent)>);

        websocket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        // Set up close handling
        let onclose_callback = Closure::wrap(Box::new(move |e: CloseEvent| {
            web_sys::console::log_1(&"WebSocket closed".into());
        }) as Box<dyn FnMut(CloseEvent)>);

        websocket.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();

        Ok(Self {
            websocket,
            message_queue,
        })
    }

    pub fn send_message(&self, message: ClientMessage) -> Result<(), JsValue> {
        if let Ok(json) = serde_json::to_string(&message) {
            self.websocket.send_with_str(&json) // Fix: return the result
        } else {
            Err(JsValue::from_str("Failed to serialize message"))
        }
    }

    pub fn receive_message(&self) -> Option<ServerMessage> {
        if let Ok(mut queue) = self.message_queue.lock() {
            queue.pop_front()
        } else {
            None
        }
    }
}

// Alternative implementation if you want to keep the original signature
// (but this is less safe as it can panic)
impl WebSocketService {
    pub async fn new_panicking(url: &str) -> Self {
        let websocket = WebSocket::new(url).expect("Failed to create WebSocket");
        let message_queue = Arc::new(Mutex::new(VecDeque::new()));

        // ... rest of the implementation stays the same

        Self {
            websocket,
            message_queue,
        }
    }
}
