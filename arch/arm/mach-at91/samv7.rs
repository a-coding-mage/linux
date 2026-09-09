// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Setup code for SAMv7x
 *
 *  Copyright (C) 2013 Atmel,
 *                2016 Andras Szemzo <szemzo.andras@gmail.com>
 */

// Dependency supplied by <asm/mach/arch.h> in the C source.

#[used]
#[cfg_attr(any(), link_section = ".init.rodata")]
static SAMV7_DT_BOARD_COMPAT: [Option<&'static core::ffi::CStr>; 2] = [
    Some(unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"atmel,samv7\0") }),
    None,
];

// DT_MACHINE_START(samv7_dt, "Atmel SAMV7")
//     .dt_compat = samv7_dt_board_compat,
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
