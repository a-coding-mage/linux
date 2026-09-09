/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/svc_xprt.h
 *
 * RPC server transport I/O
 */

// C dependency: <linux/sunrpc/svc.h>

use core::ffi::c_char;

#[repr(C)]
pub struct module;

#[repr(C)]
pub struct svc_xprt_ops {
    pub xpo_create: Option<unsafe extern "C" fn(*mut svc_serv, *mut net, *mut sockaddr, i32, i32) -> *mut svc_xprt>,
    pub xpo_accept: Option<unsafe extern "C" fn(*mut svc_xprt) -> *mut svc_xprt>,
    pub xpo_has_wspace: Option<unsafe extern "C" fn(*mut svc_xprt) -> i32>,
    pub xpo_recvfrom: Option<unsafe extern "C" fn(*mut svc_rqst) -> i32>,
    pub xpo_sendto: Option<unsafe extern "C" fn(*mut svc_rqst) -> i32>,
    pub xpo_result_payload: Option<unsafe extern "C" fn(*mut svc_rqst, u32, u32) -> i32>,
    pub xpo_release_ctxt: Option<unsafe extern "C" fn(*mut svc_xprt, *mut core::ffi::c_void)>,
    pub xpo_detach: Option<unsafe extern "C" fn(*mut svc_xprt)>,
    pub xpo_free: Option<unsafe extern "C" fn(*mut svc_xprt)>,
    pub xpo_kill_temp_xprt: Option<unsafe extern "C" fn(*mut svc_xprt)>,
    pub xpo_handshake: Option<unsafe extern "C" fn(*mut svc_xprt)>,
}

#[repr(C)]
pub struct svc_xprt_class {
    pub xcl_name: *const c_char,
    pub xcl_owner: *mut module,
    pub xcl_ops: *const svc_xprt_ops,
    pub xcl_list: list_head,
    pub xcl_max_payload: u32,
    pub xcl_ident: i32,
}

#[repr(C)]
pub struct svc_xpt_user {
    pub list: list_head,
    pub callback: Option<unsafe extern "C" fn(*mut svc_xpt_user)>,
}

#[repr(C)]
pub struct svc_xprt {
    pub xpt_class: *mut svc_xprt_class,
    pub xpt_ops: *const svc_xprt_ops,
    pub xpt_ref: kref,
    pub xpt_qtime: ktime_t,
    pub xpt_list: list_head,
    pub xpt_ready: lwq_node,
    pub xpt_flags: c_ulong,
    pub xpt_server: *mut svc_serv,
    pub xpt_reserved: atomic_t,
    pub xpt_nr_rqsts: atomic_t,
    pub xpt_mutex: mutex,
    pub xpt_lock: spinlock_t,
    pub xpt_auth_cache: *mut core::ffi::c_void,
    pub xpt_deferred: list_head,
    pub xpt_local: sockaddr_storage,
    pub xpt_locallen: usize,
    pub xpt_remote: sockaddr_storage,
    pub xpt_remotelen: usize,
    pub xpt_remotebuf: [c_char; INET6_ADDRSTRLEN + 10],
    pub xpt_users: list_head,
    pub xpt_net: *mut net,
    pub ns_tracker: netns_tracker,
    pub xpt_cred: *const cred,
    pub xpt_bc_xprt: *mut rpc_xprt,
    pub xpt_bc_xps: *mut rpc_xprt_switch,
}

pub const XPT_BUSY: i32 = 0;
pub const XPT_CONN: i32 = 1;
pub const XPT_CLOSE: i32 = 2;
pub const XPT_DATA: i32 = 3;
pub const XPT_TEMP: i32 = 4;
pub const XPT_DEAD: i32 = 5;
pub const XPT_CHNGBUF: i32 = 6;
pub const XPT_DEFERRED: i32 = 7;
pub const XPT_OLD: i32 = 8;
pub const XPT_LISTENER: i32 = 9;
pub const XPT_CACHE_AUTH: i32 = 10;
pub const XPT_LOCAL: i32 = 11;
pub const XPT_KILL_TEMP: i32 = 12;
pub const XPT_CONG_CTRL: i32 = 13;
pub const XPT_HANDSHAKE: i32 = 14;
pub const XPT_TLS_SESSION: i32 = 15;
pub const XPT_PEER_AUTH: i32 = 16;
pub const XPT_PEER_VALID: i32 = 17;
pub const XPT_RPCB_UNREG: i32 = 18;

pub const XPT_MAX_TMP_CONN: i32 = 64;

#[inline]
pub unsafe fn svc_xprt_set_valid(xpt: *mut svc_xprt) {
    if test_bit(XPT_TEMP, &(*xpt).xpt_flags) != 0 && test_and_set_bit(XPT_PEER_VALID, &mut (*xpt).xpt_flags) == 0 {
        let serv = (*xpt).xpt_server;
        spin_lock(&mut (*serv).sv_lock);
        (*serv).sv_tmpcnt -= 1;
        spin_unlock(&mut (*serv).sv_lock);
    }
}

#[inline]
pub unsafe fn unregister_xpt_user(xpt: *mut svc_xprt, u: *mut svc_xpt_user) {
    spin_lock(&mut (*xpt).xpt_lock);
    list_del_init(&mut (*u).list);
    spin_unlock(&mut (*xpt).xpt_lock);
}

#[inline]
pub unsafe fn register_xpt_user(xpt: *mut svc_xprt, u: *mut svc_xpt_user) -> i32 {
    spin_lock(&mut (*xpt).xpt_lock);
    if test_bit(XPT_CLOSE, &(*xpt).xpt_flags) != 0 {
        spin_unlock(&mut (*xpt).xpt_lock);
        return -ENOTCONN;
    }
    list_add(&mut (*u).list, &mut (*xpt).xpt_users);
    spin_unlock(&mut (*xpt).xpt_lock);
    0
}

