/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2019 Arm Ltd. */

/* The below structure is defined in ARM DEN0057A */
#[repr(C, packed)]
pub struct pvclock_vcpu_stolen_time {
    pub revision: u32,
    pub attributes: u32,
    pub stolen_time: u64,
    /* Structure must be 64 byte aligned, pad to that size */
    pub padding: [u8; 48],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
