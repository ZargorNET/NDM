use std::sync::Arc;

use chrono::Utc;
use serenity::model::channel::{Message, Reaction};
use serenity::model::user::User;

use crate::command_framework::{CommandArguments, CommandManager};
use crate::command_framework::prelude::{Context, RwLock};
use crate::scheduler::ScheduleArguments;
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
    pub timeout: i64,
    pub eventwaiter: Arc<Eventwaiter>,
    pub author: User,
    pub message: Message,
    pub author_message: Message,
    pub handler: Arc<RwLock<CommandManager>>,
    pub settings: Arc<StaticSettings>,
    pub callback: fn(&Context, &mut ReactionEvent, &Reaction) -> EventAction,
}

impl ReactionEvent {
    pub fn new(access: ResponseAccess, timeout: i64, message: &Message, callback: fn(&Context, &mut ReactionEvent, &Reaction) -> EventAction, args: &CommandArguments) -> Self {
        ReactionEvent {
            access,
            timeout,
            eventwaiter: Arc::clone(&args.event_waiter),
            author: args.m.author.clone(),
            message: message.clone(),
            author_message: args.m.clone(),
            handler: Arc::clone(&args.handler),
            settings: Arc::clone(&args.settings),
            callback,
        }
    }
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

        for (i, mut event) in events.into_iter().enumerate().rev() {
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

    pub fn clean_timeouts(&self, args: &ScheduleArguments) {
        let events = self.events.read().clone();
        for (i, event) in events.into_iter().enumerate().rev() {
            if event.timeout == 0 {
                continue;
            }
            let message = match args.serenity.http.get_message(event.message.channel_id.0, event.message.id.0) {
                Err(_) => {
                    self.unregister_event(i);
                    continue;
                }
                Ok(message) => message
            };

            let timestamp_to_use = match message.edited_timestamp {
                Option::None => event.message.timestamp,
                Option::Some(t) => t
            };
            if timestamp_to_use.timestamp() + event.timeout < Utc::now().timestamp() {
                self.unregister_event(i);

                let _ = event.message.delete(args.serenity.clone());
                let _ = event.author_message.delete(args.serenity.clone());
            }
        }
    }
}