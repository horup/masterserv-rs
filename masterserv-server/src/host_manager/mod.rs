use masterserv::{Game, GameType, uuid::{self, Uuid}};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

mod host_handle;
pub use host_handle::*;

mod host_msg;
pub use host_msg::*;

mod host_manager_msg;
pub use host_manager_msg::*;

use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
    time::{interval, sleep},
};

pub struct HostManager {
    pub game_types: HashMap<&'static str, Arc<dyn Fn() -> Box<dyn Game> + Sync + Send>>,
    pub hosts: Vec<HostHandle>,
    pub rx: UnboundedReceiver<HostManagerMsg>,
    pub tx: UnboundedSender<HostManagerMsg>,
}

impl HostManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<HostManagerMsg>();
        Self {
            game_types: Default::default(),
            hosts: Default::default(),
            rx,
            tx,
        }
    }
    pub fn register_game_type<T: GameType + Send>(&mut self) {
        if self.game_types.contains_key(T::NAME) {
            panic!("Game Type Name already registered!");
        }
        self.game_types.insert(
            T::NAME,
            Arc::new(|| {
                return Box::new(T::default());
            }),
        );
    }

    pub fn spawn_host(&mut self, id:Uuid, game_type_name: &str, name:String) {
        let mut handle = HostHandle {
            id,
            name: name.clone(),
            messages: Arc::new(Mutex::new(Vec::new())),
        };

        if let Some(game) = self.game_types.get(game_type_name) {
            let create_game = game.clone();
            tokio::spawn(async move {
                let messages = handle.messages.clone();
                let mut game = create_game();
                let period = Duration::from_millis(1000 / game.tick_rate());
                let mut timer = interval(period);
                game.start(id, name);
                loop {
                    // pop messages
                    let mut new_messages = Vec::new();
                    if let Ok(mut messages) = messages.lock() {
                        new_messages = messages.clone();
                        messages.clear();
                    }

                    game.update(period.as_secs_f32());
                    timer.tick().await;
                }
            });
        }
    }

    pub fn spawn(mut self) -> (UnboundedSender<HostManagerMsg>, JoinHandle<()>) {
        return (
            self.tx.clone(),
            tokio::spawn(async move {
                loop {
                    if let Some(msg) = self.rx.recv().await {
                        match msg {
                            HostManagerMsg::SpawnHost { id, game_type , name} => {
                                self.spawn_host(id, &game_type, name);
                            },
                            HostManagerMsg::KillHost { id } => todo!(),
                        }
                    } else {
                        break;
                    }
                }
            }),
        );
    }
}
