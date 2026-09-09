/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2013 Broadcom Corporation */

/* Broadcom Secure Service API service IDs, return codes, and exit codes */
pub const SSAPI_ENABLE_L2_CACHE: u32 = 0x01000002;
pub const SEC_ROM_RET_OK: u32 = 0x00000001;
pub const SEC_EXIT_NORMAL: u32 = 0x1;

/* Original declaration uses the kernel __init annotation. */
extern "C" {
    pub fn bcm_kona_smc_init() -> i32;

    pub fn bcm_kona_smc(
        service_id: u32,
        arg0: u32,
        arg1: u32,
        arg2: u32,
        arg3: u32,
    ) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
