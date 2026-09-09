/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 *  Generic hook for SMC handshake flow.
 *
 *  Copyright IBM Corp. 2016
 *  Copyright (c) 2025, Alibaba Inc.
 *
 *  Author: D. Wythe <alibuda@linux.alibaba.com>
 */

use core::ffi::{c_char, c_int};

/* Dependency supplied by net/smc.h in the C source. */

/* Find hs_ctrl by the target name, which required to be a c-string.
 * Return NULL if no such ctrl was found,otherwise, return a valid ctrl.
 *
 * Note: Caller MUST ensure it's was invoked under rcu_read_lock.
 */
extern "C" {
    pub fn smc_hs_ctrl_find_by_name(name: *const c_char) -> *mut smc_hs_ctrl;
}

/* Opaque declaration supplied by net/smc.h in the C source. */
#[repr(C)]
pub struct smc_hs_ctrl {
    _private: [u8; 0],
}

/* CONFIG_SMC_HS_CTRL_BPF is a build-time kernel configuration condition. */
#[cfg(feature = "CONFIG_SMC_HS_CTRL_BPF")]
extern "C" {
    pub fn bpf_smc_hs_ctrl_init() -> c_int;
}

#[cfg(not(feature = "CONFIG_SMC_HS_CTRL_BPF"))]
#[inline]
pub fn bpf_smc_hs_ctrl_init() -> c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
