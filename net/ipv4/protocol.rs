// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		INET protocol dispatch tables.
 *
 * Authors:	Ross Biro
 *		Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *
 * Fixes:
 *		Alan Cox	: Ahah! udp icmp errors don't work because
 *				  udp_err is never called!
 *		Alan Cox	: Added new fields for init and ready for
 *				  proper fragmentation (_NO_ 4K limits!)
 *		Richard Colella	: Hang on hash collision
 *		Vince Laviano	: Modified inet_del_protocol() to correctly
 *				  maintain copy bit.
 */

// Symbols and types supplied by the surrounding kernel translation.
pub const MAX_INET_PROTOS: usize = 256;

#[repr(C)]
pub struct net_protocol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_offload {
    _private: [u8; 0],
}

extern "C" {
    fn synchronize_net();
}

#[no_mangle]
pub static mut inet_protos: [*mut net_protocol; MAX_INET_PROTOS] =
    [core::ptr::null_mut(); MAX_INET_PROTOS];

#[no_mangle]
pub static mut inet_offloads: [*const net_offload; MAX_INET_PROTOS] =
    [core::ptr::null(); MAX_INET_PROTOS];

#[no_mangle]
pub unsafe extern "C" fn inet_add_protocol(
    prot: *const net_protocol,
    protocol: u8,
) -> i32 {
    let slot = &mut inet_protos[protocol as usize];
    if (*slot).is_null() {
        *slot = prot as *mut net_protocol;
        0
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn inet_add_offload(
    prot: *const net_offload,
    protocol: u8,
) -> i32 {
    let slot = &mut inet_offloads[protocol as usize];
    if (*slot).is_null() {
        *slot = prot;
        0
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn inet_del_protocol(
    prot: *const net_protocol,
    protocol: u8,
) -> i32 {
    let slot = &mut inet_protos[protocol as usize];
    let ret = if *slot == prot as *mut net_protocol {
        *slot = core::ptr::null_mut();
        0
    } else {
        -1
    };

    synchronize_net();

    ret
}

#[no_mangle]
pub unsafe extern "C" fn inet_del_offload(
    prot: *const net_offload,
    protocol: u8,
) -> i32 {
    let slot = &mut inet_offloads[protocol as usize];
    let ret = if *slot == prot {
        *slot = core::ptr::null();
        0
    } else {
        -1
    };

    synchronize_net();

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
