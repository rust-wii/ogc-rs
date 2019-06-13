//! The ``network`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the networking functions found in ``network.h``.

use crate::{OgcError, Result, bitflags};

bitflags! {
    /// Optional flags for sockets.
    pub struct SocketFlags: i32 {
        const SO_DEBUG        = 0x0001;
        const SO_ACCEPTCONN   = 0x0002;
        const SO_REUSEADDR    = 0x0004;
        const SO_KEEPALIVE    =	0x0008;
        const SO_DONTROUTE	  = 0x0010;
        const SO_BROADCAST	  = 0x0020;
        const SO_USELOOPBACK  = 0x0040;
        const SO_LINGER		  = 0x0080;
        const SO_OOBINLINE	  = 0x0100;
        const SO_REUSEPORT	  = 0x0200;
        const SO_DONTLINGER   = !0x0080;
    }
}

bitflags! {
    /// Additional socket options.
    pub struct SocketOptions: u32 {
        const SO_SNDBUF    = 0x1001;
        const SO_RCVBUF    = 0x1002;
        const SO_SNDLOWAT  = 0x1003;
        const SO_RCVLOWAT  = 0x1004;
        const SO_SNDTIMEO  = 0x1005;
        const SO_RCVTIMEO  = 0x1006;
        const SO_ERROR     = 0x1007;
        const SO_TYPE      = 0x1008;
    }
}

bitflags! {
    /// Incoming Address Routing
    pub struct AddressRouting: u32 {
        const INADDR_ANY        = 0;
        const INADDR_BROADCAST  = 0xffffffff;
    }
}

bitflags! {
    /// Definitions for IP precedence
    pub struct IPPrecedence: u32 {
        const IPTOS_PREC_MASK            = 0xe0;
        const IPTOS_PREC_NETCONTROL      = 0xe0;
        const IPTOS_PREC_INTERNETCONTROL = 0xc0;
        const IPTOS_PREC_CRITIC_ECP      = 0xa0;
        const IPTOS_PREC_FLASHOVERRIDE   = 0x80;
        const IPTOS_PREC_FLASH           = 0x60;
        const IPTOS_PREC_IMMEDIATE       = 0x40;
        const IPTOS_PREC_PRIORITY        = 0x20;
        const IPTOS_PREC_ROUTINE         = 0x00;
    }
}

bitflags! {
    /// IPV4 ToS Bits
    pub struct TOSBits: u32 {
        const IPTOS_TOS_MASK    = 0x1E;
        const IPTOS_LOWDELAY    = 0x10;
        const IPTOS_THROUGHPUT  = 0x08;
        const IPTOS_RELIABILITY = 0x04;
        const IPTOS_LOWCOST     = 0x02;
        const IPTOS_MINCOST     = 0x02;
    }
}

bitflags! {
    /// Ioctl Commands
    pub struct IoctlCommands: u32 {
        const IOCPARM_MASK = 0x7f;
        const IOC_VOID     = 0x20000000;
        const IOC_OUT      = 0x40000000;
        const IOC_IN       = 0x80000000;
        const IOC_INOUT    = (0x80000000 | 0x40000000);
    }
}

/// Non Blocking IO
pub const O_NONBLOCK: u32 = 2048;

/// Send and Recieve Flags
pub const MSG_DONTWAIT: u32 = 0x40;

/// Socket TCP Options
pub const TCP_NODELAY: u32 = 0x01;
pub const TCP_KEEPALIVE: u32 = 0x02;

/// Socket Error Codes
pub const INVALID_SOCKET: i32 = !0;
pub const SOCKET_ERROR: i32   = -1;

/// Socket Types
pub const SOCK_STREAM: u32 = 1;
pub const SOCK_DGRAM: u32  = 2;
pub const SOCK_RAW: u32    = 3;

/// Socket Levels
pub const SOL_SOCKET: u32 = 0xffff;

/// IP Protocols
pub const IPPROTO_IP: u32  = 0;
pub const IPPROTO_TCP: u32 = 6;
pub const IPPROTO_UDP: u32 = 17;

