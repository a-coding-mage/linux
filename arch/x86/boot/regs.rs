// SPDX-License-Identifier: GPL-2.0-or-later
/* -----------------------------------------------------------------------
 *
 *   Copyright 2009 Intel Corporation; author H. Peter Anvin
 *
 * ----------------------------------------------------------------------- */

/*
 * Simple helper function for initializing a register set.
 *
 * Note that this sets EFLAGS_CF in the input register set; this
 * makes it easier to catch functions which do nothing but don't
 * explicitly set CF.
 */

// Dependencies supplied by boot.h and string.h are intentionally left external.

pub unsafe fn initregs(reg: *mut biosregs) {
    memset(
        reg as *mut core::ffi::c_void,
        0,
        core::mem::size_of::<biosregs>(),
    );
    (*reg).eflags |= X86_EFLAGS_CF;
    (*reg).ds = ds();
    (*reg).es = ds();
    (*reg).fs = fs();
    (*reg).gs = gs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
