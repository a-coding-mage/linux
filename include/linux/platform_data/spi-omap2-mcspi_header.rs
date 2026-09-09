/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _OMAP2_MCSPI_H

pub const OMAP4_MCSPI_REG_OFFSET: u32 = 0x100;

pub const MCSPI_PINDIR_D0_IN_D1_OUT: u32 = 0;
pub const MCSPI_PINDIR_D0_OUT_D1_IN: u32 = 1;

#[repr(C)]
pub struct omap2_mcspi_platform_config {
    pub num_cs: u16,
    pub regs_offset: u32,
    // C bit-field: unsigned int pin_dir:1;
    pub pin_dir: u32,
    pub max_xfer_len: usize,
}

#[repr(C)]
pub struct omap2_mcspi_device_config {
    // C bit-field: unsigned turbo_mode:1;
    pub turbo_mode: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
