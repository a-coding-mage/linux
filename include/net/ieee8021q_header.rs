/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2024 Pengutronix, Oleksij Rempel <kernel@pengutronix.de> */

// Translated from net/ieee8021q.h.
// Dependency: linux/errno.h supplies EOPNOTSUPP.

/**
 * enum ieee8021q_traffic_type - 802.1Q traffic type priority values (802.1Q-2022)
 *
 * @IEEE8021Q_TT_BK: Background
 * @IEEE8021Q_TT_BE: Best Effort (default). According to 802.1Q-2022, BE is 0
 * but has higher priority than BK which is 1.
 * @IEEE8021Q_TT_EE: Excellent Effort
 * @IEEE8021Q_TT_CA: Critical Applications
 * @IEEE8021Q_TT_VI: Video, < 100 ms latency and jitter
 * @IEEE8021Q_TT_VO: Voice, < 10 ms latency and jitter
 * @IEEE8021Q_TT_IC: Internetwork Control
 * @IEEE8021Q_TT_NC: Network Control
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ieee8021q_traffic_type {
    IEEE8021Q_TT_BK = 0,
    IEEE8021Q_TT_BE = 1,
    IEEE8021Q_TT_EE = 2,
    IEEE8021Q_TT_CA = 3,
    IEEE8021Q_TT_VI = 4,
    IEEE8021Q_TT_VO = 5,
    IEEE8021Q_TT_IC = 6,
    IEEE8021Q_TT_NC = 7,

    /* private: */
    IEEE8021Q_TT_MAX = 8,
}

#[inline]
pub const fn SIMPLE_IETF_DSCP_TO_IEEE8021Q_TT(dscp: u8) -> u8 {
    (dscp >> 3) & 0x7
}

// Corresponds to IS_ENABLED(CONFIG_NET_IEEE8021Q_HELPERS).
#[cfg(feature = "CONFIG_NET_IEEE8021Q_HELPERS")]
extern "C" {
    pub fn ietf_dscp_to_ieee8021q_tt(dscp: u8) -> i32;
    pub fn ieee8021q_tt_to_tc(
        tt: ieee8021q_traffic_type,
        num_queues: u32,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_NET_IEEE8021Q_HELPERS"))]
#[inline]
pub fn ietf_dscp_to_ieee8021q_tt(dscp: u8) -> i32 {
    let _ = dscp;
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_NET_IEEE8021Q_HELPERS"))]
#[inline]
pub fn ieee8021q_tt_to_tc(
    tt: ieee8021q_traffic_type,
    num_queues: u32,
) -> i32 {
    let _ = (tt, num_queues);
    -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
