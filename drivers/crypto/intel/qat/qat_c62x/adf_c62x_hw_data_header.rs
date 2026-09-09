/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependency supplied by the Linux units header: HZ_PER_MHZ.

/* PCIe configuration space */
pub const ADF_C62X_SRAM_BAR: u32 = 0;
pub const ADF_C62X_PMISC_BAR: u32 = 1;
pub const ADF_C62X_ETR_BAR: u32 = 2;
pub const ADF_C62X_MAX_ACCELERATORS: u32 = 5;
pub const ADF_C62X_MAX_ACCELENGINES: u32 = 10;
pub const ADF_C62X_ACCELERATORS_REG_OFFSET: u32 = 16;
pub const ADF_C62X_ACCELERATORS_MASK: u32 = 0x1F;
pub const ADF_C62X_ACCELENGINES_MASK: u32 = 0x3FF;
pub const ADF_C62X_ETR_MAX_BANKS: u32 = 16;
pub const ADF_C62X_SOFTSTRAP_CSR_OFFSET: u32 = 0x2EC;

/* AE to function mapping */
pub const ADF_C62X_AE2FUNC_MAP_GRP_A_NUM_REGS: u32 = 80;
pub const ADF_C62X_AE2FUNC_MAP_GRP_B_NUM_REGS: u32 = 10;

/* Clocks frequency */
pub const ADF_C62X_AE_FREQ: u64 = 685 * HZ_PER_MHZ;
pub const ADF_C62X_MIN_AE_FREQ: u64 = 533 * HZ_PER_MHZ;
pub const ADF_C62X_MAX_AE_FREQ: u64 = 800 * HZ_PER_MHZ;

/* Firmware Binary */
pub const ADF_C62X_FW: &[u8] = b"qat_c62x.bin\0";
pub const ADF_C62X_MMP: &[u8] = b"qat_c62x_mmp.bin\0";

// External dependency supplied by the surrounding repository.
pub enum adf_hw_device_data {}

extern "C" {
    pub fn adf_init_hw_data_c62x(hw_data: *mut adf_hw_device_data);
    pub fn adf_clean_hw_data_c62x(hw_data: *mut adf_hw_device_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
