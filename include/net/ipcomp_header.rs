/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/skbuff.h>

#[repr(C)]
pub struct ip_comp_hdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfrm_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

extern "C" {
    pub fn ipcomp_input(x: *mut xfrm_state, skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn ipcomp_output(x: *mut xfrm_state, skb: *mut sk_buff) -> ::core::ffi::c_int;
    pub fn ipcomp_destroy(x: *mut xfrm_state);
    pub fn ipcomp_init_state(
        x: *mut xfrm_state,
        extack: *mut netlink_ext_ack,
    ) -> ::core::ffi::c_int;

    // Supplied by <linux/skbuff.h>.
    pub fn skb_transport_header(skb: *const sk_buff) -> *mut u8;
}

pub unsafe fn ip_comp_hdr(skb: *const sk_buff) -> *mut ip_comp_hdr {
    skb_transport_header(skb) as *mut ip_comp_hdr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
