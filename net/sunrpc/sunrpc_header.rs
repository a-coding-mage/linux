/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************

(c) 2008 NetApp.  All Rights Reserved.


******************************************************************************/

/*
 * Functions and macros used internally by RPC
 *
 * The declarations referenced by this header are supplied by the corresponding
 * networking/RPC translation units.
 */

use core::ffi::{c_char, c_int, c_void};

/* Header for dynamically allocated rpc buffers. */
#[repr(C)]
pub struct rpc_buffer {
    pub len: usize,
    pub data: [c_char; 0],
}

/*
 * C inline function.  `sock`, `dst_entry`, and the RCU/network symbols are
 * defined by the translated Linux networking dependencies.
 */
#[inline]
pub unsafe fn sock_is_loopback(sk: *mut sock) -> c_int {
    let dst: *mut dst_entry;
    let mut loopback: c_int = 0;

    rcu_read_lock();
    dst = rcu_dereference((*sk).sk_dst_cache);
    if !dst.is_null()
        && !(*dst).dev.is_null()
        && ((*(*dst).dev).features & NETIF_F_LOOPBACK) != 0
    {
        loopback = 1;
    }
    rcu_read_unlock();
    loopback
}

/* Opaque types supplied by the networking/RPC dependencies. */
pub enum sock {}
pub enum dst_entry {}
pub enum svc_serv {}
pub enum svc_rqst {}
pub enum svc_auth_status {}

extern "C" {
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn rcu_dereference<T>(ptr: *mut T) -> *mut T;

    pub fn rpc_clients_notifier_register() -> c_int;
    pub fn rpc_clients_notifier_unregister();
    pub fn auth_domain_cleanup();
    pub fn svc_sock_update_bufs(serv: *mut svc_serv);
    pub fn svc_authenticate(rqstp: *mut svc_rqst) -> svc_auth_status;
}

/* Supplied by <linux/net.h>; retained as an external dependency symbol. */
extern "C" {
    static NETIF_F_LOOPBACK: usize;
}

/* `c_void` is retained for compatibility with C networking declarations. */
#[allow(dead_code)]
type _SunrpcVoid = c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
