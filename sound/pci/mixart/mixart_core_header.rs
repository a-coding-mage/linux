/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for Digigram miXart soundcards
 *
 * low level interface with interrupt handling and mail box implementation
 *
 * Copyright (c) 2003 by Digigram <alsa@digigram.com>
 */

use core::ffi::{c_int, c_void};
use core::mem::size_of;

#[repr(C)]
pub enum mixart_message_id {
    MSG_CONNECTOR_GET_AUDIO_INFO = 0x050008,
    MSG_CONNECTOR_GET_OUT_AUDIO_LEVEL = 0x050009,
    MSG_CONNECTOR_SET_OUT_AUDIO_LEVEL = 0x05000A,

    MSG_CONSOLE_MANAGER = 0x070000,
    MSG_CONSOLE_GET_CLOCK_UID = 0x070003,

    MSG_PHYSICALIO_SET_LEVEL = 0x0F0008,

    MSG_STREAM_ADD_INPUT_GROUP = 0x130000,
    MSG_STREAM_ADD_OUTPUT_GROUP = 0x130001,
    MSG_STREAM_DELETE_GROUP = 0x130004,
    MSG_STREAM_START_STREAM_GRP_PACKET = 0x130006,
    MSG_STREAM_START_INPUT_STAGE_PACKET = 0x130007,
    MSG_STREAM_START_OUTPUT_STAGE_PACKET = 0x130008,
    MSG_STREAM_STOP_STREAM_GRP_PACKET = 0x130009,
    MSG_STREAM_STOP_INPUT_STAGE_PACKET = 0x13000A,
    MSG_STREAM_STOP_OUTPUT_STAGE_PACKET = 0x13000B,
    MSG_STREAM_SET_INPUT_STAGE_PARAM = 0x13000F,
    MSG_STREAM_SET_OUTPUT_STAGE_PARAM = 0x130010,
    MSG_STREAM_SET_IN_AUDIO_LEVEL = 0x130015,
    MSG_STREAM_SET_OUT_STREAM_LEVEL = 0x130017,

    MSG_SYSTEM_FIRST_ID = 0x160000,
    MSG_SYSTEM_ENUM_PHYSICAL_IO = 0x16000E,
    MSG_SYSTEM_ENUM_PLAY_CONNECTOR = 0x160017,
    MSG_SYSTEM_ENUM_RECORD_CONNECTOR = 0x160018,
    MSG_SYSTEM_WAIT_SYNCHRO_CMD = 0x16002C,
    MSG_SYSTEM_SEND_SYNCHRO_CMD = 0x16002D,

    MSG_SERVICES_TIMER_NOTIFY = 0x1D0404,
    MSG_SERVICES_REPORT_TRACES = 0x1D0700,

    MSG_CLOCK_CHECK_PROPERTIES = 0x200001,
    MSG_CLOCK_SET_PROPERTIES = 0x200002,
}

pub const MSG_DEFAULT_SIZE: usize = 512;

#[repr(C)]
pub struct mixart_msg {
    pub message_id: u32,
    pub uid: mixart_uid,
    pub data: *mut c_void,
    pub size: usize,
}

/* structs used to communicate with miXart */

#[repr(C, packed)]
pub struct mixart_enum_connector_resp {
    pub error_code: u32,
    pub first_uid_offset: u32,
    pub uid_count: u32,
    pub current_uid_index: u32,
    pub uid: [mixart_uid; MIXART_MAX_PHYS_CONNECTORS],
}

/* used for following struct */
pub const MIXART_FLOAT_P_22_0_TO_HEX: u32 = 0x41b00000; /* 22.0f */
pub const MIXART_FLOAT_M_20_0_TO_HEX: u32 = 0xc1a00000; /* -20.0f */
pub const MIXART_FLOAT____0_0_TO_HEX: u32 = 0x00000000; /* 0.0f */

#[repr(C, packed)]
pub struct mixart_audio_info_req {
    pub line_max_level: u32,  /* float */
    pub micro_max_level: u32, /* float */
    pub cd_max_level: u32,    /* float */
}

#[repr(C, packed)]
pub struct mixart_analog_hw_info {
    pub is_present: u32,
    pub hw_connection_type: u32,
    pub max_level: u32,      /* float */
    pub min_var_level: u32,  /* float */
    pub max_var_level: u32,  /* float */
    pub step_var_level: u32, /* float */
    pub fix_gain: u32,       /* float */
    pub zero_var: u32,       /* float */
}

