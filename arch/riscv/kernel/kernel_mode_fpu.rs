// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 SiFive
 */

// Dependencies supplied by the surrounding kernel translation.
unsafe extern "C" {
    fn preempt_disable();
    fn preempt_enable();
    fn current() -> *mut core::ffi::c_void;
    fn task_pt_regs(task: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn fstate_save(task: *mut core::ffi::c_void, regs: *mut core::ffi::c_void);
    fn fstate_restore(task: *mut core::ffi::c_void, regs: *mut core::ffi::c_void);
    fn csr_set(csr: core::ffi::c_ulong, value: core::ffi::c_ulong);
    fn csr_clear(csr: core::ffi::c_ulong, value: core::ffi::c_ulong);
}

// Constants supplied by <asm/csr.h>.
unsafe extern "C" {
    static CSR_SSTATUS: core::ffi::c_ulong;
    static SR_FS: core::ffi::c_ulong;
}

pub unsafe fn kernel_fpu_begin() {
    preempt_disable();
    let task = current();
    fstate_save(task, task_pt_regs(task));
    csr_set(CSR_SSTATUS, SR_FS);
}

// EXPORT_SYMBOL_GPL(kernel_fpu_begin);

pub unsafe fn kernel_fpu_end() {
    csr_clear(CSR_SSTATUS, SR_FS);
    let task = current();
    fstate_restore(task, task_pt_regs(task));
    preempt_enable();
}

// EXPORT_SYMBOL_GPL(kernel_fpu_end);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
