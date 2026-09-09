/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CPU feature definitions for module loading, used by
 * module_cpu_feature_match(), see uapi/asm/hwcap.h for LoongArch CPU features.
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the corresponding UAPI and ELF modules are
// intentionally referenced here rather than reimplemented.

/// Maximum number of CPU feature bits represented by `elf_hwcap`.
pub const MAX_CPU_FEATURES: usize = 8 * core::mem::size_of::<usize>();

/// Equivalent of the C `cpu_feature(x)` macro. Pass the corresponding
/// `HWCAP_*` constant as the argument.
#[macro_export]
macro_rules! cpu_feature {
    ($x:expr) => {
        ilog2($x)
    };
}

/// External hardware capability bitmap supplied by the ELF support code.
extern "C" {
    pub static mut elf_hwcap: usize;
}

/// Return whether the numbered CPU feature is present in `elf_hwcap`.
#[inline]
pub unsafe fn cpu_have_feature(num: u32) -> bool {
    (elf_hwcap & (1usize << num)) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
