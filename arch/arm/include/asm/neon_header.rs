/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/arch/arm/include/asm/neon.h
 *
 * Copyright (C) 2013 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// Dependency supplied by the corresponding ARM hardware-capability module.

/// Whether the current CPU has NEON support.
#[inline]
pub unsafe fn cpu_has_neon() -> bool {
    (elf_hwcap & HWCAP_NEON) != 0
}

/*
 * If you are affected by the BUILD_BUG below, it probably means that you are
 * using NEON code /and/ calling the kernel_neon_begin() function from the same
 * compilation unit. To prevent issues that may arise from GCC reordering or
 * generating(1) NEON instructions outside of these begin/end functions, the
 * only supported way of using NEON code in the kernel is by isolating it in a
 * separate compilation unit, and calling it from another unit from inside a
 * kernel_neon_begin/kernel_neon_end pair.
 *
 * (1) Current GCC (4.7) might generate NEON instructions at O3 level if
 *     -mpfu=neon is set.
 */

// Corresponds to the C __ARM_NEON__ conditional compilation branch.
#[cfg(target_feature = "neon")]
macro_rules! kernel_neon_begin {
    () => {
        compile_error!("kernel_neon_begin() called from NEON code");
    };
}

#[cfg(not(target_feature = "neon"))]
extern "C" {
    pub fn kernel_neon_begin();
}

extern "C" {
    pub fn kernel_neon_end();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
