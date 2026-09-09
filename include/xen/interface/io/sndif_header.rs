/* SPDX-License-Identifier: MIT */
/*
 * Rust translation of Xen's sndif.h.
 * The ring definitions and grant_ref_t are supplied by the corresponding
 * Xen interface dependencies.
 */

pub const XENSND_PROTOCOL_VERSION: u32 = 2;

pub const XENSND_PCM_FORMAT_S8: u32 = 0;
pub const XENSND_PCM_FORMAT_U8: u32 = 1;
pub const XENSND_PCM_FORMAT_S16_LE: u32 = 2;
pub const XENSND_PCM_FORMAT_S16_BE: u32 = 3;
pub const XENSND_PCM_FORMAT_U16_LE: u32 = 4;
pub const XENSND_PCM_FORMAT_U16_BE: u32 = 5;
pub const XENSND_PCM_FORMAT_S24_LE: u32 = 6;
pub const XENSND_PCM_FORMAT_S24_BE: u32 = 7;
pub const XENSND_PCM_FORMAT_U24_LE: u32 = 8;
pub const XENSND_PCM_FORMAT_U24_BE: u32 = 9;
pub const XENSND_PCM_FORMAT_S32_LE: u32 = 10;
pub const XENSND_PCM_FORMAT_S32_BE: u32 = 11;
pub const XENSND_PCM_FORMAT_U32_LE: u32 = 12;
pub const XENSND_PCM_FORMAT_U32_BE: u32 = 13;
pub const XENSND_PCM_FORMAT_F32_LE: u32 = 14;
pub const XENSND_PCM_FORMAT_F32_BE: u32 = 15;
pub const XENSND_PCM_FORMAT_F64_LE: u32 = 16;
pub const XENSND_PCM_FORMAT_F64_BE: u32 = 17;
pub const XENSND_PCM_FORMAT_IEC958_SUBFRAME_LE: u32 = 18;
pub const XENSND_PCM_FORMAT_IEC958_SUBFRAME_BE: u32 = 19;
pub const XENSND_PCM_FORMAT_MU_LAW: u32 = 20;
pub const XENSND_PCM_FORMAT_A_LAW: u32 = 21;
pub const XENSND_PCM_FORMAT_IMA_ADPCM: u32 = 22;
pub const XENSND_PCM_FORMAT_MPEG: u32 = 23;
pub const XENSND_PCM_FORMAT_GSM: u32 = 24;

pub const XENSND_OP_OPEN: u32 = 0;
pub const XENSND_OP_CLOSE: u32 = 1;
pub const XENSND_OP_READ: u32 = 2;
pub const XENSND_OP_WRITE: u32 = 3;
pub const XENSND_OP_SET_VOLUME: u32 = 4;
pub const XENSND_OP_GET_VOLUME: u32 = 5;
pub const XENSND_OP_MUTE: u32 = 6;
pub const XENSND_OP_UNMUTE: u32 = 7;
pub const XENSND_OP_TRIGGER: u32 = 8;
pub const XENSND_OP_HW_PARAM_QUERY: u32 = 9;
pub const XENSND_OP_TRIGGER_START: u32 = 0;
pub const XENSND_OP_TRIGGER_PAUSE: u32 = 1;
pub const XENSND_OP_TRIGGER_STOP: u32 = 2;
pub const XENSND_OP_TRIGGER_RESUME: u32 = 3;
pub const XENSND_EVT_CUR_POS: u32 = 0;

pub const XENSND_DRIVER_NAME: &str = "vsnd";
pub const XENSND_LIST_SEPARATOR: &str = ",";
pub const XENSND_FIELD_BE_VERSIONS: &str = "versions";
pub const XENSND_FIELD_FE_VERSION: &str = "version";
pub const XENSND_FIELD_VCARD_SHORT_NAME: &str = "short-name";
pub const XENSND_FIELD_VCARD_LONG_NAME: &str = "long-name";
pub const XENSND_FIELD_RING_REF: &str = "ring-ref";
pub const XENSND_FIELD_EVT_CHNL: &str = "event-channel";
pub const XENSND_FIELD_EVT_RING_REF: &str = "evt-ring-ref";
pub const XENSND_FIELD_EVT_EVT_CHNL: &str = "evt-event-channel";
pub const XENSND_FIELD_DEVICE_NAME: &str = "name";
pub const XENSND_FIELD_TYPE: &str = "type";
pub const XENSND_FIELD_STREAM_UNIQUE_ID: &str = "unique-id";
pub const XENSND_FIELD_CHANNELS_MIN: &str = "channels-min";
pub const XENSND_FIELD_CHANNELS_MAX: &str = "channels-max";
pub const XENSND_FIELD_SAMPLE_RATES: &str = "sample-rates";
pub const XENSND_FIELD_SAMPLE_FORMATS: &str = "sample-formats";
pub const XENSND_FIELD_BUFFER_SIZE: &str = "buffer-size";
pub const XENSND_STREAM_TYPE_PLAYBACK: &str = "p";
pub const XENSND_STREAM_TYPE_CAPTURE: &str = "c";
pub const XENSND_SAMPLE_RATE_MAX_LEN: usize = 11;
pub const XENSND_SAMPLE_FORMAT_MAX_LEN: usize = 24;