#[repr(C, packed)]
pub struct mixart_digital_hw_info {
    pub hw_connection_type: u32,
    pub presence: u32,
    pub clock: u32,
    pub reserved: u32,
}

#[repr(C, packed)]
pub struct mixart_analog_info {
    pub type_mask: u32,
    pub micro_info: mixart_analog_hw_info,
    pub line_info: mixart_analog_hw_info,
    pub cd_info: mixart_analog_hw_info,
    pub analog_level_present: u32,
}

#[repr(C, packed)]
pub struct mixart_digital_info {
    pub type_mask: u32,
    pub aes_info: mixart_digital_hw_info,
    pub adat_info: mixart_digital_hw_info,
}

#[repr(C, packed)]
pub struct mixart_audio_info {
    pub clock_type_mask: u32,
    pub analog_info: mixart_analog_info,
    pub digital_info: mixart_digital_info,
}

#[repr(C, packed)]
pub struct mixart_audio_info_resp {
    pub txx_status: u32,
    pub info: mixart_audio_info,
}

/* used for nb_bytes_max_per_sample */
pub const MIXART_FLOAT_P__4_0_TO_HEX: u32 = 0x40800000; /* +4.0f */
pub const MIXART_FLOAT_P__8_0_TO_HEX: u32 = 0x41000000; /* +8.0f */

#[repr(C, packed)]
pub struct mixart_stream_info {
    pub size_max_byte_frame: u32,
    pub size_max_sample_frame: u32,
    pub nb_bytes_max_per_sample: u32, /* float */
}

/*  MSG_STREAM_ADD_INPUT_GROUP */
/*  MSG_STREAM_ADD_OUTPUT_GROUP */

#[repr(C, packed)]
pub struct mixart_streaming_group_req {
    pub stream_count: u32,
    pub channel_count: u32,
    pub user_grp_number: u32,
    pub first_phys_audio: u32,
    pub latency: u32,
    pub stream_info: [mixart_stream_info; 32],
    pub connector: mixart_uid,
    pub flow_entry: [u32; 32],
}

#[repr(C, packed)]
pub struct mixart_stream_desc {
    pub stream_uid: mixart_uid,
    pub stream_desc: u32,
}

#[repr(C, packed)]
pub struct mixart_streaming_group {
    pub status: u32,
    pub group: mixart_uid,
    pub pipe_desc: u32,
    pub stream_count: u32,
    pub stream: [mixart_stream_desc; 32],
}

/* MSG_STREAM_DELETE_GROUP */

/* request : mixart_uid_t group */

#[repr(C, packed)]
pub struct mixart_delete_group_resp {
    pub status: u32,
    pub unused: [u32; 2],
}

/*      MSG_STREAM_START_INPUT_STAGE_PACKET  = 0x130000 + 7,
 *      MSG_STREAM_START_OUTPUT_STAGE_PACKET = 0x130000 + 8,
 *      MSG_STREAM_STOP_INPUT_STAGE_PACKET   = 0x130000 + 10,
 *      MSG_STREAM_STOP_OUTPUT_STAGE_PACKET  = 0x130000 + 11,
 */

#[repr(C, packed)]
pub struct mixart_fx_couple_uid {
    pub uid_fx_code: mixart_uid,
    pub uid_fx_data: mixart_uid,
}

#[repr(C, packed)]
pub struct mixart_txx_stream_desc {
    pub uid_pipe: mixart_uid,
    pub stream_idx: u32,
    pub fx_number: u32,
    pub uid_fx: [mixart_fx_couple_uid; 4],
}

#[repr(C, packed)]
pub struct mixart_flow_info {
    pub stream_desc: mixart_txx_stream_desc,
    pub flow_entry: u32,
    pub flow_phy_addr: u32,
}

#[repr(C, packed)]
pub struct mixart_stream_state_req {
    pub delayed: u32,
    pub scheduler: u64,
    pub reserved4np: [u32; 3],
    pub stream_count: u32,             /* set to 1 for instance */
    pub stream_info: mixart_flow_info, /* could be an array[stream_count] */
}

/*      MSG_STREAM_START_STREAM_GRP_PACKET   = 0x130000 + 6
 *      MSG_STREAM_STOP_STREAM_GRP_PACKET    = 0x130000 + 9
 */

