/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * CAN driver for PEAK System micro-CAN based adapters
 *
 * Copyright (C) 2003-2025 PEAK System-Technik GmbH
 * Author: Stéphane Grosjean <s.grosjean@peak-system.fr>
 */

/* uCAN commands opcodes list (low-order 10 bits) */
pub const PUCAN_CMD_NOP: u16 = 0x000;
pub const PUCAN_CMD_RESET_MODE: u16 = 0x001;
pub const PUCAN_CMD_NORMAL_MODE: u16 = 0x002;
pub const PUCAN_CMD_LISTEN_ONLY_MODE: u16 = 0x003;
pub const PUCAN_CMD_TIMING_SLOW: u16 = 0x004;
pub const PUCAN_CMD_TIMING_FAST: u16 = 0x005;
pub const PUCAN_CMD_SET_STD_FILTER: u16 = 0x006;
pub const PUCAN_CMD_RESERVED2: u16 = 0x007;
pub const PUCAN_CMD_FILTER_STD: u16 = 0x008;
pub const PUCAN_CMD_TX_ABORT: u16 = 0x009;
pub const PUCAN_CMD_WR_ERR_CNT: u16 = 0x00a;
pub const PUCAN_CMD_SET_EN_OPTION: u16 = 0x00b;
pub const PUCAN_CMD_CLR_DIS_OPTION: u16 = 0x00c;
pub const PUCAN_CMD_RX_BARRIER: u16 = 0x010;
pub const PUCAN_CMD_END_OF_COLLECTION: u16 = 0x3ff;

/* uCAN received messages list */
pub const PUCAN_MSG_CAN_RX: u16 = 0x0001;
pub const PUCAN_MSG_ERROR: u16 = 0x0002;
pub const PUCAN_MSG_STATUS: u16 = 0x0003;
pub const PUCAN_MSG_BUSLOAD: u16 = 0x0004;
pub const PUCAN_MSG_CACHE_CRITICAL: u16 = 0x0102;

/* uCAN transmitted messages */
pub const PUCAN_MSG_CAN_TX: u16 = 0x1000;

/* uCAN command common header */
#[repr(C, packed)]
pub struct pucan_command { pub opcode_channel: u16, pub args: [u16; 3] }

/* return the opcode from the opcode_channel field of a command */
#[inline]
pub fn pucan_cmd_get_opcode(c: *mut pucan_command) -> u16 { unsafe { u16::from_le((*c).opcode_channel) & 0x3ff } }

pub const PUCAN_TSLOW_BRP_BITS: u32 = 10;
pub const PUCAN_TSLOW_TSGEG1_BITS: u32 = 8;
pub const PUCAN_TSLOW_TSGEG2_BITS: u32 = 7;
pub const PUCAN_TSLOW_SJW_BITS: u32 = 7;
pub const PUCAN_TSLOW_BRP_MASK: u16 = (1u16 << PUCAN_TSLOW_BRP_BITS) - 1;
pub const PUCAN_TSLOW_TSEG1_MASK: u8 = (1u8 << PUCAN_TSLOW_TSGEG1_BITS) - 1;
pub const PUCAN_TSLOW_TSEG2_MASK: u8 = (1u8 << PUCAN_TSLOW_TSGEG2_BITS) - 1;
pub const PUCAN_TSLOW_SJW_MASK: u8 = (1u8 << PUCAN_TSLOW_SJW_BITS) - 1;

#[inline] pub const fn PUCAN_TSLOW_SJW_T(s: u8, t: bool) -> u8 { (s & PUCAN_TSLOW_SJW_MASK) | ((t as u8) << 7) }
#[inline] pub const fn PUCAN_TSLOW_TSEG2(t: u8) -> u8 { t & PUCAN_TSLOW_TSEG2_MASK }
#[inline] pub const fn PUCAN_TSLOW_TSEG1(t: u8) -> u8 { t & PUCAN_TSLOW_TSEG1_MASK }
#[inline] pub const fn PUCAN_TSLOW_BRP(b: u16) -> u16 { b & PUCAN_TSLOW_BRP_MASK }

#[repr(C, packed)]
pub struct pucan_timing_slow { pub opcode_channel: u16, pub ewl: u8, pub sjw_t: u8, pub tseg2: u8, pub tseg1: u8, pub brp: u16 }

