/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2025 Intel Corporation */

// Dependency equivalent of: #include <linux/types.h>

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ring_config {
    pub base: u64,
    pub config: u32,
    pub head: u32,
    pub tail: u32,
    pub reserved0: u32,
}

#[repr(C)]
pub struct adf_bank_state {
    pub ringstat0: u32,
    pub ringstat1: u32,
    pub ringuostat: u32,
    pub ringestat: u32,
    pub ringnestat: u32,
    pub ringnfstat: u32,
    pub ringfstat: u32,
    pub ringcstat0: u32,
    pub ringcstat1: u32,
    pub ringcstat2: u32,
    pub ringcstat3: u32,
    pub iaintflagen: u32,
    pub iaintflagreg: u32,
    pub iaintflagsrcsel0: u32,
    pub iaintflagsrcsel1: u32,
    pub iaintcolen: u32,
    pub iaintcolctl: u32,
    pub iaintflagandcolen: u32,
    pub ringexpstat: u32,
    pub ringexpintenable: u32,
    pub ringsrvarben: u32,
    pub reserved0: u32,
    pub rings: [ring_config; ADF_ETR_MAX_RINGS_PER_BANK],
}

unsafe extern "C" {
    pub fn adf_bank_state_restore(
        accel_dev: *mut adf_accel_dev,
        bank_number: u32,
        state: *mut adf_bank_state,
    ) -> i32;

    pub fn adf_bank_state_save(
        accel_dev: *mut adf_accel_dev,
        bank_number: u32,
        state: *mut adf_bank_state,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
