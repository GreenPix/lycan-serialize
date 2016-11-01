#[cfg(feature = "json")]
include!(concat!(env!("OUT_DIR"), "/defs.rs"));

#[cfg(not(feature = "json"))]
include!("defs.in.rs");
