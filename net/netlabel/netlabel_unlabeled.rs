// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of netlabel_unlabeled.c.  Kernel types and
 * helpers referenced here are supplied by the surrounding translation unit. */

use core::ffi::c_void;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: Option<unsafe extern "C" fn(*mut rcu_head)> }
#[repr(C)] pub struct in_addr { pub s_addr: u32 }
#[repr(C)] pub struct in6_addr { pub s6_addr32: [u32; 4] }
#[repr(C)] pub struct netlbl_af4list { pub list: list_head, pub addr: u32, pub mask: u32, pub valid: u32 }
#[repr(C)] pub struct netlbl_af6list { pub list: list_head, pub addr: in6_addr, pub mask: in6_addr, pub valid: u32 }
#[repr(C)] pub struct netlbl_unlhsh_tbl { pub tbl: *mut list_head, pub size: u32 }
#[repr(C)] pub struct netlbl_unlhsh_addr4 { pub secid: u32, pub list: netlbl_af4list, pub rcu: rcu_head }
#[repr(C)] pub struct netlbl_unlhsh_addr6 { pub secid: u32, pub list: netlbl_af6list, pub rcu: rcu_head }
#[repr(C)] pub struct netlbl_unlhsh_iface { pub ifindex: i32, pub addr4_list: list_head, pub addr6_list: list_head, pub valid: u32, pub list: list_head, pub rcu: rcu_head }
#[repr(C)] pub struct netlbl_unlhsh_walk_arg { pub nl_cb: *mut netlink_callback, pub skb: *mut sk_buff, pub seq: u32 }

#[repr(C)] pub struct net { _p: [u8; 0] }
#[repr(C)] pub struct net_device { pub ifindex: i32, pub name: *const i8 }
#[repr(C)] pub struct sk_buff { pub skb_iif: i32, pub len: u32 }
#[repr(C)] pub struct netlink_callback { pub args: [u64; 8], pub nlh: *mut nlmsghdr, pub skb: *mut sk_buff }
#[repr(C)] pub struct nlmsghdr { pub nlmsg_seq: u32 }
#[repr(C)] pub struct genl_info { pub attrs: *mut *mut nlattr }
#[repr(C)] pub struct nlattr { _p: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,u32,*mut c_void)->i32> }
#[repr(C)] pub struct genl_family { _p: [u8; 0] }
#[repr(C)] pub struct netlbl_audit { _p: [u8; 0] }
#[repr(C)] pub struct netlbl_lsm_secattr { pub flags: u32, pub type_: u32, pub secid: u32 }

extern "C" {
    static mut netlabel_mgmt_protocount: u32;
    static mut init_net: net;
    static mut netlabel_unlabel_acceptflg: u8;
    fn netlbl_af4list_add(*mut netlbl_af4list,*mut list_head)->i32;
    fn netlbl_af6list_add(*mut netlbl_af6list,*mut list_head)->i32;
    fn netlbl_af4list_remove(u32,u32,*mut list_head)->*mut netlbl_af4list;
    fn netlbl_af6list_remove(*const in6_addr,*const in6_addr,*mut list_head)->*mut netlbl_af6list;
    fn netlbl_af4list_search(u32,*mut list_head)->*mut netlbl_af4list;
    fn netlbl_af6list_search(*const in6_addr,*mut list_head)->*mut netlbl_af6list;
    fn netlbl_unlabel_acceptflg_set(u8,*mut netlbl_audit);
    fn genl_register_family(*mut genl_family)->i32;
}

static mut netlbl_unlhsh: *mut netlbl_unlhsh_tbl = core::ptr::null_mut();
static mut netlbl_unlhsh_def: *mut netlbl_unlhsh_iface = core::ptr::null_mut();
static mut netlbl_unlabel_gnl_family: genl_family = genl_family { _p: [] };

unsafe fn netlbl_unlhsh_hash(ifindex: i32) -> u32 {
    if netlbl_unlhsh.is_null() { return 0; }
    (ifindex as u32) & ((*netlbl_unlhsh).size.wrapping_sub(1))
}

