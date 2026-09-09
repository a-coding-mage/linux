/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2014-2020 Intel Corporation.  All rights reserved.
 */

// Dependency: definitions supplied by rdma/opa_smi.h are expected externally.

pub const OPA_PORT_LINK_MODE_NOP: u32 = 0;
pub const OPA_PORT_LINK_MODE_OPA: u32 = 4;
pub const OPA_PORT_PACKET_FORMAT_NOP: u32 = 0;
pub const OPA_PORT_PACKET_FORMAT_8B: u32 = 1;
pub const OPA_PORT_PACKET_FORMAT_9B: u32 = 2;
pub const OPA_PORT_PACKET_FORMAT_10B: u32 = 4;
pub const OPA_PORT_PACKET_FORMAT_16B: u32 = 8;
pub const OPA_PORT_LTP_CRC_MODE_NONE: u32 = 0;
pub const OPA_PORT_LTP_CRC_MODE_14: u32 = 1;
pub const OPA_PORT_LTP_CRC_MODE_16: u32 = 2;
pub const OPA_PORT_LTP_CRC_MODE_48: u32 = 4;
pub const OPA_PORT_LTP_CRC_MODE_PER_LANE: u32 = 8;

/* Link Down / Neighbor Link Down Reason; indicated as follows: */
pub const OPA_LINKDOWN_REASON_NONE: u32 = 0;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_0: u32 = 1;
pub const OPA_LINKDOWN_REASON_BAD_PKT_LEN: u32 = 2;
pub const OPA_LINKDOWN_REASON_PKT_TOO_LONG: u32 = 3;
pub const OPA_LINKDOWN_REASON_PKT_TOO_SHORT: u32 = 4;
pub const OPA_LINKDOWN_REASON_BAD_SLID: u32 = 5;
pub const OPA_LINKDOWN_REASON_BAD_DLID: u32 = 6;
pub const OPA_LINKDOWN_REASON_BAD_L2: u32 = 7;
pub const OPA_LINKDOWN_REASON_BAD_SC: u32 = 8;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_8: u32 = 9;
pub const OPA_LINKDOWN_REASON_BAD_MID_TAIL: u32 = 10;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_10: u32 = 11;
pub const OPA_LINKDOWN_REASON_PREEMPT_ERROR: u32 = 12;
pub const OPA_LINKDOWN_REASON_PREEMPT_VL15: u32 = 13;
pub const OPA_LINKDOWN_REASON_BAD_VL_MARKER: u32 = 14;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_14: u32 = 15;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_15: u32 = 16;
pub const OPA_LINKDOWN_REASON_BAD_HEAD_DIST: u32 = 17;
pub const OPA_LINKDOWN_REASON_BAD_TAIL_DIST: u32 = 18;
pub const OPA_LINKDOWN_REASON_BAD_CTRL_DIST: u32 = 19;
pub const OPA_LINKDOWN_REASON_BAD_CREDIT_ACK: u32 = 20;
pub const OPA_LINKDOWN_REASON_UNSUPPORTED_VL_MARKER: u32 = 21;
pub const OPA_LINKDOWN_REASON_BAD_PREEMPT: u32 = 22;
pub const OPA_LINKDOWN_REASON_BAD_CONTROL_FLIT: u32 = 23;
pub const OPA_LINKDOWN_REASON_EXCEED_MULTICAST_LIMIT: u32 = 24;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_24: u32 = 25;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_25: u32 = 26;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_26: u32 = 27;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_27: u32 = 28;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_28: u32 = 29;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_29: u32 = 30;
pub const OPA_LINKDOWN_REASON_RCV_ERROR_30: u32 = 31;
pub const OPA_LINKDOWN_REASON_EXCESSIVE_BUFFER_OVERRUN: u32 = 32;
pub const OPA_LINKDOWN_REASON_UNKNOWN: u32 = 33;
pub const OPA_LINKDOWN_REASON_REBOOT: u32 = 35;
pub const OPA_LINKDOWN_REASON_NEIGHBOR_UNKNOWN: u32 = 36;
pub const OPA_LINKDOWN_REASON_FM_BOUNCE: u32 = 39;
pub const OPA_LINKDOWN_REASON_SPEED_POLICY: u32 = 40;
pub const OPA_LINKDOWN_REASON_WIDTH_POLICY: u32 = 41;
pub const OPA_LINKDOWN_REASON_DISCONNECTED: u32 = 49;
pub const OPA_LINKDOWN_REASON_LOCAL_MEDIA_NOT_INSTALLED: u32 = 50;
pub const OPA_LINKDOWN_REASON_NOT_INSTALLED: u32 = 51;
pub const OPA_LINKDOWN_REASON_CHASSIS_CONFIG: u32 = 52;
pub const OPA_LINKDOWN_REASON_END_TO_END_NOT_INSTALLED: u32 = 54;
pub const OPA_LINKDOWN_REASON_POWER_POLICY: u32 = 56;
pub const OPA_LINKDOWN_REASON_LINKSPEED_POLICY: u32 = 57;
pub const OPA_LINKDOWN_REASON_LINKWIDTH_POLICY: u32 = 58;
pub const OPA_LINKDOWN_REASON_SWITCH_MGMT: u32 = 60;
pub const OPA_LINKDOWN_REASON_SMA_DISABLED: u32 = 61;
pub const OPA_LINKDOWN_REASON_TRANSIENT: u32 = 63;

