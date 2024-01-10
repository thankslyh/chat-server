use actix::prelude::*;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// socket 通信基础数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SocketEvent {
    pub from: String,
    pub to: String,
    pub payload: String,
    pub hash: usize,
}

impl FromStr for SocketEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = serde_json::from_str::<Self>(s);
        let res = res.map_err(|_| ())?;
        Ok(res)
    }
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub skip_id: usize,
    pub message: Message,
}

pub struct ChatServer {
    pub sessions: HashMap<usize, Recipient<Message>>,
    pub rng: ThreadRng,
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            sessions: HashMap::new(),
            rng: thread_rng(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl ChatServer {
    pub fn send_message(&self, message: String, skip_id: usize) {
        for (id, session) in self.sessions.iter() {
            if id.eq(&skip_id) {
                continue;
            }
            session.do_send(Message(message.clone()));
        }
    }
}

impl Handler<Connect> for ChatServer {
    type Result = usize;
    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        println!("new connection join");
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);
        self.send_message(format!("session-{}加入了聊天室", id), id);
        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        let _ = self.sessions.remove(&msg.id).is_some();
        ()
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        self.send_message(msg.message.0, msg.skip_id)
    }
}
