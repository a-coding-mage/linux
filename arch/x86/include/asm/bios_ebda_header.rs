/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/* Dependency supplied by asm/io.h. */
unsafe extern "C" {
    fn phys_to_virt(address: usize) -> *mut c_void;
}

/*
 * Returns physical address of EBDA. Returns 0 if there is no EBDA.
 */
#[inline]
pub unsafe fn get_bios_ebda() -> u32 {
    /*
     * There is a real-mode segmented pointer pointing to the
     * 4K EBDA area at 0x40E.
     */
    let address = core::ptr::read(phys_to_virt(0x40E) as *const u16);
    let address = (address as u32) << 4;
    address /* 0 means none */
}

unsafe extern "C" {
    pub fn reserve_bios_regions();
}

/* Build-time condition corresponding to CONFIG_X86_CHECK_BIOS_CORRUPTION. */
#[cfg(feature = "CONFIG_X86_CHECK_BIOS_CORRUPTION")]
unsafe extern "C" {
    /*
     * This is obviously not a great place for this, but we want to be
     * able to scatter it around anywhere in the kernel.
     */
    pub fn check_for_bios_corruption();
    pub fn start_periodic_check_for_corruption();
}

#[cfg(not(feature = "CONFIG_X86_CHECK_BIOS_CORRUPTION"))]
#[inline]
pub fn check_for_bios_corruption() {}

#[cfg(not(feature = "CONFIG_X86_CHECK_BIOS_CORRUPTION"))]
#[inline]
pub fn start_periodic_check_for_corruption() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