pub const OPA_LINKINIT_REASON_NOP: u32 = 0;
pub const OPA_LINKINIT_REASON_LINKUP: u32 = 1 << 4;
pub const OPA_LINKINIT_REASON_FLAPPING: u32 = 2 << 4;
pub const OPA_LINKINIT_REASON_CLEAR: u32 = 8 << 4;
pub const OPA_LINKINIT_OUTSIDE_POLICY: u32 = 8 << 4;
pub const OPA_LINKINIT_QUARANTINED: u32 = 9 << 4;
pub const OPA_LINKINIT_INSUFIC_CAPABILITY: u32 = 10 << 4;

pub const OPA_LINK_SPEED_NOP: u32 = 0x0000;
pub const OPA_LINK_SPEED_12_5G: u32 = 0x0001;
pub const OPA_LINK_SPEED_25G: u32 = 0x0002;
pub const OPA_LINK_SPEED_50G: u32 = 0x0004;
pub const OPA_LINK_SPEED_100G: u32 = 0x0008;
pub const OPA_LINK_WIDTH_1X: u32 = 0x0001;
pub const OPA_LINK_WIDTH_2X: u32 = 0x0002;
pub const OPA_LINK_WIDTH_3X: u32 = 0x0004;
pub const OPA_LINK_WIDTH_4X: u32 = 0x0008;

pub const OPA_CAP_MASK3_IsEthOnFabricSupported: u32 = 1 << 13;
pub const OPA_CAP_MASK3_IsSnoopSupported: u32 = 1 << 7;
pub const OPA_CAP_MASK3_IsAsyncSC2VLSupported: u32 = 1 << 6;
pub const OPA_CAP_MASK3_IsAddrRangeConfigSupported: u32 = 1 << 5;
pub const OPA_CAP_MASK3_IsPassThroughSupported: u32 = 1 << 4;
pub const OPA_CAP_MASK3_IsSharedSpaceSupported: u32 = 1 << 3;
pub const OPA_CAP_MASK3_IsVLMarkerSupported: u32 = 1 << 1;
pub const OPA_CAP_MASK3_IsVLrSupported: u32 = 1 << 0;

pub const OPA_PORT_PHYS_CONF_DISCONNECTED: u32 = 0;
pub const OPA_PORT_PHYS_CONF_STANDARD: u32 = 1;
pub const OPA_PORT_PHYS_CONF_FIXED: u32 = 2;
pub const OPA_PORT_PHYS_CONF_VARIABLE: u32 = 3;
pub const OPA_PORT_PHYS_CONF_SI_PHOTO: u32 = 4;

