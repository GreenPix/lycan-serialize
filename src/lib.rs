extern crate capnp;
#[macro_use] extern crate log;
extern crate byteorder;
#[cfg(feature="json")] extern crate rustc_serialize;

#[allow(dead_code)]
mod notifications_capnp {
    include!(concat!(env!("OUT_DIR"), "/notifications_capnp.rs"));
}

#[allow(dead_code)]
mod commands_capnp {
    include!(concat!(env!("OUT_DIR"), "/commands_capnp.rs"));
}

#[allow(dead_code)]
mod common_capnp {
    include!(concat!(env!("OUT_DIR"), "/common_capnp.rs"));
}

mod serialize;
pub mod deserialize;
mod util;

#[derive(Debug,Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub description: String,
}

#[derive(Debug,Clone,Copy)]
pub enum ErrorKind {
    Failed,
    Disconnected,
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match self.kind {
            ErrorKind::Failed => {
                write!(f, "Failed: {}", self.description)
            }
            ErrorKind::Disconnected => {
                write!(f, "Disconnected: {}", self.description)
            }
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }
}

impl Error {
    pub fn failed(description: String) -> Error {
        Error {
            description: description,
            kind: ErrorKind::Failed,
        }
    }

    pub fn disconnected(description: String) -> Error {
        Error {
            description: description,
            kind: ErrorKind::Disconnected,
        }
    }
}

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
    Attack,
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
        pv: u64,
    },
    ThisIsYou {
        entity: u64,
    },
    Response {
        code: ErrorCode,
    },
    EntityHasQuit {
        entity: u64,
    },
    NewEntity {
        entity: u64,
        position: Vec2d,
        skin: u64,
        pv: u64,
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

    pub fn position(id: u64, position: Vec2d, speed: Vec2d, pv: u64) -> Notification {
        Notification::Position {
            entity: id,
            position: position,
            speed: speed,
            pv: pv,
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

    pub fn entity_has_quit(id: u64) -> Notification {
        Notification::EntityHasQuit {
            entity: id,
        }
    }

    pub fn new_entity(id: u64, position: Vec2d, skin: u64, pv: u64) -> Notification {
        Notification::NewEntity {
            entity: id,
            position: position,
            skin: skin,
            pv: pv,
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
