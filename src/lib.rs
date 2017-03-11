#[macro_use] extern crate log;
extern crate byteorder;
#[cfg(feature="json")] extern crate serde;
#[cfg(feature="json")] extern crate serde_json;
#[cfg(feature="json")] #[macro_use] extern crate serde_derive;
extern crate uuid;

use uuid::Uuid;

mod serialize;
pub mod deserialize;
mod util;
mod defs;

pub use defs::*;

// Hack, to be removed later
pub fn forge_authentication_tokens() -> Vec<(Uuid,AuthenticationToken)> {
    (0..100).map(|i| {
        let uuid = Uuid::from_fields(i, 0, 0, &[0,0,0,0,0,0,0,0]).unwrap();
        (uuid, AuthenticationToken(i.to_string()))
    }).collect()
}
