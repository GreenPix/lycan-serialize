use std::io::{Write,Error};
use capnp::serialize;
use capnp::message::{Allocator,Builder,HeapAllocator};

use messages::{Order,EntityOrder,Direction,Notification,ErrorCode,AuthenticationToken};

use commands_capnp::command::Builder as CommandBuilder;
use authentication_capnp::authentication_token::Builder as AuthBuilder;
use authentication_capnp::response::Builder as ResponseBuilder;
use notifications_capnp::notification::Builder as NotifBuilder;

impl EntityOrder {
    pub fn serialize(&self) -> Builder<HeapAllocator> {
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
        builder
    }
}

impl Notification {
    pub fn serialize(&self) -> Builder<HeapAllocator> {
        let mut builder = Builder::new_default();
        {
            let mut root = builder.init_root::<NotifBuilder>();
            match *self {
                Notification::Location{entity, x, y} => {
                    let mut builder = root.init_entity_location();
                    builder.set_id(entity);
                    let mut location = builder.init_location();
                    location.set_x(x);
                    location.set_y(y);
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
            }
        }
        builder
    }
}

impl ErrorCode {
    pub fn serialize(&self) -> Builder<HeapAllocator> {
        let mut builder = Builder::new_default();
        {
            let mut root = builder.init_root::<ResponseBuilder>();
            let code = self.clone().into();
            root.set_code(code);
        }
        builder
    }
}

impl AuthenticationToken {
    pub fn serialize(&self) -> Builder<HeapAllocator> {
        let mut builder = Builder::new_default();
        {
            let mut root = builder.init_root::<AuthBuilder>();
            root.set_data0(self.data0);
        }
        builder
    }
}
/*
pub fn serialize<T: Write>(writer: &mut T, order: &EntityOrder) -> Result<(),Error> {
    let id = order.target;
    match order.order {
        Order::Walk(ref direction) => serialize_walk(writer, id, direction),
        _ => unimplemented!(),
    }
}

fn serialize_capnp<A,T>(writer: &mut T, message: &mut Builder<A>) -> Result<(),Error>
where A: Allocator,
      T: Write {
    let size = serialize::compute_serialized_size_in_words(message) * 8;
    try!(writer.write_u64::<LittleEndian>(size as u64));

    serialize::write_message(writer, message)
}

fn serialize_walk<T: Write>(writer: &mut T, id: u64, walk: &Option<Direction>) -> Result<(),Error> {
    let mut message_builder = Builder::new_default();
    {
        let mut message = message_builder.init_root::<CommandBuilder>().init_entity_order();
        message.set_origin(id);
        message.set_walk(walk.clone().into());
    }
    serialize_capnp(writer, &mut message_builder)
}

// TODO: Make it 256 bits
pub fn forge_authentication_tokens() -> Vec<AuthenticationToken> {
    (0..30).map(|i| {
        AuthenticationToken {
            data0: i,
        }
    }).collect()
}

pub fn send_authentication_token<T: Write>(writer: &mut T, token: &AuthenticationToken) -> Result<(),Error> {
    let mut message_builder = Builder::new_default();
    {
        let mut message = message_builder.init_root::<AuthBuilder>();
        message.set_data0(token.data0);
    }
    serialize_capnp(writer, &mut message_builder)
}
*/
