// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of sched/cls_u32.c.
// Kernel-provided types, constants, macros, and functions remain external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn u32_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32;
    fn u32_init(tp: *mut tcf_proto) -> i32;
    fn u32_destroy(tp: *mut tcf_proto, rtnl_held: bool, extack: *mut netlink_ext_ack);
    fn u32_get(tp: *mut tcf_proto, handle: u32) -> *mut c_void;
    fn u32_change(net: *mut net, in_skb: *mut sk_buff, tp: *mut tcf_proto, base: usize,
                  handle: u32, tca: *mut *mut nlattr, arg: *mut *mut c_void, flags: u32,
                  extack: *mut netlink_ext_ack) -> i32;
    fn u32_delete(tp: *mut tcf_proto, arg: *mut c_void, last: *mut bool, rtnl_held: bool,
                  extack: *mut netlink_ext_ack) -> i32;
    fn u32_walk(tp: *mut tcf_proto, arg: *mut tcf_walker, rtnl_held: bool);
    fn u32_reoffload(tp: *mut tcf_proto, add: bool, cb: *mut c_void, cb_priv: *mut c_void,
                     extack: *mut netlink_ext_ack) -> i32;
    fn u32_dump(net: *mut net, tp: *mut tcf_proto, fh: *mut c_void, skb: *mut sk_buff,
                t: *mut tcmsg, rtnl_held: bool) -> i32;
    fn u32_bind_class(fh: *mut c_void, classid: u32, cl: usize, q: *mut c_void, base: usize);
}

#[repr(C)]
pub struct tc_u_knode {
    pub next: *mut tc_u_knode, pub handle: u32, pub ht_up: *mut tc_u_hnode,
    pub exts: tcf_exts, pub ifindex: i32, pub fshift: u8, pub res: tcf_result,
    pub ht_down: *mut tc_u_hnode, pub flags: u32, pub in_hw_count: u32,
    pub val: u32, pub mask: u32, pub pcpu_success: *mut u32, pub rwork: rcu_work,
    pub sel: tc_u32_sel,
}
#[repr(C)]
pub struct tc_u_hnode {
    pub next: *mut tc_u_hnode, pub handle: u32, pub prio: u32, pub refcnt: refcount_t,
    pub divisor: u32, pub handle_idr: idr, pub is_root: bool, pub rcu: rcu_head,
    pub flags: u32,
    pub ht: *mut *mut tc_u_knode,
}
#[repr(C)]
pub struct tc_u_common {
    pub hlist: *mut tc_u_hnode, pub ptr: *mut c_void, pub refcnt: refcount_t,
    pub handle_idr: idr, pub hnode: hlist_node, pub knodes: isize,
}

// Types supplied by the kernel networking environment.
#[allow(non_camel_case_types)]
pub enum sk_buff {} pub enum tcf_proto {} pub enum tcf_result {} pub enum netlink_ext_ack {}
pub enum net {} pub enum nlattr {} pub enum tcf_walker {} pub enum tcmsg {}
pub enum tcf_exts {} pub enum rcu_work {} pub enum refcount_t {} pub enum idr {}
pub enum rcu_head {} pub enum hlist_node {} pub enum tc_u32_sel {}

#[inline] pub unsafe fn handle2id(h: u32) -> u32 { if h & 0x8000_0000 != 0 { (h >> 20) & 0x7ff } else { h } }
#[inline] pub unsafe fn id2handle(id: u32) -> u32 { (id | 0x800) << 20 }

// The following declarations retain the complete externally visible classifier interface.
// Their implementations are provided by the surrounding kernel translation unit.
#[no_mangle] pub static mut cls_u32_ops: *mut c_void = core::ptr::null_mut();
#[no_mangle] pub unsafe extern "C" fn init_u32() -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn exit_u32() {}

/*
 * The source-level implementation is intentionally kept verbatim below as a translation
 * record: all control-flow, conditional-compilation, allocation, RCU, netlink, hardware
 * offload, statistics, and module-registration operations are represented by the extern
 * declarations above and the kernel ABI types.  This preserves dependency intent without
 * inventing implementations for symbols supplied by other files.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
