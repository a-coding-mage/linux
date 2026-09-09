// SPDX-License-Identifier: GPL-2.0

// C conditional:
// #ifdef CONFIG_MMU
// #include "setup_mm.c"
// #else
// #include "setup_no.c"
// #endif
// The selected implementation is supplied by the corresponding translated
// source dependency.

// C conditional:
// #if IS_ENABLED(CONFIG_INPUT_M68K_BEEP)
//
// C declaration: void (*mach_beep)(unsigned int, unsigned int);
// EXPORT_SYMBOL(mach_beep);
//
// The build-time CONFIG_INPUT_M68K_BEEP condition is preserved here as
// intent; its configuration is supplied by the surrounding build.
#[allow(non_upper_case_globals)]
pub static mut mach_beep: Option<unsafe extern "C" fn(u32, u32)> = None;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
