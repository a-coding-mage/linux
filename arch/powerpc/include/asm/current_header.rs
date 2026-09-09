/* SPDX-License-Identifier: GPL-2.0-or-later */

/* The declarations in this file are available only when building the kernel. */

pub struct task_struct;

#[cfg(target_arch = "powerpc64")]
#[inline]
pub unsafe fn get_current() -> *mut task_struct {
    let task: *mut task_struct;

    /* get_current can be cached by the compiler, so no volatile */
    core::arch::asm!(
        "ld {task}, {offset}(13)",
        task = out(reg) task,
        offset = const core::mem::offset_of!(paca_struct, __current),
    );

    task
}

#[cfg(target_arch = "powerpc64")]
#[allow(non_snake_case)]
pub unsafe fn current() -> *mut task_struct {
    get_current()
}

#[cfg(not(target_arch = "powerpc64"))]
/* We keep `current' in r2 for speed. */
#[allow(non_upper_case_globals)]
pub static mut current: *mut task_struct = core::ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
