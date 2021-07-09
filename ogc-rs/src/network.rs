//! The ``network`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the networking functions found in ``network.h``.

use crate::{bitflags, raw_to_string, raw_to_strings, OgcError, Result};
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::{ffi::c_void, ptr};
use num_enum::IntoPrimitive;

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

bitflags! {
    /// Bits that may be set/returned in events and revents from net_poll
    pub struct PollBits: u32 {
        const POLLIN   = 0x0001;
        const POLLPRI  = 0x0002;
        const POLLOUT  = 0x0004;
        const POLLERR  = 0x0008;
        const POLLHUP  = 0x0010;
        const POLLNVAL = 0x0020;
    }
}

/// Protocol Families
#[derive(IntoPrimitive, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ProtocolFamily {
    AfUnspec = 0,
    AfInet = 2,
}

/// Socket Types
#[derive(IntoPrimitive, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum SocketType {
    SockStream = 1,
    SockDgram = 2,
    SockRaw = 3,
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
pub const SOCKET_ERROR: i32 = -1;

/// Socket Levels
pub const SOL_SOCKET: u32 = 0xffff;

/// IP Protocols
pub const IPPROTO_IP: u32 = 0;
pub const IPPROTO_TCP: u32 = 6;
pub const IPPROTO_UDP: u32 = 17;

/// IP Protocol Levels
pub const IP_TOS: u32 = 1;
pub const IP_TTL: u32 = 2;

/// Structure to hold the 32 bit address needed for ``sockaddr``.
pub struct IPV4Address {
    // A 32-bit IP address in Network Byte Order.
    pub address: u32,
}

/// Implementation to convert from a ``IPV4Address`` into a ``in_addr`` used by ogc_sys.
impl Into<ogc_sys::in_addr> for &mut IPV4Address {
    fn into(self) -> ogc_sys::in_addr {
        ogc_sys::in_addr {
            s_addr: self.address,
        }
    }
}

impl Into<*mut ogc_sys::in_addr> for &mut IPV4Address {
    fn into(self) -> *mut ogc_sys::in_addr {
        Box::into_raw(Box::new(ogc_sys::in_addr {
            s_addr: self.address,
        }))
    }
}

/// Structure to represent the address elements.
pub struct AddressElements {
    // Length of the address.
    pub length: u8,
    // The address family.
    pub family: ProtocolFamily,
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
    pub family: ProtocolFamily,
    // The address data.
    pub data: [i8; 14],
}

/// Convert ``SocketAddress`` into a ``ogc_sys::sockaddr``.
impl Into<*mut ogc_sys::sockaddr> for SocketAddress {
    fn into(self) -> *mut ogc_sys::sockaddr {
        // TODO: Check implementation.
        let sa_family: u32 = self.family.into();
        Box::into_raw(Box::new(ogc_sys::sockaddr {
            sa_len: self.length,
            sa_family: sa_family as u8,
            sa_data: self.data,
        }))
    }
}

/// The hostent structure is used by functions to store
/// information about a given host, such as host name and IPv4 address.
#[derive(Debug, Clone)]
pub struct HostInformation {
    /// The official name of the host.
    pub name: String,
    /// A NULL-terminated array of alternate names.
    pub aliases: Vec<String>,
    /// The type of address being returned.
    pub address_type: u16,
    /// The length, in bytes, of each address.
    pub length: u16,
    /// A NULL-terminated list of addresses for the host.
    pub address_list: Vec<String>,
}

/// This function converts the specified string in the Internet standard dot notation
/// to an integer value suitable for use as an Internet address.
/// The converted address will be in Network Byte Order.
pub fn dot_to_nbo(dot: &str) -> Result<IPV4Address> {
    let r = unsafe { ogc_sys::inet_addr(dot.as_ptr()) };

    if r == 0 {
        Err(OgcError::Network("network dot_to_nbo failed".to_string()))
    } else {
        Ok(IPV4Address { address: r })
    }
}

/// This function call converts the specified string in the Internet standard dot notation
/// to a network address, and stores the address in the structure provided.
/// The converted address will be in Network Byte Order.
pub fn dot_to_net_addr(dot: &str, addr: &mut IPV4Address) -> Result<()> {
    let r = unsafe { ogc_sys::inet_aton(dot.as_ptr(), addr.into()) };

    if r < 0 {
        Err(OgcError::Network(format!("network dot_to_net_addr: {}", r)))
    } else {
        Ok(())
    }
}

/// This function call converts the specified Internet host address
/// to a string in the Internet standard dot notation.
pub fn addr_to_dot(addr: &mut IPV4Address) -> Result<String> {
    let r = unsafe { ogc_sys::inet_ntoa(addr.into()) };
    let r = raw_to_string(r);

    if r.is_empty() {
        Err(OgcError::Network("addr_to_dot empty".to_string()))
    } else {
        Ok(r)
    }
}

