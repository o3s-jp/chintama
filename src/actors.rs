use crate::server::Server;
use actix::prelude::*;
use actix_web_actors::ws::{self, WebsocketContext as WsContext};
use gunma::protocol::{Message as GMessage, SendAction};
use log::*;

#[derive(Message, Clone)]
pub struct PlayerSyncAddr(pub Addr<PlayerSync>);

#[derive(Message, Clone, Debug)]
pub struct BroadcastAction(pub SendAction);

struct Sender<'a, A>
where
    A: Actor<Context = WsContext<A>>,
{
    ctx: &'a mut WsContext<A>,
}

impl<'a, A> Sender<'a, A>
where
    A: Actor<Context = WsContext<A>>,
{
    fn new(ctx: &'a mut WsContext<A>) -> Self {
        Self { ctx }
    }

    fn send(&mut self, msg: GMessage) {
        trace!("Sending {:?}", msg);

        match serde_json::to_vec(&msg) {
            Ok(bin) => {
                let _ = self.ctx.binary(bin);
            }
            Err(e) => error!("Coudln't send message: {}: {:?}", e, msg),
        }
    }
}

pub struct PlayerSync(pub Addr<Core>);

impl Actor for PlayerSync {
    type Context = WsContext<PlayerSync>;
}

impl Handler<BroadcastAction> for PlayerSync {
    type Result = ();

    fn handle(&mut self, msg: BroadcastAction, ctx: &mut Self::Context) -> Self::Result {
        info!("Send action: {:?}", msg.0);
        let mut sender = Sender::new(ctx);
        sender.send(GMessage::SendAction(msg.0));
    }
}

impl PlayerSync {
    fn handler<A>(&mut self, mut sender: Sender<A>, msg: GMessage)
    where
        A: Actor<Context = WsContext<A>>,
    {
        match msg {
            GMessage::SendAction(info) => {
                info!("Got action: {:?}", info);
                self.0.do_send(BroadcastAction(info));
            }
            msg => warn!("Received unsupported request: {:?}", msg),
        }
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for PlayerSync {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Binary(bin) => {
                let msg = match serde_json::from_slice(&bin) {
                    Ok(msg) => msg,
                    Err(e) => return warn!("Couldn't parse message: {}", e),
                };

                let sender = Sender::new(ctx);
                self.handler(sender, msg);
            }
            msg => warn!("Invalid message: {:?}", msg),
        }
    }
}

pub struct Core(pub Server);

impl Actor for Core {
    type Context = Context<Self>;
}

impl Handler<PlayerSyncAddr> for Core {
    type Result = ();

    fn handle(&mut self, msg: PlayerSyncAddr, ctx: &mut Self::Context) -> Self::Result {
        self.0.register(msg.0);
    }
}

impl Handler<BroadcastAction> for Core {
    type Result = ();

    fn handle(&mut self, msg: BroadcastAction, ctx: &mut Self::Context) -> Self::Result {
        self.0.broadcast(msg);
    }
}
