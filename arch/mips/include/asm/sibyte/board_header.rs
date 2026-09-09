/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2000,2001,2002,2003,2004 Broadcom Corporation
 */

// Original header guard: _SIBYTE_BOARD_H

// Build-time dependencies from the C header:
// CONFIG_SIBYTE_SWARM, CONFIG_SIBYTE_CRHONE, and CONFIG_SIBYTE_LITTLESUR
// include <asm/sibyte/swarm.h>.
// CONFIG_SIBYTE_SENTOSA and CONFIG_SIBYTE_RHONE include <asm/sibyte/sentosa.h>.
// CONFIG_SIBYTE_BIGSUR includes <asm/sibyte/bigsur.h>.

// The __ASSEMBLER__ branch contains a MIPS assembler-only macro and has no
// direct Rust executable equivalent. When LEDS_PHYS is defined, setleds(t0,
// t1, c0, c1, c2, c3) loads LEDS_PHYS|0xa0000000 and stores c0..c3 at offsets
// 0x18, 0x10, 0x08, and 0x00 respectively; otherwise it is empty.

unsafe extern "C" {
    pub fn swarm_setup();
}

#[cfg(feature = "LEDS_PHYS")]
unsafe extern "C" {
    pub fn setleds(str_: *mut core::ffi::c_char);
}

#[cfg(not(feature = "LEDS_PHYS"))]
#[macro_export]
macro_rules! setleds {
    ($s:expr) => {{
        let _ = &$s;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
