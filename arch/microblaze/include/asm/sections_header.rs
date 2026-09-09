/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// C header guard: _ASM_MICROBLAZE_SECTIONS_H
// Dependency: <asm-generic/sections.h>
// The declarations below correspond to the non-assembler portion of the C header.

unsafe extern "C" {
    pub static mut _ssbss: [core::ffi::c_char; 0];
    pub static mut _esbss: [core::ffi::c_char; 0];
    pub static mut __ivt_start: [core::ffi::c_ulong; 0];
    pub static mut __ivt_end: [core::ffi::c_ulong; 0];

    pub static mut _fdt_start: [u32; 0];
    pub static mut _fdt_end: [u32; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
