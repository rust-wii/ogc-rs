//! The ``debug`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the debug functions.

use crate::ffi;

/// Default EXI channel. channel can be 0 or 1. Note: Used for device type USBGecko
pub const DEF_EXICHAN: i32 = ffi::GDBSTUB_DEF_CHANNEL as _;

/// Default TCP port. Note: Used for device type TCP
pub const DEF_TCPPORT: i32 = ffi::GDBSTUB_DEF_TCPPORT as _;

/// Enum for gdb stub types.
#[derive(Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum GDBStubDevice {
    /// device type: USBGecko
    Usb = ffi::GDBSTUB_DEVICE_USB as _,
    /// device type: BBA-TCP
    Tcp = ffi::GDBSTUB_DEVICE_TCP as _,
}

/// Performs the initialization of the debug stub.
///
/// * `device_type`: type of device to use. can be either USB or TCP.
/// * `channel_port`: depending on the used device this can be either the EXI channel or the TCP port.
pub fn debug_init(device_type: GDBStubDevice, channel_port: i32) {
    unsafe {
        ffi::DEBUG_Init(device_type as _, channel_port);
    }
}

/// Stub function to insert the hardware break instruction.
///
/// This function is used to enter the debug stub and to connect with the host.
/// The developer is free to insert this function at any position in project's source code.
pub fn insert_break() {
    unsafe {
        ffi::_break();
    }
}
