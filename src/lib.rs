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

pub mod messages;
