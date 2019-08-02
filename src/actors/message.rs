use crate::actors::stream::Stream;
use actix::prelude::*;
use gunma::protocol::Message as GunmaMessage;

#[derive(Message, Clone)]
pub struct StreamAddr(pub Addr<Stream>);

#[derive(Message, Clone)]
pub struct ProtoMessage(pub GunmaMessage, pub Addr<Stream>);

impl ProtoMessage {
    pub fn reply(&self, msg: ProtoMessage) {
        self.1.do_send(msg);
    }
}
