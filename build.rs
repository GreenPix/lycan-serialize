#[cfg(capnpc)]
extern crate capnpc;
#[cfg(capnpc)]
mod capnproto {
    use std::fs;
    use std::env;
    use std::path::Path;

    const FILES: [&'static str; 3] = [
        "schemas/commands.capnp",
        "schemas/notifications.capnp",
        "schemas/common.capnp",
    ];
    fn compile() {
        let out_dir = &env::var("OUT_DIR").unwrap();
        let _ = fs::create_dir(&Path::new(out_dir).join("schemas"));
        capnpc::compile("schemas", &FILES).unwrap();
    }
}
#[cfg(capnpc)]
fn main() {
    capnproto::compile();
}

#[cfg(not(capnpc))]
fn main () {}
