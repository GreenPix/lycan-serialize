#[cfg(not(feature="json"))]
mod capnp {
    use std::io::{Write,Error};

    use capnp::serialize;
    use capnp::message::{Builder,Allocator};
    use byteorder::{LittleEndian, WriteBytesExt};

    use super::super::{Order,EntityOrder,Notification,GameCommand};

    use commands_capnp::command::Builder as CommandBuilder;
    use notifications_capnp::notification::Builder as NotifBuilder;

    fn serialize_capnp<A,T>(writer: &mut T, message: &Builder<A>) -> Result<(),Error>
        where A: Allocator,
              T: Write {
                  let size = serialize::compute_serialized_size_in_words(message) * 8;
                  try!(writer.write_u64::<LittleEndian>(size as u64));

                  serialize::write_message(writer, message)
              }

    impl EntityOrder {
        pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<(),Error> {
            let mut builder = Builder::new_default();
            {
                let mut root = builder.init_root::<CommandBuilder>().init_entity_order();
                root.set_entity(self.entity);
                match self.order {
                    Order::Walk(ref direction) => {
                        root.set_walk(direction.clone().into());
                    }
                    Order::Say(ref message) => {
                        root.set_say(message);
                    }
                }
            }
            serialize_capnp(writer, &builder)
        }
    }

    impl GameCommand {
        pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<(),Error> {
            let mut builder = Builder::new_default();
            {
                let mut root = builder.init_root::<CommandBuilder>().init_game_command();
                match *self {
                    GameCommand::Disconnect => {
                        root.set_disconnect(());
                    }
                    GameCommand::Authenticate(ref token) => {
                        let mut builder = root.init_authenticate();
                        builder.set_data0(token.data0);
                    }
                }
            }
            serialize_capnp(writer, &builder)
        }
    }

    impl Notification {
        pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<(),Error> {
            let mut builder = Builder::new_default();
            {
                let root = builder.init_root::<NotifBuilder>();
                match *self {
                    Notification::Position{entity, position: Pnt2{x, y}} => {
                        let mut builder = root.init_entity_position();
                        builder.set_id(entity);
                        let mut position = builder.init_position();
                        position.set_x(x);
                        position.set_y(y);
                    }
                    Notification::Say{entity, ref message} => {
                        let mut builder = root.init_entity_say();
                        builder.set_id(entity);
                        builder.set_message(&message);
                    }
                    Notification::Walk{entity, orientation} => {
                        let mut builder = root.init_entity_walk();
                        builder.set_id(entity);
                        builder.set_orientation(orientation.into());
                    }
                    Notification::ThisIsYou{entity} => {
                        let mut builder = root.init_this_is_you();
                        builder.set_id(entity);
                    }
                    Notification::Response{code} => {
                        let mut builder = root.init_response();
                        builder.set_code(code.into());
                    }
                }
            }
            serialize_capnp(writer, &builder)
        }
    }
}

#[cfg(feature="json")]
mod json {
    use std::io::{Write,Error};

    use byteorder::{LittleEndian, WriteBytesExt};
    use serde::Serialize;
    use serde_json;

    use super::super::{EntityOrder,Notification,GameCommand,Command};

    fn serialize_json<T: Serialize,W: Write>(object: &T, writer: &mut W) -> Result<(),Error> {
        // Not really zero-copy ...
        let json = serde_json::to_string(object).unwrap();
        let size = json.len();
        try!(writer.write_u64::<LittleEndian>(size as u64));
        trace!("Writing {}", json);
        write!(writer, "{}", json)
    }

    impl EntityOrder {
        pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<(),Error> {
            let command = Command::EntityOrder(self.clone());
            serialize_json(&command, writer)
        }
    }

    impl GameCommand {
        pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<(),Error> {
            let command = Command::GameCommand(self.clone());
            serialize_json(&command, writer)
        }
    }

    impl Notification {
        pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<(),Error> {
            serialize_json(self, writer)
        }
    }
}
