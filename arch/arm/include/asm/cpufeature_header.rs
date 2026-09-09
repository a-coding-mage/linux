/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

// C header guard: __ASM_CPUFEATURE_H

// Dependencies supplied by other translated headers:
// linux/log2.h and asm/hwcap.h

/*
 * Due to the fact that ELF_HWCAP is a 32-bit type on ARM, and given the number
 * of optional CPU features it defines, ARM's CPU hardware capability bits have
 * been distributed over separate elf_hwcap and elf_hwcap2 variables, each of
 * which covers a subset of the available CPU features.
 *
 * Currently, only a few of those are suitable for automatic module loading
 * (which is the primary use case of this facility) and those happen to be all
 * covered by HWCAP2. So let's only cover those via the cpu_feature()
 * convenience macro for now (which is used by module_cpu_feature_match()).
 * However, all capabilities are exposed via the modalias, and can be matched
 * using an explicit MODULE_DEVICE_TABLE() that uses __hwcap_feature() directly.
 */
pub const MAX_CPU_FEATURES: u32 = 64;

// The C token-pasting macros accept a feature suffix; Rust callers pass the
// corresponding HWCAP/HWCAP2 constant expression directly.
#[macro_export]
macro_rules! __hwcap_feature {
    ($x:expr) => {{
        ilog2($x)
    }};
}

#[macro_export]
macro_rules! __hwcap2_feature {
    ($x:expr) => {{
        32 + ilog2($x)
    }};
}

#[macro_export]
macro_rules! cpu_feature {
    ($x:expr) => {{
        __hwcap2_feature!($x)
    }};
}

#[inline]
pub unsafe fn cpu_have_feature(num: core::ffi::c_uint) -> bool {
    if num < 32 {
        (elf_hwcap & BIT(num)) != 0
    } else {
        (elf_hwcap2 & BIT(num - 32)) != 0
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
