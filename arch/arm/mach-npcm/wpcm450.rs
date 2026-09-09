// SPDX-License-Identifier: GPL-2.0
// Copyright 2021 Jonathan Neuschäfer

use core::ffi::c_char;

// Translated from the dependency supplied by <asm/mach/arch.h>.
static wpcm450_dt_match: [*const c_char; 2] = [
    b"nuvoton,wpcm450\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(WPCM450_DT, "WPCM450 chip")
// MACHINE_END
// The machine-description type and registration are supplied by the architecture
// dependencies; this preserves the fields emitted by the C macros.
static WPCM450_DT: MachineDesc = MachineDesc {
    dt_compat: wpcm450_dt_match.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
