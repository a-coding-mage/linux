/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/stats.h
 *
 * Client statistics collection for SUN RPC
 *
 * Copyright (C) 1996 Olaf Kirch <okir@monad.swb.de>
 */

/* Dependency: linux/proc_fs.h */

use core::ffi::c_char;

#[repr(C)]
pub struct rpc_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct svc_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rpc_stat {
    pub program: *const rpc_program,

    pub netcnt: ::core::ffi::c_uint,
    pub netudpcnt: ::core::ffi::c_uint,
    pub nettcpcnt: ::core::ffi::c_uint,
    pub nettcpconn: ::core::ffi::c_uint,
    pub netreconn: ::core::ffi::c_uint,
    pub rpccnt: ::core::ffi::c_uint,
    pub rpcretrans: ::core::ffi::c_uint,
    pub rpcauthrefresh: ::core::ffi::c_uint,
    pub rpcgarbage: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct svc_stat {
    pub program: *mut svc_program,

    pub netcnt: ::core::ffi::c_uint,
    pub netudpcnt: ::core::ffi::c_uint,
    pub nettcpcnt: ::core::ffi::c_uint,
    pub nettcpconn: ::core::ffi::c_uint,
    pub rpccnt: ::core::ffi::c_uint,
    pub rpcbadfmt: ::core::ffi::c_uint,
    pub rpcbadauth: ::core::ffi::c_uint,
    pub rpcbadclnt: ::core::ffi::c_uint,

    /* Per-version per-procedure call counts (per-cpu, per-netns) */
    pub vs_count: *mut *mut ::core::ffi::c_ulong,
}

extern "C" {
    pub fn svc_stat_alloc_counts(statp: *mut svc_stat) -> ::core::ffi::c_int;
    pub fn svc_stat_free_counts(statp: *mut svc_stat);
}

#[cfg(CONFIG_PROC_FS)]
extern "C" {
    pub fn rpc_proc_init(net: *mut net) -> ::core::ffi::c_int;
    pub fn rpc_proc_exit(net: *mut net);
    pub fn rpc_proc_register(
        net: *mut net,
        stat: *mut rpc_stat,
    ) -> *mut proc_dir_entry;
    pub fn rpc_proc_unregister(net: *mut net, p: *const c_char);
    pub fn rpc_proc_zero(program: *const rpc_program);
    pub fn svc_proc_register(
        net: *mut net,
        stat: *mut svc_stat,
        proc_ops: *const proc_ops,
    ) -> *mut proc_dir_entry;
    pub fn svc_proc_unregister(net: *mut net, p: *const c_char);
    pub fn svc_seq_show(seq: *mut seq_file, stat: *const svc_stat);
}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn rpc_proc_init(_net: *mut net) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn rpc_proc_exit(_net: *mut net) {}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn rpc_proc_register(_net: *mut net, _s: *mut rpc_stat) -> *mut proc_dir_entry {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn rpc_proc_unregister(_net: *mut net, _p: *const c_char) {}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn rpc_proc_zero(_p: *const rpc_program) {}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn svc_proc_register(
    _net: *mut net,
    _s: *mut svc_stat,
    _proc_ops: *const proc_ops,
) -> *mut proc_dir_entry {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn svc_proc_unregister(_net: *mut net, _p: *const c_char) {}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn svc_seq_show(_seq: *mut seq_file, _st: *const svc_stat) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