/* Field masks are preserved as C-style constants. */
pub const OPA_PI_MASK_VL_CAP: u32 = 0x1F;
pub const OPA_PI_MASK_OFFLINE_REASON: u32 = 0x0F;
pub const OPA_PI_MASK_LED_ENABLE: u32 = 0x40;
pub const OPA_PI_MASK_UNSLEEP_STATE: u32 = 0xF0;
pub const OPA_PI_MASK_DOWNDEF_STATE: u32 = 0x0F;
pub const OPA_PI_MASK_PORT_PHYSICAL_STATE: u32 = 0xF0;
pub const OPA_PI_MASK_PORT_STATE: u32 = 0x0F;
pub const OPA_PI_MASK_PORT_PHYSICAL_CONF: u32 = 0x0F;
pub const OPA_PI_MASK_COLLECT_MASK: u32 = 0x38;
pub const OPA_PI_MASK_MULTICAST_MASK: u32 = 0x07;
pub const OPA_PI_MASK_MKEY_PROT_BIT: u32 = 0xC0;
pub const OPA_PI_MASK_LMC: u32 = 0x0F;
pub const OPA_PI_MASK_SMSL: u32 = 0x1F;
pub const OPA_PI_MASK_LINKINIT_REASON: u32 = 0xF0;
pub const OPA_PI_MASK_PARTITION_ENFORCE_IN: u32 = 0x08;
pub const OPA_PI_MASK_PARTITION_ENFORCE_OUT: u32 = 0x04;
pub const OPA_PI_MASK_OPERATIONAL_VL: u32 = 0x1F;
pub const OPA_PI_MASK_SA_QP: u32 = 0x00FFFFFF;
pub const OPA_PI_MASK_SM_TRAP_QP: u32 = 0x00FFFFFF;
pub const OPA_PI_MASK_LOCAL_PHY_ERRORS: u32 = 0xF0;
pub const OPA_PI_MASK_OVERRUN_ERRORS: u32 = 0x0F;
pub const OPA_PI_MASK_CLIENT_REREGISTER: u32 = 0x80;
pub const OPA_PI_MASK_SUBNET_TIMEOUT: u32 = 0x1F;
pub const OPA_PI_MASK_PORT_LINK_SUPPORTED: u32 = 0x001F << 10;
pub const OPA_PI_MASK_PORT_LINK_ENABLED: u32 = 0x001F << 5;
pub const OPA_PI_MASK_PORT_LINK_ACTIVE: u32 = 0x001F;
pub const OPA_PI_MASK_PORT_LINK_CRC_SUPPORTED: u32 = 0x0F00;
pub const OPA_PI_MASK_PORT_LINK_CRC_ENABLED: u32 = 0x00F0;
pub const OPA_PI_MASK_PORT_LINK_CRC_ACTIVE: u32 = 0x000F;
pub const OPA_PI_MASK_PORT_MODE_SECURITY_CHECK: u32 = 0x0001;
pub const OPA_PI_MASK_PORT_MODE_16B_TRAP_QUERY: u32 = 0x0002;
pub const OPA_PI_MASK_PORT_MODE_PKEY_CONVERT: u32 = 0x0004;
pub const OPA_PI_MASK_PORT_MODE_SC2SC_MAPPING: u32 = 0x0008;
pub const OPA_PI_MASK_PORT_MODE_VL_MARKER: u32 = 0x0010;
pub const OPA_PI_MASK_PORT_PASS_THROUGH: u32 = 0x0020;
pub const OPA_PI_MASK_PORT_ACTIVE_OPTOMIZE: u32 = 0x0040;
pub const OPA_PI_MASK_INTERLEAVE_DIST_SUP: u32 = 0x0003 << 12;
pub const OPA_PI_MASK_INTERLEAVE_DIST_ENABLE: u32 = 0x0003 << 10;
pub const OPA_PI_MASK_INTERLEAVE_MAX_NEST_TX: u32 = 0x001F << 5;
pub const OPA_PI_MASK_INTERLEAVE_MAX_NEST_RX: u32 = 0x001F;
pub const OPA_PI_MASK_EX_BUFFER_OVERRUN: u32 = 0x80000000;
pub const OPA_PI_MASK_FM_CFG_ERR_EXCEED_MULTICAST_LIMIT: u32 = 0x00800000;
pub const OPA_PI_MASK_FM_CFG_BAD_CONTROL_FLIT: u32 = 0x00400000;
pub const OPA_PI_MASK_FM_CFG_BAD_PREEMPT: u32 = 0x00200000;
pub const OPA_PI_MASK_FM_CFG_UNSUPPORTED_VL_MARKER: u32 = 0x00100000;
pub const OPA_PI_MASK_FM_CFG_BAD_CRDT_ACK: u32 = 0x00080000;
pub const OPA_PI_MASK_FM_CFG_BAD_CTRL_DIST: u32 = 0x00040000;
pub const OPA_PI_MASK_FM_CFG_BAD_TAIL_DIST: u32 = 0x00020000;
pub const OPA_PI_MASK_FM_CFG_BAD_HEAD_DIST: u32 = 0x00010000;
pub const OPA_PI_MASK_PORT_RCV_BAD_VL_MARKER: u32 = 0x00002000;
pub const OPA_PI_MASK_PORT_RCV_PREEMPT_VL15: u32 = 0x00001000;
pub const OPA_PI_MASK_PORT_RCV_PREEMPT_ERROR: u32 = 0x00000800;
pub const OPA_PI_MASK_PORT_RCV_BAD_MidTail: u32 = 0x00000200;
pub const OPA_PI_MASK_PORT_RCV_BAD_SC: u32 = 0x00000080;
pub const OPA_PI_MASK_PORT_RCV_BAD_L2: u32 = 0x00000040;
pub const OPA_PI_MASK_PORT_RCV_BAD_DLID: u32 = 0x00000020;
pub const OPA_PI_MASK_PORT_RCV_BAD_SLID: u32 = 0x00000010;
pub const OPA_PI_MASK_PORT_RCV_PKTLEN_TOOSHORT: u32 = 0x00000008;
pub const OPA_PI_MASK_PORT_RCV_PKTLEN_TOOLONG: u32 = 0x00000004;
pub const OPA_PI_MASK_PORT_RCV_BAD_PKTLEN: u32 = 0x00000002;
pub const OPA_PI_MASK_PORT_RCV_BAD_LT: u32 = 0x00000001;
pub const OPA_PI_MASK_PASS_THROUGH_DR_CONTROL: u32 = 0x01;
pub const OPA_PI_MASK_BUF_UNIT_VL15_INIT: u32 = 0x00000FFF << 11;
pub const OPA_PI_MASK_BUF_UNIT_VL15_CREDIT_RATE: u32 = 0x0000001F << 6;
pub const OPA_PI_MASK_BUF_UNIT_CREDIT_ACK: u32 = 0x00000003 << 3;
pub const OPA_PI_MASK_BUF_UNIT_BUF_ALLOC: u32 = 0x00000003;
pub const OPA_PI_MASK_NEIGH_MTU_PVL0: u32 = 0xF0;
pub const OPA_PI_MASK_NEIGH_MTU_PVL1: u32 = 0x0F;
pub const OPA_PI_MASK_VL_STALL: u32 = 0x03 << 5;
pub const OPA_PI_MASK_HOQ_LIFE: u32 = 0x1F;
pub const OPA_PI_MASK_NEIGH_MGMT_ALLOWED: u32 = 0x01 << 3;
pub const OPA_PI_MASK_NEIGH_FW_AUTH_BYPASS: u32 = 0x01 << 2;
pub const OPA_PI_MASK_NEIGH_NODE_TYPE: u32 = 0x03;
pub const OPA_PI_MASK_RESPONSE_TIME_VALUE: u32 = 0x1F;
pub const OPA_PI_MASK_MTU_CAP: u32 = 0x0F;