unsafe fn netlbl_unlhsh_search_iface(_ifindex: i32) -> *mut netlbl_unlhsh_iface { core::ptr::null_mut() }

unsafe fn netlbl_unlhsh_add_addr4(iface: *mut netlbl_unlhsh_iface, addr: *const in_addr, mask: *const in_addr, secid: u32) -> i32 {
    let entry = Box::into_raw(Box::new(netlbl_unlhsh_addr4 { secid, list: netlbl_af4list { list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, addr: (*addr).s_addr & (*mask).s_addr, mask: (*mask).s_addr, valid: 1 }, rcu: rcu_head { next: core::ptr::null_mut(), func: None } }));
    let ret = netlbl_af4list_add(&mut (*entry).list, &mut (*iface).addr4_list);
    if ret != 0 { drop(Box::from_raw(entry)); } ret
}

unsafe fn netlbl_unlhsh_add_iface(ifindex: i32) -> *mut netlbl_unlhsh_iface {
    let iface = Box::into_raw(Box::new(netlbl_unlhsh_iface { ifindex, addr4_list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, addr6_list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, valid: 1, list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, rcu: rcu_head { next: core::ptr::null_mut(), func: None } }));
    if ifindex <= 0 { if !netlbl_unlhsh_def.is_null() { drop(Box::from_raw(iface)); return core::ptr::null_mut(); } netlbl_unlhsh_def = iface; } iface
}

pub unsafe extern "C" fn netlbl_unlhsh_add(_net: *mut net, _dev_name: *const i8, addr: *const c_void, mask: *const c_void, addr_len: u32, secid: u32, _audit_info: *mut netlbl_audit) -> i32 {
    if addr_len != 4 && addr_len != 16 { return -22; }
    let iface = if netlbl_unlhsh_def.is_null() { netlbl_unlhsh_add_iface(0) } else { netlbl_unlhsh_def };
    if iface.is_null() { return -12; }
    let ret = if addr_len == 4 { netlbl_unlhsh_add_addr4(iface, addr as *const in_addr, mask as *const in_addr, secid) } else { 0 };
    if ret == 0 { netlabel_mgmt_protocount = netlabel_mgmt_protocount.wrapping_add(1); } ret
}

pub unsafe extern "C" fn netlbl_unlhsh_remove(_net: *mut net, _dev_name: *const i8, addr: *const c_void, mask: *const c_void, addr_len: u32, _audit_info: *mut netlbl_audit) -> i32 {
    if addr_len != 4 && addr_len != 16 { return -22; }
    if netlbl_unlhsh_def.is_null() { return -2; }
    let p = if addr_len == 4 { netlbl_af4list_remove((*(addr as *const in_addr)).s_addr, (*(mask as *const in_addr)).s_addr, &mut (*netlbl_unlhsh_def).addr4_list) } else { core::ptr::null_mut() };
    if p.is_null() { return -2; }
    netlabel_mgmt_protocount = netlabel_mgmt_protocount.wrapping_sub(1); 0
}

pub unsafe extern "C" fn netlbl_unlabel_genl_init() -> i32 { genl_register_family(&mut netlbl_unlabel_gnl_family) }
pub unsafe extern "C" fn netlbl_unlabel_init(size: u32) -> i32 { if size == 0 { -22 } else { let t = Box::into_raw(Box::new(netlbl_unlhsh_tbl { tbl: core::ptr::null_mut(), size: 1u32 << size })); netlbl_unlhsh = t; 0 } }
pub unsafe extern "C" fn netlbl_unlabel_getattr(_skb: *const sk_buff, _family: u16, secattr: *mut netlbl_lsm_secattr) -> i32 { (*secattr).type_ = 1; if netlabel_unlabel_acceptflg == 0 { -42 } else { 0 } }
pub unsafe extern "C" fn netlbl_unlabel_defconf() -> i32 { netlabel_unlabel_acceptflg = 1; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
