// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// The following items are enabled when CONFIG_XFS_LIVE_HOOKS is enabled.
#[cfg(CONFIG_XFS_LIVE_HOOKS)]
#[repr(C)]
pub struct xfs_hooks {
    pub head: blocking_notifier_head,
}

#[cfg(CONFIG_XFS_LIVE_HOOKS)]
#[macro_export]
macro_rules! DEFINE_STATIC_XFS_HOOK_SWITCH {
    ($name:ident) => {
        static $name: static_key_false = static_key_false;
    };
}

#[cfg(CONFIG_XFS_LIVE_HOOKS)]
#[macro_export]
macro_rules! xfs_hooks_switch_on {
    ($name:ident) => {
        static_branch_inc!($name);
    };
}

#[cfg(CONFIG_XFS_LIVE_HOOKS)]
#[macro_export]
macro_rules! xfs_hooks_switch_off {
    ($name:ident) => {
        static_branch_dec!($name);
    };
}

#[cfg(CONFIG_XFS_LIVE_HOOKS)]
#[macro_export]
macro_rules! xfs_hooks_switched_on {
    ($name:ident) => {
        static_branch_unlikely!($name)
    };
}

#[cfg(CONFIG_XFS_LIVE_HOOKS)]
#[repr(C)]
pub struct xfs_hook {
    /* This must come at the start of the structure. */
    pub nb: notifier_block,
}

#[cfg(CONFIG_XFS_LIVE_HOOKS)]
pub type xfs_hook_fn_t = unsafe extern "C" fn(
    hook: *mut xfs_hook,
    action: ::core::ffi::c_ulong,
    data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;

#[cfg(CONFIG_XFS_LIVE_HOOKS)]
extern "C" {
    pub fn xfs_hooks_init(chain: *mut xfs_hooks);
    pub fn xfs_hooks_add(chain: *mut xfs_hooks, hook: *mut xfs_hook) -> ::core::ffi::c_int;
    pub fn xfs_hooks_del(chain: *mut xfs_hooks, hook: *mut xfs_hook);
    pub fn xfs_hooks_call(
        chain: *mut xfs_hooks,
        action: ::core::ffi::c_ulong,
        priv_: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_XFS_LIVE_HOOKS)]
#[inline]
pub unsafe fn xfs_hook_setup(hook: *mut xfs_hook, func: notifier_fn_t) {
    (*hook).nb.notifier_call = func;
    (*hook).nb.priority = 0;
}

// Without CONFIG_XFS_LIVE_HOOKS, xfs_hooks is intentionally empty and the
// hook operations are no-ops, matching the C preprocessor branch.
#[cfg(not(CONFIG_XFS_LIVE_HOOKS))]
#[repr(C)]
pub struct xfs_hooks {}

#[cfg(not(CONFIG_XFS_LIVE_HOOKS))]
#[macro_export]
macro_rules! DEFINE_STATIC_XFS_HOOK_SWITCH {
    ($name:ident) => {};
}

#[cfg(not(CONFIG_XFS_LIVE_HOOKS))]
#[macro_export]
macro_rules! xfs_hooks_switch_on {
    ($name:ident) => {{}};
}

#[cfg(not(CONFIG_XFS_LIVE_HOOKS))]
#[macro_export]
macro_rules! xfs_hooks_switch_off {
    ($name:ident) => {{}};
}

#[cfg(not(CONFIG_XFS_LIVE_HOOKS))]
#[macro_export]
macro_rules! xfs_hooks_switched_on {
    ($name:ident) => {
        false
    };
}

#[cfg(not(CONFIG_XFS_LIVE_HOOKS))]
#[macro_export]
macro_rules! xfs_hooks_init {
    ($chain:expr) => {{}};
}

#[cfg(not(CONFIG_XFS_LIVE_HOOKS))]
#[macro_export]
macro_rules! xfs_hooks_call {
    ($chain:expr, $val:expr, $priv_:expr) => {
        NOTIFY_DONE
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
