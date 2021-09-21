use masterserv::{
    uuid::{self, Uuid},
    Game, GameType,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

mod host_handle;
pub use host_handle::*;

mod msg;
pub use msg::*;

mod host_manager_msg;
pub use host_manager_msg::*;

use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
    time::{interval, sleep},
};

pub struct HostServer {
    pub game_types: HashMap<&'static str, Arc<dyn Fn() -> Box<dyn Game> + Sync + Send>>,
    pub hosts: Vec<HostHandle>,
    pub rx: UnboundedReceiver<HostServerMsg>,
    pub tx: UnboundedSender<HostServerMsg>,
}

impl HostServer {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<HostServerMsg>();
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

    pub fn spawn_host(&mut self, id: Uuid, game_type_name: &str, name: String) {
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

    pub fn spawn(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                if let Some(msg) = self.rx.recv().await {
                    match msg {
                        HostServerMsg::SpawnHost {
                            id,
                            game_type,
                            name,
                        } => {
                            self.spawn_host(id, &game_type, name);
                        }
                        HostServerMsg::KillHost { id } => todo!(),
                    }
                } else {
                    break;
                }
            }
        })
    }
}
