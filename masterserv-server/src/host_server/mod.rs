use masterserv::{Game, GameType, log::{debug, info}, uuid::{Uuid}};
use std::{collections::HashMap, sync::{Arc, Mutex}};

mod msg;
pub use msg::*;

mod host;
pub use host::*;

use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle
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
        
        if let Some(game) = self.game_types.get(game_type_name) {
            let create_game = game.clone();
            let handle = HostHandle {
                id:id,
                game_type_name:game_type_name.into(),
                name: name.clone(),
                messages: Arc::new(Mutex::new(Vec::new())),
                create_game:create_game.clone()
            };

            
            // terminate existing host if any
            let existing_handle = self.hosts.get(&id);
            if let Some(existing_handle) = existing_handle {
                let id = existing_handle.id;
                self.kill_host(id);
            }

            // spawn host
            let host = Host::new(handle.clone());
            self.hosts.insert(id, handle);
            host.spawn();
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
                        HostServerMsg::TerminateHost { id } => {
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
                messages.push(HostMsg::Terminate);
            }
        }

        self.hosts.remove(&host_id);
    }
}
