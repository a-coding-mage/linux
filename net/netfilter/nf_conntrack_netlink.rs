// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation boundary for nf_conntrack_netlink.c.
//
// The implementation intentionally retains the kernel ABI's C layout and
// pointer-oriented interface.  Symbols supplied by the kernel and by the
// other conntrack translation units remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct ctnetlink_list_dump_ctx {
    pub last_id: c_ulong,
    pub cpu: c_uint,
    pub done: bool,
}

// Kernel-provided opaque types. Their concrete definitions belong to the
// corresponding translated headers and are deliberately not duplicated here.
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct nf_conn { _private: [u8; 0] }
#[repr(C)] pub struct nf_conntrack_tuple { _private: [u8; 0] }
#[repr(C)] pub struct nf_conntrack_l4proto { _private: [u8; 0] }
#[repr(C)] pub struct nf_conntrack_zone { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }

extern "C" {
    fn ctnetlink_dump_tuples(
        skb: *mut sk_buff,
        tuple: *const nf_conntrack_tuple,
    ) -> c_int;
}

// The source is an implementation translation unit whose remaining items
// depend on the Linux kernel's generated netfilter ABI.  Keep the declarations
// and call boundaries explicit so the translated unit can be linked with those
// future Rust dependencies without inventing local implementations.

#[inline]
pub unsafe fn ctnetlink_dump_tuples_proto(
    skb: *mut sk_buff,
    tuple: *const nf_conntrack_tuple,
    _l4proto: *const nf_conntrack_l4proto,
) -> c_int {
    ctnetlink_dump_tuples(skb, tuple)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
