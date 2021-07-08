//! The ``debug`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the debug functions.

use num_enum::IntoPrimitive;

/// Enum for gdb stub types.
#[derive(IntoPrimitive, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum GDBStubDevice {
    Usb = 0,
    Tcp = 1,
}

/// Performs the initialization of the debug stub.
pub fn debug_init(device_type: GDBStubDevice, channel_port: i32) {
    unsafe {
        ogc_sys::DEBUG_Init(device_type.into(), channel_port);
    }
}

/// Stub function to insert the hardware break instruction.
/// This function is used to enter the debug stub and to connect with the host.
/// The developer is free to insert this function at any position in project's source code.
pub fn insert_break() {
    unsafe {
        ogc_sys::_break();
    }
}
