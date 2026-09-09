/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Universal MIDI Packet (UMP) Support */

/* Dependency supplied by the surrounding translation unit: sound/rawmidi.h */

pub struct snd_ump_endpoint;
pub struct snd_ump_block;
pub struct snd_ump_ops;
pub struct ump_cvt_to_ump;
pub struct snd_seq_ump_ops;

#[repr(C)]
pub struct snd_ump_group {
    pub group: i32,
    pub dir_bits: u32,
    pub active: bool,
    pub valid: bool,
    pub is_midi1: bool,
    pub name: [core::ffi::c_char; 64],
}

#[repr(C)]
pub struct snd_ump_endpoint {
    pub core: snd_rawmidi,
    pub info: snd_ump_endpoint_info,
    pub ops: *const snd_ump_ops,
    pub substreams: [*mut snd_rawmidi_substream; 2],
    pub private_data: *mut core::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ump_endpoint)>,
    pub stream_wait_for: u32,
    pub stream_finished: bool,
    pub parsed: bool,
    pub no_process_stream: bool,
    pub stream_wait: wait_queue_head_t,
    pub stream_rfile: snd_rawmidi_file,
    pub block_list: list_head,
    pub input_buf: [u32; 4],
    pub input_buf_head: i32,
    pub input_pending: i32,
    pub open_mutex: mutex,
    pub groups: [snd_ump_group; SNDRV_UMP_MAX_GROUPS],
    #[cfg(feature = "CONFIG_SND_UMP_LEGACY_RAWMIDI")]
    pub legacy_locks: [spinlock_t; 2],
    #[cfg(feature = "CONFIG_SND_UMP_LEGACY_RAWMIDI")]
    pub legacy_rmidi: *mut snd_rawmidi,
    #[cfg(feature = "CONFIG_SND_UMP_LEGACY_RAWMIDI")]
    pub legacy_substreams: [[*mut snd_rawmidi_substream; SNDRV_UMP_MAX_GROUPS]; 2],
    #[cfg(feature = "CONFIG_SND_UMP_LEGACY_RAWMIDI")]
    pub legacy_mapping: [core::ffi::c_uchar; SNDRV_UMP_MAX_GROUPS],
    #[cfg(feature = "CONFIG_SND_UMP_LEGACY_RAWMIDI")]
    pub legacy_out_opens: i32,
    #[cfg(feature = "CONFIG_SND_UMP_LEGACY_RAWMIDI")]
    pub legacy_out_rfile: snd_rawmidi_file,
    #[cfg(feature = "CONFIG_SND_UMP_LEGACY_RAWMIDI")]
    pub out_cvts: *mut ump_cvt_to_ump,
    #[cfg(feature = "CONFIG_SND_SEQUENCER")]
    pub seq_dev: *mut snd_seq_device,
    #[cfg(feature = "CONFIG_SND_SEQUENCER")]
    pub seq_ops: *const snd_seq_ump_ops,
    #[cfg(feature = "CONFIG_SND_SEQUENCER")]
    pub seq_client: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct snd_ump_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, i32) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, i32)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, i32, i32)>,
    pub drain: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, i32)>,
}

#[repr(C)]
pub struct snd_seq_ump_ops {
    pub input_receive: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, *const u32, i32)>,
    pub notify_ep_change: Option<unsafe extern "C" fn(*mut snd_ump_endpoint) -> i32>,
    pub notify_fb_change: Option<unsafe extern "C" fn(*mut snd_ump_endpoint, *mut snd_ump_block) -> i32>,
    pub switch_protocol: Option<unsafe extern "C" fn(*mut snd_ump_endpoint) -> i32>,
}

#[repr(C)]
pub struct snd_ump_block {
    pub info: snd_ump_block_info,
    pub ump: *mut snd_ump_endpoint,
    pub private_data: *mut core::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ump_block)>,
    pub list: list_head,
}

#[inline]
pub unsafe fn rawmidi_to_ump(rmidi: *mut snd_rawmidi) -> *mut snd_ump_endpoint {
    (rmidi as *mut u8).sub(core::mem::offset_of!(snd_ump_endpoint, core)) as *mut snd_ump_endpoint
}

unsafe extern "C" {
    pub fn snd_ump_endpoint_new(card: *mut snd_card, id: *mut core::ffi::c_char, device: i32,
        output: i32, input: i32, ump_ret: *mut *mut snd_ump_endpoint) -> i32;
    pub fn snd_ump_parse_endpoint(ump: *mut snd_ump_endpoint) -> i32;
    pub fn snd_ump_block_new(ump: *mut snd_ump_endpoint, blk: u32, direction: u32,
        first_group: u32, num_groups: u32, blk_ret: *mut *mut snd_ump_block) -> i32;
    pub fn snd_ump_receive(ump: *mut snd_ump_endpoint, buffer: *const u32, count: i32) -> i32;
    pub fn snd_ump_transmit(ump: *mut snd_ump_endpoint, buffer: *mut u32, count: i32) -> i32;
    pub fn snd_ump_attach_legacy_rawmidi(ump: *mut snd_ump_endpoint,
        id: *mut core::ffi::c_char, device: i32) -> i32;
    pub fn snd_ump_receive_ump_val(ump: *mut snd_ump_endpoint, val: u32) -> i32;
    pub fn snd_ump_switch_protocol(ump: *mut snd_ump_endpoint, protocol: u32) -> i32;
    pub fn snd_ump_update_group_attrs(ump: *mut snd_ump_endpoint);
}

