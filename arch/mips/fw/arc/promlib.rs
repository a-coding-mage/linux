/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996 David S. Miller (dm@sgi.com)
 * Compatibility with board caches, Ulf Carlsson
 */

// Dependencies supplied by the corresponding kernel/ARC headers remain
// external to this translation.
extern "C" {
    fn bc_disable();
    fn bc_enable();
    fn ArcWrite(channel: i32, buffer: *mut CHAR, length: i32, count: *mut ULONG);
    fn ArcRead(channel: i32, buffer: *mut CHAR, length: i32, count: *mut ULONG);
}

// For 64bit kernels working with a 32bit ARC PROM pointer arguments for ARC
// calls need to reside in CKSEG0/1.  Under this configuration these values
// are static storage; otherwise the C macro leaves them as automatic locals.

pub unsafe fn prom_putchar(c: core::ffi::c_char) {
    #[cfg(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32"))]
    static mut CNT: ULONG = unsafe { core::mem::zeroed() };
    #[cfg(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32"))]
    static mut IT: CHAR = unsafe { core::mem::zeroed() };

    #[cfg(not(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32")))]
    let mut cnt: ULONG = unsafe { core::mem::zeroed() };
    #[cfg(not(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32")))]
    let mut it: CHAR = unsafe { core::mem::zeroed() };

    #[cfg(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32"))]
    {
        IT = c as CHAR;
        bc_disable();
        ArcWrite(1, &mut IT, 1, &mut CNT);
        bc_enable();
    }
    #[cfg(not(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32")))]
    {
        it = c as CHAR;
        bc_disable();
        ArcWrite(1, &mut it, 1, &mut cnt);
        bc_enable();
    }
}

pub unsafe fn prom_getchar() -> core::ffi::c_char {
    #[cfg(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32"))]
    static mut CNT: ULONG = unsafe { core::mem::zeroed() };
    #[cfg(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32"))]
    static mut C: CHAR = unsafe { core::mem::zeroed() };

    #[cfg(not(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32")))]
    let mut cnt: ULONG = unsafe { core::mem::zeroed() };
    #[cfg(not(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32")))]
    let mut c: CHAR = unsafe { core::mem::zeroed() };

    #[cfg(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32"))]
    {
        bc_disable();
        ArcRead(0, &mut C, 1, &mut CNT);
        bc_enable();
        C as core::ffi::c_char
    }
    #[cfg(not(all(target_pointer_width = "64", feature = "CONFIG_FW_ARC32")))]
    {
        bc_disable();
        ArcRead(0, &mut c, 1, &mut cnt);
        bc_enable();
        c as core::ffi::c_char
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
