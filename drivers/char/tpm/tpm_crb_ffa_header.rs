/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Arm Ltd.
 *
 * This device driver implements the TPM CRB start method
 * as defined in the TPM Service Command Response Buffer
 * Interface Over FF-A (DEN0138).
 */

use core::ffi::c_int;

// Equivalent to: #if IS_REACHABLE(CONFIG_TCG_ARM_CRB_FFA)
// The feature name is a Rust representation of the build-time condition.
#[cfg(feature = "tcg_arm_crb_ffa")]
extern "C" {
    pub fn tpm_crb_ffa_init() -> c_int;
    pub fn tpm_crb_ffa_start(request_type: c_int, locality: c_int) -> c_int;
}

// Equivalent to the !IS_REACHABLE(CONFIG_TCG_ARM_CRB_FFA) branch.
#[cfg(not(feature = "tcg_arm_crb_ffa"))]
#[inline]
pub fn tpm_crb_ffa_init() -> c_int {
    0
}

#[cfg(not(feature = "tcg_arm_crb_ffa"))]
#[inline]
pub fn tpm_crb_ffa_start(_request_type: c_int, _locality: c_int) -> c_int {
    0
}

pub const CRB_FFA_START_TYPE_COMMAND: c_int = 0;
pub const CRB_FFA_START_TYPE_LOCALITY_REQUEST: c_int = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
