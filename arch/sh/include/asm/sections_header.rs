/* SPDX-License-Identifier: GPL-2.0 */

// Corresponds to: #include <asm-generic/sections.h>

unsafe extern "C" {
    pub static __machvec_start: [core::ffi::c_char; 0];
    pub static __machvec_end: [core::ffi::c_char; 0];
    pub static mut __uncached_start: core::ffi::c_char;
    pub static mut __uncached_end: core::ffi::c_char;
    pub static __start_eh_frame: [core::ffi::c_char; 0];
    pub static __stop_eh_frame: [core::ffi::c_char; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
