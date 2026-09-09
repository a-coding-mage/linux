/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Transactional memory support routines to reclaim and recheckpoint
 * transactional process state.
 *
 * Copyright 2012 Matt Evans & Michael Neuling, IBM Corporation.
 */

// Dependency corresponding to <uapi/asm/tm.h>.

// The declarations below correspond to the non-assembler portion of the C
// header. The `thread_struct` type is supplied by another translation unit.
extern "C" {
    pub fn tm_reclaim(thread: *mut crate::thread_struct, cause: u8);
    pub fn tm_reclaim_current(cause: u8);
    pub fn tm_recheckpoint(thread: *mut crate::thread_struct);
    pub fn tm_save_sprs(thread: *mut crate::thread_struct);
    pub fn tm_restore_sprs(thread: *mut crate::thread_struct);

    pub static mut tm_suspend_disabled: bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
