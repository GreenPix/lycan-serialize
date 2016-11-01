#[cfg(feature = "capnpc")]
fn main() {
    extern crate capnpc;
    use std::fs;
    use std::env;
    use std::path::Path;

    const FILES: [&'static str; 3] = [
        "schemas/commands.capnp",
        "schemas/notifications.capnp",
        "schemas/common.capnp",
    ];

    let out_dir = &env::var("OUT_DIR").unwrap();
    let _ = fs::create_dir(&Path::new(out_dir).join("schemas"));
    capnpc::compile("schemas", &FILES).unwrap();
}

#[cfg(feature = "json")]
fn main () {
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = Path::new("src/defs.in.rs");
    let dst = Path::new(&out_dir).join("defs.rs");

    serde_codegen::expand(&src, &dst).unwrap();
}
