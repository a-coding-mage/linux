/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by asm-generic/sections.h are external dependencies.

#[cfg(CONFIG_VECTORS_ADDR)]
extern "C" {
    pub static mut _WindowVectors_text_start: [core::ffi::c_char; 0];
    pub static mut _WindowVectors_text_end: [core::ffi::c_char; 0];
    pub static mut _DebugInterruptVector_text_start: [core::ffi::c_char; 0];
    pub static mut _DebugInterruptVector_text_end: [core::ffi::c_char; 0];
    pub static mut _KernelExceptionVector_text_start: [core::ffi::c_char; 0];
    pub static mut _KernelExceptionVector_text_end: [core::ffi::c_char; 0];
    pub static mut _UserExceptionVector_text_start: [core::ffi::c_char; 0];
    pub static mut _UserExceptionVector_text_end: [core::ffi::c_char; 0];
    pub static mut _DoubleExceptionVector_text_start: [core::ffi::c_char; 0];
    pub static mut _DoubleExceptionVector_text_end: [core::ffi::c_char; 0];
    pub static mut _exception_text_start: [core::ffi::c_char; 0];
    pub static mut _exception_text_end: [core::ffi::c_char; 0];
    pub static mut _Level2InterruptVector_text_start: [core::ffi::c_char; 0];
    pub static mut _Level2InterruptVector_text_end: [core::ffi::c_char; 0];
    pub static mut _Level3InterruptVector_text_start: [core::ffi::c_char; 0];
    pub static mut _Level3InterruptVector_text_end: [core::ffi::c_char; 0];
    pub static mut _Level4InterruptVector_text_start: [core::ffi::c_char; 0];
    pub static mut _Level4InterruptVector_text_end: [core::ffi::c_char; 0];
    pub static mut _Level5InterruptVector_text_start: [core::ffi::c_char; 0];
    pub static mut _Level5InterruptVector_text_end: [core::ffi::c_char; 0];
    pub static mut _Level6InterruptVector_text_start: [core::ffi::c_char; 0];
    pub static mut _Level6InterruptVector_text_end: [core::ffi::c_char; 0];
}

#[cfg(CONFIG_SECONDARY_RESET_VECTOR)]
extern "C" {
    pub static mut _SecondaryResetVector_text_start: [core::ffi::c_char; 0];
    pub static mut _SecondaryResetVector_text_end: [core::ffi::c_char; 0];
}

#[cfg(CONFIG_XIP_KERNEL)]
extern "C" {
    #[cfg(CONFIG_VECTORS_ADDR)]
    pub static mut _xip_text_start: [core::ffi::c_char; 0];
    #[cfg(CONFIG_VECTORS_ADDR)]
    pub static mut _xip_text_end: [core::ffi::c_char; 0];
    pub static mut _xip_start: [core::ffi::c_char; 0];
    pub static mut _xip_end: [core::ffi::c_char; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
