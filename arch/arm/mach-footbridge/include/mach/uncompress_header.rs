/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-footbridge/include/mach/uncompress.h
 *
 *  Copyright (C) 1996-1999 Russell King
 */
// Dependency supplied by <asm/mach-types.h>.

/*
 * Note! This could cause problems on the NetWinder
 */
const DC21285_BASE: *mut core::ffi::c_uint = 0x42000160 as *mut core::ffi::c_uint;
const SER0_BASE: *mut core::ffi::c_uchar = 0x7c0003f8 as *mut core::ffi::c_uchar;

unsafe extern "C" {
    fn machine_is_netwinder() -> bool;
    fn barrier();
}

#[inline]
unsafe fn putc(c: core::ffi::c_char) {
    if machine_is_netwinder() {
        while (core::ptr::read_volatile(SER0_BASE.add(5)) & 0x60) != 0x60 {
            barrier();
        }
        core::ptr::write_volatile(SER0_BASE, c as core::ffi::c_uchar);
    } else {
        while core::ptr::read_volatile(DC21285_BASE.add(6)) & 8 != 0 {}
        core::ptr::write_volatile(DC21285_BASE, c as core::ffi::c_uint);
    }
}

#[inline]
unsafe fn flush() {}

/*
 * nothing to do
 */
#[inline]
fn arch_decomp_setup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
