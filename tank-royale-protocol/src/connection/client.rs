use std::sync::mpsc::{self, Sender, Receiver};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use serde::Serialize;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use crate::bot::BotHandler;
use crate::connection::message_types::{BotHandshake, BotHandshakeInfo};
use super::enums::*;

pub struct RoboClient {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    message_log: Vec<WebsocketMessage>,
    server_secret: String
}

impl RoboClient {
    pub fn new(server_uri: &String, server_secret: String) -> Self {
        let (mut socket, response) = connect(server_uri).expect("Can't connect");
        Self {
            socket, message_log: vec![], server_secret
        }
    }

    pub fn server_loop<T>(&mut self, mut bot: T) where T : BotHandler {
        loop {
            let msg = self.socket.read().expect("Error reading message");
            if msg.is_text() {
                let msg: WebsocketMessage = serde_json::from_str(&msg.into_text().unwrap()).unwrap();
                match &msg {
                    WebsocketMessage::TickEventForBot(e) => { bot.tick(e) }
                    WebsocketMessage::RoundStartedEvent(e) => {}
                    WebsocketMessage::RoundEndedEventForBot(e) => {}
                    WebsocketMessage::GameStartedEventForBot(e) => {}
                    WebsocketMessage::GameEndedEventForBot(e) => {}
                    WebsocketMessage::SkippedTurnEvent(e) => {}
                    WebsocketMessage::ServerHandshake(e) => {
                        let info = bot.get_bot_info();
                        let handshake = BotHandshake::from_bot_info(
                            &e.session_id,
                            *info.clone(),
                            None,
                            bot.is_droid(),
                            self.server_secret.clone()
                        );
                        serde_json::to_writer(&self.socket, &MessageType::BotHandshake(handshake))
                    }
                    WebsocketMessage::GameAbortedEvent(e) => {}
                };
                self.message_log.push(msg);
            }
        }
    }
}