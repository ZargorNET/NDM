use crate::util;
use crate::util::eventwaiter::Eventwaiter;

use super::prelude::*;

/// This struct will be passed for every command
///
/// ctx: The current context
///
/// m: The message without the prefix
///
/// handler: The handler wrapped in a already cloned Arc
#[derive(Clone)]
pub struct CommandArguments<'a> {
    pub ctx: &'a Context,
    pub m: &'a Message,
    pub handler: Arc<RwLock<CommandManager>>,
    pub safe: Arc<RwLock<Safe>>,
    pub image: Arc<util::image::ImageStorage>,
    pub settings: Arc<StaticSettings>,
    pub command: &'a Command,
    pub event_waiter: Arc<Eventwaiter>
}


impl<'a> CommandArguments<'a> {
    pub fn new(ctx: &'a Context, m: &'a Message, handler: Arc<RwLock<CommandManager>>, safe: Arc<RwLock<Safe>>, image: Arc<util::image::ImageStorage>, settings: Arc<StaticSettings>, command: &'a Command, event_waiter: Arc<Eventwaiter>) -> CommandArguments<'a> {
        CommandArguments {
            ctx,
            m,
            handler,
            safe,
            image,
            settings,
            command,
            event_waiter,
        }
    }
}
