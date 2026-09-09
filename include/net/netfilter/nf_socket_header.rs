/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <net/sock.h> are supplied externally.

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn nf_sk_lookup_slow_v4(
        net: *mut net,
        skb: *const sk_buff,
        indev: *const net_device,
    ) -> *mut sock;

    pub fn nf_sk_lookup_slow_v6(
        net: *mut net,
        skb: *const sk_buff,
        indev: *const net_device,
    ) -> *mut sock;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
