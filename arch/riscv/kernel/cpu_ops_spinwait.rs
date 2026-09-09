// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020 Western Digital Corporation or its affiliates.
 */

// The following names are supplied by the Linux/RISC-V headers and other
// translation units: NR_CPUS, INVALID_HARTID, cpu_operations, task_struct,
// cpuid_to_hartid_map, task_pt_regs, and smp_mb.

use core::ffi::{c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn cpuid_to_hartid_map(cpuid: c_uint) -> c_ulong;
    fn task_pt_regs(tidle: *mut task_struct) -> *mut c_void;
    fn smp_mb();
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_operations {
    pub cpu_start:
        Option<unsafe extern "C" fn(cpuid: c_uint, tidle: *mut task_struct) -> c_int>,
}

pub static mut __cpu_spinwait_stack_pointer: [*mut c_void; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];
pub static mut __cpu_spinwait_task_pointer: [*mut c_void; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];

unsafe fn cpu_update_secondary_bootdata(cpuid: c_uint, tidle: *mut task_struct) {
    let hartid: c_ulong = cpuid_to_hartid_map(cpuid);

    /*
     * The hartid must be less than NR_CPUS to avoid out-of-bound access
     * errors for __cpu_spinwait_stack/task_pointer. That is not always possible
     * for platforms with discontiguous hartid numbering scheme. That's why
     * spinwait booting is not the recommended approach for any platforms
     * booting Linux in S-mode and can be disabled in the future.
     */
    if hartid == INVALID_HARTID || hartid >= NR_CPUS as c_ulong {
        return;
    }

    /* Make sure tidle is updated */
    smp_mb();
    core::ptr::write_volatile(
        &mut __cpu_spinwait_stack_pointer[hartid as usize],
        task_pt_regs(tidle),
    );
    core::ptr::write_volatile(
        &mut __cpu_spinwait_task_pointer[hartid as usize],
        tidle as *mut c_void,
    );
}

unsafe extern "C" fn spinwait_cpu_start(
    cpuid: c_uint,
    tidle: *mut task_struct,
) -> c_int {
    /*
     * In this protocol, all cpus boot on their own accord.  _start
     * selects the first cpu to boot the kernel and causes the remainder
     * of the cpus to spin in a loop waiting for their stack pointer to be
     * setup by that main cpu.  Writing to bootdata
     * (i.e __cpu_spinwait_stack_pointer) signals to the spinning cpus that they
     * can continue the boot process.
     */
    cpu_update_secondary_bootdata(cpuid, tidle);

    0
}

pub static cpu_ops_spinwait: cpu_operations = cpu_operations {
    cpu_start: Some(spinwait_cpu_start),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