pub const PUCAN_TFAST_BRP_BITS: u32 = 10;
pub const PUCAN_TFAST_TSGEG1_BITS: u32 = 5;
pub const PUCAN_TFAST_TSGEG2_BITS: u32 = 4;
pub const PUCAN_TFAST_SJW_BITS: u32 = 4;
pub const PUCAN_TFAST_BRP_MASK: u16 = (1u16 << PUCAN_TFAST_BRP_BITS) - 1;
pub const PUCAN_TFAST_TSEG1_MASK: u8 = (1u8 << PUCAN_TFAST_TSGEG1_BITS) - 1;
pub const PUCAN_TFAST_TSEG2_MASK: u8 = (1u8 << PUCAN_TFAST_TSGEG2_BITS) - 1;
pub const PUCAN_TFAST_SJW_MASK: u8 = (1u8 << PUCAN_TFAST_SJW_BITS) - 1;
#[inline] pub const fn PUCAN_TFAST_SJW(s: u8) -> u8 { s & PUCAN_TFAST_SJW_MASK }
#[inline] pub const fn PUCAN_TFAST_TSEG2(t: u8) -> u8 { t & PUCAN_TFAST_TSEG2_MASK }
#[inline] pub const fn PUCAN_TFAST_TSEG1(t: u8) -> u8 { t & PUCAN_TFAST_TSEG1_MASK }
#[inline] pub const fn PUCAN_TFAST_BRP(b: u16) -> u16 { b & PUCAN_TFAST_BRP_MASK }

#[repr(C, packed)]
pub struct pucan_timing_fast { pub opcode_channel: u16, pub unused: u8, pub sjw: u8, pub tseg2: u8, pub tseg1: u8, pub brp: u16 }

/* uCAN FILTER_STD command fields */
pub const PUCAN_FLTSTD_ROW_IDX_BITS: u32 = 6;
#[repr(C, packed)] pub struct pucan_filter_std { pub opcode_channel: u16, pub idx: u16, pub mask: u32 }
pub const PUCAN_FLTSTD_ROW_IDX_MAX: u16 = (1u16 << PUCAN_FLTSTD_ROW_IDX_BITS) - 1;
/* uCAN SET_STD_FILTER command fields */
#[repr(C, packed)] pub struct pucan_std_filter { pub opcode_channel: u16, pub unused: u8, pub idx: u8, pub mask: u32 }

/* uCAN TX_ABORT commands fields */
pub const PUCAN_TX_ABORT_FLUSH: u16 = 0x0001;
#[repr(C, packed)] pub struct pucan_tx_abort { pub opcode_channel: u16, pub flags: u16, pub unused: u32 }
/* uCAN WR_ERR_CNT command fields */
pub const PUCAN_WRERRCNT_TE: u16 = 0x4000;
pub const PUCAN_WRERRCNT_RE: u16 = 0x8000;
#[repr(C, packed)] pub struct pucan_wr_err_cnt { pub opcode_channel: u16, pub sel_mask: u16, pub tx_counter: u8, pub rx_counter: u8, pub unused: u16 }
/* uCAN SET_EN/CLR_DIS _OPTION command fields */
pub const PUCAN_OPTION_ERROR: u16 = 0x0001;
pub const PUCAN_OPTION_BUSLOAD: u16 = 0x0002;
pub const PUCAN_OPTION_CANDFDISO: u16 = 0x0004;
#[repr(C, packed)] pub struct pucan_options { pub opcode_channel: u16, pub options: u16, pub unused: u32 }

/* uCAN received messages global format */
#[repr(C, packed)] pub struct pucan_msg { pub size: u16, pub type_: u16, pub ts_low: u32, pub ts_high: u32 }
/* uCAN flags for CAN/CANFD messages */
pub const PUCAN_MSG_SELF_RECEIVE: u16 = 0x80;
pub const PUCAN_MSG_ERROR_STATE_IND: u16 = 0x40;
pub const PUCAN_MSG_BITRATE_SWITCH: u16 = 0x20;
pub const PUCAN_MSG_EXT_DATA_LEN: u16 = 0x10;
pub const PUCAN_MSG_SINGLE_SHOT: u16 = 0x08;
pub const PUCAN_MSG_LOOPED_BACK: u16 = 0x04;
pub const PUCAN_MSG_EXT_ID: u16 = 0x02;
pub const PUCAN_MSG_RTR: u16 = 0x01;
#[repr(C, packed)] pub struct pucan_rx_msg { pub size: u16, pub type_: u16, pub ts_low: u32, pub ts_high: u32, pub tag_low: u32, pub tag_high: u32, pub channel_dlc: u8, pub client: u8, pub flags: u16, pub can_id: u32, pub d: [u8; 0] }

