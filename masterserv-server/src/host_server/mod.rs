use masterserv::{Game, GameType, log::{debug, info}, uuid::{Uuid}};
use std::{collections::HashMap, sync::{Arc, Mutex}, time::{Duration}};

mod host_handle;
pub use host_handle::*;

mod msg;
pub use msg::*;

mod host_manager_msg;
pub use host_manager_msg::*;

use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
    time::{interval},
};

pub struct HostServer {
    pub game_types: HashMap<&'static str, Arc<dyn Fn() -> Box<dyn Game> + Sync + Send>>,
    pub hosts: HashMap<Uuid, HostHandle>,
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
        let handle = HostHandle {
            id:id.clone(),
            name: name.clone(),
            messages: Arc::new(Mutex::new(Vec::new())),
        };

        if let Some(game) = self.game_types.get(game_type_name) {
            let create_game = game.clone();
            let messages = handle.messages.clone();
            tokio::spawn(async move {
                info!("{:?} Spawned", id);
                let mut game = create_game();
                let period = Duration::from_millis(1000 / game.tick_rate());
                let mut timer = interval(period);
                let mut run = true;
                game.start(id, name);
                while run {
                    // pop messages
                    let mut new_messages = Vec::new();
                    if let Ok(mut messages) = messages.lock() {
                        new_messages = messages.clone();
                        messages.clear();
                    }

                    for msg in new_messages {
                        match msg {
                            HostMsg::Kill => run = false,
                        }
                    }

                    game.update(period.as_secs_f32());
                    timer.tick().await;
                }

                info!("{:?} Terminated", id);
            });

            // terminate existing host if any
            if let Some(existing) = self.hosts.get(&id) {
                let id = existing.id.clone();
                self.kill_host(id);
            }

            self.hosts.insert(id, handle);
        }
    }

    pub fn spawn(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            info!("Spawned");

            loop {
                if let Some(msg) = self.rx.recv().await {
                    debug!("Received: {:?}", msg);
                    match msg {
                        HostServerMsg::SpawnHost {
                            id,
                            game_type,
                            name,
                        } => {
                            self.spawn_host(id, &game_type, name);
                        }
                        HostServerMsg::KillHost { id } => {
                            self.kill_host(id);
                        },
                    }
                } else {
                    break;
                }
            }
        })
    }

    pub fn push_message(&mut self, host_id:Uuid, msg:HostMsg) -> bool {
        if let Some(handle) = self.hosts.get_mut(&host_id) {
            if let Ok(mut messages) = handle.messages.lock() {
                messages.push(msg);
                return true;
            }
        }

        false
    }

    pub fn push_messages(&mut self, host_id:Uuid, msgs:Vec<HostMsg>) -> bool {
        if let Some(handle) = self.hosts.get_mut(&host_id) {
            if let Ok(mut messages) = handle.messages.lock() {
                for msg in msgs {
                    messages.push(msg);
                }

                return true;
            }
        }

        false
    }

    pub fn kill_host(&mut self, host_id:Uuid) {
        if let Some(handle) = self.hosts.get_mut(&host_id) {
            if let Ok(mut messages) = handle.messages.lock() {
                messages.push(HostMsg::Kill);
            }
        }

        self.hosts.remove(&host_id);
    }
}
