/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the included Linux networking headers.
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn nf_send_unreach(skb_in: *mut sk_buff, code: ::core::ffi::c_int, hook: ::core::ffi::c_int);
    pub fn nf_send_reset(
        net: *mut net,
        sock: *mut sock,
        oldskb: *mut sk_buff,
        hook: ::core::ffi::c_int,
    );
    pub fn nf_reject_skb_v4_unreach(
        net: *mut net,
        oldskb: *mut sk_buff,
        dev: *const net_device,
        hook: ::core::ffi::c_int,
        code: u8,
    ) -> *mut sk_buff;
    pub fn nf_reject_skb_v4_tcp_reset(
        net: *mut net,
        oldskb: *mut sk_buff,
        dev: *const net_device,
        hook: ::core::ffi::c_int,
    ) -> *mut sk_buff;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
