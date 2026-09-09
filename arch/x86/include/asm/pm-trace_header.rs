/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

extern "C" {
    pub static pm_trace_enabled: bool;
    pub fn generate_pm_trace(tracedata: *const c_void, user: bool);
}

/// Equivalent of the C `TRACE_RESUME` macro.
///
/// The inline assembly emits a `.tracedata` section containing the source
/// line and file pointer, then passes the address of that record to the trace
/// generator.  The assembler operands are retained here as an explicit
/// low-level operation; the exact `_ASM_MOV`/`_ASM_PTR` definitions are
/// supplied by the architecture assembly support.
#[macro_export]
macro_rules! TRACE_RESUME {
    ($user:expr) => {{
        if unsafe { $crate::pm_trace_enabled } {
            let tracedata: *const c_void;
            unsafe {
                core::arch::asm!(
                    "/* _ASM_MOV $1f,%0 */\n\
                     .section .tracedata,\"a\"\n\
                     1:\t.word {line}\n\t\
                     /* _ASM_PTR {file} */\n\
                     .previous",
                    out(reg) tracedata,
                    line = const line!(),
                    file = const file!(),
                );
                $crate::generate_pm_trace(tracedata, $user);
            }
        }
    }};
}

#[macro_export]
macro_rules! TRACE_SUSPEND {
    ($user:expr) => {
        $crate::TRACE_RESUME!($user)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
