/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2025 MIPS.
 */

// Dependency intent from the C header:
// #include <linux/cpumask.h>
// #include <uapi/asm/hwprobe.h>

#[cfg(CONFIG_RISCV_ISA_VENDOR_EXT_MIPS)]
unsafe extern "C" {
    pub fn hwprobe_isa_vendor_ext_mips_0(
        pair: *mut riscv_hwprobe,
        cpus: *const cpumask,
    );
}

#[cfg(not(CONFIG_RISCV_ISA_VENDOR_EXT_MIPS))]
#[inline]
pub unsafe fn hwprobe_isa_vendor_ext_mips_0(
    pair: *mut riscv_hwprobe,
    _cpus: *const cpumask,
) {
    (*pair).value = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
