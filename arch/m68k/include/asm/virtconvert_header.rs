/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Macros used for converting between virtual and physical mappings.
 *
 * The declarations below are available only for kernel builds, matching the
 * original __KERNEL__ conditional.
 */
#[cfg(feature = "kernel")]
use core::ffi::c_void;

#[cfg(feature = "kernel")]
unsafe extern "C" {
    fn __pa(address: *mut c_void) -> usize;
    fn __va(address: usize) -> *mut c_void;
}

/*
 * Change virtual addresses to physical addresses and vv.
 */
#[cfg(feature = "kernel")]
#[inline]
pub unsafe fn virt_to_phys(address: *mut c_void) -> usize {
    unsafe { __pa(address) }
}

#[cfg(feature = "kernel")]
#[inline]
pub unsafe fn phys_to_virt(address: usize) -> *mut c_void {
    unsafe { __va(address) }
}

/*
 * IO bus memory addresses are 1:1 with the physical address,
 * deprecated globally but still used on two machines.
 *
 * Equivalent to the original CONFIG_AMIGA || CONFIG_VME conditional.
 */
#[cfg(all(feature = "kernel", any(feature = "amiga", feature = "vme")))]
#[inline]
pub unsafe fn virt_to_bus(address: *mut c_void) -> usize {
    unsafe { virt_to_phys(address) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
