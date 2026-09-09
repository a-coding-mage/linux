/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  arch/arm/include/asm/byteorder.h
 *
 * ARM Endian-ness.  In little endian mode, the data bus is connected such
 * that byte accesses appear as:
 *  0 = d0...d7, 1 = d8...d15, 2 = d16...d23, 3 = d24...d31
 * and word accesses (data or instruction) appear as:
 *  d0...d31
 *
 * When in big endian mode, byte accesses appear as:
 *  0 = d24...d31, 1 = d16...d23, 2 = d8...d15, 3 = d0...d7
 * and word accesses (data or instruction) appear as:
 *  d0...d31
 */

// Dependency supplied by the corresponding UAPI header: asm/swab.h.

// Preserved from: #if __LINUX_ARM_ARCH__ >= 6

#[inline]
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub unsafe fn __arch_swahb32(mut x: u32) -> u32 {
    core::arch::asm!("rev16 {0}", inout(reg) x);
    x
}

// Preserved from: #define __arch_swahb32 __arch_swahb32

#[inline]
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub unsafe fn __arch_swab16(x: u32) -> u16 {
    __arch_swahb32(x) as u16
}

#[inline]
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub unsafe fn __arch_swab32(mut x: u32) -> u32 {
    core::arch::asm!("rev {0}", inout(reg) x);
    x
}

// Preserved from: #define __arch_swab32 __arch_swab32

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
