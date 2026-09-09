/*
 * Copyright (c) 2006 Cisco Systems, Inc.  All rights reserved.
 *
 * This software is available under a choice of one of two licenses: the
 * GNU General Public License (GPL) Version 2, or the OpenIB.org BSD license.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external to this header translation.

pub const MLX4_CMD_SYS_EN: u32 = 0x1;
pub const MLX4_CMD_SYS_DIS: u32 = 0x2;
pub const MLX4_CMD_MAP_FA: u32 = 0xfff;
pub const MLX4_CMD_UNMAP_FA: u32 = 0xffe;
pub const MLX4_CMD_RUN_FW: u32 = 0xff6;
pub const MLX4_CMD_MOD_STAT_CFG: u32 = 0x34;
pub const MLX4_CMD_QUERY_DEV_CAP: u32 = 0x3;
pub const MLX4_CMD_QUERY_FW: u32 = 0x4;
pub const MLX4_CMD_ENABLE_LAM: u32 = 0xff8;
pub const MLX4_CMD_DISABLE_LAM: u32 = 0xff7;
pub const MLX4_CMD_QUERY_DDR: u32 = 0x5;
pub const MLX4_CMD_QUERY_ADAPTER: u32 = 0x6;
pub const MLX4_CMD_INIT_HCA: u32 = 0x7;
pub const MLX4_CMD_CLOSE_HCA: u32 = 0x8;
pub const MLX4_CMD_INIT_PORT: u32 = 0x9;
pub const MLX4_CMD_CLOSE_PORT: u32 = 0xa;
pub const MLX4_CMD_QUERY_HCA: u32 = 0xb;
pub const MLX4_CMD_QUERY_PORT: u32 = 0x43;
pub const MLX4_CMD_SENSE_PORT: u32 = 0x4d;
pub const MLX4_CMD_HW_HEALTH_CHECK: u32 = 0x50;
pub const MLX4_CMD_SET_PORT: u32 = 0xc;
pub const MLX4_CMD_SET_NODE: u32 = 0x5a;
pub const MLX4_CMD_QUERY_FUNC: u32 = 0x56;
pub const MLX4_CMD_ACCESS_DDR: u32 = 0x2e;
pub const MLX4_CMD_MAP_ICM: u32 = 0xffa;
pub const MLX4_CMD_UNMAP_ICM: u32 = 0xff9;
pub const MLX4_CMD_MAP_ICM_AUX: u32 = 0xffc;
pub const MLX4_CMD_UNMAP_ICM_AUX: u32 = 0xffb;
pub const MLX4_CMD_SET_ICM_SIZE: u32 = 0xffd;
pub const MLX4_CMD_ACCESS_REG: u32 = 0x3b;
pub const MLX4_CMD_ALLOCATE_VPP: u32 = 0x80;
pub const MLX4_CMD_SET_VPORT_QOS: u32 = 0x81;
pub const MLX4_CMD_INFORM_FLR_DONE: u32 = 0x5b;
pub const MLX4_CMD_VIRT_PORT_MAP: u32 = 0x5c;
pub const MLX4_CMD_GET_OP_REQ: u32 = 0x59;
pub const MLX4_CMD_SW2HW_MPT: u32 = 0xd;
pub const MLX4_CMD_QUERY_MPT: u32 = 0xe;
pub const MLX4_CMD_HW2SW_MPT: u32 = 0xf;
pub const MLX4_CMD_READ_MTT: u32 = 0x10;
pub const MLX4_CMD_WRITE_MTT: u32 = 0x11;
pub const MLX4_CMD_SYNC_TPT: u32 = 0x2f;
pub const MLX4_CMD_MAP_EQ: u32 = 0x12;
pub const MLX4_CMD_SW2HW_EQ: u32 = 0x13;
pub const MLX4_CMD_HW2SW_EQ: u32 = 0x14;
pub const MLX4_CMD_QUERY_EQ: u32 = 0x15;
pub const MLX4_CMD_SW2HW_CQ: u32 = 0x16;
pub const MLX4_CMD_HW2SW_CQ: u32 = 0x17;
pub const MLX4_CMD_QUERY_CQ: u32 = 0x18;
pub const MLX4_CMD_MODIFY_CQ: u32 = 0x2c;
pub const MLX4_CMD_SW2HW_SRQ: u32 = 0x35;
pub const MLX4_CMD_HW2SW_SRQ: u32 = 0x36;
pub const MLX4_CMD_QUERY_SRQ: u32 = 0x37;
pub const MLX4_CMD_ARM_SRQ: u32 = 0x40;
pub const MLX4_CMD_RST2INIT_QP: u32 = 0x19;
pub const MLX4_CMD_INIT2RTR_QP: u32 = 0x1a;
pub const MLX4_CMD_RTR2RTS_QP: u32 = 0x1b;
pub const MLX4_CMD_RTS2RTS_QP: u32 = 0x1c;
pub const MLX4_CMD_SQERR2RTS_QP: u32 = 0x1d;
pub const MLX4_CMD_2ERR_QP: u32 = 0x1e;
pub const MLX4_CMD_RTS2SQD_QP: u32 = 0x1f;
pub const MLX4_CMD_SQD2SQD_QP: u32 = 0x38;
pub const MLX4_CMD_SQD2RTS_QP: u32 = 0x20;
pub const MLX4_CMD_2RST_QP: u32 = 0x21;
pub const MLX4_CMD_QUERY_QP: u32 = 0x22;
pub const MLX4_CMD_INIT2INIT_QP: u32 = 0x2d;
pub const MLX4_CMD_SUSPEND_QP: u32 = 0x32;
pub const MLX4_CMD_UNSUSPEND_QP: u32 = 0x33;
pub const MLX4_CMD_UPDATE_QP: u32 = 0x61;
pub const MLX4_CMD_CONF_SPECIAL_QP: u32 = 0x23;
pub const MLX4_CMD_MAD_IFC: u32 = 0x24;
pub const MLX4_CMD_MAD_DEMUX: u32 = 0x203;
pub const MLX4_CMD_READ_MCG: u32 = 0x25;
pub const MLX4_CMD_WRITE_MCG: u32 = 0x26;
pub const MLX4_CMD_MGID_HASH: u32 = 0x27;
pub const MLX4_CMD_DIAG_RPRT: u32 = 0x30;
pub const MLX4_CMD_NOP: u32 = 0x31;
pub const MLX4_CMD_CONFIG_DEV: u32 = 0x3a;
pub const MLX4_CMD_ACCESS_MEM: u32 = 0x2e;
pub const MLX4_CMD_SET_VEP: u32 = 0x52;
pub const MLX4_CMD_SET_VLAN_FLTR: u32 = 0x47;
pub const MLX4_CMD_SET_MCAST_FLTR: u32 = 0x48;
pub const MLX4_CMD_DUMP_ETH_STATS: u32 = 0x49;
pub const MLX4_CMD_ARM_COMM_CHANNEL: u32 = 0x57;
pub const MLX4_CMD_GEN_EQE: u32 = 0x58;
pub const MLX4_CMD_ALLOC_RES: u32 = 0xf00;
pub const MLX4_CMD_FREE_RES: u32 = 0xf01;
pub const MLX4_CMD_MCAST_ATTACH: u32 = 0xf05;
pub const MLX4_CMD_UCAST_ATTACH: u32 = 0xf06;
pub const MLX4_CMD_PROMISC: u32 = 0xf08;
pub const MLX4_CMD_QUERY_FUNC_CAP: u32 = 0xf0a;
pub const MLX4_CMD_QP_ATTACH: u32 = 0xf0b;
pub const MLX4_CMD_QUERY_DEBUG_MSG: u32 = 0x2a;
pub const MLX4_CMD_SET_DEBUG_MSG: u32 = 0x2b;
pub const MLX4_CMD_QUERY_IF_STAT: u32 = 0x54;
pub const MLX4_CMD_SET_IF_STAT: u32 = 0x55;
pub const MLX4_QP_FLOW_STEERING_ATTACH: u32 = 0x65;
pub const MLX4_QP_FLOW_STEERING_DETACH: u32 = 0x66;
pub const MLX4_FLOW_STEERING_IB_UC_QP_RANGE: u32 = 0x64;
pub const MLX4_CMD_CONGESTION_CTRL_OPCODE: u32 = 0x68;

pub const MLX4_CMD_TIME_CLASS_A: u32 = 60000;
pub const MLX4_CMD_TIME_CLASS_B: u32 = 60000;
pub const MLX4_CMD_TIME_CLASS_C: u32 = 60000;
pub const MLX4_GET_PORT_VIRT2PHY: u32 = 0;
pub const MLX4_SET_PORT_VIRT2PHY: u32 = 1;
pub const MLX4_MAILBOX_SIZE: u32 = 4096;
pub const MLX4_ACCESS_MEM_ALIGN: u32 = 256;
pub const MLX4_SET_PORT_IB_OPCODE: u32 = 0;
pub const MLX4_SET_PORT_ETH_OPCODE: u32 = 1;
pub const MLX4_SET_PORT_BEACON_OPCODE: u32 = 4;
pub const MLX4_SET_PORT_GENERAL: u32 = 0;
pub const MLX4_SET_PORT_RQP_CALC: u32 = 1;
pub const MLX4_SET_PORT_MAC_TABLE: u32 = 2;
pub const MLX4_SET_PORT_VLAN_TABLE: u32 = 3;
pub const MLX4_SET_PORT_PRIO_MAP: u32 = 4;
pub const MLX4_SET_PORT_GID_TABLE: u32 = 5;
pub const MLX4_SET_PORT_PRIO2TC: u32 = 8;
pub const MLX4_SET_PORT_SCHEDULER: u32 = 9;
pub const MLX4_SET_PORT_VXLAN: u32 = 0xB;
pub const MLX4_SET_PORT_ROCE_ADDR: u32 = 0xD;
pub const MLX4_CMD_MAD_DEMUX_CONFIG: u32 = 0;
pub const MLX4_CMD_MAD_DEMUX_QUERY_STATE: u32 = 1;
pub const MLX4_CMD_MAD_DEMUX_QUERY_RESTR: u32 = 2;
pub const MLX4_CMD_WRAPPED: u32 = 0;
pub const MLX4_CMD_NATIVE: u32 = 1;

pub const MLX4_RX_CSUM_MODE_VAL_NON_TCP_UDP: u32 = 1u32 << 0;
pub const MLX4_RX_CSUM_MODE_L4: u32 = 1u32 << 1;
pub const MLX4_RX_CSUM_MODE_IP_OK_IP_NON_TCP_UDP: u32 = 1u32 << 2;
pub const MLX4_RX_CSUM_MODE_MULTI_VLAN: u32 = 1u32 << 3;

#[repr(C)]
pub struct mlx4_config_dev_params {
    pub vxlan_udp_dport: u16,
    pub rx_csum_flags_port_1: u8,
    pub rx_csum_flags_port_2: u8,
}

pub const MLX4_CTRL_ALGO_802_1_QAU_REACTION_POINT: u32 = 0;
pub const MLX4_CONGESTION_CONTROL_GET_PARAMS: u32 = 0;
pub const MLX4_CONGESTION_CONTROL_GET_STATISTICS: u32 = 1;
pub const MLX4_CONGESTION_CONTROL_SET_PARAMS: u32 = 4;

#[repr(C)]
pub struct mlx4_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mlx4_cmd_mailbox {
    pub buf: *mut core::ffi::c_void,
    pub dma: dma_addr_t,
}

extern "C" {
    pub fn __mlx4_cmd(dev: *mut mlx4_dev, in_param: u64, out_param: *mut u64,
                      out_is_imm: i32, in_modifier: u32, op_modifier: u8,
                      op: u16, timeout: usize, native: i32) -> i32;
}

#[inline]
pub unsafe fn mlx4_cmd(dev: *mut mlx4_dev, in_param: u64, in_modifier: u32,
                       op_modifier: u8, op: u16, timeout: usize, native: i32) -> i32 {
    __mlx4_cmd(dev, in_param, core::ptr::null_mut(), 0, in_modifier,
               op_modifier, op, timeout, native)
}

#[inline]
pub unsafe fn mlx4_cmd_box(dev: *mut mlx4_dev, in_param: u64, mut out_param: u64,
                           in_modifier: u32, op_modifier: u8, op: u16,
                           timeout: usize, native: i32) -> i32 {
    __mlx4_cmd(dev, in_param, &mut out_param, 0, in_modifier,
               op_modifier, op, timeout, native)
}

#[inline]
pub unsafe fn mlx4_cmd_imm(dev: *mut mlx4_dev, in_param: u64, out_param: *mut u64,
                           in_modifier: u32, op_modifier: u8, op: u16,
                           timeout: usize, native: i32) -> i32 {
    __mlx4_cmd(dev, in_param, out_param, 1, in_modifier,
               op_modifier, op, timeout, native)
}

extern "C" {
    pub fn mlx4_alloc_cmd_mailbox(dev: *mut mlx4_dev) -> *mut mlx4_cmd_mailbox;
    pub fn mlx4_free_cmd_mailbox(dev: *mut mlx4_dev, mailbox: *mut mlx4_cmd_mailbox);
    pub fn mlx4_get_counter_stats(dev: *mut mlx4_dev, counter_index: i32,
                                  counter_stats: *mut mlx4_counter, reset: i32) -> i32;
    pub fn mlx4_get_vf_stats(dev: *mut mlx4_dev, port: i32, vf_idx: i32,
                             vf_stats: *mut ifla_vf_stats) -> i32;
    pub fn mlx4_comm_get_version() -> u32;
    pub fn mlx4_set_vf_mac(dev: *mut mlx4_dev, port: i32, vf: i32, mac: *mut u8) -> i32;
    pub fn mlx4_set_vf_vlan(dev: *mut mlx4_dev, port: i32, vf: i32, vlan: u16,
                            qos: u8, proto: __be16) -> i32;
    pub fn mlx4_set_vf_rate(dev: *mut mlx4_dev, port: i32, vf: i32,
                            min_tx_rate: i32, max_tx_rate: i32) -> i32;
    pub fn mlx4_set_vf_spoofchk(dev: *mut mlx4_dev, port: i32, vf: i32, setting: bool) -> i32;
    pub fn mlx4_get_vf_config(dev: *mut mlx4_dev, port: i32, vf: i32,
                              ivf: *mut ifla_vf_info) -> i32;
    pub fn mlx4_set_vf_link_state(dev: *mut mlx4_dev, port: i32, vf: i32,
                                  link_state: i32) -> i32;
    pub fn mlx4_config_dev_retrieval(dev: *mut mlx4_dev,
                                     params: *mut mlx4_config_dev_params) -> i32;
    pub fn mlx4_cmd_wake_completions(dev: *mut mlx4_dev);
    pub fn mlx4_report_internal_err_comm_event(dev: *mut mlx4_dev);
    pub fn mlx4_get_slave_default_vlan(dev: *mut mlx4_dev, port: i32, slave: i32,
                                       vlan: *mut u16, qos: *mut u8) -> bool;
}

// External types supplied by included kernel headers.
extern "C" {
    type dma_addr_t;
    type mlx4_counter;
    type ifla_vf_stats;
    type ifla_vf_info;
    type __be16;
}

#[inline]
pub const fn MLX4_COMM_GET_IF_REV(cmd_chan_ver: u16) -> u8 {
    (cmd_chan_ver >> 8) as u8
}

pub const COMM_CHAN_EVENT_INTERNAL_ERR: u32 = 1 << 17;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