pub const XENSND_PCM_FORMAT_S8_STR: &str = "s8";
pub const XENSND_PCM_FORMAT_U8_STR: &str = "u8";
pub const XENSND_PCM_FORMAT_S16_LE_STR: &str = "s16_le";
pub const XENSND_PCM_FORMAT_S16_BE_STR: &str = "s16_be";
pub const XENSND_PCM_FORMAT_U16_LE_STR: &str = "u16_le";
pub const XENSND_PCM_FORMAT_U16_BE_STR: &str = "u16_be";
pub const XENSND_PCM_FORMAT_S24_LE_STR: &str = "s24_le";
pub const XENSND_PCM_FORMAT_S24_BE_STR: &str = "s24_be";
pub const XENSND_PCM_FORMAT_U24_LE_STR: &str = "u24_le";
pub const XENSND_PCM_FORMAT_U24_BE_STR: &str = "u24_be";
pub const XENSND_PCM_FORMAT_S32_LE_STR: &str = "s32_le";
pub const XENSND_PCM_FORMAT_S32_BE_STR: &str = "s32_be";
pub const XENSND_PCM_FORMAT_U32_LE_STR: &str = "u32_le";
pub const XENSND_PCM_FORMAT_U32_BE_STR: &str = "u32_be";
pub const XENSND_PCM_FORMAT_F32_LE_STR: &str = "float_le";
pub const XENSND_PCM_FORMAT_F32_BE_STR: &str = "float_be";
pub const XENSND_PCM_FORMAT_F64_LE_STR: &str = "float64_le";
pub const XENSND_PCM_FORMAT_F64_BE_STR: &str = "float64_be";
pub const XENSND_PCM_FORMAT_IEC958_SUBFRAME_LE_STR: &str = "iec958_subframe_le";
pub const XENSND_PCM_FORMAT_IEC958_SUBFRAME_BE_STR: &str = "iec958_subframe_be";
pub const XENSND_PCM_FORMAT_MU_LAW_STR: &str = "mu_law";
pub const XENSND_PCM_FORMAT_A_LAW_STR: &str = "a_law";
pub const XENSND_PCM_FORMAT_IMA_ADPCM_STR: &str = "ima_adpcm";
pub const XENSND_PCM_FORMAT_MPEG_STR: &str = "mpeg";
pub const XENSND_PCM_FORMAT_GSM_STR: &str = "gsm";

#[repr(C)]
pub struct xensnd_open_req {
    pub pcm_rate: u32,
    pub pcm_format: u8,
    pub pcm_channels: u8,
    pub reserved: u16,
    pub buffer_sz: u32,
    pub gref_directory: grant_ref_t,
    pub period_sz: u32,
}

#[repr(C)]
pub struct xensnd_page_directory {
    pub gref_dir_next_page: grant_ref_t,
    pub gref: [grant_ref_t; 0],
}

#[repr(C)]
pub struct xensnd_rw_req { pub offset: u32, pub length: u32 }

#[repr(C)]
pub struct xensnd_trigger_req { pub r#type: u8 }

#[repr(C)]
pub struct xensnd_query_hw_param {
    pub formats: u64,
    pub rates: xensnd_interval,
    pub channels: xensnd_interval,
    pub buffer: xensnd_interval,
    pub period: xensnd_interval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensnd_interval { pub min: u32, pub max: u32 }

#[repr(C)]
pub struct xensnd_cur_pos_evt { pub position: u64 }

#[repr(C)]
pub union xensnd_req_op {
    pub open: xensnd_open_req,
    pub rw: xensnd_rw_req,
    pub trigger: xensnd_trigger_req,
    pub hw_param: xensnd_query_hw_param,
    pub reserved: [u8; 56],
}

#[repr(C)]
pub struct xensnd_req {
    pub id: u16,
    pub operation: u8,
    pub reserved: [u8; 5],
    pub op: xensnd_req_op,
}

#[repr(C)]
pub union xensnd_resp_union {
    pub hw_param: xensnd_query_hw_param,
    pub reserved1: [u8; 56],
}

#[repr(C)]
pub struct xensnd_resp {
    pub id: u16,
    pub operation: u8,
    pub reserved: u8,
    pub status: i32,
    pub resp: xensnd_resp_union,
}

#[repr(C)]
pub union xensnd_evt_op {
    pub cur_pos: xensnd_cur_pos_evt,
    pub reserved: [u8; 56],
}

#[repr(C)]
pub struct xensnd_evt {
    pub id: u16,
    pub r#type: u8,
    pub reserved: [u8; 5],
    pub op: xensnd_evt_op,
}

/* DEFINE_RING_TYPES(xen_sndif, struct xensnd_req, struct xensnd_resp); */

#[repr(C)]
pub struct xensnd_event_page {
    pub in_cons: u32,
    pub in_prod: u32,
    pub reserved: [u8; 56],
}

/* XEN_PAGE_SIZE is supplied by the Xen interface dependency. */
pub const XENSND_EVENT_PAGE_SIZE: usize = XEN_PAGE_SIZE;
pub const XENSND_IN_RING_OFFS: usize = core::mem::size_of::<xensnd_event_page>();
pub const XENSND_IN_RING_SIZE: usize = XENSND_EVENT_PAGE_SIZE - XENSND_IN_RING_OFFS;
pub const XENSND_IN_RING_LEN: usize = XENSND_IN_RING_SIZE / core::mem::size_of::<xensnd_evt>();

#[inline]
pub unsafe fn XENSND_IN_RING(page: *mut core::ffi::c_void) -> *mut xensnd_evt {
    (page as *mut u8).add(XENSND_IN_RING_OFFS) as *mut xensnd_evt
}

#[inline]
pub unsafe fn XENSND_IN_RING_REF(page: *mut core::ffi::c_void, idx: usize) -> *mut xensnd_evt {
    XENSND_IN_RING(page).add(idx % XENSND_IN_RING_LEN)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
