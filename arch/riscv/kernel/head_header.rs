/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 SiFive, Inc.
 */

// External declarations supplied by the Linux/RISC-V environment.
extern "C" {
    static mut hart_lottery: atomic_t;
}

// `asmlinkage` and `__init` are C-side linkage/section annotations.
pub unsafe extern "C" fn setup_vm(dtb_pa: usize);

// Preserved from CONFIG_RISCV_BOOT_SPINWAIT.
#[cfg(CONFIG_RISCV_BOOT_SPINWAIT)]
extern "C" {
    static mut __cpu_spinwait_stack_pointer: [*mut core::ffi::c_void; 0];
    static mut __cpu_spinwait_task_pointer: [*mut core::ffi::c_void; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
