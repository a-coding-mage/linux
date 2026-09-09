/*
 * Architecture specific parts of the Floppy driver
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995 - 2000 Ralf Baechle
 */

// Dependency supplied by the architecture I/O implementation.

pub unsafe fn fd_cacheflush(addr: *mut core::ffi::c_char, size: core::ffi::c_long) {
    dma_cache_wback_inv(addr as core::ffi::c_ulong, size);
}

pub const MAX_BUFFER_SECTORS: i32 = 24;

/*
 * And on Mips's the CMOS info fails also ...
 *
 * FIXME: This information should come from the ARC configuration tree
 *       or wherever a particular machine has stored this ...
 */
macro_rules! FLOPPY0_TYPE {
    () => {
        fd_drive_type(0)
    };
}

macro_rules! FLOPPY1_TYPE {
    () => {
        fd_drive_type(1)
    };
}

macro_rules! FDC1 {
    () => {
        fd_getfdaddr1()
    };
}

pub const N_FDC: i32 = 1; /* do you *really* want a second controller? */
pub const N_DRIVE: i32 = 8;

// C: #define EXTRA_FLOPPY_PARAMS

// C: #include <floppy.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