/// IP Protocol Levels
pub const IP_TOS: u32 = 1;
pub const IP_TTL: u32 = 2;

/// Address Families
pub const AF_UNSPEC: u32 = 0;
pub const AF_INET: u32   = 2;

/// Poll Results
pub const POLLIN: u32   = 0x0001;
pub const POLLPRI: u32  = 0x0002;
pub const POLLOUT: u32  = 0x0004;
pub const POLLERR: u32  = 0x0008;
pub const POLLHUP: u32  = 0x0010;
pub const POLLNVAL: u32 = 0x0020;

/// Structure to hold the 32 bit address needed for ``sockaddr``.
pub struct IPV4Address {
    // A 32-bit IP address in Network Byte Order.
    pub address: u32,
}

/// Implementation to convert from a ``IPV4Address`` into a ``in_addr`` used by ogc_sys.
impl Into<ogc_sys::in_addr> for &mut IPV4Address {
    fn into(self) -> ogc_sys::in_addr { 
        ogc_sys::in_addr {
            s_addr: self.address
        }
    }
}

impl Into<*mut ogc_sys::in_addr> for &mut IPV4Address {
    fn into(self) -> *mut ogc_sys::in_addr { 
        Box::into_raw(Box::new(ogc_sys::in_addr {
            s_addr: self.address
        }))
    }
}

/// Structure to represent the address elements.
pub struct AddressElements {
    // Length of the address.
    pub length: u8,
    // The address family.
    pub family: u32,
    // A 16-bit port number in Network Byte Order.
    pub port: u16,
    // IPV4 Address.
    pub addr: IPV4Address,
}

/// Structure to represent the socket address.
pub struct SocketAddress {
    // Length of the address.
    pub length: u8,
    // The address family.
    pub family: u32,
    // The address data.
    pub data: [i8; 14]
}

/// This function converts the specified string in the Internet standard dot notation 
/// to an integer value suitable for use as an Internet address. 
/// The converted address will be in Network Byte Order.
pub fn dot_to_nbo(dot: &str) -> Result<IPV4Address> {
    unsafe {
        let r = ogc_sys::inet_addr(dot.as_ptr() as *const u8);

        if r == 0 {
            Err(OgcError::Network("network dot_to_nbo failed".to_string()))
        } else {
            Ok(IPV4Address {
                address: r
            })
        }
    }
}

/// This function call converts the specified string in the Internet standard dot notation 
/// to a network address, and stores the address in the structure provided. 
/// The converted address will be in Network Byte Order.
pub fn dot_to_net_addr(dot: &str, addr: &mut IPV4Address) -> Result<()> {
    unsafe {
        let r = ogc_sys::inet_aton(dot.as_ptr() as *const u8, addr.into());
        
        if r < 0 {
            Err(OgcError::Network(format!("network dot_to_net_addr: {}", r)))
        } else {
            Ok(())
        }
    }
}

/// This function call converts the specified Internet host address 
/// to a string in the Internet standard dot notation.
pub fn addr_to_dot(addr: &mut IPV4Address) -> Result<String> {
    unsafe {
        // TODO: FIX THIS MESS
        let r = ogc_sys::inet_ntoa(addr.into());
        let r = std::slice::from_raw_parts(r, 1);
        let r = String::from_utf8(r.to_vec()).unwrap();
        
        if r.is_empty() {
            Err(OgcError::Network("addr_to_dot empty".to_string()))
        } else {
            Ok(r)
        }
    }
}

/// Represents the networking service.
/// No networking can be done until an instance of this struct is created.
/// This service can only be created once!
///
/// The service exits when all instances of this struct go out of scope. 
pub struct Network(());

/// Implementation of the networking service.
impl Network {

    /// Initialization of the networking service.
    pub fn init() -> Result<Network> {
        unsafe {
            let r = ogc_sys::net_init();

            if r < 0 {
                Err(OgcError::Network(format!("network init: {}", r)))
            } else {
                Ok(Network(()))
            }
        }
    }
}