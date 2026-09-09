// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C)2006 USAGI/WIDE Project
 *
 * Author:
 *	Masahide NAKAMURA @USAGI <masahide.nakamura.cz@hitachi.com>
 *
 * Based on net/netfilter/xt_tcpudp.c
 */
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Dependencies supplied by the corresponding kernel headers are intentionally
// left as external Rust declarations.

use core::ffi::c_void;

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
    fn skb_header_pointer(
        skb: *const sk_buff,
        thoff: u32,
        len: usize,
        buffer: *mut c_void,
    ) -> *const ip6_mh;
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip6_mh {
    pub ip6mh_proto: u8,
    pub ip6mh_type: u8,
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip6t_mh {
    pub types: [u8; 2],
    pub invflags: u8,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const c_void,
    pub fragoff: u16,
    pub thoff: u32,
    pub hotdrop: bool,
}

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *const c_void,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub family: u16,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub matchsize: usize,
    pub proto: u8,
    pub me: *mut c_void,
}

const EINVAL: i32 = 22;
const IPPROTO_NONE: u8 = 59;
const IPPROTO_MH: u8 = 135;
const NFPROTO_IPV6: u16 = 10;
const IP6T_MH_INV_TYPE: u8 = 0x01;
const IP6T_MH_INV_MASK: u8 = IP6T_MH_INV_TYPE;

// Returns 1 if the type is matched by the range, 0 otherwise
#[inline]
unsafe fn type_match(min: u8, max: u8, type_: u8, invert: bool) -> bool {
    ((type_ >= min && type_ <= max) as u8 ^ invert as u8) != 0
}

unsafe extern "C" fn mh_mt6(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let mut _mh = core::mem::MaybeUninit::<ip6_mh>::uninit();
    let mh: *const ip6_mh;
    let mhinfo = (*par).matchinfo as *const ip6t_mh;

    // Must not be a fragment.
    if (*par).fragoff != 0 {
        return false;
    }

    mh = skb_header_pointer(
        skb,
        (*par).thoff,
        core::mem::size_of::<ip6_mh>(),
        _mh.as_mut_ptr() as *mut c_void,
    );
    if mh.is_null() {
        // We've been asked to examine this packet, and we
        // can't. Hence, no choice but to drop.
        (*par).hotdrop = true;
        return false;
    }

    if (*mh).ip6mh_proto != IPPROTO_NONE {
        (*par).hotdrop = true;
        return false;
    }

    type_match(
        (*mhinfo).types[0],
        (*mhinfo).types[1],
        (*mh).ip6mh_type,
        ((*mhinfo).invflags & IP6T_MH_INV_TYPE) != 0,
    )
}

unsafe extern "C" fn mh_mt6_check(par: *const xt_mtchk_param) -> i32 {
    let mhinfo = (*par).matchinfo as *const ip6t_mh;

    // Must specify no unknown invflags
    if ((*mhinfo).invflags & !IP6T_MH_INV_MASK) != 0 {
        -EINVAL
    } else {
        0
    }
}

static mut mh_mt6_reg: xt_match = xt_match {
    name: b"mh\0".as_ptr(),
    family: NFPROTO_IPV6,
    checkentry: Some(mh_mt6_check),
    match_: Some(mh_mt6),
    matchsize: core::mem::size_of::<ip6t_mh>(),
    proto: IPPROTO_MH,
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn mh_mt6_init() -> i32 {
    xt_register_match(&mut mh_mt6_reg)
}

unsafe extern "C" fn mh_mt6_exit() {
    xt_unregister_match(&mut mh_mt6_reg);
}

// module_init(mh_mt6_init);
// module_exit(mh_mt6_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
