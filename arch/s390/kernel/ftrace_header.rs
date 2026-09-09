/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header _FTRACE_H.
// Dependency intent: asm/types.h supplies the fixed-width integer types.

#[repr(C, packed)]
pub struct ftrace_hotpatch_trampoline {
    pub brasl_opc: u16,
    pub brasl_disp: i32,
    // C bit-field: s16: 16; (unnamed, occupying one 16-bit slot)
    pub _unnamed: i16,
    pub rest_of_intercepted_function: u64,
    pub interceptor: u64,
}

extern "C" {
    pub static mut __ftrace_hotpatch_trampolines_start: [ftrace_hotpatch_trampoline; 0];
    pub static mut __ftrace_hotpatch_trampolines_end: [ftrace_hotpatch_trampoline; 0];
    pub static ftrace_shared_hotpatch_trampoline_br: [i8; 0];
    pub static ftrace_shared_hotpatch_trampoline_br_end: [i8; 0];
    pub static ftrace_shared_hotpatch_trampoline_exrl: [i8; 0];
    pub static ftrace_shared_hotpatch_trampoline_exrl_end: [i8; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
