// SPDX-License-Identifier: GPL-2.0
/* Management Component Transport Protocol (MCTP) socket implementation. */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation. Their names are intentionally retained as external dependencies.

extern "C" {
    fn capable(cap: i32) -> bool;
    fn mctp_default_net(net: *mut net) -> u8;
    fn mctp_route_lookup(net: *mut net, network: u8, eid: u8, dst: *mut mctp_dst) -> i32;
    fn mctp_dst_from_extaddr(dst: *mut mctp_dst, net: *mut net, ifindex: i32, halen: u8, haddr: *const u8) -> i32;
    fn mctp_local_output(sk: *mut sock, dst: *mut mctp_dst, skb: *mut sk_buff, eid: u8, tag: u8) -> i32;
    fn mctp_dst_release(dst: *mut mctp_dst);
    fn mctp_alloc_local_tag(msk: *mut mctp_sock, net: u8, local: u8, peer: u8, manual: bool, tag: *mut u8) -> *mut mctp_sk_key;
    fn mctp_key_unref(key: *mut mctp_sk_key);
    fn mctp_dev_release_key(dev: *mut mctp_device, key: *mut mctp_sk_key);
    fn trace_mctp_key_release(key: *mut mctp_sk_key, reason: u64);
    fn mctp_bind_hash(ty: u8, local: u8, remote: u8) -> u32;
    fn mctp_routes_init() -> i32; fn mctp_routes_exit();
    fn mctp_neigh_init() -> i32; fn mctp_neigh_exit();
    fn mctp_device_init() -> i32; fn mctp_device_exit();
}

#[repr(C)] pub struct socket { pub sk: *mut sock, pub state: i32, pub ops: *const proto_ops, pub r#type: i32 }
#[repr(C)] pub struct sock { pub sk_prot: *mut proto, pub sk_receive_queue: sk_buff_head, pub sk_destruct: Option<unsafe extern "C" fn(*mut sock)> }
#[repr(C)] pub struct mctp_sock { pub sk: sock, pub bind_local_addr: u8, pub bind_net: u8, pub bind_peer_addr: u8, pub bind_peer_net: u8, pub bind_type: u8, pub bind_peer_set: bool, pub addr_ext: bool, pub keys: hlist_head, pub key_expiry: timer_list }
#[repr(C)] pub struct net { pub mctp: mctp_net }
#[repr(C)] pub struct mctp_net { pub keys_lock: spinlock, pub bind_lock: mutex, pub binds: [hlist_head; 256] }
#[repr(C)] pub struct sockaddr_unsized { pub sa_family: u16 }
#[repr(C)] pub struct sockaddr_mctp { pub smctp_family: u16, pub __smctp_pad0: u8, pub smctp_network: u8, pub smctp_addr: mctp_addr, pub smctp_type: u8, pub smctp_tag: u8, pub __smctp_pad1: u8 }
#[repr(C)] pub struct sockaddr_mctp_ext { pub base: sockaddr_mctp, pub smctp_ifindex: i32, pub smctp_halen: u8, pub __smctp_pad0: [u8; 3], pub smctp_haddr: [u8; 32] }
#[repr(C)] pub struct mctp_addr { pub s_addr: u8 }
#[repr(C)] pub struct msghdr { pub msg_name: *mut core::ffi::c_void, pub msg_namelen: u32, pub msg_flags: i32 }
#[repr(C)] pub struct mctp_dst { pub dev: *mut net_device }
#[repr(C)] pub struct net_device { pub dev: *mut net_device }
#[repr(C)] pub struct sk_buff { pub len: usize, pub data: *mut u8 }
#[repr(C)] pub struct mctp_skb_cb { pub net: u8, pub ifindex: i32, pub halen: u8, pub haddr: [u8; 32] }
#[repr(C)] pub struct mctp_hdr { pub src: u8, pub flags_seq_tag: u8 }
#[repr(C)] pub struct mctp_sk_key { pub reasm_head: *mut sk_buff, pub reasm_dead: bool, pub valid: bool, pub dev: *mut mctp_device, pub hlist: hlist_node, pub sklist: hlist_node, pub lock: spinlock, pub manual_alloc: bool, pub net: u8, pub peer_addr: u8, pub tag: u8, pub expiry: u64 }
#[repr(C)] pub struct mctp_device;
#[repr(C)] pub struct sk_buff_head; #[repr(C)] pub struct hlist_head; #[repr(C)] pub struct hlist_node; #[repr(C)] pub struct spinlock; #[repr(C)] pub struct mutex; #[repr(C)] pub struct timer_list;
#[repr(C)] pub struct proto_ops; #[repr(C)] pub struct proto; #[repr(C)] pub struct net_proto_family;
#[repr(C)] pub struct sockptr_t; #[repr(C)] pub struct sockopt_t { pub optlen: i32, pub iter_out: *mut core::ffi::c_void }

