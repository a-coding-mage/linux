// SPDX-License-Identifier: GPL-2.0-only
/*
 * Support for Conexant Digicolor SoCs
 */

// Dependency supplied by the architecture support code in the original source:
// #include <asm/mach/arch.h>

// __initconst: this table is intended for initialization-time use.
pub static DIGICOLOR_DT_COMPAT: [*const core::ffi::c_char; 2] = [
    c"cnxt,cx92755".as_ptr(),
    core::ptr::null(),
];

// DT_MACHINE_START(DIGICOLOR, "Conexant Digicolor (Flattened Device Tree)")
//     .dt_compat = digicolor_dt_compat,
// MACHINE_END
// The machine descriptor is emitted by the architecture's DT_MACHINE_START /
// MACHINE_END declarations, which are supplied externally.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
