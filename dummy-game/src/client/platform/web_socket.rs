use web_sys::WebSocket as HTMLWebSocket;
use web_sys::window;

pub struct WebSocket {
    socket:HTMLWebSocket
}

impl WebSocket {
    pub fn new() -> Self {
        let host = window().expect("window not found").location().host().expect("host not found");
        let url = format!("ws://{}", host);
        let socket = HTMLWebSocket::new(&url).expect("failed to create web socket");

        Self {
            socket
        }
    }
}