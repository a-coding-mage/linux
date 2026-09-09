/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2022, Microchip
 */

// Dependency intent: declarations from <linux/arm-smccc.h> are supplied by
// the surrounding translation unit.

/* Secure Monitor mode APIs */
pub const SAMA5_SMC_SIP_SET_SUSPEND_MODE: u32 = 0x400;
pub const SAMA5_SMC_SIP_GET_SUSPEND_MODE: u32 = 0x401;

extern "C" {
    // __init annotation from the C declaration has no direct Rust equivalent.
    pub fn sam_secure_init();
    pub fn sam_smccc_call(fn_: u32, arg0: u32, arg1: u32) -> arm_smccc_res;
    pub fn sam_linux_is_optee_available() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
