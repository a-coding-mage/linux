// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_pkttype
 *
 *	Authors:
 *	Bart De Schuymer <bdschuym@pandora.be>
 *
 *  April, 2003
 *
 */

// Dependencies supplied by the surrounding kernel/netfilter translation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct sk_buff {
    pub pkt_type: u32,
}

#[repr(C)]
pub struct ebt_pkttype_info {
    pub pkt_type: u32,
    pub invert: u8,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const c_void,
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *const c_void,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const c_char,
    pub revision: u8,
    pub family: u16,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> c_int>,
    pub matchsize: usize,
    pub me: *mut c_void,
}

const NFPROTO_BRIDGE: u16 = 7;
const EINVAL: c_int = 22;

extern "C" {
    fn xt_register_match(m: *mut xt_match) -> c_int;
    fn xt_unregister_match(m: *mut xt_match);
}

static mut ebt_pkttype_mt_reg: xt_match = xt_match {
    name: b"pkttype\0".as_ptr() as *const c_char,
    revision: 0,
    family: NFPROTO_BRIDGE,
    match_: Some(ebt_pkttype_mt),
    checkentry: Some(ebt_pkttype_mt_check),
    matchsize: core::mem::size_of::<ebt_pkttype_info>(),
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn ebt_pkttype_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info = (*par).matchinfo as *const ebt_pkttype_info;

    ((*skb).pkt_type == (*info).pkt_type) ^ ((*info).invert != 0)
}

unsafe extern "C" fn ebt_pkttype_mt_check(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *const ebt_pkttype_info;

    if (*info).invert != 0 && (*info).invert != 1 {
        return -EINVAL;
    }
    /* Allow any pkt_type value */
    0
}

unsafe extern "C" fn ebt_pkttype_init() -> c_int {
    xt_register_match(&raw mut ebt_pkttype_mt_reg)
}

unsafe extern "C" fn ebt_pkttype_fini() {
    xt_unregister_match(&raw mut ebt_pkttype_mt_reg);
}

// module_init(ebt_pkttype_init);
// module_exit(ebt_pkttype_fini);
// MODULE_DESCRIPTION("Ebtables: Link layer packet type match");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
