use std;
use std::fmt;

use uuid::Uuid;

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

#[cfg_attr(feature="json",derive(Serialize,Deserialize))]
#[derive(Debug,Clone,Copy)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

#[cfg_attr(feature="json",derive(Serialize,Deserialize))]
#[derive(Debug,Clone,Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg_attr(feature="json",derive(Serialize,Deserialize))]
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

#[cfg_attr(feature="json",derive(Serialize,Deserialize))]
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

#[cfg_attr(feature="json",derive(Serialize,Deserialize))]
#[derive(Debug,Clone)]
pub enum GameCommand {
    Disconnect,
    Authenticate(Uuid, AuthenticationToken),
}

#[cfg_attr(feature="json",derive(Serialize,Deserialize))]
#[derive(Debug,Clone)]
pub enum Command {
    EntityOrder(EntityOrder),
    GameCommand(GameCommand),
}

#[cfg_attr(feature="json",derive(Serialize,Deserialize))]
#[derive(Debug,Clone)]
pub struct EntityUpdate {
    pub entity_id: u64,
    pub position: Vec2d,
    pub speed: Vec2d,
    pub pv: u64,
}

#[cfg_attr(feature="json",derive(Serialize,Deserialize))]
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
    GameUpdate {
        tick_id: u64,
        entities: Vec<EntityUpdate>,
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
        nominal_speed: f32,
    },
    Damage {
        source: u64,
        victim: u64,
        amount: u64,
    },
    Death {
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

    pub fn new_entity(id: u64, position: Vec2d, skin: u64, pv: u64, nominal_speed: f32) -> Notification {
        Notification::NewEntity {
            entity: id,
            position: position,
            skin: skin,
            pv: pv,
            nominal_speed: nominal_speed,
        }
    }
}

#[cfg_attr(feature="json",derive(Serialize,Deserialize))]
#[derive(Debug,Clone,Hash,PartialEq,Eq)]
pub struct AuthenticationToken(pub String);

#[cfg_attr(feature="json",derive(Serialize,Deserialize))]
#[derive(Debug,Clone,Copy)]
pub enum ErrorCode {
    Success,
    Error,
}
