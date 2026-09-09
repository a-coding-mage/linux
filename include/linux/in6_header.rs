/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	Types and definitions for AF_INET6
 *	Linux INET6 implementation
 *
 *	Authors:
 *	Pedro Roque		<roque@di.fc.ul.pt>
 *
 *	Sources:
 *	IPv6 Program Interfaces for BSD Systems
 *      <draft-ietf-ipngwg-bsd-api-05.txt>
 *
 *	Advanced Sockets API for IPv6
 *	<draft-stevens-advanced-api-00.txt>
 */

// Dependency intent: the C header includes <uapi/linux/in6.h>, which supplies
// `in6_addr` and `sockaddr_in6`.

/* Large enough to hold both sockaddr_in and sockaddr_in6. */
#[repr(C)]
pub struct sockaddr_inet {
    pub sa_family: u16,
    pub sa_data: [core::ffi::c_char;
        core::mem::size_of::<sockaddr_in6>() - core::mem::size_of::<u16>()],
}

/* IPv6 Wildcard Address (::) and Loopback Address (::1) defined in RFC2553
 * NOTE: Be aware the IN6ADDR_* constants and in6addr_* externals are defined
 * in network byte order, not in host byte order as are the IPv4 equivalents
 */
unsafe extern "C" {
    pub static in6addr_any: in6_addr;
}

pub const IN6ADDR_ANY_INIT: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

unsafe extern "C" {
    pub static in6addr_loopback: in6_addr;
}

pub const IN6ADDR_LOOPBACK_INIT: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
};

unsafe extern "C" {
    pub static in6addr_linklocal_allnodes: in6_addr;
}

pub const IN6ADDR_LINKLOCAL_ALLNODES_INIT: in6_addr = in6_addr {
    s6_addr: [0xff, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
};

unsafe extern "C" {
    pub static in6addr_linklocal_allrouters: in6_addr;
}

pub const IN6ADDR_LINKLOCAL_ALLROUTERS_INIT: in6_addr = in6_addr {
    s6_addr: [0xff, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
};

unsafe extern "C" {
    pub static in6addr_interfacelocal_allnodes: in6_addr;
}

pub const IN6ADDR_INTERFACELOCAL_ALLNODES_INIT: in6_addr = in6_addr {
    s6_addr: [0xff, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
};

unsafe extern "C" {
    pub static in6addr_interfacelocal_allrouters: in6_addr;
}

pub const IN6ADDR_INTERFACELOCAL_ALLROUTERS_INIT: in6_addr = in6_addr {
    s6_addr: [0xff, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
};

unsafe extern "C" {
    pub static in6addr_sitelocal_allrouters: in6_addr;
}

pub const IN6ADDR_SITELOCAL_ALLROUTERS_INIT: in6_addr = in6_addr {
    s6_addr: [0xff, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
