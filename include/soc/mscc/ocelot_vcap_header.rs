/* SPDX-License-Identifier: (GPL-2.0 OR MIT)
 * Microsemi Ocelot Switch driver
 * Copyright (c) 2019 Microsemi Corporation
 */

// Dependency: soc/mscc/ocelot.h

macro_rules! OCELOT_VCAP_ES0_TAG_8021Q_RXVLAN { ($ocelot:expr, $port:expr, $upstream:expr) => { (($upstream) << 16 | ($port)) }; }
macro_rules! OCELOT_VCAP_IS1_TAG_8021Q_TXVLAN { ($ocelot:expr, $port:expr) => { ($port) }; }
macro_rules! OCELOT_VCAP_IS1_VLAN_RECLASSIFY { ($ocelot:expr, $port:expr) => { (($ocelot).num_phys_ports + ($port)) }; }
macro_rules! OCELOT_VCAP_IS2_TAG_8021Q_TXVLAN { ($ocelot:expr, $port:expr) => { ($port) }; }
macro_rules! OCELOT_VCAP_IS2_MRP_REDIRECT { ($ocelot:expr, $port:expr) => { (($ocelot).num_phys_ports + ($port)) }; }
macro_rules! OCELOT_VCAP_IS2_MRP_TRAP { ($ocelot:expr) => { (($ocelot).num_phys_ports * 2) }; }
macro_rules! OCELOT_VCAP_IS2_L2_PTP_TRAP { ($ocelot:expr) => { (($ocelot).num_phys_ports * 2 + 1) }; }
macro_rules! OCELOT_VCAP_IS2_IPV4_GEN_PTP_TRAP { ($ocelot:expr) => { (($ocelot).num_phys_ports * 2 + 2) }; }
macro_rules! OCELOT_VCAP_IS2_IPV4_EV_PTP_TRAP { ($ocelot:expr) => { (($ocelot).num_phys_ports * 2 + 3) }; }
macro_rules! OCELOT_VCAP_IS2_IPV6_GEN_PTP_TRAP { ($ocelot:expr) => { (($ocelot).num_phys_ports * 2 + 4) }; }
macro_rules! OCELOT_VCAP_IS2_IPV6_EV_PTP_TRAP { ($ocelot:expr) => { (($ocelot).num_phys_ports * 2 + 5) }; }

#[repr(i32)]
pub enum VcapBlock { VCAP_ES0, VCAP_IS1, VCAP_IS2, __VCAP_COUNT }
pub const OCELOT_NUM_VCAP_BLOCKS: i32 = VcapBlock::__VCAP_COUNT as i32;

#[repr(C)]
pub struct vcap_props {
    pub tg_width: u16, pub sw_count: u16, pub entry_count: u16, pub entry_words: u16,
    pub entry_width: u16, pub action_count: u16, pub action_words: u16, pub action_width: u16,
    pub action_type_width: u16,
    pub action_table: [vcap_action_table; 2],
    pub counter_words: u16, pub counter_width: u16,
    pub target: ocelot_target,
    pub keys: *const vcap_field, pub actions: *const vcap_field,
}
#[repr(C)] pub struct vcap_action_table { pub width: u16, pub count: u16 }

