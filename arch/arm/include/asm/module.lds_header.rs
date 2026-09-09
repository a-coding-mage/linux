/* SPDX-License-Identifier: GPL-2.0 */

// Equivalent of CONFIG_ARM_MODULE_PLTS. The original linker-script sections
// are represented here as zero-filled section markers.
#[cfg(feature = "CONFIG_ARM_MODULE_PLTS")]
pub mod sections {
    pub const PLT: u8 = 0;
    pub const INIT_PLT: u8 = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
