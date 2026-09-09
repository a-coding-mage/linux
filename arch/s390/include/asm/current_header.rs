/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *    Copyright IBM Corp. 1999
 *    Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
 *
 *  Derived from "include/asm-i386/current.h"
 */

// Dependencies supplied by the surrounding translation unit:
// `lowcore::Lowcore`, `LOWCORE_ALT_ADDRESS`, and the machine feature
// configuration corresponding to MFEATURE_LOWCORE.

use core::arch::asm;

pub struct TaskStruct;

#[inline(always)]
pub unsafe fn get_current() -> *mut TaskStruct {
    let mut ptr: usize;
    let lc_current = core::mem::offset_of!(crate::lowcore::Lowcore, current_task);

    // The C source selects between these two instructions through the
    // build-time ALTERNATIVE(MFEATURE_LOWCORE) mechanism.
    #[cfg(feature = "mfeature_lowcore")]
    asm!(
        "lg {ptr}, {off}({zero})",
        ptr = lateout(reg) ptr,
        off = const lc_current + crate::lowcore::LOWCORE_ALT_ADDRESS,
        zero = const 0usize,
    );
    #[cfg(not(feature = "mfeature_lowcore"))]
    asm!(
        "lg {ptr}, {off}({zero})",
        ptr = lateout(reg) ptr,
        off = const lc_current,
        zero = const 0usize,
    );

    ptr as *mut TaskStruct
}

// C equivalent: #define current get_current()
#[macro_export]
macro_rules! current {
    () => {
        unsafe { $crate::get_current() }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
