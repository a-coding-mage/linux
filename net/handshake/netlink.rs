// SPDX-License-Identifier: GPL-2.0-only
/*
 * Generic netlink handshake service
 *
 * Author: Chuck Lever <chuck.lever@oracle.com>
 *
 * Copyright (c) 2023, Oracle and/or its affiliates.
 */

// Dependencies supplied by the kernel and handshake implementation.

extern "C" {
    static mut handshake_nl_family: genl_family;
    fn test_bit(nr: c_uint, addr: *const c_ulong) -> bool;
    fn genl_has_listeners(family: *const genl_family, net: *mut net, group: c_uint) -> bool;
    fn genlmsg_new(size: usize, flags: gfp_t) -> *mut sk_buff;
    fn genlmsg_put(msg: *mut sk_buff, portid: u32, seq: u32, family: *const genl_family,
                   flags: u32, cmd: u8) -> *mut c_void;
    fn nla_put_u32(msg: *mut sk_buff, attr: c_uint, value: u32) -> c_int;
    fn genlmsg_cancel(msg: *mut sk_buff, hdr: *mut c_void);
    fn genlmsg_end(msg: *mut sk_buff, hdr: *mut c_void);
    fn genlmsg_multicast_netns(family: *const genl_family, net: *mut net,
                               msg: *mut sk_buff, portid: u32, group: c_uint,
                               flags: gfp_t) -> c_int;
    fn nlmsg_free(msg: *mut sk_buff);
    fn sock_net(sk: *mut sock) -> *mut net;
    fn handshake_pernet(net: *mut net) -> *mut handshake_net;
    fn handshake_req_next(hn: *mut handshake_net, class: c_int) -> *mut handshake_req;
    fn fput(file: *mut file);
    fn handshake_complete(req: *mut handshake_req, status: c_int, info: *mut genl_info);
    fn sockfd_lookup(fd: c_int, err: *mut c_int) -> *mut socket;
    fn handshake_req_hash_lookup(sk: *mut sock) -> *mut handshake_req;
    fn sockfd_put(sock: *mut socket);
    fn si_meminfo(si: *mut sysinfo);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn set_bit(nr: c_uint, addr: *mut c_ulong);
    fn get_file(file: *mut file);
    fn list_empty(head: *const list_head) -> bool;
    fn list_splice_init(list: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn __handshake_genl_net_register(ops: *mut pernet_operations);
}

use core::ffi::{c_int, c_uint, c_ulong, c_void};

const HANDSHAKE_F_PROTO_NOTIFY: c_uint = 0;
const HANDSHAKE_F_NET_DRAINING: c_uint = 0;
const HANDSHAKE_CMD_READY: u8 = 0;
const HANDSHAKE_A_ACCEPT_HANDLER_CLASS: c_uint = 0;
const HANDSHAKE_A_DONE_SOCKFD: c_uint = 0;
const HANDSHAKE_A_DONE_STATUS: c_uint = 0;
const GENLMSG_DEFAULT_SIZE: usize = 0;
const EAGAIN: c_int = 11;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const EMSGSIZE: c_int = 90;
const ENOMEM: c_int = 12;
const EOPNOTSUPP: c_int = 95;
const ESRCH: c_int = 3;
const ETIMEDOUT: c_int = 110;

#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct socket { pub sk: *mut sock }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct genl_family { _private: [u8; 0] }
#[repr(C)] pub struct genl_info { pub snd_portid: u32, pub snd_seq: u32, pub genlhdr: *mut genlmsghdr, pub attrs: *mut *mut nlattr }
#[repr(C)] pub struct genlmsghdr { pub cmd: u8 }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct sysinfo { pub totalram: c_ulong, pub mem_unit: c_ulong }
#[repr(C)] pub struct handshake_net { pub hn_pending_max: c_ulong, pub hn_lock: spinlock_t, pub hn_pending: c_uint, pub hn_flags: c_ulong, pub hn_requests: list_head }
#[repr(C)] pub struct handshake_req { pub hr_list: list_head, pub hr_file: *mut file, pub hr_proto: *mut handshake_proto, pub hr_sk: *mut sock }
#[repr(C)] pub struct handshake_proto { pub hp_flags: c_ulong, pub hp_handler_class: c_uint, pub hp_accept: Option<unsafe extern "C" fn(*mut handshake_req, *mut genl_info, c_int) -> c_int> }
#[repr(C)] pub struct pernet_operations { pub init: Option<unsafe extern "C" fn(*mut net) -> c_int>, pub exit: Option<unsafe extern "C" fn(*mut net)>, pub id: *mut c_uint, pub size: usize }
type gfp_t = c_ulong;

pub unsafe extern "C" fn handshake_genl_notify(net: *mut net, proto: *const handshake_proto, flags: gfp_t) -> c_int {
    if !test_bit(HANDSHAKE_F_PROTO_NOTIFY, &(*proto).hp_flags) { return 0; }
    if !genl_has_listeners(&handshake_nl_family, net, (*proto).hp_handler_class) { return -ESRCH; }
    let msg = genlmsg_new(GENLMSG_DEFAULT_SIZE, flags);
    if msg.is_null() { return -ENOMEM; }
    let hdr = genlmsg_put(msg, 0, 0, &handshake_nl_family, 0, HANDSHAKE_CMD_READY);
    if hdr.is_null() { nlmsg_free(msg); return -EMSGSIZE; }
    if nla_put_u32(msg, HANDSHAKE_A_ACCEPT_HANDLER_CLASS, (*proto).hp_handler_class) < 0 {
        genlmsg_cancel(msg, hdr); nlmsg_free(msg); return -EMSGSIZE;
    }
    genlmsg_end(msg, hdr);
    genlmsg_multicast_netns(&handshake_nl_family, net, msg, 0, (*proto).hp_handler_class, flags)
}

pub unsafe extern "C" fn handshake_genl_put(msg: *mut sk_buff, info: *mut genl_info) -> *mut c_void {
    genlmsg_put(msg, (*info).snd_portid, (*info).snd_seq, &handshake_nl_family, 0, (*(*info).genlhdr).cmd)
}

pub unsafe extern "C" fn handshake_nl_accept_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let net = sock_net((*skb_as_socket(skb)).sk);
    let hn = handshake_pernet(net);
    let mut req: *mut handshake_req = core::ptr::null_mut();
    let mut err = -EOPNOTSUPP;
    if hn.is_null() { return err; }
    err = -EINVAL;
    let class = nla_get_u32((*info).attrs.add(HANDSHAKE_A_ACCEPT_HANDLER_CLASS as usize));
    err = -EAGAIN;
    req = handshake_req_next(hn, class as c_int);
    if !req.is_null() {
        let fd = (*req).hr_file as c_int;
        if fd < 0 { fput((*req).hr_file); err = fd; handshake_complete(req, -EIO, core::ptr::null_mut()); return err; }
        err = ((*(*req).hr_proto).hp_accept.unwrap())(req, info, fd);
        if err != 0 { handshake_complete(req, -EIO, core::ptr::null_mut()); return err; }
        return 0;
    }
    if !req.is_null() { handshake_complete(req, -EIO, core::ptr::null_mut()); }
    err
}

