// SPDX-License-Identifier: GPL-2.0
// Kernel headers and build-time declarations are supplied by the surrounding crate.

use core::ffi::c_void;

static mut SOCK_DIAG_HANDLERS: [*const sock_diag_handler; AF_MAX as usize] = [core::ptr::null(); AF_MAX as usize];
static mut INET_RCV_COMPAT: *const sock_diag_inet_compat = core::ptr::null();
static mut BROADCAST_WQ: *mut workqueue_struct = core::ptr::null_mut();

// DEFINE_COOKIE(sock_cookie)
static mut SOCK_COOKIE: cookie = cookie { _private: 0 };

#[repr(C)]
pub struct cookie { _private: u64 }
pub enum sock_diag_handler {}
pub enum sock_diag_inet_compat {}
pub enum workqueue_struct {}
pub enum sock {}
pub enum sk_buff {}
pub enum nlmsghdr {}
pub enum net {}
pub enum work_struct {}
pub enum nlattr {}
pub enum sk_filter {}
pub enum sock_fprog_kern {}
pub enum netlink_ext_ack {}

extern "C" {
    fn atomic64_read(p: *const c_void) -> u64;
    fn gen_cookie_next(c: *mut cookie) -> u64;
    fn atomic64_cmpxchg(p: *mut c_void, old: u64, new: u64) -> u64;
    fn sock_gen_cookie(sk: *mut sock) -> u64;
    fn sk_get_meminfo(sk: *mut sock, mem: *mut u32);
    fn nla_put(skb: *mut sk_buff, attrtype: i32, len: usize, data: *const c_void) -> i32;
    fn nla_reserve(skb: *mut sk_buff, attrtype: i32, len: usize) -> *mut nlattr;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference<T>(p: *const T) -> *const T;
    fn rcu_access_pointer<T>(p: *const T) -> *const T;
    fn bpf_classic_proglen(p: *const sock_fprog_kern) -> usize;
    fn nla_data(a: *mut nlattr) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize);
    fn nlmsg_new(size: usize, flags: u32) -> *mut sk_buff;
    fn try_module_get(owner: *mut c_void) -> bool;
    fn module_put(owner: *mut c_void);
    fn sock_diag_destroy_group(sk: *mut sock) -> i32;
    fn nlmsg_multicast(nlsk: *mut c_void, skb: *mut sk_buff, portid: u32, group: i32, flags: u32) -> i32;
    fn kfree_skb(skb: *mut sk_buff);
    fn sk_destruct(sk: *mut sock);
    fn kfree(p: *mut c_void);
    fn kmalloc_obj<T>(flags: u32) -> *mut T;
    fn init_work(work: *mut work_struct, f: unsafe extern "C" fn(*mut work_struct));
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn xchg<T>(p: *mut T, v: T) -> T;
    fn cmpxchg<T>(p: *mut T, old: T, new: T) -> T;
    fn sock_load_diag_module(family: i32, flags: i32);
    fn netlink_rcv_skb(skb: *mut sk_buff, f: unsafe extern "C" fn(*mut sk_buff, *mut nlmsghdr, *mut netlink_ext_ack) -> i32) -> i32;
    fn ns_capable(user_ns: *mut c_void, cap: i32) -> bool;
    fn netlink_kernel_create(net: *mut net, proto: i32, cfg: *const netlink_kernel_cfg) -> *mut c_void;
    fn netlink_kernel_release(sk: *mut c_void);
    fn register_pernet_subsys(ops: *mut pernet_operations) -> i32;
    fn alloc_workqueue(name: *const u8, flags: u32, max_active: u32) -> *mut workqueue_struct;
}

const AF_MAX: i32 = 46;
const INET_DIAG_NOCOOKIE: u32 = !0;
const EINVAL: i32 = 22;
const ESTALE: i32 = 116;
const EMSGSIZE: i32 = 90;
const ENOENT: i32 = 2;
const EOPNOTSUPP: i32 = 95;
const ENOMEM: i32 = 12;
const EPERM: i32 = 1;
const GFP_KERNEL: u32 = 0;
const GFP_ATOMIC: u32 = 0;
const SK_MEMINFO_VARS: usize = 0;
const SOCK_DIAG_BY_FAMILY: u16 = 20;
const SOCK_DESTROY: u16 = 21;
const TCPDIAG_GETSOCK: u16 = 18;
const SKNLGRP_NONE: i32 = 0;
const SKNLGRP_MAX: u32 = 0;
const NETLINK_SOCK_DIAG: i32 = 4;
const CAP_NET_ADMIN: i32 = 12;
const WQ_PERCPU: u32 = 0;
const NL_CFG_F_NONROOT_RECV: u32 = 0;

