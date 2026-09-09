/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_GENERIC_CURRENT_H

// C-only dependency: <linux/thread_info.h>
// The declarations required by this header are supplied by that dependency.

// Equivalent to: #define get_current() (current_thread_info()->task)
#[macro_export]
macro_rules! get_current {
    () => {
        unsafe { (*$crate::current_thread_info()).task }
    };
}

// Equivalent to: #define current get_current()
#[macro_export]
macro_rules! current {
    () => {
        $crate::get_current!()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
