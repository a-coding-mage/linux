/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2021 ARM Ltd.
 */

// Dependencies supplied by the corresponding Linux ARM FF-A and SMCCC headers.

pub type ffa_value_t = arm_smccc_1_2_regs;

pub type ffa_fn = unsafe extern "C" fn(ffa_value_t, *mut ffa_value_t);

unsafe extern "C" {
    pub fn ffa_device_is_valid(ffa_dev: *mut ffa_device) -> bool;
    pub fn ffa_device_match_uuid(ffa_dev: *mut ffa_device, uuid: *const uuid_t);
}

#[cfg(CONFIG_ARM_FFA_SMCCC)]
unsafe extern "C" {
    pub fn ffa_transport_init(invoke_ffa_fn: *mut *mut ffa_fn) -> i32;
}

#[cfg(not(CONFIG_ARM_FFA_SMCCC))]
pub unsafe fn ffa_transport_init(_invoke_ffa_fn: *mut *mut ffa_fn) -> i32 {
    -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
