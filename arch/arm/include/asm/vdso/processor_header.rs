/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 ARM Ltd.
 */

/*
 * The C header excludes this declaration when assembled as assembly.  Rust
 * translation applies to the Rust compilation unit only.
 */

/*
 * Equivalent of:
 *   #if __LINUX_ARM_ARCH__ == 6 || defined(CONFIG_ARM_ERRATA_754327)
 * Build configurations corresponding to those C preprocessor conditions
 * should enable either `linux_arm_arch_6` or `arm_errata_754327`.
 */
#[cfg(any(feature = "linux_arm_arch_6", feature = "arm_errata_754327"))]
macro_rules! cpu_relax {
    () => {{
        smp_mb();
        unsafe {
            core::arch::asm!(
                "nop; nop; nop; nop; nop; nop; nop; nop; nop; nop;",
                options(nostack, preserves_flags)
            );
        }
    }};
}

#[cfg(not(any(feature = "linux_arm_arch_6", feature = "arm_errata_754327")))]
macro_rules! cpu_relax {
    () => {{
        barrier();
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
