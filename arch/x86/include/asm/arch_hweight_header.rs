/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: <asm/cpufeatures.h> supplies X86_FEATURE_POPCNT.
// The C implementation selects between a software helper and POPCNT through
// the kernel's ALTERNATIVE inline-assembly mechanism. Rust's count_ones is
// the direct semantic equivalent of that operation.

#[inline(always)]
pub fn __arch_hweight32(w: u32) -> u32 {
    w.count_ones()
}

#[inline]
pub fn __arch_hweight16(w: u32) -> u32 {
    __arch_hweight32(w & 0xffff)
}

#[inline]
pub fn __arch_hweight8(w: u32) -> u32 {
    __arch_hweight32(w & 0xff)
}

// CONFIG_X86_32 selects the two-half implementation, matching the C header.
#[cfg(CONFIG_X86_32)]
#[inline]
pub fn __arch_hweight64(w: u64) -> usize {
    (__arch_hweight32(w as u32) + __arch_hweight32((w >> 32) as u32)) as usize
}

// On 64-bit x86 the C implementation uses POPCNTQ, with the kernel's
// software fallback selected by ALTERNATIVE when POPCNT is unavailable.
#[cfg(not(CONFIG_X86_32))]
#[inline(always)]
pub fn __arch_hweight64(w: u64) -> usize {
    w.count_ones() as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
