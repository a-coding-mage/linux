/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation unit. */

use core::ffi::c_char;

#[allow(non_camel_case_types)]
pub type u8 = core::ffi::c_uchar;

#[allow(non_camel_case_types)]
pub type size_t = usize;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ipv6hdr {
    _private: [u8; 0],
}

/**
 * LOWPAN_NHC - helper macro to generate nh id fields and lowpan_nhc struct
 */
#[macro_export]
macro_rules! LOWPAN_NHC {
    ($nhc:ident, $name:expr, $nexthdr:expr, $hdrlen:expr, $id:expr,
     $idmask:expr, $uncompress:expr, $compress:expr) => {
        static $nhc: $crate::lowpan_nhc = $crate::lowpan_nhc {
            name: $name,
            nexthdr: $nexthdr,
            nexthdrlen: $hdrlen,
            id: $id,
            idmask: $idmask,
            uncompress: $uncompress,
            compress: $compress,
        };
    };
}

/* C token pasting in module_lowpan_nhc is represented by explicit init/exit names. */
#[macro_export]
macro_rules! module_lowpan_nhc {
    ($nhc:ident, $init:ident, $exit:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $init() -> i32 {
            lowpan_nhc_add(&$nhc)
        }
        #[no_mangle]
        pub unsafe extern "C" fn $exit() {
            lowpan_nhc_del(&$nhc);
        }
    };
}

/** struct lowpan_nhc - hold 6lowpan next hdr compression information */
#[repr(C)]
pub struct lowpan_nhc {
    pub name: *const c_char,
    pub nexthdr: u8,
    pub nexthdrlen: size_t,
    pub id: u8,
    pub idmask: u8,
    pub uncompress: Option<unsafe extern "C" fn(*mut sk_buff, size_t) -> i32>,
    pub compress: Option<unsafe extern "C" fn(*mut sk_buff, *mut *mut u8) -> i32>,
}

unsafe extern "C" {
    pub fn lowpan_nhc_by_nexthdr(nexthdr: u8) -> *mut lowpan_nhc;

    pub fn lowpan_nhc_check_compression(
        skb: *mut sk_buff,
        hdr: *const ipv6hdr,
        hc_ptr: *mut *mut u8,
    ) -> i32;

    pub fn lowpan_nhc_do_compression(
        skb: *mut sk_buff,
        hdr: *const ipv6hdr,
        hc_ptr: *mut *mut u8,
    ) -> i32;

    pub fn lowpan_nhc_do_uncompression(
        skb: *mut sk_buff,
        dev: *const net_device,
        hdr: *mut ipv6hdr,
    ) -> i32;

    pub fn lowpan_nhc_add(nhc: *const lowpan_nhc) -> i32;
    pub fn lowpan_nhc_del(nhc: *const lowpan_nhc);
    pub fn lowpan_nhc_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
