/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent preserved from: #include <asm-generic/sections.h>

unsafe extern "C" {
    pub static mut __binary_start: [::core::ffi::c_char; 0];
    pub static mut __syscall_stub_start: [::core::ffi::c_char; 0];
    pub static mut __syscall_stub_end: [::core::ffi::c_char; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
