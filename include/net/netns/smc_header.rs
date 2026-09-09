/* SPDX-License-Identifier: GPL-2.0 */

// Translated from net/netns/smc.h.
// External declarations supplied by the surrounding kernel translation:
// struct smc_stats_rsn;
// struct smc_stats;
// struct mutex;
// struct ctl_table_header;
// struct smc_hs_ctrl;

use core::ffi::c_int;

#[repr(C)]
pub struct netns_smc {
    /* per cpu counters for SMC */
    pub smc_stats: *mut smc_stats,
    /* protect fback_rsn */
    pub mutex_fback_rsn: mutex,
    pub fback_rsn: *mut smc_stats_rsn,

    pub limit_smc_hs: bool, /* constraint on handshake */

    // #ifdef CONFIG_SYSCTL
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub smc_hdr: *mut ctl_table_header,
    // #endif

    // #if IS_ENABLED(CONFIG_SMC_HS_CTRL_BPF)
    #[cfg(feature = "CONFIG_SMC_HS_CTRL_BPF")]
    pub hs_ctrl: *mut smc_hs_ctrl,
    // #endif /* CONFIG_SMC_HS_CTRL_BPF */

    pub sysctl_autocorking_size: core::ffi::c_uint,
    pub sysctl_smcr_buf_type: core::ffi::c_uint,
    pub sysctl_smcr_testlink_time: c_int,
    pub sysctl_wmem: c_int,
    pub sysctl_rmem: c_int,
    pub sysctl_max_links_per_lgr: c_int,
    pub sysctl_max_conns_per_lgr: c_int,
    pub sysctl_smcr_max_send_wr: core::ffi::c_uint,
    pub sysctl_smcr_max_recv_wr: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