pub const VCAP_TG_NONE: u32 = 0; pub const VCAP_TG_FULL: u32 = 1; pub const VCAP_TG_HALF: u32 = 2; pub const VCAP_TG_QUARTER: u32 = 3;
macro_rules! VCAP_CORE_UPDATE_CTRL_UPDATE_CMD { ($x:expr) => { (($x << 22) & GENMASK(24, 22)) }; }
pub const VCAP_CORE_UPDATE_CTRL_UPDATE_CMD_M: u32 = GENMASK(24, 22);
macro_rules! VCAP_CORE_UPDATE_CTRL_UPDATE_CMD_X { ($x:expr) => { (($x & GENMASK(24, 22)) >> 22) }; }
pub const VCAP_CORE_UPDATE_CTRL_UPDATE_ENTRY_DIS: u32 = BIT(21); pub const VCAP_CORE_UPDATE_CTRL_UPDATE_ACTION_DIS: u32 = BIT(20); pub const VCAP_CORE_UPDATE_CTRL_UPDATE_CNT_DIS: u32 = BIT(19);
macro_rules! VCAP_CORE_UPDATE_CTRL_UPDATE_ADDR { ($x:expr) => { (($x << 3) & GENMASK(18, 3)) }; }
pub const VCAP_CORE_UPDATE_CTRL_UPDATE_ADDR_M: u32 = GENMASK(18, 3);
macro_rules! VCAP_CORE_UPDATE_CTRL_UPDATE_ADDR_X { ($x:expr) => { (($x & GENMASK(18, 3)) >> 3) }; }
pub const VCAP_CORE_UPDATE_CTRL_UPDATE_SHOT: u32 = BIT(2); pub const VCAP_CORE_UPDATE_CTRL_CLEAR_CACHE: u32 = BIT(1); pub const VCAP_CORE_UPDATE_CTRL_MV_TRAFFIC_IGN: u32 = BIT(0);
macro_rules! VCAP_CORE_MV_CFG_MV_NUM_POS { ($x:expr) => { (($x << 16) & GENMASK(31, 16)) }; }
pub const VCAP_CORE_MV_CFG_MV_NUM_POS_M: u32 = GENMASK(31, 16);
macro_rules! VCAP_CORE_MV_CFG_MV_NUM_POS_X { ($x:expr) => { (($x & GENMASK(31, 16)) >> 16) }; }
macro_rules! VCAP_CORE_MV_CFG_MV_SIZE { ($x:expr) => { (($x) & GENMASK(15, 0)) }; }
pub const VCAP_CORE_MV_CFG_MV_SIZE_M: u32 = GENMASK(15, 0);
pub const VCAP_CACHE_ENTRY_DAT_RSZ: u32 = 0x4; pub const VCAP_CACHE_MASK_DAT_RSZ: u32 = 0x4; pub const VCAP_CACHE_ACTION_DAT_RSZ: u32 = 0x4; pub const VCAP_CACHE_CNT_DAT_RSZ: u32 = 0x4;
pub const VCAP_STICKY_VCAP_ROW_DELETED_STICKY: u32 = BIT(0);
pub const TCAM_BIST_CTRL_TCAM_BIST: u32 = BIT(1); pub const TCAM_BIST_CTRL_TCAM_INIT: u32 = BIT(0);
pub const TCAM_BIST_CFG_TCAM_BIST_SOE_ENA: u32 = BIT(8); pub const TCAM_BIST_CFG_TCAM_HCG_DIS: u32 = BIT(7); pub const TCAM_BIST_CFG_TCAM_CG_DIS: u32 = BIT(6);
macro_rules! TCAM_BIST_CFG_TCAM_BIAS { ($x:expr) => { (($x) & GENMASK(5, 0)) }; } pub const TCAM_BIST_CFG_TCAM_BIAS_M: u32 = GENMASK(5, 0);
pub const TCAM_BIST_STAT_BIST_RT_ERR: u32 = BIT(15); pub const TCAM_BIST_STAT_BIST_PENC_ERR: u32 = BIT(14); pub const TCAM_BIST_STAT_BIST_COMP_ERR: u32 = BIT(13); pub const TCAM_BIST_STAT_BIST_ADDR_ERR: u32 = BIT(12); pub const TCAM_BIST_STAT_BIST_BL1E_ERR: u32 = BIT(11); pub const TCAM_BIST_STAT_BIST_BL1_ERR: u32 = BIT(10); pub const TCAM_BIST_STAT_BIST_BL0E_ERR: u32 = BIT(9); pub const TCAM_BIST_STAT_BIST_BL0_ERR: u32 = BIT(8); pub const TCAM_BIST_STAT_BIST_PH1_ERR: u32 = BIT(7); pub const TCAM_BIST_STAT_BIST_PH0_ERR: u32 = BIT(6); pub const TCAM_BIST_STAT_BIST_PV1_ERR: u32 = BIT(5); pub const TCAM_BIST_STAT_BIST_PV0_ERR: u32 = BIT(4); pub const TCAM_BIST_STAT_BIST_RUN: u32 = BIT(3); pub const TCAM_BIST_STAT_BIST_ERR: u32 = BIT(2); pub const TCAM_BIST_STAT_BIST_BUSY: u32 = BIT(1); pub const TCAM_BIST_STAT_TCAM_RDY: u32 = BIT(0);