/* uCAN error types */
pub const PUCAN_ERMSG_BIT_ERROR: u8 = 0; pub const PUCAN_ERMSG_FORM_ERROR: u8 = 1; pub const PUCAN_ERMSG_STUFF_ERROR: u8 = 2; pub const PUCAN_ERMSG_OTHER_ERROR: u8 = 3; pub const PUCAN_ERMSG_ERR_CNT_DEC: u8 = 4;
#[repr(C, packed)] pub struct pucan_error_msg { pub size: u16, pub type_: u16, pub ts_low: u32, pub ts_high: u32, pub channel_type_d: u8, pub code_g: u8, pub tx_err_cnt: u8, pub rx_err_cnt: u8 }
#[inline] pub fn pucan_error_get_channel(msg: *const pucan_error_msg) -> i32 { unsafe { ((*msg).channel_type_d & 0x0f) as i32 } }

pub const PUCAN_RX_BARRIER: u8 = 0x10; pub const PUCAN_BUS_PASSIVE: u8 = 0x20; pub const PUCAN_BUS_WARNING: u8 = 0x40; pub const PUCAN_BUS_BUSOFF: u8 = 0x80;
#[repr(C, packed)] pub struct pucan_status_msg { pub size: u16, pub type_: u16, pub ts_low: u32, pub ts_high: u32, pub channel_p_w_b: u8, pub unused: [u8; 3] }
#[inline] pub fn pucan_status_get_channel(msg: *const pucan_status_msg) -> i32 { unsafe { ((*msg).channel_p_w_b & 0x0f) as i32 } }
#[inline] pub fn pucan_status_is_rx_barrier(msg: *const pucan_status_msg) -> i32 { unsafe { ((*msg).channel_p_w_b & PUCAN_RX_BARRIER) as i32 } }
#[inline] pub fn pucan_status_is_passive(msg: *const pucan_status_msg) -> i32 { unsafe { ((*msg).channel_p_w_b & PUCAN_BUS_PASSIVE) as i32 } }
#[inline] pub fn pucan_status_is_warning(msg: *const pucan_status_msg) -> i32 { unsafe { ((*msg).channel_p_w_b & PUCAN_BUS_WARNING) as i32 } }
#[inline] pub fn pucan_status_is_busoff(msg: *const pucan_status_msg) -> i32 { unsafe { ((*msg).channel_p_w_b & PUCAN_BUS_BUSOFF) as i32 } }

/* uCAN transmitted message format */
#[inline] pub const fn PUCAN_MSG_CHANNEL_DLC(c: u8, d: u8) -> u8 { (c & 0xf) | (d << 4) }
#[repr(C, packed)] pub struct pucan_tx_msg { pub size: u16, pub type_: u16, pub tag_low: u32, pub tag_high: u32, pub channel_dlc: u8, pub client: u8, pub flags: u16, pub can_id: u32, pub d: [u8; 0] }
#[inline] pub fn pucan_cmd_opcode_channel(index: i32, opcode: i32) -> u16 { ((((index << 12) | (opcode & 0x3ff)) as u16)).to_le() }
#[inline] pub fn pucan_msg_get_channel(msg: *const pucan_rx_msg) -> i32 { unsafe { ( (*msg).channel_dlc & 0xf) as i32 } }
#[inline] pub fn pucan_msg_get_dlc(msg: *const pucan_rx_msg) -> u8 { unsafe { (*msg).channel_dlc >> 4 } }
#[inline] pub fn pucan_ermsg_get_channel(msg: *const pucan_error_msg) -> i32 { unsafe { ((*msg).channel_type_d & 0x0f) as i32 } }
#[inline] pub fn pucan_stmsg_get_channel(msg: *const pucan_status_msg) -> i32 { unsafe { ((*msg).channel_p_w_b & 0x0f) as i32 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
