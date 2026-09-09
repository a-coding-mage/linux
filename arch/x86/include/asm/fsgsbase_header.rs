/* SPDX-License-Identifier: GPL-2.0 */

// This header is active only for non-assembler code and CONFIG_X86_64.

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    /*
     * Read/write a task's FSBASE or GSBASE. This returns the value that
     * the FS/GS base would have (if the task were to be resumed). These
     * work on the current task or on a non-running (typically stopped
     * ptrace child) task.
     */
    pub fn x86_fsbase_read_task(task: *mut task_struct) -> u64;
    pub fn x86_gsbase_read_task(task: *mut task_struct) -> u64;
    pub fn x86_fsbase_write_task(task: *mut task_struct, fsbase: u64);
    pub fn x86_gsbase_write_task(task: *mut task_struct, gsbase: u64);

    pub fn x86_gsbase_read_cpu_inactive() -> u64;
    pub fn x86_gsbase_write_cpu_inactive(gsbase: u64);
    pub fn x86_fsgsbase_read_task(task: *mut task_struct, selector: u16) -> u64;
}

/* Must be protected by an X86_FEATURE_FSGSBASE check. */

#[inline(always)]
pub unsafe fn rdfsbase() -> u64 {
    let fsbase: u64;
    core::arch::asm!("rdfsbase {0}", out(reg) fsbase);
    fsbase
}

#[inline(always)]
pub unsafe fn rdgsbase() -> u64 {
    let gsbase: u64;
    core::arch::asm!("rdgsbase {0}", out(reg) gsbase);
    gsbase
}

#[inline(always)]
pub unsafe fn wrfsbase(fsbase: u64) {
    core::arch::asm!("wrfsbase {0}", in(reg) fsbase, options(nostack));
}

#[inline(always)]
pub unsafe fn wrgsbase(gsbase: u64) {
    core::arch::asm!("wrgsbase {0}", in(reg) gsbase, options(nostack));
}

// The following symbols are supplied by the cpufeature and msr dependencies.
extern "C" {
    fn boot_cpu_has(feature: u32) -> bool;
    fn rdmsrq(msr: u32, value: *mut u64);
    fn wrmsrq(msr: u32, value: u64);
}

// Helper functions for reading/writing FS/GS base.
#[inline]
pub unsafe fn x86_fsbase_read_cpu() -> u64 {
    let mut fsbase: u64 = 0;

    if boot_cpu_has(X86_FEATURE_FSGSBASE) {
        fsbase = rdfsbase();
    } else {
        rdmsrq(MSR_FS_BASE, &mut fsbase);
    }

    fsbase
}

#[inline]
pub unsafe fn x86_fsbase_write_cpu(fsbase: u64) {
    if boot_cpu_has(X86_FEATURE_FSGSBASE) {
        wrfsbase(fsbase);
    } else {
        wrmsrq(MSR_FS_BASE, fsbase);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
