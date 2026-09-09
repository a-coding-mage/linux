/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 */

// Dependency equivalent of: #include <asm-generic/setup.h>

// The declarations below are present only for non-assembler kernel builds
// in the original header.
#[cfg(feature = "kernel")]
extern "C" {
    pub static mut exception_handler_hook: [core::ffi::c_char; 0];
    pub static mut fast_handler: [core::ffi::c_char; 0];
    pub static mut fast_handler_end: [core::ffi::c_char; 0];

    pub fn pagetable_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
