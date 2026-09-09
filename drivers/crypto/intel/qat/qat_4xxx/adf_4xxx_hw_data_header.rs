/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Translated from adf_4xxx_hw_data.h.
// The declarations below depend on types and constants supplied by other headers.

pub const ADF_4XXX_MAX_ACCELENGINES: u32 = 9;

pub const ADF_4XXX_ACCELENGINES_MASK: u32 = 0x1FF;
pub const ADF_4XXX_ADMIN_AE_MASK: u32 = 0x100;

pub const ADF_4XXX_HICPPAGENTCMDPARERRLOG_MASK: u32 = 0x1F;
pub const ADF_4XXX_PARITYERRORMASK_ATH_CPH_MASK: u32 = 0xF000F;
pub const ADF_4XXX_PARITYERRORMASK_CPR_XLT_MASK: u32 = 0x10001;
pub const ADF_4XXX_PARITYERRORMASK_DCPR_UCS_MASK: u32 = 0x30007;
pub const ADF_4XXX_PARITYERRORMASK_PKE_MASK: u32 = 0x3F;

/*
 * SSMFEATREN bit mask
 * BIT(4) - enables parity detection on CPP
 * BIT(12) - enables the logging of push/pull data errors
 *           in pperr register
 * BIT(16) - BIT(23) - enable parity detection on SPPs
 */
pub const ADF_4XXX_SSMFEATREN_MASK: u32 =
    (1u32 << 4) |
    (1u32 << 12) |
    (1u32 << 16) |
    (1u32 << 17) |
    (1u32 << 18) |
    (1u32 << 19) |
    (1u32 << 20) |
    (1u32 << 21) |
    (1u32 << 22) |
    (1u32 << 23);

/* Firmware Binaries */
pub const ADF_4XXX_FW: &str = "qat_4xxx.bin";
pub const ADF_4XXX_MMP: &str = "qat_4xxx_mmp.bin";
pub const ADF_4XXX_SYM_OBJ: &str = "qat_4xxx_sym.bin";
pub const ADF_4XXX_DC_OBJ: &str = "qat_4xxx_dc.bin";
pub const ADF_4XXX_ASYM_OBJ: &str = "qat_4xxx_asym.bin";
pub const ADF_4XXX_ADMIN_OBJ: &str = "qat_4xxx_admin.bin";
/* Firmware for 402XXX */
pub const ADF_402XX_FW: &str = "qat_402xx.bin";
pub const ADF_402XX_MMP: &str = "qat_402xx_mmp.bin";
pub const ADF_402XX_SYM_OBJ: &str = "qat_402xx_sym.bin";
pub const ADF_402XX_DC_OBJ: &str = "qat_402xx_dc.bin";
pub const ADF_402XX_ASYM_OBJ: &str = "qat_402xx_asym.bin";
pub const ADF_402XX_ADMIN_OBJ: &str = "qat_402xx_admin.bin";

/* RL constants */
pub const ADF_4XXX_RL_PCIE_SCALE_FACTOR_DIV: u32 = 100;
pub const ADF_4XXX_RL_PCIE_SCALE_FACTOR_MUL: u32 = 102;
pub const ADF_4XXX_RL_DCPR_CORRECTION: u32 = 1;
pub const ADF_4XXX_RL_SCANS_PER_SEC: u32 = 954;
pub const ADF_4XXX_RL_MAX_TP_ASYM: u32 = 173750;
pub const ADF_4XXX_RL_MAX_TP_SYM: u32 = 95000;
pub const ADF_4XXX_RL_MAX_TP_DC: u32 = 45000;
pub const ADF_4XXX_RL_SLICE_REF: u32 = 1000;

/* Clocks frequency */
pub const ADF_4XXX_AE_FREQ: u32 = 1000 * HZ_PER_MHZ;

unsafe extern "C" {
    pub fn adf_init_hw_data_4xxx(hw_data: *mut adf_hw_device_data, dev_id: u32);
    pub fn adf_clean_hw_data_4xxx(hw_data: *mut adf_hw_device_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
