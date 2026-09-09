/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner
 */

// Dependency supplied by the translated main header.

/**
 * enum batadv_bandwidth_units - bandwidth unit types
 */
#[repr(C)]
pub enum batadv_bandwidth_units {
    /** @BATADV_BW_UNIT_KBIT: unit type kbit */
    BATADV_BW_UNIT_KBIT,

    /** @BATADV_BW_UNIT_MBIT: unit type mbit */
    BATADV_BW_UNIT_MBIT,
}

pub const BATADV_GW_MODE_OFF_NAME: &str = "off";
pub const BATADV_GW_MODE_CLIENT_NAME: &str = "client";
pub const BATADV_GW_MODE_SERVER_NAME: &str = "server";

unsafe extern "C" {
    pub fn batadv_gw_tvlv_container_update(bat_priv: *mut batadv_priv);
    pub fn batadv_gw_init(bat_priv: *mut batadv_priv);
    pub fn batadv_gw_free(bat_priv: *mut batadv_priv);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
