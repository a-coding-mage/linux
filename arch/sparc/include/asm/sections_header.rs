/* SPDX-License-Identifier: GPL-2.0 */

/* The C header includes <asm-generic/sections.h>; its declarations are supplied
 * by the corresponding Rust dependency. */

/* sparc entry point */
unsafe extern "C" {
    pub static mut _start: core::ffi::c_char;

    pub static mut __leon_1insn_patch: core::ffi::c_char;
    pub static mut __leon_1insn_patch_end: core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
