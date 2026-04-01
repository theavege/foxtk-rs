#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
#![allow(non_snake_case)]
#![allow(unnecessary_transmutes)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::type_complexity)]
#![allow(clippy::approx_constant)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
