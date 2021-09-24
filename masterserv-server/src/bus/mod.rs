use tokio::sync::broadcast::{Receiver, Sender, channel};
mod events;
pub use events::*;

pub struct Bus {
    send:Sender<BusEvent>,
    recv:Receiver<BusEvent>
}

impl Clone for Bus {
    fn clone(&self) -> Self {
        let mut send = self.send.clone();
        let recv = send.subscribe();
        Self { send, recv }
    }
}

impl Default for Bus {
    fn default() -> Self {
        let (send, recv) = channel::<BusEvent>(1024);
        Self { 
            send, recv
        }
    }
}