#[inline]
pub unsafe fn svc_xprt_set_local(xprt: *mut svc_xprt, sa: *const sockaddr, salen: usize) {
    memcpy(&mut (*xprt).xpt_local as *mut _ as *mut c_void, sa as *const c_void, salen);
    (*xprt).xpt_locallen = salen;
}

#[inline]
pub unsafe fn svc_xprt_set_remote(xprt: *mut svc_xprt, sa: *const sockaddr, salen: usize) {
    memcpy(&mut (*xprt).xpt_remote as *mut _ as *mut c_void, sa as *const c_void, salen);
    (*xprt).xpt_remotelen = salen;
    snprintf((*xprt).xpt_remotebuf.as_mut_ptr(), core::mem::size_of_val(&(*xprt).xpt_remotebuf) - 1, c"%pISpc".as_ptr(), sa);
}

// The following declarations and inline functions depend on Linux kernel
// types and helpers supplied by the included svc.h and related headers.
extern "C" {
    pub fn svc_reg_xprt_class(class: *mut svc_xprt_class) -> i32;
    pub fn svc_unreg_xprt_class(class: *mut svc_xprt_class);
    pub fn svc_xprt_init(net: *mut net, class: *mut svc_xprt_class, xprt: *mut svc_xprt, serv: *mut svc_serv);
    pub fn svc_xprt_create_from_sa(serv: *mut svc_serv, xprt_name: *const c_char, net: *mut net, sap: *mut sockaddr, flags: i32, cred: *const cred) -> i32;
    pub fn svc_xprt_create(serv: *mut svc_serv, xprt_name: *const c_char, net: *mut net, family: i32, port: u16, flags: i32, cred: *const cred) -> i32;
    pub fn svc_xprt_destroy_all(serv: *mut svc_serv, net: *mut net, unregister: bool);
    pub fn svc_xprt_received(xprt: *mut svc_xprt);
    pub fn svc_xprt_enqueue(xprt: *mut svc_xprt);
    pub fn svc_xprt_put(xprt: *mut svc_xprt);
    pub fn svc_xprt_copy_addrs(rqstp: *mut svc_rqst, xprt: *mut svc_xprt);
    pub fn svc_xprt_close(xprt: *mut svc_xprt);
    pub fn svc_port_is_privileged(sin: *mut sockaddr) -> i32;
    pub fn svc_print_xprts(buf: *mut c_char, maxlen: i32) -> i32;
    pub fn svc_find_listener(serv: *mut svc_serv, xcl_name: *const c_char, net: *mut net, sa: *const sockaddr) -> *mut svc_xprt;
    pub fn svc_find_xprt(serv: *mut svc_serv, xcl_name: *const c_char, net: *mut net, af: sa_family_t, port: u16) -> *mut svc_xprt;
    pub fn svc_xprt_names(serv: *mut svc_serv, buf: *mut c_char, buflen: i32) -> i32;
    pub fn svc_add_new_perm_xprt(serv: *mut svc_serv, xprt: *mut svc_xprt);
    pub fn svc_age_temp_xprts_now(serv: *mut svc_serv, sa: *mut sockaddr);
    pub fn svc_xprt_deferred_close(xprt: *mut svc_xprt);
}

#[inline]
pub unsafe fn svc_xprt_is_dead(xprt: *const svc_xprt) -> bool {
    test_bit(XPT_DEAD, &(*xprt).xpt_flags) != 0 || test_bit(XPT_CLOSE, &(*xprt).xpt_flags) != 0
}

#[inline]
pub unsafe fn svc_xprt_get(xprt: *mut svc_xprt) { kref_get(&mut (*xprt).xpt_ref); }

#[inline]
pub unsafe fn svc_xprt_local_port(xprt: *const svc_xprt) -> u16 {
    svc_addr_port(&(*xprt).xpt_local as *const _ as *const sockaddr)
}

#[inline]
pub unsafe fn svc_xprt_remote_port(xprt: *const svc_xprt) -> u16 {
    svc_addr_port(&(*xprt).xpt_remote as *const _ as *const sockaddr)
}

#[inline]
pub unsafe fn svc_addr_port(sa: *const sockaddr) -> u16 {
    match (*sa).sa_family {
        AF_INET => ntohs((sa as *const sockaddr_in).read().sin_port),
        AF_INET6 => ntohs((sa as *const sockaddr_in6).read().sin6_port),
        _ => 0,
    }
}

#[inline]
pub unsafe fn svc_addr_len(sa: *const sockaddr) -> usize {
    match (*sa).sa_family {
        AF_INET => core::mem::size_of::<sockaddr_in>(),
        AF_INET6 => core::mem::size_of::<sockaddr_in6>(),
        _ => { BUG(); 0 }
    }
}

#[inline]
pub unsafe fn __svc_print_addr(addr: *const sockaddr, buf: *mut c_char, len: usize) -> *mut c_char {
    // Kernel snprintf formatting (%pI4/%pI6) is preserved as an external dependency.
    match (*addr).sa_family {
        AF_INET => snprintf(buf, len, c"%pI4, port=%u".as_ptr(), &(*(addr as *const sockaddr_in)).sin_addr, ntohs((addr as *const sockaddr_in).read().sin_port)),
        AF_INET6 => snprintf(buf, len, c"%pI6, port=%u".as_ptr(), &(*(addr as *const sockaddr_in6)).sin6_addr, ntohs((addr as *const sockaddr_in6).read().sin6_port)),
        _ => snprintf(buf, len, c"unknown address type: %d".as_ptr(), (*addr).sa_family),
    };
    buf
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
