/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependency supplied by the surrounding translation unit: linux/units.h.

/* PCIe configuration space */
pub const ADF_C3XXX_PMISC_BAR: i32 = 0;
pub const ADF_C3XXX_ETR_BAR: i32 = 1;
pub const ADF_C3XXX_SRAM_BAR: i32 = 0;
pub const ADF_C3XXX_MAX_ACCELERATORS: i32 = 3;
pub const ADF_C3XXX_MAX_ACCELENGINES: i32 = 6;
pub const ADF_C3XXX_ACCELERATORS_REG_OFFSET: i32 = 16;
pub const ADF_C3XXX_ACCELERATORS_MASK: i32 = 0x7;
pub const ADF_C3XXX_ACCELENGINES_MASK: i32 = 0x3F;
pub const ADF_C3XXX_ETR_MAX_BANKS: i32 = 16;
pub const ADF_C3XXX_SOFTSTRAP_CSR_OFFSET: i32 = 0x2EC;

/* AE to function mapping */
pub const ADF_C3XXX_AE2FUNC_MAP_GRP_A_NUM_REGS: i32 = 48;
pub const ADF_C3XXX_AE2FUNC_MAP_GRP_B_NUM_REGS: i32 = 6;

/* Clocks frequency */
// HZ_PER_MHZ is supplied by the translated linux/units.h dependency.
pub const ADF_C3XXX_AE_FREQ: i32 = 685 * HZ_PER_MHZ;
pub const ADF_C3XXX_MIN_AE_FREQ: i32 = 533 * HZ_PER_MHZ;
pub const ADF_C3XXX_MAX_AE_FREQ: i32 = 685 * HZ_PER_MHZ;

/* Firmware Binary */
pub const ADF_C3XXX_FW: &str = "qat_c3xxx.bin";
pub const ADF_C3XXX_MMP: &str = "qat_c3xxx_mmp.bin";

// adf_hw_device_data is supplied by the surrounding translation unit.
extern "C" {
    pub fn adf_init_hw_data_c3xxx(hw_data: *mut adf_hw_device_data);
    pub fn adf_clean_hw_data_c3xxx(hw_data: *mut adf_hw_device_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
