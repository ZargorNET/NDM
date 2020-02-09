use std::sync::Arc;

use serenity::model::channel::{Message, Reaction};
use serenity::model::user::User;

use crate::command_framework::CommandManager;
use crate::command_framework::prelude::{Context, RwLock};
use crate::StaticSettings;

#[derive(Clone)]
#[allow(dead_code)]
pub enum ResponseAccess {
    User,
    Everyone,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum EventAction {
    Keep,
    Remove,
}

#[derive(Clone)]
pub struct ReactionEvent {
    pub access: ResponseAccess,
    pub timeout: u32,
    pub eventwaiter: Arc<Eventwaiter>,
    pub author: User,
    pub message: Message,
    pub handler: Arc<RwLock<CommandManager>>,
    pub settings: Arc<StaticSettings>,
    pub callback: fn(&Context, &mut ReactionEvent, &Reaction) -> EventAction,
}

pub struct Eventwaiter {
    events: RwLock<Vec<ReactionEvent>>
}

impl Eventwaiter {
    pub fn new() -> Self {
        Eventwaiter {
            events: RwLock::new(Vec::new())
        }
    }

    pub fn fire_reaction(&self, ctx: Context, reaction: Reaction) {
        if reaction.user(&ctx).unwrap().bot {
            return;
        }
        let events = self.events.read().clone();

        for (i, mut event) in events.into_iter().enumerate() {
            if reaction.message_id == event.message.id {
                let can_access: bool = match event.access {
                    ResponseAccess::User => {
                        reaction.user_id == event.author.id
                    }
                    ResponseAccess::Everyone => {
                        true
                    }
                };
                if can_access {
                    &reaction.delete(&ctx);
                    match (event.callback)(&ctx, &mut event, &reaction) {
                        EventAction::Remove => self.unregister_event(i),
                        EventAction::Keep => {}
                    }
                }
            }
        }
    }

    pub fn register_event(&self, wait_event: ReactionEvent) {
        self.events.write().push(wait_event)
    }

    fn unregister_event(&self, index: usize) {
        self.events.write().remove(index);
    }
}