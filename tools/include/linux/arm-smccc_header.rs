/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015, Linaro Limited
 */

/* Dependency intent from C header: <linux/const.h> supplied _AC(). */

/*
 * This file provides common defines for ARM SMC Calling Convention as
 * specified in
 * https://developer.arm.com/docs/den0028/latest
 *
 * This code is up-to-date with version DEN 0028 C
 */

pub const ARM_SMCCC_STD_CALL: u32 = 0;
pub const ARM_SMCCC_FAST_CALL: u32 = 1;
pub const ARM_SMCCC_TYPE_SHIFT: u32 = 31;

pub const ARM_SMCCC_SMC_32: u32 = 0;
pub const ARM_SMCCC_SMC_64: u32 = 1;
pub const ARM_SMCCC_CALL_CONV_SHIFT: u32 = 30;

pub const ARM_SMCCC_OWNER_MASK: u32 = 0x3F;
pub const ARM_SMCCC_OWNER_SHIFT: u32 = 24;

pub const ARM_SMCCC_FUNC_MASK: u32 = 0xFFFF;

pub const fn ARM_SMCCC_IS_FAST_CALL(smc_val: u32) -> u32 {
    smc_val & (ARM_SMCCC_FAST_CALL << ARM_SMCCC_TYPE_SHIFT)
}

pub const fn ARM_SMCCC_IS_64(smc_val: u32) -> u32 {
    smc_val & (ARM_SMCCC_SMC_64 << ARM_SMCCC_CALL_CONV_SHIFT)
}

pub const fn ARM_SMCCC_FUNC_NUM(smc_val: u32) -> u32 {
    smc_val & ARM_SMCCC_FUNC_MASK
}

pub const fn ARM_SMCCC_OWNER_NUM(smc_val: u32) -> u32 {
    (smc_val >> ARM_SMCCC_OWNER_SHIFT) & ARM_SMCCC_OWNER_MASK
}

pub const fn ARM_SMCCC_CALL_VAL(
    r#type: u32,
    calling_convention: u32,
    owner: u32,
    func_num: u32,
) -> u32 {
    (r#type << ARM_SMCCC_TYPE_SHIFT)
        | (calling_convention << ARM_SMCCC_CALL_CONV_SHIFT)
        | ((owner & ARM_SMCCC_OWNER_MASK) << ARM_SMCCC_OWNER_SHIFT)
        | (func_num & ARM_SMCCC_FUNC_MASK)
}

pub const ARM_SMCCC_OWNER_ARCH: u32 = 0;
pub const ARM_SMCCC_OWNER_CPU: u32 = 1;
pub const ARM_SMCCC_OWNER_SIP: u32 = 2;
pub const ARM_SMCCC_OWNER_OEM: u32 = 3;
pub const ARM_SMCCC_OWNER_STANDARD: u32 = 4;
pub const ARM_SMCCC_OWNER_STANDARD_HYP: u32 = 5;
pub const ARM_SMCCC_OWNER_VENDOR_HYP: u32 = 6;
pub const ARM_SMCCC_OWNER_TRUSTED_APP: u32 = 48;
pub const ARM_SMCCC_OWNER_TRUSTED_APP_END: u32 = 49;
pub const ARM_SMCCC_OWNER_TRUSTED_OS: u32 = 50;
pub const ARM_SMCCC_OWNER_TRUSTED_OS_END: u32 = 63;

pub const ARM_SMCCC_FUNC_QUERY_CALL_UID: u32 = 0xff01;

pub const ARM_SMCCC_QUIRK_NONE: u32 = 0;
pub const ARM_SMCCC_QUIRK_QCOM_A6: u32 = 1; /* Save/restore register a6 */

pub const ARM_SMCCC_VERSION_1_0: u32 = 0x10000;
pub const ARM_SMCCC_VERSION_1_1: u32 = 0x10001;
pub const ARM_SMCCC_VERSION_1_2: u32 = 0x10002;
pub const ARM_SMCCC_VERSION_1_3: u32 = 0x10003;

pub const ARM_SMCCC_1_3_SVE_HINT: u32 = 0x10000;

pub const ARM_SMCCC_VERSION_FUNC_ID: u32 =
    ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL, ARM_SMCCC_SMC_32, 0, 0);

pub const ARM_SMCCC_ARCH_FEATURES_FUNC_ID: u32 =
    ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL, ARM_SMCCC_SMC_32, 0, 1);

pub const ARM_SMCCC_ARCH_SOC_ID: u32 =
    ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL, ARM_SMCCC_SMC_32, 0, 2);

pub const ARM_SMCCC_ARCH_WORKAROUND_1: u32 =
    ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL, ARM_SMCCC_SMC_32, 0, 0x8000);

pub const ARM_SMCCC_ARCH_WORKAROUND_2: u32 =
    ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL, ARM_SMCCC_SMC_32, 0, 0x7fff);

pub const ARM_SMCCC_ARCH_WORKAROUND_3: u32 =
    ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL, ARM_SMCCC_SMC_32, 0, 0x3fff);

