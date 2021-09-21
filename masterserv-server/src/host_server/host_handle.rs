use std::sync::{Arc, Mutex};

use masterserv::uuid::Uuid;

use crate::HostMsg;

pub struct HostHandle {
    pub id:Uuid,
    pub name:String,
    pub messages:Arc<Mutex<Vec<HostMsg>>>
}
