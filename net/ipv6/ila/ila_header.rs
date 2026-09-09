/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2015 Tom Herbert <tom@herbertland.com>
 */

// Translated from ila.h. C includes and externally supplied kernel symbols are
// intentionally left to the containing translation environment.

#[repr(C)]
pub union ila_locator {
    pub v8: [u8; 8],
    pub v16: [__be16; 4],
    pub v32: [__be32; 2],
    pub v64: __be64,
}

#[repr(C)]
pub struct ila_identifier_bits {
    pub bits: u8,
    pub __space2: [u8; 7],
}

#[repr(C)]
pub union ila_identifier {
    pub bits: ila_identifier_bits,
    pub v8: [u8; 8],
    pub v16: [__be16; 4],
    pub v32: [__be32; 2],
    pub v64: __be64,
}

pub const CSUM_NEUTRAL_FLAG: u32 = 0x10000000u32.to_be();

#[repr(C)]
pub union ila_addr {
    pub addr: ::core::mem::ManuallyDrop<in6_addr>,
    pub loc_ident: ila_addr_loc_ident,
}

#[repr(C)]
pub struct ila_addr_loc_ident {
    pub loc: ila_locator,
    pub ident: ila_identifier,
}

#[inline]
pub unsafe fn ila_a2i(addr: *mut in6_addr) -> *mut ila_addr {
    addr as *mut ila_addr
}

#[repr(C)]
pub struct ila_params {
    pub locator: ila_locator,
    pub locator_match: ila_locator,
    pub csum_diff: __wsum,
    pub csum_mode: u8,
    pub ident_type: u8,
}

#[inline]
pub unsafe fn compute_csum_diff8(from: *const __be32, to: *const __be32) -> __wsum {
    let diff: [__be32; 4] = [
        !*from.add(0), !*from.add(1), *to.add(0), *to.add(1),
    ];
    csum_partial(diff.as_ptr() as *const core::ffi::c_void,
                 core::mem::size_of_val(&diff), 0)
}

#[inline]
pub unsafe fn ila_csum_neutral_set(ident: ila_identifier) -> bool {
    #[cfg(target_endian = "little")]
    { ((*ident.bits).bits & 0x10) != 0 }
    #[cfg(target_endian = "big")]
    { ((*ident.bits).bits & 0x08) != 0 }
}

extern "C" {
    pub fn ila_update_ipv6_locator(skb: *mut sk_buff, p: *mut ila_params,
                                   set_csum_neutral: bool);
    pub fn ila_init_saved_csum(p: *mut ila_params);
}

#[repr(C)]
pub struct ila_net_xlat {
    pub rhash_table: rhashtable,
    pub locks: *mut spinlock_t, /* Bucket locks for entry manipulation */
    pub locks_mask: libc::c_uint,
    pub hooks_registered: bool,
}

#[repr(C)]
pub struct ila_net {
    pub xlat: ila_net_xlat,
}

extern "C" {
    pub fn ila_lwt_init() -> libc::c_int;
    pub fn ila_lwt_fini();
    pub fn ila_xlat_init_net(net: *mut net) -> libc::c_int;
    pub fn ila_xlat_pre_exit_net(net: *mut net);
    pub fn ila_xlat_exit_net(net: *mut net);
    pub fn ila_xlat_nl_cmd_add_mapping(skb: *mut sk_buff, info: *mut genl_info) -> libc::c_int;
    pub fn ila_xlat_nl_cmd_del_mapping(skb: *mut sk_buff, info: *mut genl_info) -> libc::c_int;
    pub fn ila_xlat_nl_cmd_get_mapping(skb: *mut sk_buff, info: *mut genl_info) -> libc::c_int;
    pub fn ila_xlat_nl_cmd_flush(skb: *mut sk_buff, info: *mut genl_info) -> libc::c_int;
    pub fn ila_xlat_nl_dump_start(cb: *mut netlink_callback) -> libc::c_int;
    pub fn ila_xlat_nl_dump_done(cb: *mut netlink_callback) -> libc::c_int;
    pub fn ila_xlat_nl_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> libc::c_int;
    pub static mut ila_net_id: libc::c_uint;
    pub static mut ila_nl_family: genl_family;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
