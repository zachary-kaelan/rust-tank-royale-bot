use crate::bot::BotInfo;
use crate::connection::message_types::{BotIntent, TickEventForBot};

pub trait BotHandler {
    fn get_bot_info(&self) -> &BotInfo;
    fn is_droid(&self) -> bool;
    fn tick(&mut self, tick_event: &TickEventForBot) -> BotIntent;
}