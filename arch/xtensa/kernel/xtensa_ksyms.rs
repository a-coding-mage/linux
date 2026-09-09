/*
 * arch/xtensa/kernel/xtensa_ksyms.c
 *
 * Export Xtensa-specific functions for loadable modules.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005  Tensilica Inc.
 *
 * Joe Taylor <joe@tensilica.com>
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn BUG() -> !;
}

pub unsafe fn __sync_fetch_and_and_4(p: *mut core::ffi::c_void, v: u32) -> u32 {
    let _ = (p, v);
    BUG();
}

// EXPORT_SYMBOL(__sync_fetch_and_and_4);

pub unsafe fn __sync_fetch_and_or_4(p: *mut core::ffi::c_void, v: u32) -> u32 {
    let _ = (p, v);
    BUG();
}

// EXPORT_SYMBOL(__sync_fetch_and_or_4);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
