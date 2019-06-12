//! The ``network`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the networking functions found in ``network.h``.

/// Socket Types
bitflags! {
    pub struct SocketType: u32 {
        const SOCK_STREAM = 1;
        const SOCK_DGRAM  = 2;
        const SOCK_RAW    = 3;
    }
}

/// Socket Error Codes
bitflags! {
    pub struct SocketError: i32 {
        const INVALID_SOCKET = !0;
        const SOCKET_ERROR   = -1;
    }
}

/// Optional flags for sockets.
bitflags! {
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

/// Additional socket options.
bitflags! {
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

/// Socket Levels
bitflags! {
    pub struct SocketLevel: u32 {
        const SOL_SOCKET = 0xffff;
    }
}

/// Address Families
bitflags! {
    pub struct AddressFamily: u32 {
        const AF_UNSPEC	= 0;
        const AF_INET	= 2;
        const PF_INET	= AF_INET;
        const PF_UNSPEC	= AF_UNSPEC;
    }
}

/// IP Protocols
bitflags! {
    pub struct IPProtocol: u32 {
        const IPPROTO_IP  = 0;
        const IPPROTO_TCP = 6;
        const IPPROTO_UDP = 17;
    }
}

/// Incoming Address Routing
bitflags! {
    pub struct AddressRouting: u32 {
        const INADDR_ANY        = 0;
        const INADDR_BROADCAST  = 0xffffffff;
    }
}

/// IP Protocol Levels
bitflags! {
    pub struct IPLevels: u32 {
        const IP_TOS = 1;
        const IP_TTL = 2;
    }
}

/// Definitions for IP precedence
bitflags! {
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

/// IPV4 ToS Bits
bitflags! {
    pub struct TOSBits: u32 {
        const IPTOS_TOS_MASK    = 0x1E;
        const IPTOS_LOWDELAY    = 0x10;
        const IPTOS_THROUGHPUT  = 0x08;
        const IPTOS_RELIABILITY = 0x04;
        const IPTOS_LOWCOST     = 0x02;
        const IPTOS_MINCOST     = IPTOS_LOWCOST;
    }
}

/// Ioctl Commands
bitflags! {
    pub struct IoctlCommands: u32 {
        const IOCPARM_MASK = 0x7f;
        const IOC_VOID     = 0x20000000;
        const IOC_OUT      = 0x40000000;
        const IOC_IN       = 0x80000000;
        const IOC_INOUT    = (IOC_IN | IOC_OUT);
    }
}

/// Socket Poll Results
bitflags! {
    pub struct SocketPoll: u32 {
        const POLLIN   = 0x0001;
        const POLLPRI  = 0x0002;
        const POLLOUT  = 0x0004;
        const POLLERR  = 0x0008;
        const POLLHUP  = 0x0010;
        const POLLNVAL = 0x0020;
    }
}

/// Non Blocking IO
pub const O_NONBLOCK: u32 = 2048;

/// Send and Recieve Flags
pub const MSG_DONTWAIT: u32 = 0x40;

/// Socket TCP Options
pub const TCP_NODELAY: u32 = 0x01;
pub const TCP_KEEPALIVE: u32 = 0x02;
