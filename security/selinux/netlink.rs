// SPDX-License-Identifier: GPL-2.0-only
/*
 * Netlink event notifications for SELinux.
 *
 * Author: James Morris <jmorris@redhat.com>
 *
 * Copyright (C) 2004 Red Hat, Inc., James Morris <jmorris@redhat.com>
 */

// C dependencies translated from:
// <linux/init.h>, <linux/types.h>, <linux/slab.h>, <linux/stddef.h>,
// <linux/kernel.h>, <linux/export.h>, <linux/skbuff.h>,
// <linux/selinux_netlink.h>, <net/net_namespace.h>, <net/netlink.h>,
// "initcalls.h", and "security.h".

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;

type u32 = core::ffi::c_uint;
type sk_buff_data_t = usize;

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    pub tail: sk_buff_data_t,
}

#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: u32,
}

#[repr(C)]
pub struct selnl_msg_setenforce {
    pub val: c_int,
}

#[repr(C)]
pub struct selnl_msg_policyload {
    pub seqno: u32,
}

#[repr(C)]
pub struct netlink_kernel_cfg {
    pub groups: c_int,
    pub flags: c_int,
}

#[repr(C)]
pub struct netlink_skb_parms {
    pub dst_group: c_int,
}

extern "C" {
    static mut init_net: c_void;

    static SELNL_MSG_SETENFORCE: c_int;
    static SELNL_MSG_POLICYLOAD: c_int;
    static SELNLGRP_AVC: c_int;
    static SELNLGRP_MAX: c_int;
    static NL_CFG_F_NONROOT_RECV: c_int;
    static NETLINK_SELINUX: c_int;
    static GFP_USER: c_int;

    fn BUG() -> !;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn nlmsg_data(nlh: *mut nlmsghdr) -> *mut c_void;
    fn nlmsg_new(payload: c_int, flags: c_int) -> *mut sk_buff;
    fn nlmsg_put(
        skb: *mut sk_buff,
        portid: u32,
        seq: u32,
        msgtype: c_int,
        payload: c_int,
        flags: c_int,
    ) -> *mut nlmsghdr;
    fn NETLINK_CB(skb: *mut sk_buff) -> *mut netlink_skb_parms;
    fn netlink_broadcast(
        ss: *mut sock,
        skb: *mut sk_buff,
        portid: u32,
        group: c_int,
        allocation: c_int,
    ) -> c_int;
    fn kfree_skb(skb: *mut sk_buff);
    fn pr_err(fmt: *const c_char, ...);
    fn netlink_kernel_create(
        net: *mut c_void,
        unit: c_int,
        cfg: *mut netlink_kernel_cfg,
    ) -> *mut sock;
    fn panic(fmt: *const c_char, ...) -> !;
}

static mut selnl: *mut sock = core::ptr::null_mut();

unsafe fn selnl_msglen(msgtype: c_int) -> c_int {
    let mut ret: c_int = 0;

    if msgtype == SELNL_MSG_SETENFORCE {
        ret = size_of::<selnl_msg_setenforce>() as c_int;
    } else if msgtype == SELNL_MSG_POLICYLOAD {
        ret = size_of::<selnl_msg_policyload>() as c_int;
    } else {
        BUG();
    }
    ret
}

unsafe fn selnl_add_payload(nlh: *mut nlmsghdr, len: c_int, msgtype: c_int, data: *mut c_void) {
    if msgtype == SELNL_MSG_SETENFORCE {
        let msg: *mut selnl_msg_setenforce = nlmsg_data(nlh) as *mut selnl_msg_setenforce;

        memset(msg as *mut c_void, 0, len as usize);
        (*msg).val = *(data as *mut c_int);
    } else if msgtype == SELNL_MSG_POLICYLOAD {
        let msg: *mut selnl_msg_policyload = nlmsg_data(nlh) as *mut selnl_msg_policyload;

        memset(msg as *mut c_void, 0, len as usize);
        (*msg).seqno = *(data as *mut u32);
    } else {
        BUG();
    }
}

unsafe fn selnl_notify(msgtype: c_int, data: *mut c_void) {
    let len: c_int;
    let tmp: sk_buff_data_t;
    let skb: *mut sk_buff;
    let nlh: *mut nlmsghdr;

    len = selnl_msglen(msgtype);

    skb = nlmsg_new(len, GFP_USER);
    if skb.is_null() {
        pr_err(c"SELinux:  OOM in %s\n".as_ptr(), c"selnl_notify".as_ptr());
        return;
    }

    tmp = (*skb).tail;
    nlh = nlmsg_put(skb, 0, 0, msgtype, len, 0);
    if nlh.is_null() {
        kfree_skb(skb);
        pr_err(c"SELinux:  OOM in %s\n".as_ptr(), c"selnl_notify".as_ptr());
        return;
    }
    selnl_add_payload(nlh, len, msgtype, data);
    (*nlh).nlmsg_len = ((*skb).tail).wrapping_sub(tmp) as u32;
    (*NETLINK_CB(skb)).dst_group = SELNLGRP_AVC;
    netlink_broadcast(selnl, skb, 0, SELNLGRP_AVC, GFP_USER);
}

#[no_mangle]
pub unsafe extern "C" fn selnl_notify_setenforce(val: c_int) {
    let mut val = val;
    selnl_notify(
        SELNL_MSG_SETENFORCE,
        (&mut val as *mut c_int).cast::<c_void>(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn selnl_notify_policyload(seqno: u32) {
    let mut seqno = seqno;
    selnl_notify(
        SELNL_MSG_POLICYLOAD,
        (&mut seqno as *mut u32).cast::<c_void>(),
    );
}

// C spelling: int __init sel_netlink_init(void)
#[no_mangle]
pub unsafe extern "C" fn sel_netlink_init() -> c_int {
    let mut cfg = netlink_kernel_cfg {
        groups: SELNLGRP_MAX,
        flags: NL_CFG_F_NONROOT_RECV,
    };

    selnl = netlink_kernel_create(
        (&mut init_net as *mut c_void),
        NETLINK_SELINUX,
        &mut cfg as *mut netlink_kernel_cfg,
    );
    if selnl.is_null() {
        panic(c"SELinux:  Cannot create netlink socket.".as_ptr());
    }
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
