mod actors;
mod server;

use crate::{
    actors::{core::Core, message::StreamAddr, stream::Stream},
    server::Server,
};
use actix::{Actor, Addr, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use gunma::{components::*, protocol::*, Config};
use log::*;
use structopt::StructOpt;

fn index(
    data: web::Data<Addr<Core>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (addr, resp) = ws::start_with_addr(Stream::new((*data).clone()), &req, stream)?;

    data.do_send(StreamAddr(addr));

    info!("Response: {:?}", resp);

    Ok(resp)
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "bind", default_value = "127.0.0.1:8090")]
    bind: String,
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    let system = actix::System::new("chintama");

    let core = Core(Server::new(Config::default()).unwrap()).start();

    let app = HttpServer::new(move || {
        App::new()
            .data(core.clone())
            .route("/ws/", web::get().to(index))
    })
    .bind(&opt.bind)
    .unwrap()
    .start();

    system.run();
}
