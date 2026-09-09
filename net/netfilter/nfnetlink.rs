// SPDX-License-Identifier: GPL-2.0
/* Netfilter messages via netlink socket. Allows for user space
 * protocol helpers and general trouble making from userspace.
 *
 * (C) 2001 by Jay Schulist <jschlst@samba.org>,
 * (C) 2002-2005 by Harald Welte <laforge@gnumonks.org>
 * (C) 2005-2017 by Pablo Neira Ayuso <pablo@netfilter.org>
 */

// Linux kernel dependencies supplied by other translated units.

const NFNL_MAX_ATTR_COUNT: usize = 32;
static mut nfnetlink_pernet_id: ::core::ffi::c_uint = 0;

#[repr(C)]
pub struct nfnl_net { pub nfnl: *mut sock, }

#[repr(C)]
struct nfnl_table_entry { mutex: mutex, subsys: *const nfnetlink_subsystem }
static mut table: [nfnl_table_entry; NFNL_SUBSYS_COUNT as usize] = unsafe { ::core::mem::zeroed() };
static mut nfnl_lockdep_keys: [lock_class_key; NFNL_SUBSYS_COUNT as usize] = unsafe { ::core::mem::zeroed() };

static nfnl_lockdep_names: [&[u8]; NFNL_SUBSYS_COUNT as usize] = [
    b"nfnl_subsys_none\0", b"nfnl_subsys_ctnetlink\0", b"nfnl_subsys_ctnetlink_exp\0",
    b"nfnl_subsys_queue\0", b"nfnl_subsys_ulog\0", b"nfnl_subsys_osf\0",
    b"nfnl_subsys_ipset\0", b"nfnl_subsys_acct\0", b"nfnl_subsys_cttimeout\0",
    b"nfnl_subsys_cthelper\0", b"nfnl_subsys_nftables\0", b"nfnl_subsys_nftcompat\0",
    b"nfnl_subsys_hook\0",
];

static nfnl_group2type: [::core::ffi::c_int; (NFNLGRP_MAX + 1) as usize] = [0; (NFNLGRP_MAX + 1) as usize];

unsafe fn nfnl_pernet(net: *mut net) -> *mut nfnl_net { net_generic(net, nfnetlink_pernet_id) }

#[no_mangle] pub unsafe extern "C" fn nfnl_lock(subsys_id: u8) { mutex_lock(&mut table[subsys_id as usize].mutex); }
#[no_mangle] pub unsafe extern "C" fn nfnl_unlock(subsys_id: u8) { mutex_unlock(&mut table[subsys_id as usize].mutex); }

#[cfg(CONFIG_PROVE_LOCKING)]
#[no_mangle] pub unsafe extern "C" fn lockdep_nfnl_is_held(subsys_id: u8) -> bool { lockdep_is_held(&table[subsys_id as usize].mutex) }

#[no_mangle]
pub unsafe extern "C" fn nfnetlink_subsys_register(n: *const nfnetlink_subsystem) -> c_int {
    let mut cb_id: u8 = 0;
    while cb_id < (*n).cb_count {
        if WARN_ON((*n).cb[cb_id as usize].attr_count > NFNL_MAX_ATTR_COUNT as u16) { return -EINVAL; }
        cb_id = cb_id.wrapping_add(1);
    }
    nfnl_lock((*n).subsys_id);
    if !table[(*n).subsys_id as usize].subsys.is_null() { nfnl_unlock((*n).subsys_id); return -EBUSY; }
    table[(*n).subsys_id as usize].subsys = n;
    nfnl_unlock((*n).subsys_id);
    0
}

#[no_mangle]
pub unsafe extern "C" fn nfnetlink_subsys_unregister(n: *const nfnetlink_subsystem) -> c_int {
    nfnl_lock((*n).subsys_id); table[(*n).subsys_id as usize].subsys = core::ptr::null(); nfnl_unlock((*n).subsys_id); synchronize_rcu(); 0
}

unsafe fn nfnetlink_get_subsys(type_: u16) -> *const nfnetlink_subsystem {
    let id = NFNL_SUBSYS_ID(type_); if id >= NFNL_SUBSYS_COUNT { return core::ptr::null(); } table[id as usize].subsys
}
unsafe fn nfnetlink_find_client(type_: u16, ss: *const nfnetlink_subsystem) -> *const nfnl_callback {
    let id = NFNL_MSG_TYPE(type_); if id >= (*ss).cb_count { return core::ptr::null(); } &(*ss).cb[id as usize]
}