/// This function returns a structure of type ``HostInformation`` for the given host name.
/// Here ``addr_string`` is either a hostname, or an IPv4 address in standard dot notation.
pub fn get_host_by_name(addr_string: &str) -> Result<HostInformation> {
    unsafe {
        let r = ogc_sys::net_gethostbyname(addr_string.as_ptr());

        if r == ptr::null_mut() {
            Err(OgcError::Network("host provided doesnt exist".into()))
        } else {
            Ok(HostInformation {
                name: raw_to_string((*r).h_name),
                aliases: raw_to_strings((*r).h_aliases),
                address_type: (*r).h_addrtype,
                length: (*r).h_length,
                address_list: raw_to_strings((*r).h_addr_list),
            })
        }
    }
}

/// Represents the networking service.
/// No networking can be done until an instance of this struct is created.
/// This service can only be created once!
///
/// The service exits when all instances of this struct go out of scope.
pub struct Network;

/// Implementation of the networking service.
impl Network {
    /// Initialization of the networking service.
    pub fn init() -> Result<Self> {
        let r = unsafe { ogc_sys::net_init() };

        if r < 0 {
            Err(OgcError::Network(format!("network init: {}", r)))
        } else {
            Ok(Self)
        }
    }

    /// Create a socket.
    pub fn new(domain: ProtocolFamily, socket_type: SocketType) -> Result<Socket> {
        let r = unsafe { ogc_sys::net_socket(domain.into(), socket_type.into(), 0) };

        if r == INVALID_SOCKET {
            Err(OgcError::Network(format!("network socket creation: {}", r)))
        } else {
            Ok(Socket(r))
        }
    }
}

/// Represents a unix socket.
/// No networking can be done until an instance of ``Network`` is created.
///
/// The socket closes when this struct go out of scope.
pub struct Socket(i32);

impl Socket {
    /// Initiate a connection on a socket.
    pub fn connect(&self, socket_addr: SocketAddress, address_length: u32) -> Result<()> {
        let r = unsafe { ogc_sys::net_connect(self.0, socket_addr.into(), address_length) };

        if r < 0 {
            Err(OgcError::Network(format!("network socket connect: {}", r)))
        } else {
            Ok(())
        }
    }

    /// Assign a local protocol address to a socket.
    pub fn bind(&self, socket_addr: SocketAddress, address_length: u32) -> Result<()> {
        let r = unsafe { ogc_sys::net_bind(self.0, socket_addr.into(), address_length) };

        if r < 0 {
            Err(OgcError::Network(format!("network socket bind: {}", r)))
        } else {
            Ok(())
        }
    }

    /// This function is called only by a TCP server to listen for the client request.
    pub fn listen(&self, backlog: u32) -> Result<()> {
        let r = unsafe { ogc_sys::net_listen(self.0, backlog) };

        if r < 0 {
            Err(OgcError::Network(format!("network socket listen: {}", r)))
        } else {
            Ok(())
        }
    }

    /// The accept function is called by a TCP server to accept client requests and
    /// to establish actual connection.
    pub fn accept(&self, socket_addr: SocketAddress, address_length: &mut u32) -> Result<i32> {
        let r = unsafe { ogc_sys::net_accept(self.0, socket_addr.into(), address_length) };

        if r < 0 {
            Err(OgcError::Network(format!("network socket accept: {}", r)))
        } else {
            Ok(r)
        }
    }

    /// Write to the file descriptor, in this case the socket.
    pub fn write(descriptor: i32, buffer: &[u8], count: i32) -> Result<i32> {
        let r = unsafe { ogc_sys::net_write(descriptor, buffer.as_ptr() as *const c_void, count) };

        if r < 0 {
            Err(OgcError::Network(format!("network writing failure: {}", r)))
        } else {
            Ok(r)
        }
    }

    /// Send data over stream sockets or CONNECTED datagram sockets.
    pub fn send(descriptor: i32, buffer: &[u8], length: i32, flags: u32) -> Result<i32> {
        let r = unsafe {
            ogc_sys::net_send(descriptor, buffer.as_ptr() as *const c_void, length, flags)
        };

        if r < 0 {
            Err(OgcError::Network(format!("network sending failure: {}", r)))
        } else {
            Ok(r)
        }
    }

    /// Read from the file descriptor, in this case the socket.
    pub fn read(descriptor: i32, buffer: &mut [u8], count: i32) -> Result<i32> {
        let r = unsafe { ogc_sys::net_read(descriptor, buffer.as_ptr() as *mut c_void, count) };

        if r < 0 {
            Err(OgcError::Network(format!("network reading failure: {}", r)))
        } else {
            Ok(r)
        }
    }

    /// Receive data over stream sockets or CONNECTED datagram sockets.
    pub fn recieve(descriptor: i32, buffer: &mut [u8], length: i32, flags: u32) -> Result<i32> {
        let r =
            unsafe { ogc_sys::net_recv(descriptor, buffer.as_ptr() as *mut c_void, length, flags) };

        if r < 0 {
            Err(OgcError::Network(format!("network recieve failure: {}", r)))
        } else {
            Ok(r)
        }
    }
}

/// Implements ``Drop`` for ``Socket`.
/// On ``drop`` it closes the file descriptor.
impl Drop for Socket {
    fn drop(&mut self) {
        unsafe {
            ogc_sys::net_close(self.0);
        }
    }
}
