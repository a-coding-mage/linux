/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: compiler attributes and assembly support are supplied by the
// surrounding translation environment.

#[repr(C)]
pub struct task_struct {
    _opaque: [u8; 0],
}

/*
 * We don't use read_sysreg() as we want the compiler to cache the value where
 * possible.
 */
#[inline(always)]
pub unsafe fn get_current() -> *mut task_struct {
    let sp_el0: usize;

    core::arch::asm!(
        "mrs {0}, sp_el0",
        out(reg) sp_el0,
    );

    sp_el0 as *mut task_struct
}

#[macro_export]
macro_rules! current {
    () => {
        $crate::get_current()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
