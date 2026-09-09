/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// <linux/types.h> and <linux/compiler.h> are supplied by the surrounding
// translation environment.  The C header's __u32 corresponds to Rust u32.

// #define __SWAB_64_THRU_32__
pub const __SWAB_64_THRU_32__: bool = true;

// The following target conditions correspond to the C preprocessor symbols
// __mcfisaaplus__, __mcfisac__, and __mcoldfire__.
#[cfg(any(mcfisaaplus, mcfisac))]
#[inline]
pub unsafe fn __arch_swab32(mut val: u32) -> u32 {
    core::arch::asm!("byterev {0}", inout(reg) val);
    val
}

// #define __arch_swab32 __arch_swab32

#[cfg(all(not(mcfisaaplus), not(mcfisac), not(mcoldfire)))]
#[inline]
pub unsafe fn __arch_swab32(mut val: u32) -> u32 {
    core::arch::asm!("rolw #8,{0}; swap {0}; rolw #8,{0}", inout(reg) val);
    val
}

// #define __arch_swab32 __arch_swab32

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
