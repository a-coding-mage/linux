/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2023 ARM Ltd. */

/*
 * This file describes the Realm Services Interface (RSI) Application Binary
 * Interface (ABI) for SMC calls made from within the Realm to the RMM and
 * serviced by the RMM.
 */

pub const RSI_ABI_VERSION_MAJOR: u64 = 1;
pub const RSI_ABI_VERSION_MINOR: u64 = 0;
pub const RSI_ABI_VERSION: u64 = (RSI_ABI_VERSION_MAJOR << 16) | RSI_ABI_VERSION_MINOR;

pub const fn rsi_abi_version_get_major(version: u64) -> u64 { version >> 16 }
pub const fn rsi_abi_version_get_minor(version: u64) -> u64 { version & 0xffff }

pub const RSI_SUCCESS: u64 = 0;
pub const RSI_ERROR_INPUT: u64 = 1;
pub const RSI_ERROR_STATE: u64 = 2;
pub const RSI_INCOMPLETE: u64 = 3;
pub const RSI_ERROR_UNKNOWN: u64 = 4;

/* Supplied by the ARM SMCCC definitions. */
pub const fn smc_rsi_fid(n: u64) -> u64 {
    arm_smccc_call_val(ARM_SMCCC_FAST_CALL, ARM_SMCCC_SMC_64,
                       ARM_SMCCC_OWNER_STANDARD, n)
}

pub const SMC_RSI_ABI_VERSION: u64 = smc_rsi_fid(0x190);
pub const SMC_RSI_FEATURES: u64 = smc_rsi_fid(0x191);
pub const SMC_RSI_MEASUREMENT_READ: u64 = smc_rsi_fid(0x192);
pub const SMC_RSI_MEASUREMENT_EXTEND: u64 = smc_rsi_fid(0x193);
pub const SMC_RSI_ATTESTATION_TOKEN_INIT: u64 = smc_rsi_fid(0x194);
pub const SMC_RSI_ATTESTATION_TOKEN_CONTINUE: u64 = smc_rsi_fid(0x195);

#[repr(C)]
pub struct realm_config_fields {
    pub ipa_bits: usize,
    pub hash_algo: usize,
}

#[repr(C)]
pub union realm_config_first {
    pub fields: realm_config_fields,
    pub pad: [u8; 0x200],
}

#[repr(C)]
pub union realm_config_second {
    pub rpv: [u8; 64],
    pub pad2: [u8; 0xe00],
}

#[repr(C, align(4096))]
pub struct realm_config {
    pub first: realm_config_first,
    pub second: realm_config_second,
}

pub const SMC_RSI_REALM_CONFIG: u64 = smc_rsi_fid(0x196);
pub const SMC_RSI_IPA_STATE_SET: u64 = smc_rsi_fid(0x197);

pub const RSI_NO_CHANGE_DESTROYED: u64 = 0;
pub const RSI_CHANGE_DESTROYED: u64 = 1;
pub const RSI_ACCEPT: u64 = 0;
pub const RSI_REJECT: u64 = 1;

pub const SMC_RSI_IPA_STATE_GET: u64 = smc_rsi_fid(0x198);
pub const SMC_RSI_HOST_CALL: u64 = smc_rsi_fid(0x199);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
