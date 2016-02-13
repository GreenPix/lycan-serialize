#[cfg(not(feature="json"))]
mod capnp {
    use std::io::Read;

    use capnp::Error;
    use capnp::message::{Reader,ReaderOptions};
    use capnp::serialize;

    use super::super::{Notification,EntityOrder,Command,GameCommand,Order,AuthenticationToken};
    use notifications_capnp::notification;
    use commands_capnp::{command,game_command,entity_order};

    impl Notification {
        pub fn deserialize<R: Read>(reader: &mut R, _size: u64) -> Result<Notification,Error> {
            let options = ReaderOptions::new();
            let message_reader = try!(serialize::read_message(reader, options));
            let root = try!(message_reader.get_root::<notification::Reader>());
            match try!(root.which()) {
                notification::Which::EntityWalk(walk) => {
                    deserialize_walk(walk)
                }
                notification::Which::EntityPosition(position) => {
                    deserialize_position(position)
                }
                notification::Which::ThisIsYou(id) => {
                    deserialize_this_is_you(id)
                }
                notification::Which::Response(response) => {
                    let code = try!(response.get_code());
                    Ok(Notification::response(code.into()))
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

    fn deserialize_position(reader: notification::entity_position::Reader) -> Result<Notification,Error> {
        let id = reader.get_id();
        let position = try!(reader.get_position());
        let x = position.get_x();
        let y = position.get_y();
        Ok(Notification::position(id, Pnt2::new(x, y)))
    }

    impl Command {
        pub fn deserialize<R: Read>(reader: &mut R, _size: u64) -> Result<Command,Error> {
            let options = ReaderOptions::new();
            let message_reader = try!(serialize::read_message(reader, options));
            let root = try!(message_reader.get_root::<command::Reader>());
            match try!(root.which()) {
                command::Which::GameCommand(a) => {
                    let game_command = try!(deserialize_game_command(try!(a)));
                    Ok(Command::GameCommand(game_command))
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
                Order::Say(try!(a).to_string())
            }
        };
        Ok(EntityOrder::new(entity, order))
    }

    fn deserialize_game_command(reader: game_command::Reader) -> Result<GameCommand,Error> {
        match try!(reader.which()) {
            game_command::Which::Disconnect(()) => {
                Ok(GameCommand::Disconnect)
            }
            game_command::Which::Authenticate(a) => {
                let t = try!(a);
                let data0 = t.get_data0();
                let token = AuthenticationToken::new(data0);
                Ok(GameCommand::Authenticate(token))
            }
        }
    }
}

#[cfg(feature="json")]
mod json {
    use std::io::Read;

    use Error;
    use rustc_serialize::Decodable;
    use rustc_serialize::json::{Json,Decoder,DecoderError,BuilderError};

    use super::super::{Notification,Command};

    fn deserialize_json<T: Decodable,R: Read>(reader: &mut R, size: u64) -> Result<T,Error> {
        let mut take = reader.take(size);
        trace!("Building JSON");
        let json = try!(Json::from_reader(&mut take).map_err(convert_builder_err));
        trace!("Deserializing {}", json);
        let mut decoder = Decoder::new(json);
        T::decode(&mut decoder).map_err(convert_decoder_err)
    }

    impl Notification {
        pub fn deserialize<R: Read>(reader: &mut R, size: u64) -> Result<Notification,Error> {
            deserialize_json(reader, size)
        }
    }

    impl Command {
        pub fn deserialize<R: Read>(reader: &mut R, size: u64) -> Result<Command,Error> {
            deserialize_json(reader, size)
        }
    }

    fn convert_decoder_err(e: DecoderError) -> Error {
        match e {
            DecoderError::EOF => Error::disconnected("DecoderError: EOF".into()),
            other => Error::failed(format!("JSON deserialization error: {}", other)),
        }
    }

    fn convert_builder_err(e: BuilderError) -> Error {
        Error::failed(format!("JSON building error: {}", e))
    }
}