#[repr(C, packed)]
pub struct mixart_group_state_req {
    pub delayed: u32,
    pub scheduler: u64,
    pub reserved4np: [u32; 2],
    pub pipe_count: u32,   /* set to 1 for instance */
    pub pipe_uid: mixart_uid, /* could be an array[pipe_count], in theory */
}

#[repr(C, packed)]
pub struct mixart_group_state_resp {
    pub txx_status: u32,
    pub scheduler: u64,
}

/* Structures used by the MSG_SERVICES_TIMER_NOTIFY command */

#[repr(C, packed)]
pub struct mixart_sample_pos {
    pub buffer_id: u32,
    pub validity: u32,
    pub sample_pos_high_part: u32,
    pub sample_pos_low_part: u32,
}

/*
 * This structure is limited by the size of MSG_DEFAULT_SIZE. Instead of
 * having MIXART_MAX_STREAM_PER_CARD * MIXART_MAX_CARDS many streams,
 * this is capped to have a total size below MSG_DEFAULT_SIZE.
 */
pub const MIXART_MAX_TIMER_NOTIFY_STREAMS: usize =
    (MSG_DEFAULT_SIZE - size_of::<u32>()) / size_of::<mixart_sample_pos>();

#[repr(C, packed)]
pub struct mixart_timer_notify {
    pub stream_count: u32,
    pub streams: [mixart_sample_pos; MIXART_MAX_TIMER_NOTIFY_STREAMS],
}

/*      MSG_CONSOLE_GET_CLOCK_UID            = 0x070003,
 */

/* request is a uid with desc = MSG_CONSOLE_MANAGER | cardindex */

#[repr(C, packed)]
pub struct mixart_return_uid {
    pub error_code: u32,
    pub uid: mixart_uid,
}

/*      MSG_CLOCK_CHECK_PROPERTIES           = 0x200001,
 *      MSG_CLOCK_SET_PROPERTIES             = 0x200002,
 */

#[repr(C)]
pub enum mixart_clock_generic_type {
    CGT_NO_CLOCK,
    CGT_INTERNAL_CLOCK,
    CGT_PROGRAMMABLE_CLOCK,
    CGT_INTERNAL_ENSLAVED_CLOCK,
    CGT_EXTERNAL_CLOCK,
    CGT_CURRENT_CLOCK,
}

#[repr(C)]
pub enum mixart_clock_mode {
    CM_UNDEFINED,
    CM_MASTER,
    CM_SLAVE,
    CM_STANDALONE,
    CM_NOT_CONCERNED,
}

#[repr(C, packed)]
pub struct mixart_clock_properties {
    pub error_code: u32,
    pub validation_mask: u32,
    pub frequency: u32,
    pub reference_frequency: u32,
    pub clock_generic_type: u32,
    pub clock_mode: u32,
    pub uid_clock_source: mixart_uid,
    pub uid_event_source: mixart_uid,
    pub event_mode: u32,
    pub synchro_signal_presence: u32,
    pub format: u32,
    pub board_mask: u32,
    pub nb_callers: u32, /* set to 1 (see below) */
    pub uid_caller: mixart_uid,
}

#[repr(C, packed)]
pub struct mixart_clock_properties_resp {
    pub status: u32,
    pub clock_mode: u32,
}

/*      MSG_STREAM_SET_INPUT_STAGE_PARAM     = 0x13000F */
/*      MSG_STREAM_SET_OUTPUT_STAGE_PARAM    = 0x130010 */

#[repr(C)]
pub enum mixart_coding_type {
    CT_NOT_DEFINED,
    CT_LINEAR,
    CT_MPEG_L1,
    CT_MPEG_L2,
    CT_MPEG_L3,
    CT_MPEG_L3_LSF,
    CT_GSM,
}

#[repr(C)]
pub enum mixart_sample_type {
    ST_NOT_DEFINED,
    ST_FLOATING_POINT_32BE,
    ST_FLOATING_POINT_32LE,
    ST_FLOATING_POINT_64BE,
    ST_FLOATING_POINT_64LE,
    ST_FIXED_POINT_8,
    ST_FIXED_POINT_16BE,
    ST_FIXED_POINT_16LE,
    ST_FIXED_POINT_24BE,
    ST_FIXED_POINT_24LE,
    ST_FIXED_POINT_32BE,
    ST_FIXED_POINT_32LE,
    ST_INTEGER_8,
    ST_INTEGER_16BE,
    ST_INTEGER_16LE,
    ST_INTEGER_24BE,
    ST_INTEGER_24LE,
    ST_INTEGER_32BE,
    ST_INTEGER_32LE,
}

