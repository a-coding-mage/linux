/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* QLogic qed NIC Driver
 * Copyright (c) 2015-2017  QLogic Corporation
 * Copyright (c) 2019-2020 Marvell International Ltd.
 */

/* TCP FW CONSTANTS */
pub const TCP_INVALID_TIMEOUT_VAL: i32 = -1;

#[repr(C)]
pub struct ooo_opaque {
    pub cid: __le32,
    pub drop_isle: u8,
    pub drop_size: u8,
    pub ooo_opcode: u8,
    pub ooo_isle: u8,
}

#[repr(u32)]
pub enum tcp_connect_mode {
    TCP_CONNECT_ACTIVE,
    TCP_CONNECT_PASSIVE,
    MAX_TCP_CONNECT_MODE,
}

#[repr(C)]
pub struct tcp_init_params {
    pub two_msl_timer: __le32,
    pub tx_sws_timer: __le16,
    pub max_fin_rt: u8,
    pub reserved: [u8; 9],
}

#[repr(u32)]
pub enum tcp_ip_version {
    TCP_IPV4,
    TCP_IPV6,
    MAX_TCP_IP_VERSION,
}

#[repr(C)]
pub struct tcp_offload_params {
    pub local_mac_addr_lo: __le16, pub local_mac_addr_mid: __le16, pub local_mac_addr_hi: __le16,
    pub remote_mac_addr_lo: __le16, pub remote_mac_addr_mid: __le16, pub remote_mac_addr_hi: __le16,
    pub vlan_id: __le16, pub flags: __le16,
    pub ip_version: u8, pub reserved0: [u8; 3],
    pub remote_ip: [__le32; 4], pub local_ip: [__le32; 4], pub flow_label: __le32,
    pub ttl: u8, pub tos_or_tc: u8, pub remote_port: __le16, pub local_port: __le16,
    pub mss: __le16, pub rcv_wnd_scale: u8, pub connect_mode: u8, pub srtt: __le16,
    pub ss_thresh: __le32, pub rcv_wnd: __le32, pub cwnd: __le32,
    pub ka_max_probe_cnt: u8, pub dup_ack_theshold: u8, pub reserved1: __le16,
    pub ka_timeout: __le32, pub ka_interval: __le32, pub max_rt_time: __le32,
    pub initial_rcv_wnd: __le32, pub rcv_next: __le32, pub snd_una: __le32,
    pub snd_next: __le32, pub snd_max: __le32, pub snd_wnd: __le32, pub snd_wl1: __le32,
    pub ts_recent: __le32, pub ts_recent_age: __le32, pub total_rt: __le32,
    pub ka_timeout_delta: __le32, pub rt_timeout_delta: __le32,
    pub dup_ack_cnt: u8, pub snd_wnd_probe_cnt: u8, pub ka_probe_cnt: u8, pub rt_cnt: u8,
    pub rtt_var: __le16, pub fw_internal: __le16, pub snd_wnd_scale: u8, pub ack_frequency: u8,
    pub da_timeout_value: __le16, pub reserved3: __le32,
}

pub const TCP_OFFLOAD_PARAMS_TS_EN_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_TS_EN_SHIFT: u32 = 0;
pub const TCP_OFFLOAD_PARAMS_DA_EN_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_DA_EN_SHIFT: u32 = 1;
pub const TCP_OFFLOAD_PARAMS_KA_EN_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_KA_EN_SHIFT: u32 = 2;
pub const TCP_OFFLOAD_PARAMS_ECN_SENDER_EN_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_ECN_SENDER_EN_SHIFT: u32 = 3;
pub const TCP_OFFLOAD_PARAMS_ECN_RECEIVER_EN_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_ECN_RECEIVER_EN_SHIFT: u32 = 4;
pub const TCP_OFFLOAD_PARAMS_NAGLE_EN_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_NAGLE_EN_SHIFT: u32 = 5;
pub const TCP_OFFLOAD_PARAMS_DA_CNT_EN_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_DA_CNT_EN_SHIFT: u32 = 6;
pub const TCP_OFFLOAD_PARAMS_FIN_SENT_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_FIN_SENT_SHIFT: u32 = 7;
pub const TCP_OFFLOAD_PARAMS_FIN_RECEIVED_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_FIN_RECEIVED_SHIFT: u32 = 8;
pub const TCP_OFFLOAD_PARAMS_RESERVED_MASK: u32 = 0x7F; pub const TCP_OFFLOAD_PARAMS_RESERVED_SHIFT: u32 = 9;

#[repr(C)]
pub struct tcp_offload_params_opt2 {
    pub local_mac_addr_lo: __le16, pub local_mac_addr_mid: __le16, pub local_mac_addr_hi: __le16,
    pub remote_mac_addr_lo: __le16, pub remote_mac_addr_mid: __le16, pub remote_mac_addr_hi: __le16,
    pub vlan_id: __le16, pub flags: __le16, pub ip_version: u8, pub reserved1: [u8; 3],
    pub remote_ip: [__le32; 4], pub local_ip: [__le32; 4], pub flow_label: __le32,
    pub ttl: u8, pub tos_or_tc: u8, pub remote_port: __le16, pub local_port: __le16,
    pub mss: __le16, pub rcv_wnd_scale: u8, pub connect_mode: u8, pub syn_ip_payload_length: __le16,
    pub syn_phy_addr_lo: __le32, pub syn_phy_addr_hi: __le32, pub cwnd: __le32,
    pub ka_max_probe_cnt: u8, pub reserved2: [u8; 3], pub ka_timeout: __le32,
    pub ka_interval: __le32, pub max_rt_time: __le32, pub reserved3: [__le32; 16],
}

