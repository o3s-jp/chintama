use crate::actors::{core::Core, message::ProtoMessage};
use actix::prelude::*;
use actix_web_actors::ws::{self, WebsocketContext as WsContext};
use gunma::protocol::{Message, SendAction};
use log::*;

pub struct Stream(Addr<Core>);

impl Stream {
    pub fn new(addr: Addr<Core>) -> Self {
        Self(addr)
    }
}

impl Actor for Stream {
    type Context = WsContext<Stream>;
}

impl Handler<ProtoMessage> for Stream {
    type Result = ();

    fn handle(&mut self, msg: ProtoMessage, ctx: &mut Self::Context) -> Self::Result {
        info!("Send to client: {:?}", msg.0);

        match serde_json::to_vec(&msg.0) {
            Ok(bin) => {
                let _ = ctx.binary(bin);
            }
            Err(e) => error!("Coudln't send message: {}: {:?}", e, msg.0),
        }
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Stream {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Binary(bin) => {
                let msg: Message = match serde_json::from_slice(&bin) {
                    Ok(msg) => msg,
                    Err(e) => return warn!("Couldn't parse message: {}", e),
                };

                info!("Receive from client: {:?}", msg);
                self.0.do_send(ProtoMessage(msg, ctx.address()));
            }
            msg => warn!("Invalid message: {:?}", msg),
        }
    }
}
