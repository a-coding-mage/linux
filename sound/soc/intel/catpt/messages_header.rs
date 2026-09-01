/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2020 Intel Corporation
 *
 * Author: Cezary Rojewski <cezary.rojewski@intel.com>
 */

/* Translated from C header: soc/intel/catpt/messages.h */

use core::ffi::c_char;

pub enum catpt_dev {}
pub enum resource {}

/* IPC messages base types  */

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_reply_status {
    CATPT_REPLY_SUCCESS = 0,
    CATPT_REPLY_ERROR_INVALID_PARAM = 1,
    CATPT_REPLY_UNKNOWN_MESSAGE_TYPE = 2,
    CATPT_REPLY_OUT_OF_RESOURCES = 3,
    CATPT_REPLY_BUSY = 4,
    CATPT_REPLY_PENDING = 5,
    CATPT_REPLY_FAILURE = 6,
    CATPT_REPLY_INVALID_REQUEST = 7,
    CATPT_REPLY_UNINITIALIZED = 8,
    CATPT_REPLY_NOT_FOUND = 9,
    CATPT_REPLY_SOURCE_NOT_STARTED = 10,
}

/* GLOBAL messages */

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_global_msg_type {
    CATPT_GLB_GET_FW_VERSION = 0,
    CATPT_GLB_ALLOCATE_STREAM = 3,
    CATPT_GLB_FREE_STREAM = 4,
    CATPT_GLB_STREAM_MESSAGE = 6,
    CATPT_GLB_REQUEST_CORE_DUMP = 7,
    CATPT_GLB_SET_DEVICE_FORMATS = 10,
    CATPT_GLB_ENTER_DX_STATE = 12,
    CATPT_GLB_GET_MIXER_STREAM_INFO = 13,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union catpt_global_msg {
    pub val: u32,
}

pub const CATPT_GLOBAL_MSG_STATUS_SHIFT: u32 = 0;
pub const CATPT_GLOBAL_MSG_STATUS_MASK: u32 = 0x1f << CATPT_GLOBAL_MSG_STATUS_SHIFT;
pub const CATPT_GLOBAL_MSG_CONTEXT_SHIFT: u32 = 5;
pub const CATPT_GLOBAL_MSG_CONTEXT_MASK: u32 = 0x7ffff << CATPT_GLOBAL_MSG_CONTEXT_SHIFT;
pub const CATPT_GLOBAL_MSG_GLOBAL_MSG_TYPE_SHIFT: u32 = 24;
pub const CATPT_GLOBAL_MSG_GLOBAL_MSG_TYPE_MASK: u32 = 0x1f << CATPT_GLOBAL_MSG_GLOBAL_MSG_TYPE_SHIFT;
pub const CATPT_GLOBAL_MSG_FW_READY_SHIFT: u32 = 29;
pub const CATPT_GLOBAL_MSG_FW_READY_MASK: u32 = 0x1 << CATPT_GLOBAL_MSG_FW_READY_SHIFT;
pub const CATPT_GLOBAL_MSG_DONE_SHIFT: u32 = 30;
pub const CATPT_GLOBAL_MSG_DONE_MASK: u32 = 0x1 << CATPT_GLOBAL_MSG_DONE_SHIFT;
pub const CATPT_GLOBAL_MSG_BUSY_SHIFT: u32 = 31;
pub const CATPT_GLOBAL_MSG_BUSY_MASK: u32 = 0x1 << CATPT_GLOBAL_MSG_BUSY_SHIFT;

impl catpt_global_msg {
    pub const fn CATPT_MSG(hdr: u32) -> Self {
        Self { val: hdr }
    }

    pub const fn CATPT_GLOBAL_MSG(global_msg_type: catpt_global_msg_type) -> Self {
        Self {
            val: (global_msg_type as u32) << CATPT_GLOBAL_MSG_GLOBAL_MSG_TYPE_SHIFT,
        }
    }
}

pub const BUILD_HASH_SIZE: usize = 40;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_fw_version {
    pub build: u8,
    pub minor: u8,
    pub major: u8,
    pub type_: u8,
    pub build_hash: [u8; BUILD_HASH_SIZE],
    pub log_providers_hash: u32,
}

unsafe extern "C" {
    pub fn catpt_ipc_get_fw_version(
        cdev: *mut catpt_dev,
        version: *mut catpt_fw_version,
    ) -> i32;
}

/* PIN_IDs represent both, individual streams and the general mixer. */
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_pin_id {
    CATPT_PIN_ID_SYSTEM = 0,
    CATPT_PIN_ID_REFERENCE = 1,
    CATPT_PIN_ID_CAPTURE1 = 2,
    CATPT_PIN_ID_CAPTURE2 = 3,
    CATPT_PIN_ID_OFFLOAD1 = 4,
    CATPT_PIN_ID_OFFLOAD2 = 5,
    CATPT_PIN_ID_MIXER = 7,
    CATPT_PIN_ID_BLUETOOTH_CAPTURE = 8,
    CATPT_PIN_ID_BLUETOOTH_RENDER = 9,
    /* 10 is reserved */
    CATPT_PIN_ID_INVALID = 11,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_path_id {
    CATPT_PATH_SSP0_OUT = 0,
    CATPT_PATH_SSP0_IN = 1,
    CATPT_PATH_SSP1_OUT = 2,
    CATPT_PATH_SSP1_IN = 3,
    /* duplicated audio in capture path */
    CATPT_PATH_SSP0_IN_DUP = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_stream_type {
    CATPT_STRM_TYPE_RENDER = 0, /* offload */
    CATPT_STRM_TYPE_SYSTEM = 1,
    CATPT_STRM_TYPE_CAPTURE = 2,
    CATPT_STRM_TYPE_LOOPBACK = 3,
    CATPT_STRM_TYPE_BLUETOOTH_RENDER = 4,
    CATPT_STRM_TYPE_BLUETOOTH_CAPTURE = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_format_id {
    CATPT_FORMAT_PCM = 0,
    CATPT_FORMAT_MP3 = 1,
    CATPT_FORMAT_AAC = 2,
    CATPT_FORMAT_WMA = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_channel_index {
    CATPT_CHANNEL_LEFT = 0x0,
    CATPT_CHANNEL_CENTER = 0x1,
    CATPT_CHANNEL_RIGHT = 0x2,
    CATPT_CHANNEL_LEFT_SURROUND = 0x3,
    CATPT_CHANNEL_CENTER_SURROUND = 0x3,
    CATPT_CHANNEL_RIGHT_SURROUND = 0x4,
    CATPT_CHANNEL_LFE = 0x7,
    CATPT_CHANNEL_INVALID = 0xF,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_channel_config {
    CATPT_CHANNEL_CONFIG_MONO = 0,      /* One channel only */
    CATPT_CHANNEL_CONFIG_STEREO = 1,    /* L & R */
    CATPT_CHANNEL_CONFIG_2_POINT_1 = 2, /* L, R & LFE; PCM only */
    CATPT_CHANNEL_CONFIG_3_POINT_0 = 3, /* L, C & R; MP3 & AAC only */
    CATPT_CHANNEL_CONFIG_3_POINT_1 = 4, /* L, C, R & LFE; PCM only */
    CATPT_CHANNEL_CONFIG_QUATRO = 5,    /* L, R, Ls & Rs; PCM only */
    CATPT_CHANNEL_CONFIG_4_POINT_0 = 6, /* L, C, R & Cs; MP3 & AAC only */
    CATPT_CHANNEL_CONFIG_5_POINT_0 = 7, /* L, C, R, Ls & Rs */
    CATPT_CHANNEL_CONFIG_5_POINT_1 = 8, /* L, C, R, Ls, Rs & LFE */
    CATPT_CHANNEL_CONFIG_DUAL_MONO = 9, /* One channel replicated in two */
    CATPT_CHANNEL_CONFIG_INVALID = 10,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_interleaving_style {
    CATPT_INTERLEAVING_PER_CHANNEL = 0,
    CATPT_INTERLEAVING_PER_SAMPLE = 1,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_audio_format {
    pub sample_rate: u32,
    pub bit_depth: u32,
    pub channel_map: u32,
    pub channel_config: u32,
    pub interleaving: u32,
    pub num_channels: u8,
    pub valid_bit_depth: u8,
    pub reserved: [u8; 2],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_ring_info {
    pub page_table_addr: u32,
    pub num_pages: u32,
    pub size: u32,
    pub offset: u32,
    pub ring_first_page_pfn: u32,
}

pub const CATPT_MODULE_COUNT: u32 = catpt_module_id::CATPT_MODID_LAST as u32 + 1;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_module_id {
    CATPT_MODID_BASE_FW = 0x0,
    CATPT_MODID_MP3 = 0x1,
    CATPT_MODID_AAC_5_1 = 0x2,
    CATPT_MODID_AAC_2_0 = 0x3,
    CATPT_MODID_SRC = 0x4,
    CATPT_MODID_WAVES = 0x5,
    CATPT_MODID_DOLBY = 0x6,
    CATPT_MODID_BOOST = 0x7,
    CATPT_MODID_LPAL = 0x8,
    CATPT_MODID_DTS = 0x9,
    CATPT_MODID_PCM_CAPTURE = 0xA,
    CATPT_MODID_PCM_SYSTEM = 0xB,
    CATPT_MODID_PCM_REFERENCE = 0xC,
    CATPT_MODID_PCM = 0xD, /* offload */
    CATPT_MODID_BLUETOOTH_RENDER = 0xE,
    CATPT_MODID_BLUETOOTH_CAPTURE = 0xF,
    CATPT_MODID_LAST = 0xF,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_module_entry {
    pub module_id: u32,
    pub entry_point: u32,
}

#[repr(C, packed)]
pub struct catpt_module_map {
    pub num_entries: u8,
    pub entries: [catpt_module_entry; 0],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_memory_info {
    pub offset: u32,
    pub size: u32,
}

pub const CATPT_CHANNELS_MAX: usize = 4;
pub const CATPT_ALL_CHANNELS_MASK: u32 = u32::MAX;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_stream_info {
    pub stream_hw_id: u32,
    pub reserved: u32,
    pub read_pos_regaddr: u32,
    pub pres_pos_regaddr: u32,
    pub peak_meter_regaddr: [u32; CATPT_CHANNELS_MAX],
    pub volume_regaddr: [u32; CATPT_CHANNELS_MAX],
}

unsafe extern "C" {
    pub fn catpt_ipc_alloc_stream(
        cdev: *mut catpt_dev,
        path_id: catpt_path_id,
        type_: catpt_stream_type,
        afmt: *mut catpt_audio_format,
        rinfo: *mut catpt_ring_info,
        num_modules: u8,
        modules: *mut catpt_module_entry,
        persistent: *mut resource,
        scratch: *mut resource,
        sinfo: *mut catpt_stream_info,
    ) -> i32;
    pub fn catpt_ipc_free_stream(cdev: *mut catpt_dev, stream_hw_id: u8) -> i32;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_ssp_iface {
    CATPT_SSP_IFACE_0 = 0,
    CATPT_SSP_IFACE_1 = 1,
    CATPT_SSP_COUNT = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_mclk_frequency {
    CATPT_MCLK_OFF = 0,
    CATPT_MCLK_FREQ_6_MHZ = 1,
    CATPT_MCLK_FREQ_21_MHZ = 2,
    CATPT_MCLK_FREQ_24_MHZ = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_ssp_mode {
    CATPT_SSP_MODE_I2S_CONSUMER = 0,
    CATPT_SSP_MODE_I2S_PROVIDER = 1,
    CATPT_SSP_MODE_TDM_PROVIDER = 2,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_ssp_device_format {
    pub iface: u32,
    pub mclk: u32,
    pub mode: u32,
    pub clock_divider: u16,
    pub channels: u8,
}

unsafe extern "C" {
    pub fn catpt_ipc_set_device_format(
        cdev: *mut catpt_dev,
        devfmt: *mut catpt_ssp_device_format,
    ) -> i32;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_dx_state {
    CATPT_DX_STATE_D3 = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_dx_type {
    CATPT_DX_TYPE_FW_IMAGE = 0,
    CATPT_DX_TYPE_MEMORY_DUMP = 1,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_save_meminfo {
    pub offset: u32,
    pub size: u32,
    pub source: u32,
}

pub const SAVE_MEMINFO_MAX: usize = 14;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_dx_context {
    pub num_meminfo: u32,
    pub meminfo: [catpt_save_meminfo; SAVE_MEMINFO_MAX],
}

unsafe extern "C" {
    pub fn catpt_ipc_enter_dxstate(
        cdev: *mut catpt_dev,
        state: catpt_dx_state,
        context: *mut catpt_dx_context,
    ) -> i32;
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_mixer_stream_info {
    pub mixer_hw_id: u32,
    pub peak_meter_regaddr: [u32; CATPT_CHANNELS_MAX],
    pub volume_regaddr: [u32; CATPT_CHANNELS_MAX],
}

unsafe extern "C" {
    pub fn catpt_ipc_get_mixer_stream_info(
        cdev: *mut catpt_dev,
        info: *mut catpt_mixer_stream_info,
    ) -> i32;
}

/* STREAM messages */

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_stream_msg_type {
    CATPT_STRM_RESET_STREAM = 0,
    CATPT_STRM_PAUSE_STREAM = 1,
    CATPT_STRM_RESUME_STREAM = 2,
    CATPT_STRM_STAGE_MESSAGE = 3,
    CATPT_STRM_NOTIFICATION = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_stage_action {
    CATPT_STG_SET_VOLUME = 1,
    CATPT_STG_SET_WRITE_POSITION = 2,
    CATPT_STG_MUTE_LOOPBACK = 3,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union catpt_stream_msg {
    pub val: u32,
}

pub const CATPT_STREAM_MSG_STATUS_SHIFT: u32 = 0;
pub const CATPT_STREAM_MSG_STATUS_MASK: u32 = 0x1f << CATPT_STREAM_MSG_STATUS_SHIFT;
pub const CATPT_STREAM_MSG_RESERVED_SHIFT: u32 = 5;
pub const CATPT_STREAM_MSG_RESERVED_MASK: u32 = 0x7f << CATPT_STREAM_MSG_RESERVED_SHIFT;
pub const CATPT_STREAM_MSG_STAGE_ACTION_SHIFT: u32 = 12;
pub const CATPT_STREAM_MSG_STAGE_ACTION_MASK: u32 = 0xf << CATPT_STREAM_MSG_STAGE_ACTION_SHIFT;
pub const CATPT_STREAM_MSG_STREAM_HW_ID_SHIFT: u32 = 16;
pub const CATPT_STREAM_MSG_STREAM_HW_ID_MASK: u32 = 0xf << CATPT_STREAM_MSG_STREAM_HW_ID_SHIFT;
pub const CATPT_STREAM_MSG_STREAM_MSG_TYPE_SHIFT: u32 = 20;
pub const CATPT_STREAM_MSG_STREAM_MSG_TYPE_MASK: u32 = 0xf << CATPT_STREAM_MSG_STREAM_MSG_TYPE_SHIFT;
pub const CATPT_STREAM_MSG_GLOBAL_MSG_TYPE_SHIFT: u32 = 24;
pub const CATPT_STREAM_MSG_GLOBAL_MSG_TYPE_MASK: u32 = 0x1f << CATPT_STREAM_MSG_GLOBAL_MSG_TYPE_SHIFT;
pub const CATPT_STREAM_MSG_FW_READY_SHIFT: u32 = 29;
pub const CATPT_STREAM_MSG_FW_READY_MASK: u32 = 0x1 << CATPT_STREAM_MSG_FW_READY_SHIFT;
pub const CATPT_STREAM_MSG_DONE_SHIFT: u32 = 30;
pub const CATPT_STREAM_MSG_DONE_MASK: u32 = 0x1 << CATPT_STREAM_MSG_DONE_SHIFT;
pub const CATPT_STREAM_MSG_BUSY_SHIFT: u32 = 31;
pub const CATPT_STREAM_MSG_BUSY_MASK: u32 = 0x1 << CATPT_STREAM_MSG_BUSY_SHIFT;

impl catpt_stream_msg {
    pub const fn CATPT_STREAM_MSG(stream_msg_type: catpt_stream_msg_type) -> Self {
        Self {
            val: ((stream_msg_type as u32) << CATPT_STREAM_MSG_STREAM_MSG_TYPE_SHIFT)
                | ((catpt_global_msg_type::CATPT_GLB_STREAM_MESSAGE as u32)
                    << CATPT_STREAM_MSG_GLOBAL_MSG_TYPE_SHIFT),
        }
    }

    pub const fn CATPT_STAGE_MSG(stage_action: catpt_stage_action) -> Self {
        Self {
            val: ((stage_action as u32) << CATPT_STREAM_MSG_STAGE_ACTION_SHIFT)
                | ((catpt_stream_msg_type::CATPT_STRM_STAGE_MESSAGE as u32)
                    << CATPT_STREAM_MSG_STREAM_MSG_TYPE_SHIFT)
                | ((catpt_global_msg_type::CATPT_GLB_STREAM_MESSAGE as u32)
                    << CATPT_STREAM_MSG_GLOBAL_MSG_TYPE_SHIFT),
        }
    }
}

unsafe extern "C" {
    pub fn catpt_ipc_reset_stream(cdev: *mut catpt_dev, stream_hw_id: u8) -> i32;
    pub fn catpt_ipc_pause_stream(cdev: *mut catpt_dev, stream_hw_id: u8) -> i32;
    pub fn catpt_ipc_resume_stream(cdev: *mut catpt_dev, stream_hw_id: u8) -> i32;
}

/* STREAM messages - STAGE subtype */

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_audio_curve_type {
    CATPT_AUDIO_CURVE_NONE = 0,
    CATPT_AUDIO_CURVE_WINDOWS_FADE = 1,
}

unsafe extern "C" {
    pub fn catpt_ipc_set_volume(
        cdev: *mut catpt_dev,
        stream_hw_id: u8,
        channel: u32,
        volume: u32,
        curve_duration: u32,
        curve_type: catpt_audio_curve_type,
    ) -> i32;

    pub fn catpt_ipc_set_write_pos(
        cdev: *mut catpt_dev,
        stream_hw_id: u8,
        pos: u32,
        eob: bool,
        ll: bool,
    ) -> i32;

    pub fn catpt_ipc_mute_loopback(cdev: *mut catpt_dev, stream_hw_id: u8, mute: bool) -> i32;
}

/* NOTIFICATION messages */

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_notify_reason {
    CATPT_NOTIFY_POSITION_CHANGED = 0,
    CATPT_NOTIFY_GLITCH_OCCURRED = 1,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union catpt_notify_msg {
    pub val: u32,
}

pub const CATPT_NOTIFY_MSG_MAILBOX_ADDRESS_SHIFT: u32 = 0;
pub const CATPT_NOTIFY_MSG_MAILBOX_ADDRESS_MASK: u32 = 0x1fffffff << CATPT_NOTIFY_MSG_MAILBOX_ADDRESS_SHIFT;
pub const CATPT_NOTIFY_MSG_FW_READY_SHIFT: u32 = 29;
pub const CATPT_NOTIFY_MSG_FW_READY_MASK: u32 = 0x1 << CATPT_NOTIFY_MSG_FW_READY_SHIFT;
pub const CATPT_NOTIFY_MSG_DONE_SHIFT: u32 = 30;
pub const CATPT_NOTIFY_MSG_DONE_MASK: u32 = 0x1 << CATPT_NOTIFY_MSG_DONE_SHIFT;
pub const CATPT_NOTIFY_MSG_BUSY_SHIFT: u32 = 31;
pub const CATPT_NOTIFY_MSG_BUSY_MASK: u32 = 0x1 << CATPT_NOTIFY_MSG_BUSY_SHIFT;
pub const CATPT_NOTIFY_MSG_STATUS_SHIFT: u32 = 0;
pub const CATPT_NOTIFY_MSG_STATUS_MASK: u32 = 0x1f << CATPT_NOTIFY_MSG_STATUS_SHIFT;
pub const CATPT_NOTIFY_MSG_RESERVED_SHIFT: u32 = 5;
pub const CATPT_NOTIFY_MSG_RESERVED_MASK: u32 = 0x7f << CATPT_NOTIFY_MSG_RESERVED_SHIFT;
pub const CATPT_NOTIFY_MSG_NOTIFY_REASON_SHIFT: u32 = 12;
pub const CATPT_NOTIFY_MSG_NOTIFY_REASON_MASK: u32 = 0xf << CATPT_NOTIFY_MSG_NOTIFY_REASON_SHIFT;
pub const CATPT_NOTIFY_MSG_STREAM_HW_ID_SHIFT: u32 = 16;
pub const CATPT_NOTIFY_MSG_STREAM_HW_ID_MASK: u32 = 0xf << CATPT_NOTIFY_MSG_STREAM_HW_ID_SHIFT;
pub const CATPT_NOTIFY_MSG_STREAM_MSG_TYPE_SHIFT: u32 = 20;
pub const CATPT_NOTIFY_MSG_STREAM_MSG_TYPE_MASK: u32 = 0xf << CATPT_NOTIFY_MSG_STREAM_MSG_TYPE_SHIFT;
pub const CATPT_NOTIFY_MSG_GLOBAL_MSG_TYPE_SHIFT: u32 = 24;
pub const CATPT_NOTIFY_MSG_GLOBAL_MSG_TYPE_MASK: u32 = 0x1f << CATPT_NOTIFY_MSG_GLOBAL_MSG_TYPE_SHIFT;
pub const CATPT_NOTIFY_MSG_HDR_SHIFT: u32 = 29;
pub const CATPT_NOTIFY_MSG_HDR_MASK: u32 = 0x7 << CATPT_NOTIFY_MSG_HDR_SHIFT;

pub const FW_INFO_SIZE_MAX: usize = 100;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_fw_ready {
    pub inbox_offset: u32,
    pub outbox_offset: u32,
    pub inbox_size: u32,
    pub outbox_size: u32,
    pub fw_info_size: u32,
    pub fw_info: [c_char; FW_INFO_SIZE_MAX],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_notify_position {
    pub stream_position: u32,
    pub fw_cycle_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum catpt_glitch_type {
    CATPT_GLITCH_UNDERRUN = 1,
    CATPT_GLITCH_DECODER_ERROR = 2,
    CATPT_GLITCH_DOUBLED_WRITE_POS = 3,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct catpt_notify_glitch {
    pub type_: u32,
    pub presentation_pos: u64,
    pub write_pos: u32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
