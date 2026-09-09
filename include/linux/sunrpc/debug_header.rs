/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/debug.h
 *
 * Debugging support for sunrpc module
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

// Dependency: declarations from <uapi/linux/sunrpc/debug.h> are supplied externally.

/*
 * Debugging macros etc
 */
unsafe extern "C" {
    pub static mut rpc_debug: core::ffi::c_uint;
    pub static mut nfs_debug: core::ffi::c_uint;
    pub static mut nfsd_debug: core::ffi::c_uint;
    pub static mut nlm_debug: core::ffi::c_uint;
}

// These macros depend on the externally supplied FACILITY, dfprintk, and dfprintk_rcu symbols.
#[macro_export]
macro_rules! dprintk {
    ($fmt:expr $(, $arg:expr)*) => {
        dfprintk!(FACILITY, $fmt $(, $arg)*)
    };
}

#[macro_export]
macro_rules! dprintk_rcu {
    ($fmt:expr $(, $arg:expr)*) => {
        dfprintk_rcu!(FACILITY, $fmt $(, $arg)*)
    };
}

// CONFIG_SUNRPC_DEBUG and CONFIG_SUNRPC_DEBUG_TRACE are build-time conditions.
// When enabled, ifdebug tests rpc_debug & RPCDBG_##fac and printing uses trace_printk
// when CONFIG_SUNRPC_DEBUG_TRACE is enabled, otherwise printk(KERN_DEFAULT ...).
// When disabled, ifdebug is always false and dfprintk/dfprintk_rcu use no_printk.

#[repr(C)]
pub struct rpc_clnt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rpc_xprt {
    _private: [u8; 0],
}

// Under CONFIG_SUNRPC_DEBUG these are external functions. The non-debug inline
// definitions below preserve the original no-op behavior.
#[cfg(CONFIG_SUNRPC_DEBUG)]
unsafe extern "C" {
    pub fn rpc_register_sysctl();
    pub fn rpc_unregister_sysctl();
    pub fn sunrpc_debugfs_init();
    pub fn sunrpc_debugfs_exit();
    pub fn rpc_clnt_debugfs_register(clnt: *mut rpc_clnt);
    pub fn rpc_clnt_debugfs_unregister(clnt: *mut rpc_clnt);
    pub fn rpc_xprt_debugfs_register(xprt: *mut rpc_xprt);
    pub fn rpc_xprt_debugfs_unregister(xprt: *mut rpc_xprt);
}

#[cfg(not(CONFIG_SUNRPC_DEBUG))]
#[inline]
pub unsafe fn sunrpc_debugfs_init() {
    return;
}

#[cfg(not(CONFIG_SUNRPC_DEBUG))]
#[inline]
pub unsafe fn sunrpc_debugfs_exit() {
    return;
}

#[cfg(not(CONFIG_SUNRPC_DEBUG))]
#[inline]
pub unsafe fn rpc_clnt_debugfs_register(_clnt: *mut rpc_clnt) {
    return;
}

#[cfg(not(CONFIG_SUNRPC_DEBUG))]
#[inline]
pub unsafe fn rpc_clnt_debugfs_unregister(_clnt: *mut rpc_clnt) {
    return;
}

#[cfg(not(CONFIG_SUNRPC_DEBUG))]
#[inline]
pub unsafe fn rpc_xprt_debugfs_register(_xprt: *mut rpc_xprt) {
    return;
}

#[cfg(not(CONFIG_SUNRPC_DEBUG))]
#[inline]
pub unsafe fn rpc_xprt_debugfs_unregister(_xprt: *mut rpc_xprt) {
    return;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
