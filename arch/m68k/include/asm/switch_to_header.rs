/* SPDX-License-Identifier: GPL-2.0 */

/*
 * switch_to(n) should switch tasks to task ptr, first checking that
 * ptr isn't the current task, in which case it does nothing.  This
 * also clears the TS-flag if the task we switched to has used the
 * math co-processor latest.
 */
/*
 * switch_to() saves the extra registers, that are not saved
 * automatically by SAVE_SWITCH_STACK in resume(), ie. d0-d5 and
 * a0-a1. Some of these are used by schedule() and its predecessors
 * and so we might get see unexpected behaviors when a task returns
 * with unexpected register values.
 *
 * syscall stores these registers itself and none of them are used
 * by syscall after the function in the syscall has been called.
 *
 * Beware that resume now expects *next to be in d1 and the offset of
 * tss to be in a1. This saves a few instructions as we no longer have
 * to push them onto the stack and read them back right after.
 *
 * 02/17/96 - Jes Sorensen (jds@kom.auc.dk)
 *
 * Changed 96/09/19 by Andreas Schwab
 * pass prev in a0, next in a1
 */
unsafe extern "C" {
    pub fn resume();
}

#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        let mut _prev = ($prev) as *mut core::ffi::c_void;
        let mut _next = ($next) as *mut core::ffi::c_void;
        let _last: *mut core::ffi::c_void;
        unsafe {
            core::arch::asm!(
                "jbsr resume",
                inout("a0") _prev,
                inout("a1") _next,
                lateout("d1") _last,
                out("d0") _,
                out("d2") _,
                out("d3") _,
                out("d4") _,
                out("d5") _,
            );
        }
        ($last) = _last;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