pub const ARM_SMCCC_VENDOR_HYP_CALL_UID_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(
    ARM_SMCCC_FAST_CALL,
    ARM_SMCCC_SMC_32,
    ARM_SMCCC_OWNER_VENDOR_HYP,
    ARM_SMCCC_FUNC_QUERY_CALL_UID,
);

/* KVM UID value: 28b46fb6-2ec5-11e9-a9ca-4b564d003a74 */
pub const ARM_SMCCC_VENDOR_HYP_UID_KVM_REG_0: u32 = 0xb66fb428;
pub const ARM_SMCCC_VENDOR_HYP_UID_KVM_REG_1: u32 = 0xe911c52e;
pub const ARM_SMCCC_VENDOR_HYP_UID_KVM_REG_2: u32 = 0x564bcaa9;
pub const ARM_SMCCC_VENDOR_HYP_UID_KVM_REG_3: u32 = 0x743a004d;

/* KVM "vendor specific" services */
pub const ARM_SMCCC_KVM_FUNC_FEATURES: u32 = 0;
pub const ARM_SMCCC_KVM_FUNC_PTP: u32 = 1;
pub const ARM_SMCCC_KVM_FUNC_FEATURES_2: u32 = 127;
pub const ARM_SMCCC_KVM_NUM_FUNCS: u32 = 128;

pub const ARM_SMCCC_VENDOR_HYP_KVM_FEATURES_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(
    ARM_SMCCC_FAST_CALL,
    ARM_SMCCC_SMC_32,
    ARM_SMCCC_OWNER_VENDOR_HYP,
    ARM_SMCCC_KVM_FUNC_FEATURES,
);

pub const SMCCC_ARCH_WORKAROUND_RET_UNAFFECTED: i32 = 1;

/*
 * ptp_kvm is a feature used for time sync between vm and host.
 * ptp_kvm module in guest kernel will get service from host using
 * this hypercall ID.
 */
pub const ARM_SMCCC_VENDOR_HYP_KVM_PTP_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(
    ARM_SMCCC_FAST_CALL,
    ARM_SMCCC_SMC_32,
    ARM_SMCCC_OWNER_VENDOR_HYP,
    ARM_SMCCC_KVM_FUNC_PTP,
);

/* ptp_kvm counter type ID */
pub const KVM_PTP_VIRT_COUNTER: u32 = 0;
pub const KVM_PTP_PHYS_COUNTER: u32 = 1;

/* Paravirtualised time calls (defined by ARM DEN0057A) */
pub const ARM_SMCCC_HV_PV_TIME_FEATURES: u32 = ARM_SMCCC_CALL_VAL(
    ARM_SMCCC_FAST_CALL,
    ARM_SMCCC_SMC_64,
    ARM_SMCCC_OWNER_STANDARD_HYP,
    0x20,
);

pub const ARM_SMCCC_HV_PV_TIME_ST: u32 = ARM_SMCCC_CALL_VAL(
    ARM_SMCCC_FAST_CALL,
    ARM_SMCCC_SMC_64,
    ARM_SMCCC_OWNER_STANDARD_HYP,
    0x21,
);

/* TRNG entropy source calls (defined by ARM DEN0098) */
pub const ARM_SMCCC_TRNG_VERSION: u32 = ARM_SMCCC_CALL_VAL(
    ARM_SMCCC_FAST_CALL,
    ARM_SMCCC_SMC_32,
    ARM_SMCCC_OWNER_STANDARD,
    0x50,
);

pub const ARM_SMCCC_TRNG_FEATURES: u32 = ARM_SMCCC_CALL_VAL(
    ARM_SMCCC_FAST_CALL,
    ARM_SMCCC_SMC_32,
    ARM_SMCCC_OWNER_STANDARD,
    0x51,
);

pub const ARM_SMCCC_TRNG_GET_UUID: u32 = ARM_SMCCC_CALL_VAL(
    ARM_SMCCC_FAST_CALL,
    ARM_SMCCC_SMC_32,
    ARM_SMCCC_OWNER_STANDARD,
    0x52,
);

pub const ARM_SMCCC_TRNG_RND32: u32 = ARM_SMCCC_CALL_VAL(
    ARM_SMCCC_FAST_CALL,
    ARM_SMCCC_SMC_32,
    ARM_SMCCC_OWNER_STANDARD,
    0x53,
);

pub const ARM_SMCCC_TRNG_RND64: u32 = ARM_SMCCC_CALL_VAL(
    ARM_SMCCC_FAST_CALL,
    ARM_SMCCC_SMC_64,
    ARM_SMCCC_OWNER_STANDARD,
    0x53,
);

/*
 * Return codes defined in ARM DEN 0070A
 * ARM DEN 0070A is now merged/consolidated into ARM DEN 0028 C
 */
pub const SMCCC_RET_SUCCESS: i32 = 0;
pub const SMCCC_RET_NOT_SUPPORTED: i32 = -1;
pub const SMCCC_RET_NOT_REQUIRED: i32 = -2;
pub const SMCCC_RET_INVALID_PARAMETER: i32 = -3;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
