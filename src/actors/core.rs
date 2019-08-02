use crate::{
    actors::{
        message::{ProtoMessage, StreamAddr},
        stream::Stream,
    },
    server::Server,
};
use actix::prelude::*;
use gunma::protocol::{Message, SendAction};
use log::*;

pub struct Core(pub Server);

impl Actor for Core {
    type Context = Context<Self>;
}

impl Handler<StreamAddr> for Core {
    type Result = ();

    fn handle(&mut self, msg: StreamAddr, ctx: &mut Self::Context) -> Self::Result {
        self.0.register(msg.0);
    }
}

impl Handler<ProtoMessage> for Core {
    type Result = ();

    fn handle(&mut self, msg: ProtoMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg.0 {
            Message::Login(login) => {
                self.0.login(login, msg.1);
            }
            Message::SendAction(act) => {
                let msg = ProtoMessage(Message::SendAction(act), msg.1);
                self.0.broadcast(msg);
            }
            msg => warn!("Unimplemented: {:?}", msg),
        }
    }
}
