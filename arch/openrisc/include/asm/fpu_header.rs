/* SPDX-License-Identifier: GPL-2.0 */

// External/incomplete type supplied by the surrounding kernel translation.
pub struct task_struct;

// CONFIG_FPU selects the implementation below, matching the C preprocessor
// condition from the source header.
#[cfg(feature = "CONFIG_FPU")]
#[inline]
pub unsafe fn save_fpu(task: *mut task_struct) {
    (*task).thread.fpcsr = mfspr(SPR_FPCSR);
}

#[cfg(feature = "CONFIG_FPU")]
#[inline]
pub unsafe fn restore_fpu(task: *mut task_struct) {
    mtspr(SPR_FPCSR, (*task).thread.fpcsr);
}

// CONFIG_FPU not enabled: the original macros are no-ops.
#[cfg(not(feature = "CONFIG_FPU"))]
#[inline]
pub unsafe fn save_fpu(_tsk: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_FPU"))]
#[inline]
pub unsafe fn restore_fpu(_tsk: *mut task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
