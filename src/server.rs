use crate::actors::{BroadcastAction, PlayerSync};
use actix::Addr;
use gunma::{Config, Result, Systems};
use log::*;

pub struct Server {
    sys: Systems,
    sockets: Vec<Addr<PlayerSync>>,
}

impl Server {
    pub fn new(cfg: Config) -> Result<Self> {
        let sys = Systems::new(cfg)?;
        Ok(Server {
            sys,
            sockets: Vec::new(),
        })
    }

    pub fn register(&mut self, addr: Addr<PlayerSync>) {
        self.sockets.push(addr);
    }

    pub fn broadcast(&mut self, msg: BroadcastAction) {
        for addr in &self.sockets {
            trace!("Sending {:?}", msg);
            addr.do_send(msg.clone());
        }
    }
}