#[repr(C)]
pub struct opa_port_states { pub reserved: u8, pub ledenable_offlinereason: u8, pub reserved2: u8, pub portphysstate_portstate: u8 }

#[repr(C)]
pub struct opa_port_state_info { pub port_states: opa_port_states, pub link_width_downgrade_tx_active: u16, pub link_width_downgrade_rx_active: u16 }

#[repr(C)]
pub struct opa_port_info {
    pub lid: u32, pub flow_control_mask: u32,
    pub vl: opa_port_info_vl, pub port_states: opa_port_states,
    pub port_phys_conf: u8, pub collectivemask_multicastmask: u8, pub mkeyprotect_lmc: u8, pub smsl: u8,
    pub partenforce_filterraw: u8, pub operational_vls: u8, pub pkey_8b: u16, pub pkey_10b: u16, pub mkey_violations: u16,
    pub pkey_violations: u16, pub qkey_violations: u16, pub sm_trap_qp: u32, pub sa_qp: u32,
    pub neigh_port_num: u8, pub link_down_reason: u8, pub neigh_link_down_reason: u8, pub clientrereg_subnettimeout: u8,
    pub link_speed: opa_port_info_link_speed, pub link_width: opa_port_info_link_width, pub link_width_downgrade: opa_port_info_link_width_downgrade,
    pub port_link_mode: u16, pub port_ltp_crc_mode: u16, pub port_mode: u16, pub port_packet_format: opa_port_info_packet_format,
    pub flit_control: opa_port_info_flit_control, pub reserved4: u32, pub port_error_action: u32,
    pub pass_through: opa_port_info_pass_through, pub mkey_lease_period: u16, pub buffer_units: u32, pub reserved5: u32, pub sm_lid: u32,
    pub mkey: u64, pub subnet_prefix: u64, pub neigh_mtu: opa_port_info_neigh_mtu, pub xmit_q: [opa_port_info_xmit_q; OPA_MAX_VLS as usize],
    pub ipaddr_ipv6: opa_port_info_ipv6, pub ipaddr_ipv4: opa_port_info_ipv4, pub reserved6: u32, pub reserved7: u32, pub reserved8: u32,
    pub neigh_node_guid: u64, pub ib_cap_mask: u32, pub reserved9: u16, pub opa_cap_mask: u16, pub reserved10: u32,
    pub overall_buffer_space: u16, pub reserved11: u16, pub diag_code: u16, pub replay_depth: opa_port_info_replay_depth,
    pub port_neigh_mode: u8, pub mtucap: u8, pub resptimevalue: u8, pub local_port_num: u8, pub reserved12: u8, pub reserved13: u8,
}

