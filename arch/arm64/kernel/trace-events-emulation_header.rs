/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM emulation
// The C header guard and TRACE_HEADER_MULTI_READ conditional are preserved here
// as source-level intent; their expansion is provided by the tracepoint system.

use core::ffi::c_char;

/// Entry data emitted by the `instruction_emulation` trace event.
#[repr(C)]
pub struct instruction_emulation_entry {
    pub instr: *const c_char,
    pub addr: u64,
}

/// Trace event declaration corresponding to `TRACE_EVENT(instruction_emulation, ...)`.
///
/// The tracepoint registration, string storage, and formatted output are supplied
/// by the external tracepoint implementation.
pub mod instruction_emulation {
    use super::{c_char, instruction_emulation_entry};

    #[inline]
    pub unsafe fn fast_assign(
        entry: *mut instruction_emulation_entry,
        instr: *const c_char,
        addr: u64,
    ) {
        // C: __assign_str(instr);
        // The tracepoint implementation owns the destination string storage.
        (*entry).instr = instr;
        (*entry).addr = addr;
    }

    #[inline]
    pub unsafe fn print(entry: *const instruction_emulation_entry) {
        // C: TRACE_PRINTK("instr=\"%s\" addr=0x%llx", __get_str(instr), __entry->addr)
        // Formatting and emission are provided by the external tracepoint system.
        let _ = entry;
    }
}

// TRACE_INCLUDE_PATH .
// TRACE_INCLUDE_FILE trace-events-emulation
// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
