/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 *  smc_sysctl.c: sysctl interface to SMC subsystem.
 *
 *  Copyright (c) 2022, Alibaba Inc.
 *
 *  Author: Tony Lu <tonylu@linux.alibaba.com>
 */

/* Translation of the C header guard: _SMC_SYSCTL_H. */

/* CONFIG_SYSCTL is a build-time configuration condition from the C source. */
#[cfg(feature = "CONFIG_SYSCTL")]
extern "C" {
    pub fn smc_sysctl_net_init(net: *mut crate::net) -> i32;
    pub fn smc_sysctl_net_exit(net: *mut crate::net);
}

#[cfg(not(feature = "CONFIG_SYSCTL"))]
pub unsafe fn smc_sysctl_net_init(net: *mut crate::net) -> i32 {
    (*net).smc.sysctl_autocorking_size = SMC_AUTOCORKING_DEFAULT_SIZE;
    (*net).smc.sysctl_max_links_per_lgr = SMC_LINKS_PER_LGR_MAX_PREFER;
    (*net).smc.sysctl_max_conns_per_lgr = SMC_CONN_PER_LGR_PREFER;
    (*net).smc.sysctl_smcr_max_send_wr = SMCR_MAX_SEND_WR_DEF;
    (*net).smc.sysctl_smcr_max_recv_wr = SMCR_MAX_RECV_WR_DEF;
    0
}

#[cfg(not(feature = "CONFIG_SYSCTL"))]
pub unsafe fn smc_sysctl_net_exit(_net: *mut crate::net) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
