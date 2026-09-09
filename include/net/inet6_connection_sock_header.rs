/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * NET          Generic infrastructure for INET6 connection oriented protocols.
 *
 * Authors:     Many people, see the TCPv6 sources
 *
 *              From code originally in TCPv6
 */

/* C dependency: <linux/types.h> */

#[repr(C)]
pub struct flowi {
    _private: [u8; 0],
}

#[repr(C)]
pub struct flowi6 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct request_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr {
    _private: [u8; 0],
}

/* Referenced by the declarations below; defined by another dependency. */
#[repr(C)]
pub struct dst_entry {
    _private: [u8; 0],
}

extern "C" {
    pub fn inet6_csk_route_socket(
        sk: *mut sock,
        fl6: *mut flowi6,
    ) -> *mut dst_entry;

    pub fn inet6_csk_route_req(
        sk: *const sock,
        dst: *mut dst_entry,
        fl6: *mut flowi6,
        req: *const request_sock,
        proto: u8,
    ) -> *mut dst_entry;

    pub fn inet6_csk_xmit(
        sk: *mut sock,
        skb: *mut sk_buff,
        fl: *mut flowi,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
