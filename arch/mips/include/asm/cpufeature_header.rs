/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * CPU feature definitions for module loading, used by
 * module_cpu_feature_match(), see uapi/asm/hwcap.h for MIPS CPU features.
 */

// C dependencies supplied by the surrounding translation unit:
// `elf_hwcap`, the HWCAP_* constants, and `ilog2`.

/// MAX_CPU_FEATURES (8 * sizeof(elf_hwcap))
pub const MAX_CPU_FEATURES: usize = 8 * core::mem::size_of::<usize>();

/// Rust equivalent of the C `cpu_feature(x)` macro.  Pass the corresponding
/// HWCAP_* constant as the expression argument.
#[macro_export]
macro_rules! cpu_feature {
    ($x:expr) => {
        ilog2($x)
    };
}

/// External CPU capability bitmask declared by the architecture support.
unsafe extern "C" {
    pub static elf_hwcap: usize;
}

#[inline]
pub unsafe fn cpu_have_feature(num: u32) -> bool {
    elf_hwcap & (1usize << num) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
