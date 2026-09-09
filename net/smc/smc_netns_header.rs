/* SPDX-License-Identifier: GPL-2.0 */
/* Shared Memory Communications
 *
 * Network namespace definitions.
 *
 * Copyright IBM Corp. 2018
 */

// Dependency: declarations from "smc_pnet.h" are supplied externally.

unsafe extern "C" {
    pub static mut smc_net_id: std::os::raw::c_uint;
}

/* per-network namespace private data */
#[repr(C)]
pub struct smc_net {
    pub pnettable: smc_pnettable,
    pub pnetids_ndev: smc_pnetids_ndev,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
