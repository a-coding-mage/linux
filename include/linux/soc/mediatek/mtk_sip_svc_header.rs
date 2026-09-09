/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 MediaTek Inc.
 */

/* Error Code */
pub const SIP_SVC_E_SUCCESS: i32 = 0;
pub const SIP_SVC_E_NOT_SUPPORTED: i32 = -1;
pub const SIP_SVC_E_INVALID_PARAMS: i32 = -2;
pub const SIP_SVC_E_INVALID_RANGE: i32 = -3;
pub const SIP_SVC_E_PERMISSION_DENIED: i32 = -4;

#[cfg(target_arch = "aarch64")]
pub const MTK_SIP_SMC_CONVENTION: u32 = ARM_SMCCC_SMC_64;

#[cfg(not(target_arch = "aarch64"))]
pub const MTK_SIP_SMC_CONVENTION: u32 = ARM_SMCCC_SMC_32;

/*
 * ARM_SMCCC_CALL_VAL and the ARM_SMCCC_* symbols are supplied by the
 * corresponding ARM SMCCC dependency.
 */
#[inline]
pub const fn mtk_sip_smc_cmd(fn_id: u32) -> u32 {
    ARM_SMCCC_CALL_VAL!(
        ARM_SMCCC_FAST_CALL,
        MTK_SIP_SMC_CONVENTION,
        ARM_SMCCC_OWNER_SIP,
        fn_id
    )
}

/* Modem related SMC call */
pub const MTK_SIP_KERNEL_CCCI_CONTROL: u32 = mtk_sip_smc_cmd(0x505);

/* DVFSRC SMC calls */
pub const MTK_SIP_DVFSRC_VCOREFS_CONTROL: u32 = mtk_sip_smc_cmd(0x506);

/* IOMMU related SMC call */
pub const MTK_SIP_KERNEL_IOMMU_CONTROL: u32 = mtk_sip_smc_cmd(0x514);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
