/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

// Dependencies supplied by uapi/sound/sof/header.h and sound/sof/header.h.

/* Component Mixers and Controls */

/* channel positions - uses same values as ALSA */
#[repr(u32)]
pub enum sof_ipc_chmap {
    SOF_CHMAP_UNKNOWN = 0,
    SOF_CHMAP_NA,
    SOF_CHMAP_MONO,
    SOF_CHMAP_FL,
    SOF_CHMAP_FR,
    SOF_CHMAP_RL,
    SOF_CHMAP_RR,
    SOF_CHMAP_FC,
    SOF_CHMAP_LFE,
    SOF_CHMAP_SL,
    SOF_CHMAP_SR,
    SOF_CHMAP_RC,
    SOF_CHMAP_FLC,
    SOF_CHMAP_FRC,
    SOF_CHMAP_RLC,
    SOF_CHMAP_RRC,
    SOF_CHMAP_FLW,
    SOF_CHMAP_FRW,
    SOF_CHMAP_FLH,
    SOF_CHMAP_FCH,
    SOF_CHMAP_FRH,
    SOF_CHMAP_TC,
    SOF_CHMAP_TFL,
    SOF_CHMAP_TFR,
    SOF_CHMAP_TFC,
    SOF_CHMAP_TRL,
    SOF_CHMAP_TRR,
    SOF_CHMAP_TRC,
    SOF_CHMAP_TFLC,
    SOF_CHMAP_TFRC,
    SOF_CHMAP_TSL,
    SOF_CHMAP_TSR,
    SOF_CHMAP_LLFE,
    SOF_CHMAP_RLFE,
    SOF_CHMAP_BC,
    SOF_CHMAP_BLC,
    SOF_CHMAP_BRC,
}

pub const SOF_CHMAP_LAST: sof_ipc_chmap = sof_ipc_chmap::SOF_CHMAP_BRC;

/* control data type and direction */
#[repr(u32)]
pub enum sof_ipc_ctrl_type {
    SOF_CTRL_TYPE_VALUE_CHAN_GET = 0,
    SOF_CTRL_TYPE_VALUE_CHAN_SET,
    SOF_CTRL_TYPE_VALUE_COMP_GET,
    SOF_CTRL_TYPE_VALUE_COMP_SET,
    SOF_CTRL_TYPE_DATA_GET,
    SOF_CTRL_TYPE_DATA_SET,
}

/* control command type */
#[repr(u32)]
pub enum sof_ipc_ctrl_cmd {
    SOF_CTRL_CMD_VOLUME = 0,
    SOF_CTRL_CMD_ENUM,
    SOF_CTRL_CMD_SWITCH,
    SOF_CTRL_CMD_BINARY,
}

/* generic channel mapped value data */
#[repr(C, packed)]
pub struct sof_ipc_ctrl_value_chan {
    pub channel: u32,
    pub value: u32,
}

/* generic component mapped value data */
#[repr(C)]
pub union sof_ipc_ctrl_value_comp_value {
    pub uvalue: u32,
    pub svalue: i32,
}

#[repr(C, packed)]
pub struct sof_ipc_ctrl_value_comp {
    pub index: u32,
    pub value: sof_ipc_ctrl_value_comp_value,
}

/* generic control data */
#[repr(C)]
pub union sof_ipc_ctrl_data_values {
    pub chanv: [sof_ipc_ctrl_value_chan; 0],
    pub compv: [sof_ipc_ctrl_value_comp; 0],
    pub data: [sof_abi_hdr; 0],
}

#[repr(C, packed)]
pub struct sof_ipc_ctrl_data {
    pub rhdr: sof_ipc_reply,
    pub comp_id: u32,
    pub type_: u32,
    pub cmd: u32,
    pub index: u32,
    pub buffer: sof_ipc_host_buffer,
    pub num_elems: u32,
    pub elems_remaining: u32,
    pub msg_index: u32,
    pub reserved: [u32; 6],
    pub values: sof_ipc_ctrl_data_values,
}

/** Event type */
#[repr(u32)]
pub enum sof_ipc_ctrl_event_type {
    SOF_CTRL_EVENT_GENERIC = 0,
    SOF_CTRL_EVENT_GENERIC_METADATA,
    SOF_CTRL_EVENT_KD,
    SOF_CTRL_EVENT_VAD,
}

/** Generic notification data. */
#[repr(C)]
pub union sof_ipc_comp_event_data {
    pub data: [sof_abi_hdr; 0],
    pub event_value: u32,
}

#[repr(C, packed)]
pub struct sof_ipc_comp_event {
    pub rhdr: sof_ipc_reply,
    pub src_comp_type: u16,
    pub src_comp_id: u32,
    pub event_type: u32,
    pub num_elems: u32,
    pub reserved: [u32; 8],
    pub data: sof_ipc_comp_event_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
