use crate::actors::{message::ProtoMessage, stream::Stream};
use actix::Addr;
use gunma::{components::*, prelude::*, protocol::*, Config, Io, Result, Systems};
use log::*;
use std::collections::HashMap;

pub struct Server {
    sys: Systems,
    io: Io,
    streams: Vec<Addr<Stream>>,
    id: u64,
    player: HashMap<u64, Addr<Stream>>,
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
            streams: Vec::new(),
            id: 0,
            player: HashMap::new(),
        })
    }

    pub fn register(&mut self, addr: Addr<Stream>) {
        info!("New stream created");
        self.streams.push(addr);
    }

    pub fn login(&mut self, login: Login, addr: Addr<Stream>) {
        info!("Player login: {:?}", login);

        self.id = self.id.wrapping_add(1);

        let id = self.id;
        self.player.insert(id, addr.clone());

        let player = Player::new(id, CLASS_CHIBA, 10);
        let pos = Pos::new(500.0, 300.0);

        let ack = LoginAck::new(player, pos);
        let msg = ProtoMessage(Message::LoginAck(ack), addr.clone());
        addr.do_send(msg);
    }

    pub fn broadcast(&mut self, msg: ProtoMessage) {
        self.streams.retain(|stream| stream.connected());

        for stream in &self.streams {
            trace!("Broadcasting {:?}", msg.0);
            stream.do_send(msg.clone());
        }
    }
}
