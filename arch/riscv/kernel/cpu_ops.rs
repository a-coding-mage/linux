// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020 Western Digital Corporation or its affiliates.
 */

// Dependencies supplied by the Linux/RISC-V environment:
// linux/errno.h, linux/mm.h, linux/of.h, linux/string.h, linux/sched.h,
// asm/cpu_ops.h, asm/cpu_ops_sbi.h, asm/sbi.h, and asm/smp.h.

use core::ptr;

// `struct cpu_operations` and these symbols are supplied by the surrounding
// RISC-V kernel translation unit.
extern "C" {
    static cpu_ops_spinwait: cpu_operations;
    static cpu_ops_sbi: cpu_operations;
    fn sbi_probe_extension(extension_id: i32) -> i32;
}

#[repr(C)]
pub struct cpu_operations {
    pub cpu_start: Option<unsafe extern "C" fn()>,
}

#[no_mangle]
pub static mut cpu_ops: *const cpu_operations = unsafe { &cpu_ops_spinwait };

#[cfg(not(CONFIG_RISCV_BOOT_SPINWAIT))]
#[no_mangle]
pub static cpu_ops_spinwait: cpu_operations = cpu_operations {
    cpu_start: None,
};

// CONFIG_RISCV_SBI is a build-time condition from the original source.
#[cfg(CONFIG_RISCV_SBI)]
#[no_mangle]
pub unsafe extern "C" fn cpu_set_ops() {
    if sbi_probe_extension(SBI_EXT_HSM) != 0 {
        // Original kernel logging call: pr_info("SBI HSM extension detected\n");
        cpu_ops = &cpu_ops_sbi;
    }
}

#[cfg(not(CONFIG_RISCV_SBI))]
#[no_mangle]
pub unsafe extern "C" fn cpu_set_ops() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
