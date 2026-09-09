/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to <linux/skbuff.h> is supplied externally.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct crypto_ahash {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_auth_hdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ah_data {
    pub icv_full_len: c_int,
    pub icv_trunc_len: c_int,
    pub ahash: *mut crypto_ahash,
}

unsafe extern "C" {
    pub fn skb_transport_header(skb: *const sk_buff) -> *mut c_void;
}

#[inline]
pub unsafe fn ip_auth_hdr(skb: *const sk_buff) -> *mut ip_auth_hdr {
    unsafe { skb_transport_header(skb) as *mut ip_auth_hdr }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