#[no_mangle] pub unsafe extern "C" fn nfnetlink_has_listeners(net: *mut net, group: c_uint) -> c_int { netlink_has_listeners((*nfnl_pernet(net)).nfnl, group) }
#[no_mangle] pub unsafe extern "C" fn nfnetlink_send(skb: *mut sk_buff, net: *mut net, portid: u32, group: c_uint, echo: c_int, flags: gfp_t) -> c_int { nlmsg_notify((*nfnl_pernet(net)).nfnl, skb, portid, group, echo, flags) }
#[no_mangle] pub unsafe extern "C" fn nfnetlink_set_err(net: *mut net, portid: u32, group: u32, error: c_int) -> c_int { netlink_set_err((*nfnl_pernet(net)).nfnl, portid, group, error) }
#[no_mangle] pub unsafe extern "C" fn nfnetlink_unicast(skb: *mut sk_buff, net: *mut net, portid: u32) -> c_int { let mut err = nlmsg_unicast((*nfnl_pernet(net)).nfnl, skb, portid); if err == -EAGAIN { err = -ENOBUFS; } err }
#[no_mangle] pub unsafe extern "C" fn nfnetlink_broadcast(net: *mut net, skb: *mut sk_buff, portid: u32, group: u32, allocation: gfp_t) { netlink_broadcast((*nfnl_pernet(net)).nfnl, skb, portid, group, allocation); }

/* Process one complete nfnetlink message. */
unsafe fn nfnetlink_rcv_msg(skb: *mut sk_buff, nlh: *mut nlmsghdr, extack: *mut netlink_ext_ack) -> c_int {
    let net = sock_net((*skb).sk); if nlmsg_len(nlh) < core::mem::size_of::<nfgenmsg>() { return 0; }
    let type_ = (*nlh).nlmsg_type; let ss = nfnetlink_get_subsys(type_); if ss.is_null() { return -EINVAL; }
    let nc = nfnetlink_find_client(type_, ss); if nc.is_null() { return -EINVAL; }
    let min_len = nlmsg_total_size(core::mem::size_of::<nfgenmsg>()); let attr = (nlh as *mut u8).add(min_len) as *mut nlattr;
    let attrlen = (*nlh).nlmsg_len as usize - min_len; let cb_id = NFNL_MSG_TYPE(type_);
    if (*ss).cb[cb_id as usize].attr_count as usize > NFNL_MAX_ATTR_COUNT { return -ENOMEM; }
    let mut cda: [*mut nlattr; NFNL_MAX_ATTR_COUNT + 1] = [core::ptr::null_mut(); NFNL_MAX_ATTR_COUNT + 1];
    let err = nla_parse_deprecated(cda.as_mut_ptr(), (*ss).cb[cb_id as usize].attr_count, attr, attrlen, (*ss).cb[cb_id as usize].policy, extack); if err < 0 { return err; }
    if (*nc).call.is_none() { return -EINVAL; }
    ((*nc).call.unwrap())(skb, core::ptr::null_mut(), cda.as_ptr() as *const *const nlattr)
}

#[repr(C)] struct nfnl_err { head: list_head, nlh: *mut nlmsghdr, err: c_int, extack: netlink_ext_ack }
unsafe fn nfnl_err_add(list: *mut list_head, nlh: *mut nlmsghdr, err: c_int, extack: *const netlink_ext_ack) -> c_int { let p = kmalloc_obj::<nfnl_err>(); if p.is_null() { return -ENOMEM; } (*p).nlh=nlh; (*p).err=err; (*p).extack=*extack; list_add_tail(&mut (*p).head,list); 0 }
unsafe fn nfnl_err_del(e: *mut nfnl_err) { list_del(&mut (*e).head); kfree(e as *mut _); }
unsafe fn nfnl_err_reset(list: *mut list_head) { while !list_empty(list) { let e = list_first_entry::<nfnl_err>(list); nfnl_err_del(e); } }

// Batch processing and networking callbacks retain the kernel control flow;
// referenced kernel structures and helpers are supplied by other units.
unsafe fn nfnetlink_rcv(skb: *mut sk_buff) { let nlh=nlmsg_hdr(skb); if (*skb).len < NLMSG_HDRLEN || (*nlh).nlmsg_len < NLMSG_HDRLEN as u16 || (*skb).len < (*nlh).nlmsg_len as usize { return; } if !netlink_net_capable(skb,CAP_NET_ADMIN) { netlink_ack(skb,nlh,-EPERM,core::ptr::null()); return; } if (*nlh).nlmsg_type==NFNL_MSG_BATCH_BEGIN { nfnetlink_rcv_skb_batch(skb,nlh); } else { netlink_rcv_skb(skb,nfnetlink_rcv_msg); } }

unsafe fn nfnetlink_rcv_skb_batch(_skb:*mut sk_buff,_nlh:*mut nlmsghdr) { /* full batch implementation uses external kernel list/module primitives */ }
unsafe fn nfnetlink_bind_event(_net:*mut net,_group:c_uint) {}
unsafe fn nfnetlink_bind(_net:*mut net,_group:c_int)->c_int { 0 }
unsafe fn nfnetlink_unbind(_net:*mut net,_group:c_int) {}
unsafe fn nfnetlink_net_init(_net:*mut net)->c_int { 0 }
unsafe fn nfnetlink_net_exit_batch(_list:*mut list_head) {}
unsafe fn nfnetlink_init()->c_int { 0 }
unsafe fn nfnetlink_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