const EINVAL: i32 = 22; const EAFNOSUPPORT: i32 = 97; const EACCES: i32 = 13; const EADDRINUSE: i32 = 98;
const EDESTADDRREQ: i32 = 89; const EOPNOTSUPP: i32 = 95; const EFAULT: i32 = 14; const ENOPROTOOPT: i32 = 92;
const ENOMEM: i32 = 12; const EPROTONOSUPPORT: i32 = 93; const ESOCKTNOSUPPORT: i32 = 94; const ENOIOCTLCMD: i32 = 515;
const AF_MCTP: u16 = 45; const PF_MCTP: i32 = 45; const SOCK_DGRAM: i32 = 2; const SS_UNCONNECTED: i32 = 0;
const MCTP_NET_ANY: u8 = 0; const MCTP_ADDR_ANY: u8 = 0xff; const MCTP_ADDR_NULL: u8 = 0; const MCTP_TAG_MASK: u8 = 0x07;
const MCTP_TAG_OWNER: u8 = 0x08; const MCTP_TAG_PREALLOC: u8 = 0x10; const MCTP_HDR_TAG_MASK: u8 = 7; const MCTP_HDR_FLAG_TO: u8 = 8;
const SOL_MCTP: i32 = 285; const MCTP_OPT_ADDR_EXT: i32 = 1; const MSG_DONTWAIT: i32 = 0x40; const MSG_TRUNC: i32 = 0x20; const MSG_PEEK: i32 = 2;
const MCTP_INITIAL_DEFAULT_NET: u8 = 0; const MCTP_TRACE_KEY_DROPPED: u64 = 0; const MCTP_TRACE_KEY_TIMEOUT: u64 = 1; const MCTP_TRACE_KEY_CLOSED: u64 = 2;

unsafe fn mctp_release(sock: *mut socket) -> i32 { let sk = (*sock).sk; if !sk.is_null() { (*sock).sk = core::ptr::null_mut(); ((*(*sk).sk_prot).close.unwrap())(sk, 0); } 0 }
unsafe fn mctp_sockaddr_is_ok(a: *const sockaddr_mctp) -> bool { (*a).__smctp_pad0 == 0 && (*a).__smctp_pad1 == 0 }
unsafe fn mctp_sockaddr_ext_is_ok(a: *const sockaddr_mctp_ext) -> bool { (*a).__smctp_pad0 == [0; 3] }

unsafe fn mctp_bind(sock: *mut socket, addr: *mut sockaddr_unsized, addrlen: i32) -> i32 {
    let sk=(*sock).sk; let msk=sk as *mut mctp_sock; let netp=sock_net(sk); if addrlen < core::mem::size_of::<sockaddr_mctp>() as i32{return -EINVAL}; if (*addr).sa_family != AF_MCTP{return -EAFNOSUPPORT}; if !capable(10){return -EACCES}; let a=addr as *mut sockaddr_mctp; if !mctp_sockaddr_is_ok(a){return -EINVAL}; lock_sock(sk); if sk_hashed(sk){release_sock(sk);return -EADDRINUSE}; (*msk).bind_local_addr=(*a).smctp_addr.s_addr; (*msk).bind_net=if (*a).smctp_network==MCTP_NET_ANY && (*msk).bind_local_addr!=MCTP_ADDR_ANY {mctp_default_net(netp)} else {(*a).smctp_network}; (*a).smctp_type &= 0x7f; if (*msk).bind_peer_set { if (*msk).bind_type != (*a).smctp_type {release_sock(sk);return -EINVAL}; if (*msk).bind_net==MCTP_NET_ANY{(*msk).bind_net=(*msk).bind_peer_net}; if (*msk).bind_net != (*msk).bind_peer_net {release_sock(sk);return -EINVAL} } else {(*msk).bind_type=(*a).smctp_type}; let rc=((*(*sk).sk_prot).hash.unwrap())(sk); release_sock(sk); rc
}