#[repr(C, packed)]
pub struct mixart_linear_format_info {
    pub linear_endian_ness: u32,
    pub linear_bits: u32,
    pub is_signed: u32,
    pub is_float: u32,
}

#[repr(C, packed)]
pub struct mixart_mpeg_format_info {
    pub mpeg_layer: u32,
    pub mpeg_mode: u32,
    pub mpeg_mode_extension: u32,
    pub mpeg_pre_emphasis: u32,
    pub mpeg_has_padding_bit: u32,
    pub mpeg_has_crc: u32,
    pub mpeg_has_extension: u32,
    pub mpeg_is_original: u32,
    pub mpeg_has_copyright: u32,
}

#[repr(C)]
pub union mixart_format_info {
    pub linear_format_info: mixart_linear_format_info,
    pub mpeg_format_info: mixart_mpeg_format_info,
}

#[repr(C, packed)]
pub struct mixart_stream_param_desc {
    pub coding_type: u32, /* use enum mixart_coding_type */
    pub sample_type: u32, /* use enum mixart_sample_type */
    pub format_info: mixart_format_info,
    pub delayed: u32,
    pub scheduler: u64,
    pub sample_size: u32,
    pub has_header: u32,
    pub has_suffix: u32,
    pub has_bitrate: u32,
    pub samples_per_frame: u32,
    pub bytes_per_frame: u32,
    pub bytes_per_sample: u32,
    pub sampling_freq: u32,
    pub number_of_channel: u32,
    pub stream_number: u32,
    pub buffer_size: u32,
    pub differed_time: u32,
    pub reserved4np: [u32; 3],
    pub pipe_count: u32,   /* set to 1 (array size !) */
    pub stream_count: u32, /* set to 1 (array size !) */
    pub stream_desc: mixart_txx_stream_desc, /* only one stream per command, but this could be an array, in theory */
}

/*      MSG_CONNECTOR_GET_OUT_AUDIO_LEVEL    = 0x050009,
 */

#[repr(C, packed)]
pub struct mixart_get_out_audio_level {
    pub txx_status: u32,
    pub digital_level: u32, /* float */
    pub analog_level: u32,  /* float */
    pub monitor_level: u32, /* float */
    pub mute: u32,
    pub monitor_mute1: u32,
    pub monitor_mute2: u32,
}

/*      MSG_CONNECTOR_SET_OUT_AUDIO_LEVEL    = 0x05000A,
 */

/* used for valid_mask below */
pub const MIXART_AUDIO_LEVEL_ANALOG_MASK: u32 = 0x01;
pub const MIXART_AUDIO_LEVEL_DIGITAL_MASK: u32 = 0x02;
pub const MIXART_AUDIO_LEVEL_MONITOR_MASK: u32 = 0x04;
pub const MIXART_AUDIO_LEVEL_MUTE_MASK: u32 = 0x08;
pub const MIXART_AUDIO_LEVEL_MUTE_M1_MASK: u32 = 0x10;
pub const MIXART_AUDIO_LEVEL_MUTE_M2_MASK: u32 = 0x20;

#[repr(C, packed)]
pub struct mixart_set_out_audio_level {
    pub delayed: u32,
    pub scheduler: u64,
    pub valid_mask1: u32,
    pub valid_mask2: u32,
    pub digital_level: u32, /* float */
    pub analog_level: u32,  /* float */
    pub monitor_level: u32, /* float */
    pub mute: u32,
    pub monitor_mute1: u32,
    pub monitor_mute2: u32,
    pub reserved4np: u32,
}

/*      MSG_SYSTEM_ENUM_PHYSICAL_IO          = 0x16000E,
 */

pub const MIXART_MAX_PHYS_IO: usize = MIXART_MAX_CARDS * 2 * 2; /* 4 * (analog+digital) * (playback+capture) */

#[repr(C, packed)]
pub struct mixart_uid_enumeration {
    pub error_code: u32,
    pub first_uid_offset: u32,
    pub nb_uid: u32,
    pub current_uid_index: u32,
    pub uid: [mixart_uid; MIXART_MAX_PHYS_IO],
}

