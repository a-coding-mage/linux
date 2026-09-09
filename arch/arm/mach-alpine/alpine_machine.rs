// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Machine declaration for Alpine platforms.
 *
 * Copyright (C) 2015 Annapurna Labs Ltd.
 */

use core::ffi::c_char;

// Corresponds to the C __initconst compatibility table.  The terminating
// null pointer is part of the table's ABI.
#[used]
static al_match: [*const c_char; 2] = [
    b"al,alpine\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(AL_DT, "Annapurna Labs Alpine")
//     .dt_compat = al_match,
// MACHINE_END
//
// The machine-descriptor structure and registration emitted by these macros
// are supplied by asm/mach/arch.h and are therefore retained as the
// corresponding external kernel declaration here.
extern "C" {
    static AL_DT: core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