pub const IS2_TYPE_ETYPE: u32 = 0; pub const IS2_TYPE_LLC: u32 = 1; pub const IS2_TYPE_SNAP: u32 = 2; pub const IS2_TYPE_ARP: u32 = 3; pub const IS2_TYPE_IP_UDP_TCP: u32 = 4; pub const IS2_TYPE_IP_OTHER: u32 = 5; pub const IS2_TYPE_IPV6: u32 = 6; pub const IS2_TYPE_OAM: u32 = 7; pub const IS2_TYPE_SMAC_SIP6: u32 = 8; pub const IS2_TYPE_ANY: u32 = 100; pub const IS2_TYPE_MASK_IP_ANY: u32 = 0xe;
#[repr(i32)] pub enum Is2ActionType { IS2_ACTION_TYPE_NORMAL, IS2_ACTION_TYPE_SMAC_SIP, IS2_ACTION_TYPE_MAX }
pub const IS2_ACT_MASK_MODE_NONE: u32 = 0; pub const IS2_ACT_MASK_MODE_FILTER: u32 = 1; pub const IS2_ACT_MASK_MODE_POLICY: u32 = 2; pub const IS2_ACT_MASK_MODE_REDIR: u32 = 3;
pub const IS2_ACT_REW_OP_NONE: u32 = 0; pub const IS2_ACT_REW_OP_PTP_ONE: u32 = 2; pub const IS2_ACT_REW_OP_PTP_TWO: u32 = 3; pub const IS2_ACT_REW_OP_SPECIAL: u32 = 8; pub const IS2_ACT_REW_OP_PTP_ORG: u32 = 9;
pub const IS2_ACT_REW_OP_PTP_ONE_SUB_DELAY_1: u32 = IS2_ACT_REW_OP_PTP_ONE | (1 << 3); pub const IS2_ACT_REW_OP_PTP_ONE_SUB_DELAY_2: u32 = IS2_ACT_REW_OP_PTP_ONE | (2 << 3); pub const IS2_ACT_REW_OP_PTP_ONE_ADD_DELAY: u32 = IS2_ACT_REW_OP_PTP_ONE | (1 << 5); pub const IS2_ACT_REW_OP_PTP_ONE_ADD_SUB: u32 = BIT(7);
pub const VCAP_PORT_WIDTH: u32 = 4; pub const IS2_QKO_IGR_PORT: u32 = 0; pub const IS2_QKL_IGR_PORT: u32 = VCAP_PORT_WIDTH; pub const IS2_QKO_L2_SMAC: u32 = IS2_QKO_IGR_PORT + IS2_QKL_IGR_PORT; pub const IS2_QKL_L2_SMAC: u32 = 48; pub const IS2_QKO_L3_IP4_SIP: u32 = IS2_QKO_L2_SMAC + IS2_QKL_L2_SMAC; pub const IS2_QKL_L3_IP4_SIP: u32 = 32;

#[repr(C)] pub struct vcap_field { pub offset: i32, pub length: i32 }

