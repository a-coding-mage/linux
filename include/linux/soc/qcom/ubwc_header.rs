/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2018, The Linux Foundation
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

#[repr(C)]
pub struct qcom_ubwc_cfg_data {
    pub ubwc_enc_version: u32,
    /**
     * @highest_bank_bit: Highest Bank Bit
     *
     * The Highest Bank Bit value represents the bit of the highest
     * DDR bank.  This should ideally use DRAM type detection.
     */
    pub highest_bank_bit: i32,
    pub flags: u32,
}

pub const UBWC_FLAG_DISABLE_SWIZZLE_LVL2: u32 = 1 << 0;
pub const UBWC_FLAG_DISABLE_SWIZZLE_LVL3: u32 = 1 << 1;

pub const UBWC_1_0: u32 = 0x10000000;
pub const UBWC_2_0: u32 = 0x20000000;
pub const UBWC_3_0: u32 = 0x30000000;
pub const UBWC_3_1: u32 = 0x30010000; /* UBWC 3.0 + Macrotile mode */
pub const UBWC_4_0: u32 = 0x40000000;
pub const UBWC_4_3: u32 = 0x40030000;
pub const UBWC_5_0: u32 = 0x50000000;
pub const UBWC_6_0: u32 = 0x60000000;

// Build-time condition: CONFIG_QCOM_UBWC_CONFIG.
#[cfg(CONFIG_QCOM_UBWC_CONFIG)]
extern "C" {
    pub fn qcom_ubwc_config_get_data() -> *const qcom_ubwc_cfg_data;
}

#[cfg(not(CONFIG_QCOM_UBWC_CONFIG))]
pub unsafe fn qcom_ubwc_config_get_data() -> *const qcom_ubwc_cfg_data {
    // ERR_PTR(-EOPNOTSUPP)
    core::mem::transmute::<isize, *const qcom_ubwc_cfg_data>(-95isize)
}

pub unsafe fn qcom_ubwc_get_ubwc_mode(cfg: *const qcom_ubwc_cfg_data) -> bool {
    (*cfg).ubwc_enc_version == UBWC_1_0
}

pub unsafe fn qcom_ubwc_min_acc_length_64b(cfg: *const qcom_ubwc_cfg_data) -> bool {
    (*cfg).ubwc_enc_version == UBWC_1_0
}

/*
 * @qcom_ubwc_macrotile_mode: whether to use 4-channel or 8-channel macrotiling
 *
 * The 8-channel macrotiling mode was introduced in UBWC 3.1.
 *
 * Returns: false for the 4-channel and true for 8-channel.
 */
pub unsafe fn qcom_ubwc_macrotile_mode(cfg: *const qcom_ubwc_cfg_data) -> bool {
    (*cfg).ubwc_enc_version >= UBWC_3_1
}

pub unsafe fn qcom_ubwc_bank_spread(_cfg: *const qcom_ubwc_cfg_data) -> bool {
    true
}

pub const UBWC_SWIZZLE_ENABLE_LVL1: u32 = 1 << 0;
pub const UBWC_SWIZZLE_ENABLE_LVL2: u32 = 1 << 1;
pub const UBWC_SWIZZLE_ENABLE_LVL3: u32 = 1 << 2;

/**
 * @qcom_ubwc_swizzle: Whether to enable level 1, 2 & 3 bank swizzling.
 *
 * UBWC 1.0 always enables all three levels.
 * UBWC 2.0 removes level 1 bank swizzling, leaving levels 2 & 3.
 * UBWC 4.0 adds the optional ability to disable levels 2 & 3.
 */
pub unsafe fn qcom_ubwc_swizzle(cfg: *const qcom_ubwc_cfg_data) -> u32 {
    if (*cfg).ubwc_enc_version == 0 {
        return 0;
    }

    if (*cfg).ubwc_enc_version == UBWC_1_0 {
        return UBWC_SWIZZLE_ENABLE_LVL1 |
            UBWC_SWIZZLE_ENABLE_LVL2 |
            UBWC_SWIZZLE_ENABLE_LVL3;
    }

    let mut ubwc_swizzle = UBWC_SWIZZLE_ENABLE_LVL2 |
        UBWC_SWIZZLE_ENABLE_LVL3;

    if (*cfg).flags & UBWC_FLAG_DISABLE_SWIZZLE_LVL2 != 0 {
        ubwc_swizzle &= !UBWC_SWIZZLE_ENABLE_LVL2;
    }

    if (*cfg).flags & UBWC_FLAG_DISABLE_SWIZZLE_LVL3 != 0 {
        ubwc_swizzle &= !UBWC_SWIZZLE_ENABLE_LVL3;
    }

    ubwc_swizzle
}

pub unsafe fn qcom_ubwc_version_tag(cfg: *const qcom_ubwc_cfg_data) -> u32 {
    if (*cfg).ubwc_enc_version >= UBWC_6_0 {
        return 5;
    }
    if (*cfg).ubwc_enc_version >= UBWC_5_0 {
        return 4;
    }
    if (*cfg).ubwc_enc_version >= UBWC_4_3 {
        return 3;
    }
    if (*cfg).ubwc_enc_version >= UBWC_4_0 {
        return 2;
    }
    if (*cfg).ubwc_enc_version >= UBWC_3_0 {
        return 1;
    }

    0
}

pub unsafe fn qcom_ubwc_enable_amsbc(cfg: *const qcom_ubwc_cfg_data) -> bool {
    (*cfg).ubwc_enc_version >= UBWC_3_0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