#[repr(C)] pub struct opa_port_info_vl { pub res: u8, pub cap: u8, pub high_limit: u16, pub preempt_limit: u16, pub arb_high_cap: u8, pub arb_low_cap: u8 }
#[repr(C)] pub struct opa_port_info_link_speed { pub supported: u16, pub enabled: u16, pub active: u16 }
#[repr(C)] pub struct opa_port_info_link_width { pub supported: u16, pub enabled: u16, pub active: u16 }
#[repr(C)] pub struct opa_port_info_link_width_downgrade { pub supported: u16, pub enabled: u16, pub tx_active: u16, pub rx_active: u16 }
#[repr(C)] pub struct opa_port_info_packet_format { pub supported: u16, pub enabled: u16 }
#[repr(C)] pub struct opa_port_info_flit_control { pub interleave: u16, pub preemption: opa_port_info_preemption }
#[repr(C)] pub struct opa_port_info_preemption { pub min_initial: u16, pub min_tail: u16, pub large_pkt_limit: u8, pub small_pkt_limit: u8, pub max_small_pkt_limit: u8, pub preemption_limit: u8 }
#[repr(C)] pub struct opa_port_info_pass_through { pub egress_port: u8, pub res_drctl: u8 }
#[repr(C)] pub struct opa_port_info_neigh_mtu { pub pvlx_to_mtu: [u8; (OPA_MAX_VLS as usize) / 2] }
#[repr(C)] pub struct opa_port_info_xmit_q { pub vlstall_hoqlife: u8 }
#[repr(C)] pub struct opa_port_info_ipv6 { pub addr: [u8; 16] }
#[repr(C)] pub struct opa_port_info_ipv4 { pub addr: [u8; 4] }
#[repr(C)] pub struct opa_port_info_replay_depth { pub buffer: u8, pub wire: u8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