#[repr(C)] struct netlink_kernel_cfg { groups: u32, input: Option<unsafe extern "C" fn(*mut sk_buff)>, bind: Option<unsafe extern "C" fn(*mut net, i32) -> i32>, flags: u32 }
#[repr(C)] struct pernet_operations { init: Option<unsafe extern "C" fn(*mut net) -> i32>, exit: Option<unsafe extern "C" fn(*mut net)> }

#[repr(C)] struct broadcast_sk { sk: *mut sock, work: work_struct }

#[no_mangle]
pub unsafe extern "C" fn __sock_gen_cookie(sk: *mut sock) -> u64 {
    let mut res = atomic64_read(sk as *const c_void);
    if res == 0 { let new = gen_cookie_next(&mut SOCK_COOKIE); atomic64_cmpxchg(sk as *mut c_void, res, new); res = atomic64_read(sk as *const c_void); }
    res
}

#[no_mangle] pub unsafe extern "C" fn sock_diag_check_cookie(sk: *mut sock, cookie_: *const u32) -> i32 { if (*cookie_ == INET_DIAG_NOCOOKIE) && (*cookie_.add(1) == INET_DIAG_NOCOOKIE) { return 0; } let res = sock_gen_cookie(sk); if res as u32 != *cookie_ || (res >> 32) as u32 != *cookie_.add(1) { return -ESTALE; } 0 }
#[no_mangle] pub unsafe extern "C" fn sock_diag_save_cookie(sk: *mut sock, cookie_: *mut u32) { let res = sock_gen_cookie(sk); *cookie_ = res as u32; *cookie_.add(1) = (res >> 32) as u32; }
#[no_mangle] pub unsafe extern "C" fn sock_diag_put_meminfo(sk: *mut sock, skb: *mut sk_buff, attrtype: i32) -> i32 { let mut mem = [0u32; SK_MEMINFO_VARS]; sk_get_meminfo(sk, mem.as_mut_ptr()); nla_put(skb, attrtype, core::mem::size_of_val(&mem), mem.as_ptr() as *const c_void) }
#[no_mangle] pub unsafe extern "C" fn sock_diag_put_filterinfo(may_report: bool, _sk: *mut sock, skb: *mut sk_buff, attrtype: i32) -> i32 { if !may_report { nla_reserve(skb, attrtype, 0); return 0; } rcu_read_lock(); let _filter: *const sk_filter = core::ptr::null(); rcu_read_unlock(); 0 }

#[no_mangle] pub unsafe extern "C" fn sock_diag_register_inet_compat(ptr: *const sock_diag_inet_compat) { xchg(&mut INET_RCV_COMPAT, ptr); }
#[no_mangle] pub unsafe extern "C" fn sock_diag_unregister_inet_compat(ptr: *const sock_diag_inet_compat) { let old = xchg(&mut INET_RCV_COMPAT, core::ptr::null()); let _ = (old, ptr); }
#[no_mangle] pub unsafe extern "C" fn sock_diag_register(hndl: *const sock_diag_handler) -> i32 { let _ = hndl; 0 }
#[no_mangle] pub unsafe extern "C" fn sock_diag_unregister(hndl: *const sock_diag_handler) { let _ = hndl; }

#[no_mangle] pub unsafe extern "C" fn sock_diag_destroy(sk: *mut sock, _err: i32) -> i32 { let _ = sk; -EOPNOTSUPP }

unsafe extern "C" fn sock_diag_rcv_msg(_skb: *mut sk_buff, _nlh: *mut nlmsghdr, _extack: *mut netlink_ext_ack) -> i32 { -EINVAL }
unsafe extern "C" fn sock_diag_rcv(skb: *mut sk_buff) { netlink_rcv_skb(skb, sock_diag_rcv_msg); }
unsafe extern "C" fn sock_diag_bind(_net: *mut net, _group: i32) -> i32 { 0 }

unsafe extern "C" fn diag_net_init(net_: *mut net) -> i32 {
    let cfg = netlink_kernel_cfg { groups: SKNLGRP_MAX, input: Some(sock_diag_rcv), bind: Some(sock_diag_bind), flags: NL_CFG_F_NONROOT_RECV };
    let _ = netlink_kernel_create(net_, NETLINK_SOCK_DIAG, &cfg);
    0
}
unsafe extern "C" fn diag_net_exit(net_: *mut net) { let _ = net_; }
static mut DIAG_NET_OPS: pernet_operations = pernet_operations { init: Some(diag_net_init), exit: Some(diag_net_exit) };

unsafe extern "C" fn sock_diag_init() -> i32 {
    BROADCAST_WQ = alloc_workqueue(b"sock_diag_events\0".as_ptr(), WQ_PERCPU, 0);
    register_pernet_subsys(&mut DIAG_NET_OPS)
}

// device_initcall(sock_diag_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
