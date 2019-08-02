use crate::actors::{BroadcastAction, PlayerSync};
use actix::Addr;
use gunma::{prelude::*, Config, Io, Result, Systems};
use log::*;

pub struct Server {
    sys: Systems,
    io: Io,
    sockets: Vec<Addr<PlayerSync>>,
}

impl Server {
    pub fn new(cfg: Config) -> Result<Self> {
        let mut io = Io::new(cfg)?;
        let mut sys = Systems::new()?;

        for t in io.get_all_terrain()? {
            sys.create_entity()
                .create_terrain_block(t.pos, t.size, t.asset);
        }

        Ok(Server {
            sys,
            io,
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
