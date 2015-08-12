extern crate capnp;
#[macro_use] extern crate log;
extern crate byteorder;

#[allow(dead_code)]
mod notifications_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/notifications_capnp.rs"));
}

#[allow(dead_code)]
mod commands_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/commands_capnp.rs"));
}

#[allow(dead_code)]
mod common_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/common_capnp.rs"));
}

#[allow(dead_code)]
mod authentication_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/authentication_capnp.rs"));
}

mod serialize;
pub mod deserialize;
mod util;

// Reexport the Capnp error type, as it is currently the most widely used
// In the future, we may create our own error type wrapping everything
pub use capnp::Error;

#[derive(Debug,Clone,Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct EntityOrder {
    pub entity: u64,
    pub order: Order,
}

impl EntityOrder {
    pub fn new(entity: u64, order: Order) -> EntityOrder {
        EntityOrder {
            entity: entity,
            order: order,
        }
    }
}

#[derive(Debug)]
pub enum Order {
    Walk(Option<Direction>),
    Say(String),
    // Attack
    // Cast spell
    // Talk
    // Exchange
    // ...
}

pub enum MapCommand {
    Disconnect,
}

pub enum Command {
    EntityOrder(EntityOrder),
    MapCommand(MapCommand),
}

#[derive(Debug)]
pub enum Notification {
    Walk {
        entity: u64,
        orientation: Option<Direction>,
    },
    Say {
        entity: u64,
        message: String,
    },
    Location {
        entity: u64,
        x: f32,
        y: f32,
    },
    ThisIsYou {
        entity: u64,
    },
}

impl Notification {
    pub fn walk(id: u64, orientation: Option<Direction>) -> Notification {
        Notification::Walk {
            entity: id,
            orientation: orientation,
        }
    }

    pub fn say(id: u64, message: String) -> Notification {
        Notification::Say {
            entity: id,
            message: message,
        }
    }

    pub fn location(id: u64, x: f32, y: f32) -> Notification {
        Notification::Location {
            entity: id,
            x: x,
            y: y,
        }
    }

    pub fn this_is_you(id: u64) -> Notification {
        Notification::ThisIsYou {
            entity: id,
        }
    }
}

#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
pub struct AuthenticationToken {
    data0: u64,
}

#[derive(Debug,Clone,Copy)]
pub enum ErrorCode {
    Success,
    Error,
}

// Hack, to be removed later
pub fn forge_authentication_tokens() -> Vec<AuthenticationToken> {
    (0..30).map(|i| {
        AuthenticationToken {
            data0: i,
        }
    }).collect()
}
