/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_PARISC_CURRENT_H

// This declaration is supplied by the task_struct definition elsewhere.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

/// Return the task pointer held in the PA-RISC CR30 control register.
///
/// The original C implementation deliberately uses inline assembly rather
/// than the volatile mfctl() macro.
#[inline(always)]
pub unsafe fn get_current() -> *mut task_struct {
    let ts: *mut task_struct;
    core::arch::asm!("mfctl %cr30, {0}", out(reg) ts);
    ts
}

/// C equivalent of `current`, which expands to `get_current()`.
#[macro_export]
macro_rules! current {
    () => {
        $crate::get_current()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