pub const TCP_OFFLOAD_PARAMS_OPT2_TS_EN_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_OPT2_TS_EN_SHIFT: u32 = 0;
pub const TCP_OFFLOAD_PARAMS_OPT2_DA_EN_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_OPT2_DA_EN_SHIFT: u32 = 1;
pub const TCP_OFFLOAD_PARAMS_OPT2_KA_EN_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_OPT2_KA_EN_SHIFT: u32 = 2;
pub const TCP_OFFLOAD_PARAMS_OPT2_ECN_EN_MASK: u32 = 0x1; pub const TCP_OFFLOAD_PARAMS_OPT2_ECN_EN_SHIFT: u32 = 3;
pub const TCP_OFFLOAD_PARAMS_OPT2_RESERVED0_MASK: u32 = 0xFFF; pub const TCP_OFFLOAD_PARAMS_OPT2_RESERVED0_SHIFT: u32 = 4;

#[repr(u32)]
pub enum tcp_seg_placement_event {
    TCP_EVENT_ADD_PEN, TCP_EVENT_ADD_NEW_ISLE, TCP_EVENT_ADD_ISLE_RIGHT,
    TCP_EVENT_ADD_ISLE_LEFT, TCP_EVENT_JOIN, TCP_EVENT_DELETE_ISLES,
    TCP_EVENT_NOP, MAX_TCP_SEG_PLACEMENT_EVENT,
}

#[repr(C)]
pub struct tcp_update_params {
    pub flags: __le16, pub remote_mac_addr_lo: __le16, pub remote_mac_addr_mid: __le16,
    pub remote_mac_addr_hi: __le16, pub mss: __le16, pub ttl: u8, pub tos_or_tc: u8,
    pub ka_timeout: __le32, pub ka_interval: __le32, pub max_rt_time: __le32,
    pub flow_label: __le32, pub initial_rcv_wnd: __le32, pub ka_max_probe_cnt: u8,
    pub reserved1: [u8; 7],
}

macro_rules! tcp_update_flag { ($name:ident, $value:expr) => { pub const $name: u32 = $value; }; }
tcp_update_flag!(TCP_UPDATE_PARAMS_REMOTE_MAC_ADDR_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_REMOTE_MAC_ADDR_CHANGED_SHIFT, 0);
tcp_update_flag!(TCP_UPDATE_PARAMS_MSS_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_MSS_CHANGED_SHIFT, 1);
tcp_update_flag!(TCP_UPDATE_PARAMS_TTL_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_TTL_CHANGED_SHIFT, 2);
tcp_update_flag!(TCP_UPDATE_PARAMS_TOS_OR_TC_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_TOS_OR_TC_CHANGED_SHIFT, 3);
tcp_update_flag!(TCP_UPDATE_PARAMS_KA_TIMEOUT_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_KA_TIMEOUT_CHANGED_SHIFT, 4);
tcp_update_flag!(TCP_UPDATE_PARAMS_KA_INTERVAL_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_KA_INTERVAL_CHANGED_SHIFT, 5);
tcp_update_flag!(TCP_UPDATE_PARAMS_MAX_RT_TIME_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_MAX_RT_TIME_CHANGED_SHIFT, 6);
tcp_update_flag!(TCP_UPDATE_PARAMS_FLOW_LABEL_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_FLOW_LABEL_CHANGED_SHIFT, 7);
tcp_update_flag!(TCP_UPDATE_PARAMS_INITIAL_RCV_WND_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_INITIAL_RCV_WND_CHANGED_SHIFT, 8);
tcp_update_flag!(TCP_UPDATE_PARAMS_KA_MAX_PROBE_CNT_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_KA_MAX_PROBE_CNT_CHANGED_SHIFT, 9);
tcp_update_flag!(TCP_UPDATE_PARAMS_KA_EN_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_KA_EN_CHANGED_SHIFT, 10);
tcp_update_flag!(TCP_UPDATE_PARAMS_NAGLE_EN_CHANGED_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_NAGLE_EN_CHANGED_SHIFT, 11);
tcp_update_flag!(TCP_UPDATE_PARAMS_KA_EN_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_KA_EN_SHIFT, 12);
tcp_update_flag!(TCP_UPDATE_PARAMS_NAGLE_EN_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_NAGLE_EN_SHIFT, 13);
tcp_update_flag!(TCP_UPDATE_PARAMS_KA_RESTART_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_KA_RESTART_SHIFT, 14);
tcp_update_flag!(TCP_UPDATE_PARAMS_RETRANSMIT_RESTART_MASK, 0x1); tcp_update_flag!(TCP_UPDATE_PARAMS_RETRANSMIT_RESTART_SHIFT, 15);

#[repr(C)]
pub struct tcp_upload_params {
    pub rcv_next: __le32, pub snd_una: __le32, pub snd_next: __le32, pub snd_max: __le32,
    pub snd_wnd: __le32, pub rcv_wnd: __le32, pub snd_wl1: __le32, pub cwnd: __le32,
    pub ss_thresh: __le32, pub srtt: __le16, pub rtt_var: __le16, pub ts_time: __le32,
    pub ts_recent: __le32, pub ts_recent_age: __le32, pub total_rt: __le32,
    pub ka_timeout_delta: __le32, pub rt_timeout_delta: __le32, pub dup_ack_cnt: u8,
    pub snd_wnd_probe_cnt: u8, pub ka_probe_cnt: u8, pub rt_cnt: u8, pub reserved: __le32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
