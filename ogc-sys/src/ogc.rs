mod bindings {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
