/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Linux-specific definitions for managing interactions with Microsoft's
 * Hyper-V hypervisor. The definitions in this file are specific to the ARM64
 * architecture. See include/asm-generic/mshyperv.h for definitions that are
 * architecture independent.
 *
 * Definitions that are derived from Hyper-V code or headers should not go in
 * this file, but should instead go in the relevant files in include/hyperv.
 *
 * Copyright (C) 2021, Microsoft, Inc.
 *
 * Author : Michael Kelley <mikelley@microsoft.com>
 */

// C header guard: _ASM_MSHYPERV_H
// Dependencies: linux/types.h, linux/arm-smccc.h, hyperv/hvhdk.h,
// and asm-generic/mshyperv.h provide the referenced types and constants.

/*
 * Declare calls to get and set Hyper-V VP register values on ARM64, which
 * requires a hypercall.
 */

extern "C" {
    pub fn hv_set_vpreg(reg: u32, value: u64);
    pub fn hv_get_vpreg(reg: u32) -> u64;
    pub fn hv_get_vpreg_128(reg: u32, result: *mut hv_get_vp_registers_output);
}

#[inline]
pub unsafe fn hv_set_msr(reg: core::ffi::c_uint, value: u64) {
    hv_set_vpreg(reg as u32, value);
}

#[inline]
pub unsafe fn hv_get_msr(reg: core::ffi::c_uint) -> u64 {
    hv_get_vpreg(reg as u32)
}

/*
 * Nested is not supported on arm64
 */
#[inline]
pub unsafe fn hv_set_non_nested_msr(reg: core::ffi::c_uint, value: u64) {
    hv_set_msr(reg, value);
}

#[inline]
pub unsafe fn hv_get_non_nested_msr(reg: core::ffi::c_uint) -> u64 {
    hv_get_msr(reg)
}

/* SMCCC hypercall parameters */
pub const HV_SMCCC_FUNC_NUMBER: u32 = 1;
// ARM_SMCCC_CALL_VAL is supplied by linux/arm-smccc.h; preserve the original
// build-time expression and its ARM64 SMCCC parameter intent.
pub const HV_FUNC_ID: u32 = ARM_SMCCC_CALL_VAL(
    ARM_SMCCC_STD_CALL,
    ARM_SMCCC_SMC_64,
    ARM_SMCCC_OWNER_VENDOR_HYP,
    HV_SMCCC_FUNC_NUMBER,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
