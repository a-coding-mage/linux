// SPDX-License-Identifier: GPL-2.0-only
/* IP tables module for matching the routing realm
 *
 * (C) 2003 by Sampsa Ranta <sampsa@netsonic.fi>
 */

// C dependencies: linux/module.h, linux/skbuff.h, linux/netdevice.h,
// net/route.h, linux/netfilter_ipv4.h, linux/netfilter/xt_realm.h,
// linux/netfilter/x_tables.h

use core::ffi::c_void;

// MODULE_AUTHOR("Sampsa Ranta <sampsa@netsonic.fi>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Xtables: Routing realm match");
// MODULE_ALIAS("ipt_realm");

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dst_entry {
    pub tclassid: u32,
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const c_void,
}

#[repr(C)]
pub struct xt_realm_info {
    pub id: u32,
    pub mask: u32,
    pub invert: bool,
}

pub type XtmMatchFn = unsafe extern "C" fn(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool;

#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub match_fn: Option<XtmMatchFn>,
    pub matchsize: usize,
    pub hooks: u32,
    pub family: u8,
    pub me: *mut c_void,
}

unsafe extern "C" {
    fn skb_dst(skb: *const sk_buff) -> *const dst_entry;
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
}

const NF_INET_POST_ROUTING: u32 = 4;
const NF_INET_FORWARD: u32 = 2;
const NF_INET_LOCAL_OUT: u32 = 3;
const NF_INET_LOCAL_IN: u32 = 1;
const NFPROTO_IPV4: u8 = 2;

// THIS_MODULE is supplied by the kernel module build environment.
unsafe extern "C" {
    static mut THIS_MODULE: c_void;
}

unsafe extern "C" fn realm_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info = (*par).matchinfo as *const xt_realm_info;
    let dst = skb_dst(skb);

    ((*info).id == ((*dst).tclassid & (*info).mask)) ^ (*info).invert
}

#[no_mangle]
pub static mut realm_mt_reg: xt_match = xt_match {
    name: b"realm\0".as_ptr(),
    match_fn: Some(realm_mt),
    matchsize: core::mem::size_of::<xt_realm_info>(),
    hooks: (1u32 << NF_INET_POST_ROUTING)
        | (1u32 << NF_INET_FORWARD)
        | (1u32 << NF_INET_LOCAL_OUT)
        | (1u32 << NF_INET_LOCAL_IN),
    family: NFPROTO_IPV4,
    me: unsafe { &raw mut THIS_MODULE as *mut c_void },
};

unsafe extern "C" fn realm_mt_init() -> i32 {
    xt_register_match(&raw mut realm_mt_reg)
}

unsafe extern "C" fn realm_mt_exit() {
    xt_unregister_match(&raw mut realm_mt_reg);
}

// module_init(realm_mt_init);
// module_exit(realm_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