macro_rules! enum_seq { ($name:ident { $($v:ident),* $(,)? }) => { #[repr(i32)] pub enum $name { $($v),* } }; }
enum_seq!(VcapIs2HalfKeyField { VCAP_IS2_TYPE, VCAP_IS2_HK_FIRST, VCAP_IS2_HK_PAG, VCAP_IS2_HK_RSV1, VCAP_IS2_HK_IGR_PORT_MASK, VCAP_IS2_HK_RSV2, VCAP_IS2_HK_HOST_MATCH, VCAP_IS2_HK_L2_MC, VCAP_IS2_HK_L2_BC, VCAP_IS2_HK_VLAN_TAGGED, VCAP_IS2_HK_VID, VCAP_IS2_HK_DEI, VCAP_IS2_HK_PCP, VCAP_IS2_HK_L2_DMAC, VCAP_IS2_HK_L2_SMAC, VCAP_IS2_HK_MAC_ETYPE_ETYPE, VCAP_IS2_HK_MAC_ETYPE_L2_PAYLOAD0, VCAP_IS2_HK_MAC_ETYPE_L2_PAYLOAD1, VCAP_IS2_HK_MAC_ETYPE_L2_PAYLOAD2, VCAP_IS2_HK_MAC_LLC_DMAC, VCAP_IS2_HK_MAC_LLC_SMAC, VCAP_IS2_HK_MAC_LLC_L2_LLC, VCAP_IS2_HK_MAC_SNAP_SMAC, VCAP_IS2_HK_MAC_SNAP_DMAC, VCAP_IS2_HK_MAC_SNAP_L2_SNAP, VCAP_IS2_HK_MAC_ARP_SMAC, VCAP_IS2_HK_MAC_ARP_ADDR_SPACE_OK, VCAP_IS2_HK_MAC_ARP_PROTO_SPACE_OK, VCAP_IS2_HK_MAC_ARP_LEN_OK, VCAP_IS2_HK_MAC_ARP_TARGET_MATCH, VCAP_IS2_HK_MAC_ARP_SENDER_MATCH, VCAP_IS2_HK_MAC_ARP_OPCODE_UNKNOWN, VCAP_IS2_HK_MAC_ARP_OPCODE, VCAP_IS2_HK_MAC_ARP_L3_IP4_DIP, VCAP_IS2_HK_MAC_ARP_L3_IP4_SIP, VCAP_IS2_HK_MAC_ARP_DIP_EQ_SIP, VCAP_IS2_HK_IP4, VCAP_IS2_HK_L3_FRAGMENT, VCAP_IS2_HK_L3_FRAG_OFS_GT0, VCAP_IS2_HK_L3_OPTIONS, VCAP_IS2_HK_IP4_L3_TTL_GT0, VCAP_IS2_HK_L3_TOS, VCAP_IS2_HK_L3_IP4_DIP, VCAP_IS2_HK_L3_IP4_SIP, VCAP_IS2_HK_DIP_EQ_SIP, VCAP_IS2_HK_TCP, VCAP_IS2_HK_L4_SPORT, VCAP_IS2_HK_L4_DPORT, VCAP_IS2_HK_L4_RNG, VCAP_IS2_HK_L4_SPORT_EQ_DPORT, VCAP_IS2_HK_L4_SEQUENCE_EQ0, VCAP_IS2_HK_L4_URG, VCAP_IS2_HK_L4_ACK, VCAP_IS2_HK_L4_PSH, VCAP_IS2_HK_L4_RST, VCAP_IS2_HK_L4_SYN, VCAP_IS2_HK_L4_FIN, VCAP_IS2_HK_L4_1588_DOM, VCAP_IS2_HK_L4_1588_VER, VCAP_IS2_HK_IP4_L3_PROTO, VCAP_IS2_HK_L3_PAYLOAD, VCAP_IS2_HK_IP6_L3_TTL_GT0, VCAP_IS2_HK_IP6_L3_PROTO, VCAP_IS2_HK_L3_IP6_SIP, VCAP_IS2_HK_OAM_MEL_FLAGS, VCAP_IS2_HK_OAM_VER, VCAP_IS2_HK_OAM_OPCODE, VCAP_IS2_HK_OAM_FLAGS, VCAP_IS2_HK_OAM_MEPID, VCAP_IS2_HK_OAM_CCM_CNTS_EQ0, VCAP_IS2_HK_OAM_IS_Y1731 });
enum_seq!(VcapIs2ActionField { VCAP_IS2_ACT_HIT_ME_ONCE, VCAP_IS2_ACT_CPU_COPY_ENA, VCAP_IS2_ACT_CPU_QU_NUM, VCAP_IS2_ACT_MASK_MODE, VCAP_IS2_ACT_MIRROR_ENA, VCAP_IS2_ACT_LRN_DIS, VCAP_IS2_ACT_POLICE_ENA, VCAP_IS2_ACT_POLICE_IDX, VCAP_IS2_ACT_POLICE_VCAP_ONLY, VCAP_IS2_ACT_PORT_MASK, VCAP_IS2_ACT_REW_OP, VCAP_IS2_ACT_SMAC_REPLACE_ENA, VCAP_IS2_ACT_RSV, VCAP_IS2_ACT_ACL_ID, VCAP_IS2_ACT_HIT_CNT });
pub const IS1_TYPE_S1_NORMAL: u32 = 0; pub const IS1_TYPE_S1_5TUPLE_IP4: u32 = 1; pub const IS1_TYPE_S1_NORMAL_IP6: u32 = 0; pub const IS1_TYPE_S1_7TUPLE: u32 = 1; pub const IS2_TYPE_S1_5TUPLE_IP6: u32 = 2;
#[repr(i32)] pub enum Is1ActionType { IS1_ACTION_TYPE_NORMAL, IS1_ACTION_TYPE_MAX }
enum_seq!(VcapIs1HalfKeyField { VCAP_IS1_HK_TYPE, VCAP_IS1_HK_LOOKUP, VCAP_IS1_HK_IGR_PORT_MASK, VCAP_IS1_HK_RSV, VCAP_IS1_HK_OAM_Y1731, VCAP_IS1_HK_L2_MC, VCAP_IS1_HK_L2_BC, VCAP_IS1_HK_IP_MC, VCAP_IS1_HK_VLAN_TAGGED, VCAP_IS1_HK_VLAN_DBL_TAGGED, VCAP_IS1_HK_TPID, VCAP_IS1_HK_VID, VCAP_IS1_HK_DEI, VCAP_IS1_HK_PCP, VCAP_IS1_HK_L2_SMAC, VCAP_IS1_HK_ETYPE_LEN, VCAP_IS1_HK_ETYPE, VCAP_IS1_HK_IP_SNAP, VCAP_IS1_HK_IP4, VCAP_IS1_HK_L3_FRAGMENT, VCAP_IS1_HK_L3_FRAG_OFS_GT0, VCAP_IS1_HK_L3_OPTIONS, VCAP_IS1_HK_L3_DSCP, VCAP_IS1_HK_L3_IP4_SIP, VCAP_IS1_HK_TCP_UDP, VCAP_IS1_HK_TCP, VCAP_IS1_HK_L4_SPORT, VCAP_IS1_HK_L4_RNG, VCAP_IS1_HK_IP4_INNER_TPID, VCAP_IS1_HK_IP4_INNER_VID, VCAP_IS1_HK_IP4_INNER_DEI, VCAP_IS1_HK_IP4_INNER_PCP, VCAP_IS1_HK_IP4_IP4, VCAP_IS1_HK_IP4_L3_FRAGMENT, VCAP_IS1_HK_IP4_L3_FRAG_OFS_GT0, VCAP_IS1_HK_IP4_L3_OPTIONS, VCAP_IS1_HK_IP4_L3_DSCP, VCAP_IS1_HK_IP4_L3_IP4_DIP, VCAP_IS1_HK_IP4_L3_IP4_SIP, VCAP_IS1_HK_IP4_L3_PROTO, VCAP_IS1_HK_IP4_TCP_UDP, VCAP_IS1_HK_IP4_TCP, VCAP_IS1_HK_IP4_L4_RNG, VCAP_IS1_HK_IP4_IP_PAYLOAD_S1_5TUPLE });
enum_seq!(VcapIs1ActionField { VCAP_IS1_ACT_DSCP_ENA, VCAP_IS1_ACT_DSCP_VAL, VCAP_IS1_ACT_QOS_ENA, VCAP_IS1_ACT_QOS_VAL, VCAP_IS1_ACT_DP_ENA, VCAP_IS1_ACT_DP_VAL, VCAP_IS1_ACT_PAG_OVERRIDE_MASK, VCAP_IS1_ACT_PAG_VAL, VCAP_IS1_ACT_RSV, VCAP_IS1_ACT_VID_REPLACE_ENA, VCAP_IS1_ACT_VID_ADD_VAL, VCAP_IS1_ACT_FID_SEL, VCAP_IS1_ACT_FID_VAL, VCAP_IS1_ACT_PCP_DEI_ENA, VCAP_IS1_ACT_PCP_VAL, VCAP_IS1_ACT_DEI_VAL, VCAP_IS1_ACT_VLAN_POP_CNT_ENA, VCAP_IS1_ACT_VLAN_POP_CNT, VCAP_IS1_ACT_CUSTOM_ACE_TYPE_ENA, VCAP_IS1_ACT_HIT_STICKY });
#[repr(i32)] pub enum Es0ActionType { ES0_ACTION_TYPE_NORMAL, ES0_ACTION_TYPE_MAX }
enum_seq!(VcapEs0KeyField { VCAP_ES0_EGR_PORT, VCAP_ES0_IGR_PORT, VCAP_ES0_RSV, VCAP_ES0_L2_MC, VCAP_ES0_L2_BC, VCAP_ES0_VID, VCAP_ES0_DP, VCAP_ES0_PCP });
enum_seq!(VcapEs0ActionField { VCAP_ES0_ACT_PUSH_OUTER_TAG, VCAP_ES0_ACT_PUSH_INNER_TAG, VCAP_ES0_ACT_TAG_A_TPID_SEL, VCAP_ES0_ACT_TAG_A_VID_SEL, VCAP_ES0_ACT_TAG_A_PCP_SEL, VCAP_ES0_ACT_TAG_A_DEI_SEL, VCAP_ES0_ACT_TAG_B_TPID_SEL, VCAP_ES0_ACT_TAG_B_VID_SEL, VCAP_ES0_ACT_TAG_B_PCP_SEL, VCAP_ES0_ACT_TAG_B_DEI_SEL, VCAP_ES0_ACT_VID_A_VAL, VCAP_ES0_ACT_PCP_A_VAL, VCAP_ES0_ACT_DEI_A_VAL, VCAP_ES0_ACT_VID_B_VAL, VCAP_ES0_ACT_PCP_B_VAL, VCAP_ES0_ACT_DEI_B_VAL, VCAP_ES0_ACT_RSV, VCAP_ES0_ACT_HIT_STICKY });

#[repr(C)] pub struct ocelot_ipv4 { pub addr: [u8; 4] }
#[repr(i32)] pub enum OcelotVcapBit { OCELOT_VCAP_BIT_ANY, OCELOT_VCAP_BIT_0, OCELOT_VCAP_BIT_1 }
macro_rules! byte_pair { ($name:ident, $n:expr) => { #[repr(C)] pub struct $name { pub value: [u8; $n], pub mask: [u8; $n] } }; }
byte_pair!(ocelot_vcap_u8, 1); byte_pair!(ocelot_vcap_u16, 2); byte_pair!(ocelot_vcap_u24, 3); byte_pair!(ocelot_vcap_u32, 4); byte_pair!(ocelot_vcap_u40, 5); byte_pair!(ocelot_vcap_u48, 6); byte_pair!(ocelot_vcap_u64, 8); byte_pair!(ocelot_vcap_u128, 16);
#[repr(C)] pub struct ocelot_vcap_vid { pub value: u16, pub mask: u16 }
#[repr(C)] pub struct ocelot_vcap_ipv4 { pub value: ocelot_ipv4, pub mask: ocelot_ipv4 }
#[repr(C)] pub struct ocelot_vcap_udp_tcp { pub value: u16, pub mask: u16 }
#[repr(C)] pub struct ocelot_vcap_port { pub value: u8, pub mask: u8 }
#[repr(i32)] pub enum OcelotVcapKeyType { OCELOT_VCAP_KEY_ANY, OCELOT_VCAP_KEY_ETYPE, OCELOT_VCAP_KEY_LLC, OCELOT_VCAP_KEY_SNAP, OCELOT_VCAP_KEY_ARP, OCELOT_VCAP_KEY_IPV4, OCELOT_VCAP_KEY_IPV6 }

#[repr(C)] pub struct ocelot_vcap_key_vlan { pub vid: ocelot_vcap_vid, pub pcp: ocelot_vcap_u8, pub dei: OcelotVcapBit, pub tagged: OcelotVcapBit, pub tpid: OcelotVcapBit }
#[repr(C)] pub struct ocelot_vcap_key_etype { pub dmac: ocelot_vcap_u48, pub smac: ocelot_vcap_u48, pub etype: ocelot_vcap_u16, pub data: ocelot_vcap_u16 }
#[repr(C)] pub struct ocelot_vcap_key_llc { pub dmac: ocelot_vcap_u48, pub smac: ocelot_vcap_u48, pub llc: ocelot_vcap_u32 }
#[repr(C)] pub struct ocelot_vcap_key_snap { pub dmac: ocelot_vcap_u48, pub smac: ocelot_vcap_u48, pub snap: ocelot_vcap_u40 }
#[repr(C)] pub struct ocelot_vcap_key_arp { pub smac: ocelot_vcap_u48, pub arp: OcelotVcapBit, pub req: OcelotVcapBit, pub unknown: OcelotVcapBit, pub smac_match: OcelotVcapBit, pub dmac_match: OcelotVcapBit, pub length: OcelotVcapBit, pub ip: OcelotVcapBit, pub ethernet: OcelotVcapBit, pub sip: ocelot_vcap_ipv4, pub dip: ocelot_vcap_ipv4 }
#[repr(C)] pub struct ocelot_vcap_key_ipv4 { pub ttl: OcelotVcapBit, pub fragment: OcelotVcapBit, pub options: OcelotVcapBit, pub ds: ocelot_vcap_u8, pub proto: ocelot_vcap_u8, pub sip: ocelot_vcap_ipv4, pub dip: ocelot_vcap_ipv4, pub data: ocelot_vcap_u48, pub sport: ocelot_vcap_udp_tcp, pub dport: ocelot_vcap_udp_tcp, pub tcp_fin: OcelotVcapBit, pub tcp_syn: OcelotVcapBit, pub tcp_rst: OcelotVcapBit, pub tcp_psh: OcelotVcapBit, pub tcp_ack: OcelotVcapBit, pub tcp_urg: OcelotVcapBit, pub sip_eq_dip: OcelotVcapBit, pub sport_eq_dport: OcelotVcapBit, pub seq_zero: OcelotVcapBit }
#[repr(C)] pub struct ocelot_vcap_key_ipv6 { pub proto: ocelot_vcap_u8, pub sip: ocelot_vcap_u128, pub dip: ocelot_vcap_u128, pub ttl: OcelotVcapBit, pub ds: ocelot_vcap_u8, pub data: ocelot_vcap_u48, pub sport: ocelot_vcap_udp_tcp, pub dport: ocelot_vcap_udp_tcp, pub tcp_fin: OcelotVcapBit, pub tcp_syn: OcelotVcapBit, pub tcp_rst: OcelotVcapBit, pub tcp_psh: OcelotVcapBit, pub tcp_ack: OcelotVcapBit, pub tcp_urg: OcelotVcapBit, pub sip_eq_dip: OcelotVcapBit, pub sport_eq_dport: OcelotVcapBit, pub seq_zero: OcelotVcapBit }

#[repr(i32)] pub enum OcelotMaskMode { OCELOT_MASK_MODE_NONE, OCELOT_MASK_MODE_PERMIT_DENY, OCELOT_MASK_MODE_POLICY, OCELOT_MASK_MODE_REDIRECT }
#[repr(i32)] pub enum OcelotEs0VidSel { OCELOT_ES0_VID_PLUS_CLASSIFIED_VID, OCELOT_ES0_VID }
#[repr(i32)] pub enum OcelotEs0PcpSel { OCELOT_CLASSIFIED_PCP, OCELOT_ES0_PCP }
#[repr(i32)] pub enum OcelotEs0Tag { OCELOT_NO_ES0_TAG, OCELOT_ES0_TAG, OCELOT_FORCE_PORT_TAG, OCELOT_FORCE_UNTAG }
#[repr(i32)] pub enum OcelotTagTpidSel { OCELOT_TAG_TPID_SEL_8021Q, OCELOT_TAG_TPID_SEL_8021AD }

#[repr(C)] pub union ocelot_vcap_action {
    pub es0: ocelot_vcap_action_es0, pub is1: ocelot_vcap_action_is1, pub is2: ocelot_vcap_action_is2,
}
#[repr(C)] pub struct ocelot_vcap_action_es0 { pub push_outer_tag: OcelotEs0Tag, pub push_inner_tag: OcelotEs0Tag, pub tag_a_tpid_sel: OcelotTagTpidSel, pub tag_a_vid_sel: i32, pub tag_a_pcp_sel: i32, pub vid_a_val: u16, pub pcp_a_val: u8, pub dei_a_val: u8, pub tag_b_tpid_sel: OcelotTagTpidSel, pub tag_b_vid_sel: i32, pub tag_b_pcp_sel: i32, pub vid_b_val: u16, pub pcp_b_val: u8, pub dei_b_val: u8 }
#[repr(C)] pub struct ocelot_vcap_action_is1 { pub vid_replace_ena: bool, pub vid: u16, pub vlan_pop_cnt_ena: bool, pub vlan_pop_cnt: i32, pub pcp_dei_ena: bool, pub pcp: u8, pub dei: u8, pub qos_ena: bool, pub qos_val: u8, pub pag_override_mask: u8, pub pag_val: u8 }
#[repr(C)] pub struct ocelot_vcap_action_is2 { pub cpu_copy_ena: bool, pub cpu_qu_num: u8, pub mask_mode: OcelotMaskMode, pub port_mask: usize, pub police_ena: bool, pub mirror_ena: bool, pub pol: ocelot_policer, pub pol_ix: u32 }
#[repr(C)] pub struct ocelot_vcap_stats { pub bytes: u64, pub pkts: u64, pub used: u64 }
#[repr(i32)] pub enum OcelotVcapFilterType { OCELOT_VCAP_FILTER_DUMMY, OCELOT_VCAP_FILTER_PAG, OCELOT_VCAP_FILTER_OFFLOAD, OCELOT_PSFP_FILTER_OFFLOAD }
#[repr(C)] pub struct ocelot_vcap_id { pub cookie: usize, pub tc_offload: bool }
#[repr(C)] pub union ocelot_vcap_filter_key { pub etype: ocelot_vcap_key_etype, pub llc: ocelot_vcap_key_llc, pub snap: ocelot_vcap_key_snap, pub arp: ocelot_vcap_key_arp, pub ipv4: ocelot_vcap_key_ipv4, pub ipv6: ocelot_vcap_key_ipv6 }
#[repr(C)] pub struct ocelot_vcap_filter { pub list: list_head, pub type_: OcelotVcapFilterType, pub block_id: i32, pub goto_target: i32, pub lookup: i32, pub pag: u8, pub prio: u16, pub id: ocelot_vcap_id, pub action: ocelot_vcap_action, pub stats: ocelot_vcap_stats, pub take_ts: bool, pub is_trap: bool, pub ingress_port_mask: usize, pub ingress_port: ocelot_vcap_port, pub egress_port: ocelot_vcap_port, pub dmac_mc: OcelotVcapBit, pub dmac_bc: OcelotVcapBit, pub vlan: ocelot_vcap_key_vlan, pub key_type: OcelotVcapKeyType, pub key: ocelot_vcap_filter_key }

extern "C" {
    pub fn ocelot_vcap_filter_add(ocelot: *mut ocelot, rule: *mut ocelot_vcap_filter, extack: *mut netlink_ext_ack) -> i32;
    pub fn ocelot_vcap_filter_del(ocelot: *mut ocelot, rule: *mut ocelot_vcap_filter) -> i32;
    pub fn ocelot_vcap_filter_replace(ocelot: *mut ocelot, filter: *mut ocelot_vcap_filter) -> i32;
    pub fn ocelot_vcap_block_find_filter_by_id(block: *mut ocelot_vcap_block, cookie: usize, tc_offload: bool) -> *mut ocelot_vcap_filter;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
