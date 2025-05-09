//! The ``debug`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the debug functions.

use crate::ffi;

/// Default EXI channel for use in [`debug_init()`]. Channel can be 0 or 1.
/// 
/// **Note**: Used for device type USBGecko
pub const DEF_EXICHAN: u32 = ffi::GDBSTUB_DEF_CHANNEL;

/// Default TCP port for use in [`debug_init()`].
/// 
/// **Note**: Used for device type TCP.
pub const DEF_TCPPORT: u32 = ffi::GDBSTUB_DEF_TCPPORT;

/// Enum for gdb stub types.
#[derive(Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum GDBStubDevice {
    /// device type: USBGecko
    Usb = ffi::GDBSTUB_DEVICE_USB,
    /// device type: BBA-TCP
    Tcp = ffi::GDBSTUB_DEVICE_TCP,
}

/// Performs the initialization of the debug stub.
///
/// * `device_type`: type of device to use. can be either USB or TCP.
/// * `channel_port`: depending on the used device this can be either the EXI
///   channel or the TCP port.
pub fn debug_init(device_type: GDBStubDevice, channel_port: u32) {
    unsafe {
        ffi::DEBUG_Init(device_type as _, channel_port as i32);
    }
}

/// Stub function to insert the hardware break instruction.
///
/// This function is used to enter the debug stub and to connect with the host.
/// The developer is free to insert this function at any position in project's
/// source code.
pub fn insert_break() {
    unsafe {
        ffi::_break();
    }
}
