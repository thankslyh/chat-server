use crate::socket::server;
use crate::socket::*;
use actix::prelude::*;
use actix_web_actors::ws;
use log::logger;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct ChatSession {
    pub id: usize,
    pub hb: Instant,
    pub addr: Addr<server::ChatServer>,
}

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

impl ChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if std::time::Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                if !ctx.state().stopping() {
                    // notify chat server
                    act.addr.do_send(server::Disconnect { id: act.id });
                    // stop actor
                    ctx.stop();
                }

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    pub fn new(addr: Addr<ChatServer>) -> Self {
        ChatSession {
            id: 0,
            hb: Instant::now(),
            addr,
        }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();

        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|act, cs, ctx| {
                match act {
                    Ok(id) => cs.id = id,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx)
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect { id: self.id });
        self.addr.do_send(server::ClientMessage {
            from: 0,
            to: self.id,
            message: server::Message(format!("session-{} 离开了聊天室", self.id)),
        });
        Running::Stop
    }
}

impl Handler<server::Message> for ChatSession {
    type Result = ();
    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) -> Self::Result {
        println!("handler server:Message {:#?}", msg);
        ctx.text(msg.0)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Ok(res) => res,
            Err(_) => {
                ctx.stop();
                return;
            }
        };
        log::debug!("stream handler receive: {msg:?}");
        match msg {
            ws::Message::Text(msg) => {
                self.addr.do_send(server::ClientMessage {
                    from: self.id,
                    to: self.id,
                    message: server::Message(msg.to_string()),
                });
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Ping(_) => {
                ctx.pong(b"");
                self.hb = Instant::now();
            }
            ws::Message::Close(_) => {
                self.stopping(ctx);
            }
            _ => println!("不支持的方式{:#?}", msg),
        }
    }
}
