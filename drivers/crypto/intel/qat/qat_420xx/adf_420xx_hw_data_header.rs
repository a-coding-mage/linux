/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

// Dependency equivalent of <adf_accel_devices.h> is supplied externally.

pub const ADF_420XX_MAX_ACCELENGINES: u32 = 17;

pub const ADF_420XX_ACCELENGINES_MASK: u32 = 0x1FFFF;
pub const ADF_420XX_ADMIN_AE_MASK: u32 = 0x10000;

pub const ADF_420XX_HICPPAGENTCMDPARERRLOG_MASK: u32 = 0xFF;
pub const ADF_420XX_PARITYERRORMASK_ATH_CPH_MASK: u32 = 0xFF00FF;
pub const ADF_420XX_PARITYERRORMASK_CPR_XLT_MASK: u32 = 0x10001;
pub const ADF_420XX_PARITYERRORMASK_DCPR_UCS_MASK: u32 = 0xF0007;
pub const ADF_420XX_PARITYERRORMASK_PKE_MASK: u32 = 0xFFF;
pub const ADF_420XX_PARITYERRORMASK_WAT_WCP_MASK: u32 = 0x3FF03FF;

/*
 * SSMFEATREN bit mask
 * BIT(4) - enables parity detection on CPP
 * BIT(12) - enables the logging of push/pull data errors
 *           in pperr register
 * BIT(16) - BIT(27) - enable parity detection on SPPs
 */
// BIT is supplied externally, equivalent to the C BIT() macro.
pub const ADF_420XX_SSMFEATREN_MASK: u32 =
    BIT(4) | BIT(12) | BIT(16) | BIT(17) | BIT(18) | BIT(19) | BIT(20) |
    BIT(21) | BIT(22) | BIT(23) | BIT(24) | BIT(25) | BIT(26) | BIT(27);

/* Firmware Binaries */
pub const ADF_420XX_FW: &str = "qat_420xx.bin";
pub const ADF_420XX_MMP: &str = "qat_420xx_mmp.bin";
pub const ADF_420XX_SYM_OBJ: &str = "qat_420xx_sym.bin";
pub const ADF_420XX_DC_OBJ: &str = "qat_420xx_dc.bin";
pub const ADF_420XX_ASYM_OBJ: &str = "qat_420xx_asym.bin";
pub const ADF_420XX_ADMIN_OBJ: &str = "qat_420xx_admin.bin";

/* RL constants */
pub const ADF_420XX_RL_PCIE_SCALE_FACTOR_DIV: u32 = 100;
pub const ADF_420XX_RL_PCIE_SCALE_FACTOR_MUL: u32 = 102;
pub const ADF_420XX_RL_DCPR_CORRECTION: u32 = 1;
pub const ADF_420XX_RL_SCANS_PER_SEC: u32 = 954;
pub const ADF_420XX_RL_MAX_TP_ASYM: u64 = 173750;
pub const ADF_420XX_RL_MAX_TP_SYM: u64 = 95000;
pub const ADF_420XX_RL_MAX_TP_DC: u64 = 40000;
pub const ADF_420XX_RL_SLICE_REF: u64 = 1000;

/* Clocks frequency */
// HZ_PER_MHZ is supplied externally, equivalent to the C constant.
pub const ADF_420XX_AE_FREQ: u32 = 1000 * HZ_PER_MHZ;

extern "C" {
    pub fn adf_init_hw_data_420xx(hw_data: *mut adf_hw_device_data, dev_id: u32);
    pub fn adf_clean_hw_data_420xx(hw_data: *mut adf_hw_device_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
