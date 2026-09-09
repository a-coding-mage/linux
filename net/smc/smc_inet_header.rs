/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 *  Definitions for the IPPROTO_SMC (socket related)
 *
 *  Copyright IBM Corp. 2016
 *  Copyright (c) 2024, Alibaba Inc.
 *
 *  Author: D. Wythe <alibuda@linux.alibaba.com>
 */

use core::ffi::c_int;

/* Initialize protocol registration on IPPROTO_SMC,
 * @return 0 on success
 */
unsafe extern "C" {
    pub fn smc_inet_init() -> c_int;

    pub fn smc_inet_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
