mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bootstrap.rs"));
}

pub fn initialize() -> std::io::Result<()> {
    todo!()
}

pub fn shutdown() -> std::io::Result<()> {
    todo!()
}