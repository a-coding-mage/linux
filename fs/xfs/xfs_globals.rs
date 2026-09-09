// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding XFS translation.

/*
 * Tunable XFS parameters. xfs_params is required even when CONFIG_SYSCTL=n,
 * other XFS code uses these values. Times are measured in centisecs (i.e.
 * 100ths of a second) with the exception of blockgc_timer, which is measured
 * in seconds.
 */
pub static mut xfs_params: xfs_param_t = xfs_param_t {
    panic_mask: [0, 0, XFS_PTAG_MASK],
    error_level: [0, 3, 11],
    syncd_timer: [1 * 100, 30 * 100, 7200 * 100],
    stats_clear: [0, 0, 1],
    inherit_sync: [0, 1, 1],
    inherit_nodump: [0, 1, 1],
    inherit_noatim: [0, 1, 1],
    inherit_nosym: [0, 0, 1],
    rotorstep: [1, 1, 255],
    inherit_nodfrg: [0, 1, 1],
    fstrm_timer: [1, 30 * 100, 3600 * 100],
    blockgc_timer: [1, 300, 3600 * 24],
};

pub static mut xfs_globals: xfs_globals = xfs_globals {
    log_recovery_delay: 0, // no delay by default
    mount_delay: 0,         // no delay by default

    // XFS_ASSERT_FATAL selects assert failures BUG(); otherwise WARN().
    #[cfg(feature = "XFS_ASSERT_FATAL")]
    bug_on_assert: true,
    #[cfg(not(feature = "XFS_ASSERT_FATAL"))]
    bug_on_assert: false,

    // DEBUG-only globals.
    #[cfg(feature = "DEBUG")]
    pwork_threads: -1, // automatic thread detection
    #[cfg(feature = "DEBUG")]
    larp: false, // log attribute replay

    /*
     * Leave this many record slots empty when bulk loading btrees. By
     * default we load new btree leaf blocks 75% full.
     */
    bload_leaf_slack: -1,

    /*
     * Leave this many key/ptr slots empty when bulk loading btrees. By
     * default we load new btree node blocks 75% full.
     */
    bload_node_slack: -1,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