/*      MSG_PHYSICALIO_SET_LEVEL             = 0x0F0008,
 *      MSG_PHYSICALIO_GET_LEVEL             = 0x0F000C,
 */

#[repr(C, packed)]
pub struct mixart_io_channel_level {
    pub analog_level: u32, /* float */
    pub unused: [u32; 2],
}

#[repr(C, packed)]
pub struct mixart_io_level {
    pub channel: i32, /* 0=left, 1=right, -1=both, -2=both same */
    pub level: [mixart_io_channel_level; 2],
}

/*      MSG_STREAM_SET_IN_AUDIO_LEVEL        = 0x130015,
 */

#[repr(C, packed)]
pub struct mixart_in_audio_level_info {
    pub connector: mixart_uid,
    pub valid_mask1: u32,
    pub valid_mask2: u32,
    pub digital_level: u32,
    pub analog_level: u32,
}

#[repr(C, packed)]
pub struct mixart_set_in_audio_level_req {
    pub delayed: u32,
    pub scheduler: u64,
    pub audio_count: u32, /* set to <= 2 */
    pub reserved4np: u32,
    pub level: [mixart_in_audio_level_info; 2],
}

/* response is a 32 bit status */

/*      MSG_STREAM_SET_OUT_STREAM_LEVEL      = 0x130017,
 */

/* defines used for valid_mask1 */
pub const MIXART_OUT_STREAM_SET_LEVEL_LEFT_AUDIO1: u32 = 0x01;
pub const MIXART_OUT_STREAM_SET_LEVEL_LEFT_AUDIO2: u32 = 0x02;
pub const MIXART_OUT_STREAM_SET_LEVEL_RIGHT_AUDIO1: u32 = 0x04;
pub const MIXART_OUT_STREAM_SET_LEVEL_RIGHT_AUDIO2: u32 = 0x08;
pub const MIXART_OUT_STREAM_SET_LEVEL_STREAM_1: u32 = 0x10;
pub const MIXART_OUT_STREAM_SET_LEVEL_STREAM_2: u32 = 0x20;
pub const MIXART_OUT_STREAM_SET_LEVEL_MUTE_1: u32 = 0x40;
pub const MIXART_OUT_STREAM_SET_LEVEL_MUTE_2: u32 = 0x80;

#[repr(C, packed)]
pub struct mixart_out_stream_level_info {
    pub valid_mask1: u32,
    pub valid_mask2: u32,
    pub left_to_out1_level: u32,
    pub left_to_out2_level: u32,
    pub right_to_out1_level: u32,
    pub right_to_out2_level: u32,
    pub digital_level1: u32,
    pub digital_level2: u32,
    pub mute1: u32,
    pub mute2: u32,
}

#[repr(C, packed)]
pub struct mixart_set_out_stream_level {
    pub desc: mixart_txx_stream_desc,
    pub out_level: mixart_out_stream_level_info,
}

#[repr(C, packed)]
pub struct mixart_set_out_stream_level_req {
    pub delayed: u32,
    pub scheduler: u64,
    pub reserved4np: [u32; 2],
    pub nb_of_stream: u32, /* set to 1 */
    pub stream_level: mixart_set_out_stream_level, /* could be an array */
}

/* response to this request is a u32 status value */

/* exported */
unsafe extern "C" {
    pub fn snd_mixart_init_mailbox(mgr: *mut mixart_mgr);
    pub fn snd_mixart_exit_mailbox(mgr: *mut mixart_mgr);

    pub fn snd_mixart_send_msg(
        mgr: *mut mixart_mgr,
        request: *mut mixart_msg,
        max_resp_size: c_int,
        resp_data: *mut c_void,
    ) -> c_int;
    pub fn snd_mixart_send_msg_wait_notif(
        mgr: *mut mixart_mgr,
        request: *mut mixart_msg,
        notif_event: u32,
    ) -> c_int;
    pub fn snd_mixart_send_msg_nonblock(
        mgr: *mut mixart_mgr,
        request: *mut mixart_msg,
    ) -> c_int;

    pub fn snd_mixart_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    pub fn snd_mixart_threaded_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;

    pub fn snd_mixart_reset_board(mgr: *mut mixart_mgr);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
