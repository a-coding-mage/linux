/* SPDX-License-Identifier: GPL-2.0
 * Copyright (c) 2019, Vladimir Oltean <olteanv@gmail.com>
 */

// Translated from linux/dsa/8021q.h.
// The declarations below depend on types supplied by the surrounding kernel
// translation; they are intentionally not redefined here.

/* VBID is limited to three bits only and zero is reserved.
 * Only 7 bridges can be enumerated.
 */
pub const DSA_TAG_8021Q_MAX_NUM_BRIDGES: i32 = 7;

extern "C" {
    pub fn dsa_tag_8021q_register(ds: *mut dsa_switch, proto: u16) -> i32;

    pub fn dsa_tag_8021q_unregister(ds: *mut dsa_switch);

    pub fn dsa_tag_8021q_bridge_join(
        ds: *mut dsa_switch,
        port: i32,
        bridge: dsa_bridge,
        tx_fwd_offload: *mut bool,
        extack: *mut netlink_ext_ack,
    ) -> i32;

    pub fn dsa_tag_8021q_bridge_leave(
        ds: *mut dsa_switch,
        port: i32,
        bridge: dsa_bridge,
    );

    pub fn dsa_tag_8021q_bridge_vid(bridge_num: u32) -> u16;

    pub fn dsa_tag_8021q_standalone_vid(dp: *const dsa_port) -> u16;

    pub fn dsa_8021q_rx_switch_id(vid: u16) -> i32;

    pub fn dsa_8021q_rx_source_port(vid: u16) -> i32;

    pub fn vid_is_dsa_8021q(vid: u16) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
