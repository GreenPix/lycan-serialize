use capnp::Error;
use capnp::message::{Reader,ReaderSegments};

use super::{Notification,EntityOrder,Command,MapCommand,Order};
use notifications_capnp::notification;
use authentication_capnp::error_code;
use commands_capnp::{command,map_command,entity_order};

impl Notification {
    pub fn deserialize<S: ReaderSegments>(message_reader: &Reader<S>) -> Result<Notification,Error> {
        let root = try!(message_reader.get_root::<notification::Reader>());
        match try!(root.which()) {
            notification::Which::EntityWalk(walk) => {
                deserialize_walk(walk)
            }
            notification::Which::EntityLocation(location) => {
                deserialize_location(location)
            }
            notification::Which::ThisIsYou(id) => {
                deserialize_this_is_you(id)
            }
            _ => unimplemented!(),
        }
    }
}

fn deserialize_this_is_you(reader: notification::this_is_you::Reader) -> Result<Notification,Error> {
    let id = reader.get_id();
    Ok(Notification::this_is_you(id))
}

fn deserialize_walk(reader: notification::entity_walk::Reader) -> Result<Notification,Error> {
    let id = reader.get_id();
    let orientation = try!(reader.get_orientation());
    Ok(Notification::walk(id, orientation.into()))
}

fn deserialize_location(reader: notification::entity_location::Reader) -> Result<Notification,Error> {
    let id = reader.get_id();
    let location = try!(reader.get_location());
    let x = location.get_x();
    let y = location.get_y();
    Ok(Notification::location(id, x, y))
}

pub fn deserialize_error_code<S: ReaderSegments>(message_reader: &Reader<S>) -> Result<i64,Error> {
    let root = try!(message_reader.get_root::<error_code::Reader>());
    Ok(root.get_code())
}

impl Command {
    pub fn deserialize<S: ReaderSegments>(message_reader: &Reader<S>) -> Result<Command,Error> {
        let root = try!(message_reader.get_root::<command::Reader>());
        match try!(root.which()) {
            command::Which::MapCommand(a) => {
                let map_command = try!(deserialize_map_command(try!(a)));
                Ok(Command::MapCommand(map_command))
            }
            command::Which::EntityOrder(a) => {
                let entity_order = try!(deserialize_entity_order(try!(a)));
                Ok(Command::EntityOrder(entity_order))
            }
        }
    }
}

fn deserialize_entity_order(reader: entity_order::Reader) -> Result<EntityOrder,Error> {
    let entity = reader.get_entity();
    let order = match try!(reader.which()) {
        entity_order::Which::Walk(a) => {
            let direction = try!(a);
            Order::Walk(direction.into())
        }
        entity_order::Which::Say(a) => {
            unimplemented!();
        }
    };
    Ok(EntityOrder::new(entity, order))
}

fn deserialize_map_command(reader: map_command::Reader) -> Result<MapCommand,Error> {
    unimplemented!();
}