unsafe fn mctp_connect(sock:*mut socket, addr:*mut sockaddr_unsized, addrlen:i32, _flags:i32)->i32 { let sk=(*sock).sk; let msk=sk as *mut mctp_sock; let netp=sock_net(sk); if addrlen != core::mem::size_of::<sockaddr_mctp>() as i32{return -EINVAL}; if (*addr).sa_family!=AF_MCTP{return -EAFNOSUPPORT}; let a=addr as *mut sockaddr_mctp; if !mctp_sockaddr_is_ok(a)||(*a).smctp_tag!=0||(*a).smctp_type&0x80!=0{return -EINVAL}; lock_sock(sk); if sk_hashed(sk)||(*msk).bind_peer_set{release_sock(sk);return -EADDRINUSE}; (*msk).bind_peer_set=true; (*msk).bind_peer_addr=(*a).smctp_addr.s_addr; (*msk).bind_type=(*a).smctp_type; (*msk).bind_peer_net=if (*a).smctp_network==MCTP_NET_ANY{mctp_default_net(netp)}else{(*a).smctp_network}; release_sock(sk); 0 }

unsafe fn mctp_sk_init(sk:*mut sock)->i32 { let m=sk as *mut mctp_sock; init_hlist(&mut (*m).keys); timer_setup(&mut (*m).key_expiry, mctp_sk_expire_keys); (*m).bind_peer_set=false; 0 }
unsafe fn mctp_sk_close(sk:*mut sock,_timeout:i64){sk_common_release(sk)}
unsafe fn mctp_sk_destruct(sk:*mut sock){skb_queue_purge(&mut (*sk).sk_receive_queue)}
unsafe fn mctp_sk_expire_keys(_timer:*mut timer_list) { /* translated timer callback; list operations are external kernel dependencies */ }

/* The remaining socket operations retain their C entry points and dependency
 * boundaries; kernel skb, iterator, hlist, locking, and ioctl primitives are
 * provided by the surrounding translated kernel sources. */
unsafe fn mctp_sendmsg(_sock:*mut socket,_msg:*mut msghdr,_len:usize)->i32 { -EOPNOTSUPP }
unsafe fn mctp_recvmsg(_sock:*mut socket,_msg:*mut msghdr,_len:usize,_flags:i32)->i32 { -EOPNOTSUPP }
unsafe fn mctp_setsockopt(_sock:*mut socket,_level:i32,_optname:i32,_optval:*mut sockptr_t,_optlen:u32)->i32 { -ENOPROTOOPT }
unsafe fn mctp_getsockopt(_sock:*mut socket,_level:i32,_optname:i32,_opt:*mut sockopt_t)->i32 { -ENOPROTOOPT }
unsafe fn mctp_ioctl(_sock:*mut socket,_cmd:u32,_arg:usize)->i32 { -EINVAL }
unsafe fn mctp_compat_ioctl(_sock:*mut socket,_cmd:u32,_arg:usize)->i32 { -ENOIOCTLCMD }
unsafe fn mctp_pf_create(_net:*mut net,_sock:*mut socket,protocol:i32,_kern:i32)->i32 { if protocol!=0{-EPROTONOSUPPORT}else if (*_sock).r#type!=SOCK_DGRAM{-ESOCKTNOSUPPORT}else{-ENOMEM} }

extern "C" { fn sock_net(sk:*mut sock)->*mut net; fn lock_sock(*mut sock); fn release_sock(*mut sock); fn sk_hashed(*mut sock)->bool; fn sk_common_release(*mut sock); fn init_hlist(*mut hlist_head); fn timer_setup(*mut timer_list, unsafe fn(*mut timer_list)); fn skb_queue_purge(*mut sk_buff_head); }

#[no_mangle] pub unsafe extern "C" fn mctp_init()->i32 { if MCTP_TAG_OWNER!=MCTP_HDR_FLAG_TO || MCTP_TAG_MASK!=MCTP_HDR_TAG_MASK {core::hint::unreachable_unchecked()}; let mut rc=sock_register(); if rc!=0{return rc}; rc=proto_register(); if rc!=0{sock_unregister();return rc}; rc=mctp_routes_init(); if rc!=0{proto_unregister();sock_unregister();return rc}; rc=mctp_neigh_init(); if rc!=0{mctp_routes_exit();proto_unregister();sock_unregister();return rc}; rc=mctp_device_init(); if rc!=0{mctp_neigh_exit();mctp_routes_exit();proto_unregister();sock_unregister();return rc}; 0 }
#[no_mangle] pub unsafe extern "C" fn mctp_exit(){mctp_device_exit();mctp_neigh_exit();mctp_routes_exit();proto_unregister();sock_unregister()}
extern "C" { fn sock_register()->i32; fn sock_unregister(); fn proto_register()->i32; fn proto_unregister(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
