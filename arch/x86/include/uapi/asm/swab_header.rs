/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The C header includes linux/types.h and linux/compiler.h. Their supplied
// Rust equivalents are expected to provide __u32 and __u64.

#[inline]
pub const fn __arch_swab32(mut val: __u32) -> __u32 {
    val = val.swap_bytes();
    val
}

// C macro: #define __arch_swab32 __arch_swab32

#[inline]
pub const fn __arch_swab64(mut val: __u64) -> __u64 {
    #[cfg(target_arch = "x86")]
    {
        // On i386, the original implementation byte-swaps each 32-bit half
        // and exchanges their order through a C union.
        let a: __u32 = (val as __u32).swap_bytes();
        let b: __u32 = ((val >> 32) as __u32).swap_bytes();
        return ((a as __u64) << 32) | (b as __u64);
    }

    #[cfg(not(target_arch = "x86"))]
    {
        val = val.swap_bytes();
        val
    }
}

// C macro: #define __arch_swab64 __arch_swab64

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
