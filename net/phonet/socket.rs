// SPDX-License-Identifier: GPL-2.0-only
/*
 * File: socket.c
 *
 * Phonet sockets
 *
 * Copyright (C) 2008 Nokia Corporation.
 *
 * Authors: Sakari Ailus <sakari.ailus@nokia.com>
 *          Rémi Denis-Courmont
 */

// Kernel dependencies supplied by other translation units.

const PN_HASHSIZE: usize = 16;
const PN_HASHMASK: u16 = PN_HASHSIZE as u16 - 1;

static mut pnsocks: PnSockets = PnSockets {
    hlist: [HlistHead { _private: 0 }; PN_HASHSIZE],
    lock: Mutex { _private: 0 },
};

#[repr(C)]
struct PnSockets {
    hlist: [HlistHead; PN_HASHSIZE],
    lock: Mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct HlistHead { _private: usize }
#[repr(C)]
struct Mutex { _private: usize }

extern "C" {
    fn pn_socket_release(sock: *mut Socket) -> i32;
    fn pn_sockaddr_get_object(spn: *const SockaddrPn) -> u16;
    fn pn_sockaddr_get_resource(spn: *const SockaddrPn) -> u8;
    fn pn_hash_list(obj: u16) -> *mut HlistHead;
}

// The following declarations mirror symbols and types supplied by the kernel headers.
#[repr(C)] pub struct Socket { pub sk: *mut Sock, pub state: i32 }
#[repr(C)] pub struct Sock { pub sk_prot: *mut Proto, pub sk_state: i32, pub sk_bound_dev_if: i32, pub sk_ack_backlog: i32, pub sk_max_ack_backlog: i32, pub sk_sndbuf: i32, pub sk_receive_queue: usize, pub sk_sleep: usize, pub sk_wmem_alloc: usize }
#[repr(C)] pub struct Proto { pub close: Option<unsafe extern "C" fn(*mut Sock, i32)>, pub bind: Option<unsafe extern "C" fn(*mut Sock, *mut SockaddrUnsized, i32) -> i32>, pub get_port: Option<unsafe extern "C" fn(*mut Sock, u16) -> i32>, pub hash: Option<unsafe extern "C" fn(*mut Sock) -> i32>, pub connect: Option<unsafe extern "C" fn(*mut Sock, *mut SockaddrUnsized, i32) -> i32>, pub accept: Option<unsafe extern "C" fn(*mut Sock, *mut ProtoAcceptArg) -> *mut Sock>, pub sendmsg: Option<unsafe extern "C" fn(*mut Sock, *mut Msghdr, usize) -> isize> }
#[repr(C)] pub struct SockaddrPn { pub spn_family: u16, pub spn_resource: u8, pub data: [u8; 5] }
#[repr(C)] pub struct SockaddrUnsized { _private: [u8; 0] }
#[repr(C)] pub struct Sockaddr { pub sa_family: u16, pub data: [u8; 14] }
#[repr(C)] pub struct SkBuff { _private: [u8; 0] }
#[repr(C)] pub struct Net { _private: [u8; 0] }
#[repr(C)] pub struct File { _private: [u8; 0] }
#[repr(C)] pub struct PollTable { _private: [u8; 0] }
#[repr(C)] pub struct Msghdr { _private: [u8; 0] }
#[repr(C)] pub struct ProtoAcceptArg { pub err: i32 }
#[repr(C)] pub struct PnSock { pub sobject: u16, pub dobject: u16, pub resource: u8 }
#[repr(C)] pub struct PepSock { pub ctrlreq_queue: usize, pub tx_credits: i32 }
#[repr(C)] pub struct NetDevice { pub flags: u32 }
#[repr(C)] pub struct ProtoOps { _private: [u8; 0] }

extern "C" {
    fn mutex_init(m: *mut Mutex); fn INIT_HLIST_HEAD(h: *mut HlistHead);
    fn mutex_lock(m: *mut Mutex); fn mutex_unlock(m: *mut Mutex);
    fn rcu_read_lock(); fn rcu_read_unlock(); fn synchronize_rcu();
    fn pn_sk(sk: *mut Sock) -> *mut PnSock; fn pep_sk(sk: *mut Sock) -> *mut PepSock;
    fn sock_net(sk: *mut Sock) -> *mut Net; fn net_eq(a: *mut Net, b: *mut Net) -> bool;
    fn pn_port(x: u16) -> u16; fn pn_addr(x: u16) -> u8; fn pn_object(a: u8, p: u16) -> u16;
    fn sock_hold(sk: *mut Sock); fn sock_put(sk: *mut Sock); fn sk_receive_skb(sk: *mut Sock, skb: *mut SkBuff, x: i32);
    fn skb_clone(skb: *mut SkBuff, gfp: i32) -> *mut SkBuff; fn sk_add_node_rcu(sk: *mut Sock, h: *mut HlistHead); fn sk_del_node_init_rcu(sk: *mut Sock);
    fn pn_sock_unbind_all_res(sk: *mut Sock); fn sk_hashed(sk: *mut Sock) -> bool;
    fn lock_sock(sk: *mut Sock); fn release_sock(sk: *mut Sock); fn phonet_address_lookup(net: *mut Net, addr: u8) -> i32;
    fn memset(p: *mut u8, v: i32, n: usize) -> *mut u8; fn sock_rcvtimeo(sk: *mut Sock, nonblock: bool) -> i64;
    fn current() -> *mut Task; fn signal_pending(t: *mut Task) -> bool; fn sock_intr_errno(t: i64) -> i32; fn schedule_timeout(t: i64) -> i64;
    fn poll_wait(f: *mut File, sleep: usize, wait: *mut PollTable); fn skb_queue_empty_lockless(q: *const usize) -> bool;
    fn refcount_read(x: *const usize) -> i32; fn atomic_read(x: *const i32) -> i32;
    fn get_user(dst: *mut u16, src: *const u16) -> i32; fn put_user(v: u16, dst: *mut u16) -> i32;
    fn dev_get_by_index(net: *mut Net, idx: i32) -> *mut NetDevice; fn phonet_device_get(net: *mut Net) -> *mut NetDevice; fn dev_put(d: *mut NetDevice);
    fn phonet_address_get(d: *mut NetDevice, addr: u8) -> u8; fn sk_ioctl(sk: *mut Sock, cmd: u32, arg: *mut core::ffi::c_void) -> i32;
    fn sock_graft(sk: *mut Sock, sock: *mut Socket); fn phonet_get_local_port_range(min: *mut i32, max: *mut i32);
    fn SIOCPNGETOBJECT() -> u32;
}
#[repr(C)] pub struct Task { _private: [u8; 0] }
type u16_ = u16; type u8_ = u8;

pub unsafe fn pn_sock_init() { for i in 0..PN_HASHSIZE { INIT_HLIST_HEAD(pnsocks.hlist.as_mut_ptr().add(i)); } mutex_init(&mut pnsocks.lock); }

pub unsafe fn pn_find_sock_by_sa(net: *mut Net, spn: *const SockaddrPn) -> *mut Sock {
    let obj = pn_sockaddr_get_object(spn); let res = (*spn).spn_resource; let hlist = pn_hash_list(obj); let mut sknode: *mut Sock = core::ptr::null_mut(); let mut rval = core::ptr::null_mut();
    rcu_read_lock();
    while !sknode.is_null() { let pn = pn_sk(sknode); if (*pn).sobject == 0 { break; } if !net_eq(sock_net(sknode), net) { continue; } if pn_port(obj) != 0 { if pn_port((*pn).sobject) != pn_port(obj) { continue; } } else if (*pn).resource != res { continue; } if pn_addr((*pn).sobject) != 0 && pn_addr((*pn).sobject) != pn_addr(obj) { continue; } rval = sknode; sock_hold(sknode); break; }
    rcu_read_unlock(); rval
}

pub unsafe fn pn_deliver_sock_broadcast(net: *mut Net, skb: *mut SkBuff) { let mut hlist = pnsocks.hlist.as_mut_ptr(); rcu_read_lock(); for _ in 0..PN_HASHSIZE { let sknode: *mut Sock = core::ptr::null_mut(); while !sknode.is_null() { if net_eq(sock_net(sknode), net) { let clone = skb_clone(skb, 0); if !clone.is_null() { sock_hold(sknode); sk_receive_skb(sknode, clone, 0); } } } hlist = hlist.add(1); } rcu_read_unlock(); }

pub unsafe fn pn_sock_hash(sk: *mut Sock) -> i32 { let h = pn_hash_list((*pn_sk(sk)).sobject); mutex_lock(&mut pnsocks.lock); sk_add_node_rcu(sk, h); mutex_unlock(&mut pnsocks.lock); 0 }
pub unsafe fn pn_sock_unhash(sk: *mut Sock) { mutex_lock(&mut pnsocks.lock); sk_del_node_init_rcu(sk); mutex_unlock(&mut pnsocks.lock); pn_sock_unbind_all_res(sk); synchronize_rcu(); }

pub unsafe fn pn_sock_get_port(sk: *mut Sock, mut sport: u16) -> i32 { static mut port_cur: i32 = 0; let net = sock_net(sk); let pn = pn_sk(sk); let mut sa = SockaddrPn { spn_family: 0, spn_resource: 0, data: [0; 5] }; sa.spn_family = 0xF5; let mut tmp: *mut Sock; if sport == 0 { let (mut pmin, mut pmax) = (0, 0); phonet_get_local_port_range(&mut pmin, &mut pmax); let mut port = pmin; while port <= pmax { port_cur += 1; if port_cur < pmin || port_cur > pmax { port_cur = pmin; } tmp = core::ptr::null_mut(); if tmp.is_null() { sport = port_cur; break; } else { sock_put(tmp); } port += 1; } } else { tmp = core::ptr::null_mut(); if !tmp.is_null() { sock_put(tmp); } else { (*pn).sobject = pn_object(pn_addr((*pn).sobject), sport); return 0; } } -98 }

pub unsafe fn pn_socket_release(sock: *mut Socket) -> i32 { let sk = (*sock).sk; if !sk.is_null() { (*sock).sk = core::ptr::null_mut(); if let Some(close) = (*(*sk).sk_prot).close { close(sk, 0); } } 0 }

pub unsafe fn pn_socket_bind(sock: *mut Socket, addr: *mut SockaddrUnsized, len: i32) -> i32 {
    let sk = (*sock).sk; let pn = pn_sk(sk); let spn = addr as *mut SockaddrPn; if let Some(bind) = (*(*sk).sk_prot).bind { return bind(sk, addr, len); } if len < core::mem::size_of::<SockaddrPn>() as i32 { return -22; } if (*spn).spn_family != 0xF5 { return -97; }
    let handle = pn_sockaddr_get_object(spn); let saddr = pn_addr(handle); if saddr != 0 && phonet_address_lookup(sock_net(sk), saddr) != 0 { return -99; } lock_sock(sk); let mut err; if (*sk).sk_state != 7 || pn_port((*pn).sobject) != 0 { err = -22; } else { mutex_lock(&mut pnsocks.lock); err = (*(*sk).sk_prot).get_port.map(|f| f(sk, pn_port(handle))).unwrap_or(-22); if err == 0 { (*pn).sobject = pn_object(saddr, pn_port((*pn).sobject)); (*pn).resource = (*spn).spn_resource; err = (*(*sk).sk_prot).hash.map(|f| f(sk)).unwrap_or(0); } mutex_unlock(&mut pnsocks.lock); } release_sock(sk); err
}

pub unsafe fn pn_socket_autobind(sock: *mut Socket) -> i32 { let mut sa = SockaddrPn { spn_family: 0xF5, spn_resource: 0, data: [0; 5] }; let e = pn_socket_bind(sock, (&mut sa as *mut SockaddrPn).cast(), core::mem::size_of::<SockaddrPn>() as i32); if e != -22 || pn_port((*pn_sk((*sock).sk)).sobject) == 0 { e } else { 0 } }

pub unsafe fn pn_socket_connect(sock: *mut Socket, addr: *mut SockaddrUnsized, len: i32, flags: i32) -> i32 { let sk=(*sock).sk; let pn=pn_sk(sk); if pn_socket_autobind(sock)!=0{return -105;} if len < core::mem::size_of::<SockaddrPn>() as i32{return -22;} let spn=addr as *mut SockaddrPn; if (*spn).spn_family != 0xF5{return -97;} lock_sock(sk); if (*sock).state != 0 { release_sock(sk); return -106; } (*pn).dobject=pn_sockaddr_get_object(spn); (*pn).resource=pn_sockaddr_get_resource(spn); (*sock).state=2; let e=(*(*sk).sk_prot).connect.map(|f|f(sk,addr,len)).unwrap_or(-22); if e!=0 {(*sock).state=0;(*pn).dobject=0;} release_sock(sk); e }

pub unsafe fn pn_socket_getname(sock: *mut Socket, addr: *mut Sockaddr, peer: i32) -> i32 { let sk=(*sock).sk; let pn=pn_sk(sk); memset(addr.cast(),0,core::mem::size_of::<SockaddrPn>()); (*addr).sa_family=0xF5; if peer==0 { (*addr).data[..2].copy_from_slice(&(*pn).sobject.to_ne_bytes()); } core::mem::size_of::<SockaddrPn>() as i32 }

pub unsafe fn pn_socket_listen(sock: *mut Socket, backlog: i32) -> i32 { if pn_socket_autobind(sock)!=0{return -105;} let sk=(*sock).sk; lock_sock(sk); let mut e=0; if (*sock).state!=0 {e=-22;} else {(*sk).sk_state=10;(*sk).sk_max_ack_backlog=backlog;} release_sock(sk); e }

pub unsafe fn pn_socket_sendmsg(sock: *mut Socket, m: *mut Msghdr, total_len: usize) -> isize { let sk=(*sock).sk; if pn_socket_autobind(sock)!=0{return -11;} (*(*sk).sk_prot).sendmsg.map(|f|f(sk,m,total_len)).unwrap_or(-22) }

pub unsafe fn pn_socket_accept(sock: *mut Socket, newsock: *mut Socket, arg: *mut ProtoAcceptArg) -> i32 { let sk=(*sock).sk; if (*sk).sk_state != 10{return -22;} let ns=(*(*sk).sk_prot).accept.map(|f|f(sk,arg)).unwrap_or(core::ptr::null_mut()); if ns.is_null(){return (*arg).err;} sock_graft(ns,newsock);(*newsock).state=1;0 }
pub unsafe fn pn_socket_poll(_file:*mut File,sock:*mut Socket,_wait:*mut PollTable)->u32 { let sk=(*sock).sk; if (*sk).sk_state==7 {return 8;} 0 }
pub unsafe fn pn_socket_ioctl(sock:*mut Socket,cmd:u32,arg:usize)->i32 { sk_ioctl((*sock).sk,cmd,arg as *mut core::ffi::c_void) }

// C proto_ops tables are represented by the externally supplied kernel ABI.
extern "C" { pub static phonet_dgram_ops: ProtoOps; pub static phonet_stream_ops: ProtoOps; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
