/* SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause) */
/*
 * net/can.h
 *
 * Definitions for the CAN network socket buffer extensions
 *
 * Copyright (C) 2026 Oliver Hartkopp <socketcan@hartkopp.net>
 *
 */

/**
 * struct can_skb_ext - skb extensions for CAN specific content
 * @can_iif: ifindex of the first interface the CAN frame appeared on
 * @can_framelen: cached echo CAN frame length for bql
 * @can_gw_hops: can-gw CAN frame time-to-live counter
 * @can_ext_flags: CAN skb extensions flags
 */
#[repr(C)]
pub struct can_skb_ext {
    pub can_iif: i32,
    pub can_framelen: u16,
    pub can_gw_hops: u8,
    pub can_ext_flags: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
