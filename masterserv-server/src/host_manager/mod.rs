use masterserv::{GameType, HostMsg, HostedGame, log::{debug, error, info}, uuid::{Uuid}};
use std::{collections::HashMap, sync::{Arc, Mutex}};

use tokio::{task::JoinHandle};

use crate::{Bus, Host, HostHandle};

pub struct HostManager {
    pub game_types: HashMap<&'static str, Arc<dyn Fn() -> Box<dyn HostedGame> + Sync + Send>>,
    pub hosts: HashMap<Uuid, HostHandle>,
    pub bus:Bus
}

impl HostManager {
    pub fn new(bus:Bus) -> Self {
        Self {
            game_types: Default::default(),
            hosts: Default::default(),
            bus
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
                match self.bus.recv.recv().await {
                    Ok(event) => {
                        debug!("Received: {:?}", event);
                        match event {
                            crate::BusEvent::ClientConnected { client_id: _ } => {

                            },
                            crate::BusEvent::ClientDisconnected { client_id: _ } => {

                            },
                            crate::BusEvent::SpawnHost { host_id, name, game_type } => {
                                self.spawn_host(host_id, &game_type, name);
                            },
                            crate::BusEvent::TerminateHost { host_id } => {
                                self.kill_host(host_id);
                            },
                            crate::BusEvent::ClientJoinedHost { host_id : _ } => {

                            },
                            crate::BusEvent::ClientLeftHost { host_id: _ } => {

                            },
                        }
                    },
                    Err(err) => {
                        error!("Error: {}", err);
                    },
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
