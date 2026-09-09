/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 *  SMC Generic netlink operations
 *
 *  Copyright IBM Corp. 2020
 *
 *  Author(s):	Guvenc Gulce <guvenc@linux.ibm.com>
 */

// Dependencies supplied by the surrounding networking and generic-netlink code.

use core::ffi::c_int;

extern "C" {
    pub static mut smc_gen_nl_family: genl_family;
    pub static smc_gen_ueid_policy: nla_policy;

    pub fn smc_nl_init() -> c_int;
    pub fn smc_nl_exit();
}

#[repr(C)]
pub struct smc_nl_dmp_ctx {
    pub pos: [c_int; 3],
}

#[inline]
pub unsafe fn smc_nl_dmp_ctx(c: *mut netlink_callback) -> *mut smc_nl_dmp_ctx {
    (*c).ctx as *mut smc_nl_dmp_ctx
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
