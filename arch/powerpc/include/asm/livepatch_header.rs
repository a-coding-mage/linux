/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * livepatch.h - powerpc-specific Kernel Live Patching Core
 *
 * Copyright (C) 2015-2016, SUSE, IBM Corp.
 */

// Dependencies supplied by the corresponding scheduler and task-stack modules.

#[cfg(feature = "CONFIG_LIVEPATCH_64")]
#[inline]
pub unsafe fn klp_init_thread_info(p: *mut task_struct) {
    // + 1 to account for STACK_END_MAGIC
    (*task_thread_info(p)).livepatch_sp = end_of_stack(p).add(1);
}

#[cfg(not(feature = "CONFIG_LIVEPATCH_64"))]
#[inline]
pub unsafe fn klp_init_thread_info(_p: *mut task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
