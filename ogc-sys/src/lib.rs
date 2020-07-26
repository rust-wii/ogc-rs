#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(const_fn)]
#![no_std]

include!("ogc.rs");

mod inline;
pub use inline::*;