pub const UMP_MSG_TYPE_UTILITY: u32 = 0x00;
pub const UMP_MSG_TYPE_SYSTEM: u32 = 0x01;
pub const UMP_MSG_TYPE_MIDI1_CHANNEL_VOICE: u32 = 0x02;
pub const UMP_MSG_TYPE_DATA: u32 = 0x03;
pub const UMP_MSG_TYPE_MIDI2_CHANNEL_VOICE: u32 = 0x04;
pub const UMP_MSG_TYPE_EXTENDED_DATA: u32 = 0x05;
pub const UMP_MSG_TYPE_FLEX_DATA: u32 = 0x0d;
pub const UMP_MSG_TYPE_STREAM: u32 = 0x0f;

pub const UMP_SYSEX_STATUS_SINGLE: u32 = 0;
pub const UMP_SYSEX_STATUS_START: u32 = 1;
pub const UMP_SYSEX_STATUS_CONTINUE: u32 = 2;
pub const UMP_SYSEX_STATUS_END: u32 = 3;

pub const UMP_UTILITY_MSG_STATUS_NOOP: u32 = 0x00;
pub const UMP_UTILITY_MSG_STATUS_JR_CLOCK: u32 = 0x01;
pub const UMP_UTILITY_MSG_STATUS_JR_TSTAMP: u32 = 0x02;
pub const UMP_UTILITY_MSG_STATUS_DCTPQ: u32 = 0x03;
pub const UMP_UTILITY_MSG_STATUS_DC: u32 = 0x04;

pub const UMP_STREAM_MSG_STATUS_EP_DISCOVERY: u32 = 0x00;
pub const UMP_STREAM_MSG_STATUS_EP_INFO: u32 = 0x01;
pub const UMP_STREAM_MSG_STATUS_DEVICE_INFO: u32 = 0x02;
pub const UMP_STREAM_MSG_STATUS_EP_NAME: u32 = 0x03;
pub const UMP_STREAM_MSG_STATUS_PRODUCT_ID: u32 = 0x04;
pub const UMP_STREAM_MSG_STATUS_STREAM_CFG_REQUEST: u32 = 0x05;
pub const UMP_STREAM_MSG_STATUS_STREAM_CFG: u32 = 0x06;
pub const UMP_STREAM_MSG_STATUS_FB_DISCOVERY: u32 = 0x10;
pub const UMP_STREAM_MSG_STATUS_FB_INFO: u32 = 0x11;
pub const UMP_STREAM_MSG_STATUS_FB_NAME: u32 = 0x12;
pub const UMP_STREAM_MSG_STATUS_START_CLIP: u32 = 0x20;
pub const UMP_STREAM_MSG_STATUS_END_CLIP: u32 = 0x21;

pub const UMP_STREAM_MSG_REQUEST_EP_INFO: u32 = 1u32 << 0;
pub const UMP_STREAM_MSG_REQUEST_DEVICE_INFO: u32 = 1u32 << 1;
pub const UMP_STREAM_MSG_REQUEST_EP_NAME: u32 = 1u32 << 2;
pub const UMP_STREAM_MSG_REQUEST_PRODUCT_ID: u32 = 1u32 << 3;
pub const UMP_STREAM_MSG_REQUEST_STREAM_CFG: u32 = 1u32 << 4;
pub const UMP_STREAM_MSG_REQUEST_FB_INFO: u32 = 1u32 << 0;
pub const UMP_STREAM_MSG_REQUEST_FB_NAME: u32 = 1u32 << 1;
pub const UMP_STREAM_MSG_EP_INFO_CAP_TXJR: u32 = 1u32 << 0;
pub const UMP_STREAM_MSG_EP_INFO_CAP_RXJR: u32 = 1u32 << 1;
pub const UMP_STREAM_MSG_EP_INFO_CAP_MIDI1: u32 = 1u32 << 8;
pub const UMP_STREAM_MSG_EP_INFO_CAP_MIDI2: u32 = 1u32 << 9;
pub const UMP_STREAM_MSG_FORMAT_SINGLE: u32 = 0;
pub const UMP_STREAM_MSG_FORMAT_START: u32 = 1;
pub const UMP_STREAM_MSG_FORMAT_CONTINUE: u32 = 2;
pub const UMP_STREAM_MSG_FORMAT_END: u32 = 3;

#[inline] pub fn ump_message_type(data: u32) -> u8 { (data >> 28) as u8 }
#[inline] pub fn ump_message_group(data: u32) -> u8 { ((data >> 24) & 0x0f) as u8 }
#[inline] pub fn ump_message_status_code(data: u32) -> u8 { ((data >> 20) & 0x0f) as u8 }
#[inline] pub fn ump_message_channel(data: u32) -> u8 { ((data >> 16) & 0x0f) as u8 }
#[inline] pub fn ump_message_status_channel(data: u32) -> u8 { ((data >> 16) & 0xff) as u8 }
#[inline] pub fn ump_compose(type_: u8, group: u8, status: u8, channel: u8) -> u32 {
    ((type_ as u32) << 28) | ((group as u32) << 24) | ((status as u32) << 20) | ((channel as u32) << 16)
}
#[inline] pub fn ump_sysex_message_status(data: u32) -> u8 { ((data >> 20) & 0xf) as u8 }
#[inline] pub fn ump_sysex_message_length(data: u32) -> u8 { ((data >> 16) & 0xf) as u8 }
#[inline] pub fn ump_stream_message_format(data: u32) -> u8 { ((data >> 26) & 0x03) as u8 }
#[inline] pub fn ump_stream_message_status(data: u32) -> u32 { (data >> 16) & 0x3ff }
#[inline] pub fn ump_stream_compose(status: u8, form: u16) -> u32 {
    (UMP_MSG_TYPE_STREAM << 28) | ((form as u32) << 26) | ((status as u32) << 16)
}
#[inline] pub const fn ump_is_groupless_msg(type_: u32) -> bool {
    type_ == UMP_MSG_TYPE_UTILITY || type_ == UMP_MSG_TYPE_STREAM
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
