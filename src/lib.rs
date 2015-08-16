extern crate capnp;
#[macro_use] extern crate log;
extern crate byteorder;
#[cfg(feature="json")] extern crate rustc_serialize;

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

mod serialize;
pub mod deserialize;
mod util;

// Reexport the Capnp error type, as it is currently the most widely used
// In the future, we may create our own error type wrapping everything
pub use capnp::Error;

#[cfg_attr(feature="json",derive(RustcEncodable,RustcDecodable))]
#[derive(Debug,Clone,Copy)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

#[cfg_attr(feature="json",derive(RustcEncodable,RustcDecodable))]
#[derive(Debug,Clone,Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg_attr(feature="json",derive(RustcEncodable,RustcDecodable))]
#[derive(Debug,Clone)]
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

#[cfg_attr(feature="json",derive(RustcEncodable,RustcDecodable))]
#[derive(Debug,Clone)]
pub enum Order {
    Walk(Option<Direction>),
    Say(String),
    // Attack
    // Cast spell
    // Talk
    // Exchange
    // ...
}

#[cfg_attr(feature="json",derive(RustcEncodable,RustcDecodable))]
#[derive(Debug,Clone)]
pub enum GameCommand {
    Disconnect,
    Authenticate(AuthenticationToken),
}

#[cfg_attr(feature="json",derive(RustcEncodable,RustcDecodable))]
#[derive(Debug,Clone)]
pub enum Command {
    EntityOrder(EntityOrder),
    GameCommand(GameCommand),
}

#[cfg_attr(feature="json",derive(RustcEncodable,RustcDecodable))]
#[derive(Debug,Clone)]
pub enum Notification {
    Walk {
        entity: u64,
        orientation: Option<Direction>,
    },
    Say {
        entity: u64,
        message: String,
    },
    Position {
        entity: u64,
        position: Vec2d,
        speed: Vec2d,
    },
    ThisIsYou {
        entity: u64,
    },
    Response {
        code: ErrorCode,
    },
    NewEntity {
        entity: u64,
        position: Vec2d,
        skin: u64,
    }
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

    pub fn position(id: u64, position: Vec2d, speed: Vec2d) -> Notification {
        Notification::Position {
            entity: id,
            position: position,
            speed: speed,
        }
    }

    pub fn this_is_you(id: u64) -> Notification {
        Notification::ThisIsYou {
            entity: id,
        }
    }

    pub fn response(code: ErrorCode) -> Notification {
        Notification::Response {
            code: code,
        }
    }

    pub fn new_entity(id: u64, position: Vec2d, skin: u64) -> Notification {
        Notification::NewEntity {
            entity: id,
            position: position,
            skin: skin,
        }
    }
}

#[cfg_attr(feature="json",derive(RustcEncodable,RustcDecodable))]
#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
pub struct AuthenticationToken {
    data0: u64,
}

impl AuthenticationToken {
    pub fn new(data0: u64) -> AuthenticationToken {
        AuthenticationToken {
            data0: data0,
        }
    }
}

#[cfg_attr(feature="json",derive(RustcEncodable,RustcDecodable))]
#[derive(Debug,Clone,Copy)]
pub enum ErrorCode {
    Success,
    Error,
}

// Hack, to be removed later
pub fn forge_authentication_tokens() -> Vec<AuthenticationToken> {
    (0..30).map(|i| AuthenticationToken::new(i)).collect()
}