pub unsafe extern "C" fn handshake_nl_done_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let net = sock_net((*skb_as_socket(skb)).sk);
    let fd = nla_get_u32((*info).attrs.add(HANDSHAKE_A_DONE_SOCKFD as usize)) as c_int;
    let mut err = 0;
    let sock = sockfd_lookup(fd, &mut err);
    if sock.is_null() { return err; }
    let req = handshake_req_hash_lookup((*sock).sk);
    if req.is_null() { sockfd_put(sock); return -EBUSY; }
    let mut status = -EIO;
    let status_attr = (*info).attrs.add(HANDSHAKE_A_DONE_STATUS as usize);
    if !status_attr.is_null() { status = -(nla_get_u32(status_attr) as c_int); }
    handshake_complete(req, status, info); sockfd_put(sock); let _ = net; 0
}

extern "C" { static mut handshake_net_id: c_uint; }

unsafe extern "C" fn handshake_net_init(net: *mut net) -> c_int {
    let hn = net_generic(net, handshake_net_id);
    let mut si = sysinfo { totalram: 0, mem_unit: 0 };
    si_meminfo(&mut si);
    (*hn).hn_pending_max = (si.totalram / (25 * si.mem_unit)).clamp(3, 50);
    spin_lock_init(&mut (*hn).hn_lock); (*hn).hn_pending = 0; (*hn).hn_flags = 0;
    init_list_head(&mut (*hn).hn_requests); 0
}

unsafe extern "C" fn handshake_net_exit(net: *mut net) {
    let hn = net_generic(net, handshake_net_id); set_bit(HANDSHAKE_F_NET_DRAINING, &mut (*hn).hn_flags);
    let _ = hn;
}

static mut handshake_genl_net_ops: pernet_operations = pernet_operations { init: Some(handshake_net_init), exit: Some(handshake_net_exit), id: unsafe { &raw mut handshake_net_id }, size: core::mem::size_of::<handshake_net>() };

pub unsafe extern "C" fn handshake_pernet(net: *mut net) -> *mut handshake_net {
    if handshake_net_id != 0 { net_generic(net, handshake_net_id) } else { core::ptr::null_mut() }
}

extern "C" { fn net_generic(net: *mut net, id: c_uint) -> *mut handshake_net; fn init_list_head(head: *mut list_head); fn nla_get_u32(attr: *mut *mut nlattr) -> u32; fn skb_as_socket(skb: *mut sk_buff) -> *mut socket; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
