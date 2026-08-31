/*
 * Copied from the kernel sources to tools/:
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2012 Tensilica Inc.
 */

// C header guard removed: _TOOLS_LINUX_XTENSA_SYSTEM_H.

unsafe extern "C" {
    fn barrier();
}

#[inline(always)]
pub unsafe fn mb() {
    unsafe {
        core::arch::asm!("memw", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn rmb() {
    unsafe {
        barrier();
    }
}

#[inline(always)]
pub unsafe fn wmb() {
    unsafe {
        mb();
    }
}
