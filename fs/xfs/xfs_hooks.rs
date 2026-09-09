// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Dependencies supplied by the surrounding translation unit:
// xfs_platform.h, xfs_fs.h, xfs_shared.h, xfs_format.h,
// xfs_trans_resv.h, xfs_mount.h, xfs_ag.h, and xfs_trace.h.

/* Initialize a notifier chain. */
pub unsafe fn xfs_hooks_init(chain: *mut xfs_hooks) {
    BLOCKING_INIT_NOTIFIER_HEAD(unsafe { &mut (*chain).head });
}

/* Make it so a function gets called whenever we hit a certain hook point. */
pub unsafe fn xfs_hooks_add(chain: *mut xfs_hooks, hook: *mut xfs_hook) -> ::core::ffi::c_int {
    ASSERT(unsafe { (*hook).nb.notifier_call } != None);
    BUILD_BUG_ON(::core::mem::offset_of!(xfs_hook, nb) != 0);

    blocking_notifier_chain_register(
        unsafe { &mut (*chain).head },
        unsafe { &mut (*hook).nb },
    )
}

/* Remove a previously installed hook. */
pub unsafe fn xfs_hooks_del(chain: *mut xfs_hooks, hook: *mut xfs_hook) {
    blocking_notifier_chain_unregister(
        unsafe { &mut (*chain).head },
        unsafe { &mut (*hook).nb },
    );
}

/* Call a hook.  Returns the NOTIFY_* value returned by the last hook. */
pub unsafe fn xfs_hooks_call(
    chain: *mut xfs_hooks,
    val: ::core::ffi::c_ulong,
    priv_: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    blocking_notifier_call_chain(unsafe { &mut (*chain).head }, val, priv_)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
