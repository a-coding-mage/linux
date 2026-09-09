/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Linux ethernet bridge
 *
 * Authors:
 * Lennert Buytenhek        <buytenh@gnu.org>
 */

/* C header guard: _BR_PRIVATE_STP_H */

pub const BPDU_TYPE_CONFIG: u32 = 0;
pub const BPDU_TYPE_TCN: u32 = 0x80;

/* IEEE 802.1D-1998 timer values */
pub const BR_MIN_HELLO_TIME: u32 = 1 * HZ;
pub const BR_MAX_HELLO_TIME: u32 = 10 * HZ;

pub const BR_MIN_FORWARD_DELAY: u32 = 2 * HZ;
pub const BR_MAX_FORWARD_DELAY: u32 = 30 * HZ;

pub const BR_MIN_MAX_AGE: u32 = 6 * HZ;
pub const BR_MAX_MAX_AGE: u32 = 40 * HZ;

pub const BR_MIN_PATH_COST: i32 = 1;
pub const BR_MAX_PATH_COST: i32 = 65535;

#[repr(C)]
pub struct br_config_bpdu {
    pub topology_change: u32,
    pub topology_change_ack: u32,
    pub root: bridge_id,
    pub root_path_cost: i32,
    pub bridge_id: bridge_id,
    pub port_id: port_id,
    pub message_age: i32,
    pub max_age: i32,
    pub hello_time: i32,
    pub forward_delay: i32,
}

/* called under bridge lock */
#[inline]
pub unsafe fn br_is_designated_port(p: *const net_bridge_port) -> i32 {
    (libc::memcmp(
        &(*p).designated_bridge as *const _ as *const libc::c_void,
        &(*(*p).br).bridge_id as *const _ as *const libc::c_void,
        8,
    ) == 0) as i32
        & (((*p).designated_port == (*p).port_id) as i32)
}

/* br_stp.c */
extern "C" {
    pub fn br_become_root_bridge(br: *mut net_bridge);
    pub fn br_config_bpdu_generation(br: *mut net_bridge);
    pub fn br_configuration_update(br: *mut net_bridge);
    pub fn br_port_state_selection(br: *mut net_bridge);
    pub fn br_received_config_bpdu(
        p: *mut net_bridge_port,
        bpdu: *const br_config_bpdu,
    );
    pub fn br_received_tcn_bpdu(p: *mut net_bridge_port);
    pub fn br_transmit_config(p: *mut net_bridge_port);
    pub fn br_transmit_tcn(br: *mut net_bridge);
    pub fn br_topology_change_detection(br: *mut net_bridge);
    pub fn __br_set_topology_change(br: *mut net_bridge, val: u8);

    /* br_stp_bpdu.c */
    pub fn br_send_config_bpdu(p: *mut net_bridge_port, bpdu: *mut br_config_bpdu);
    pub fn br_send_tcn_bpdu(p: *mut net_bridge_port);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
