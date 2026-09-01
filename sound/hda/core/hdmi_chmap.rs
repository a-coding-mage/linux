// SPDX-License-Identifier: GPL-2.0-only
/*
 * HDMI Channel map support helpers
 */

// Dependencies originally included from:
// <linux/module.h>, <sound/control.h>, <sound/tlv.h>, <sound/hda_chmap.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type hda_nid_t = c_uint;
pub type bool_ = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_long,
    pub count: c_uint,
    pub vd: *mut snd_kcontrol_volatile,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_tlv,
}

#[repr(C)]
pub struct snd_kcontrol_volatile {
    pub access: c_uint,
}

#[repr(C)]
pub union snd_kcontrol_tlv {
    pub c: Option<unsafe extern "C" fn(*mut snd_kcontrol, c_int, c_uint, *mut c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    _private: [u8; 0],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_chmap {
    pub private_data: *mut hdac_chmap,
    pub kctl: *mut snd_kcontrol,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub state: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_cea_channel_speaker_allocation {
    pub ca_index: c_int,
    pub speakers: [c_int; 8],
    pub channels: c_int,
    pub spk_mask: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_chmap_ops {
    pub get_spk_alloc: Option<unsafe extern "C" fn(*mut hdac_device, c_int) -> c_int>,
    pub get_chmap: Option<unsafe extern "C" fn(*mut hdac_device, c_int, *mut u8)>,
    pub set_chmap: Option<unsafe extern "C" fn(*mut hdac_device, c_int, *mut u8, bool_)>,
    pub is_pcm_attached: Option<unsafe extern "C" fn(*mut hdac_device, c_int) -> bool_>,
    pub chmap_validate: Option<unsafe extern "C" fn(*mut hdac_chmap, c_int, c_int, *mut u8) -> c_int>,
    pub chmap_cea_alloc_validate_get_type:
        Option<unsafe extern "C" fn(*mut hdac_chmap, *mut hdac_cea_channel_speaker_allocation, c_int) -> c_int>,
    pub cea_alloc_to_tlv_chmap:
        Option<unsafe extern "C" fn(*mut hdac_chmap, *mut hdac_cea_channel_speaker_allocation, *mut c_uint, c_int)>,
    pub pin_get_slot_channel: Option<unsafe extern "C" fn(*mut hdac_device, hda_nid_t, c_int) -> c_int>,
    pub pin_set_slot_channel:
        Option<unsafe extern "C" fn(*mut hdac_device, hda_nid_t, c_int, c_int) -> c_int>,
    pub set_channel_count: Option<unsafe extern "C" fn(*mut hdac_device, hda_nid_t, c_int)>,
}

#[repr(C)]
pub struct hdac_chmap {
    pub ops: hdac_chmap_ops,
    pub hdac: *mut hdac_device,
    pub channels_max: c_uint,
}

unsafe extern "C" {
    fn snd_hdac_codec_write(codec: *mut hdac_device, nid: hda_nid_t, flags: c_int, verb: c_uint, parm: c_uint) -> c_int;
    fn snd_hdac_codec_read(codec: *mut hdac_device, nid: hda_nid_t, flags: c_int, verb: c_uint, parm: c_uint) -> c_uint;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *const device, fmt: *const c_char, ...);
    fn put_user(x: c_uint, ptr: *mut c_uint) -> c_int;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: c_ulong) -> c_ulong;
    fn hweight_long(w: c_ulong) -> c_ulong;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_pcm_chmap;
    fn snd_ctl_get_ioffidx(kcontrol: *mut snd_kcontrol, id: *const snd_ctl_elem_id) -> c_uint;
    fn snd_pcm_chmap_substream(info: *mut snd_pcm_chmap, idx: c_uint) -> *mut snd_pcm_substream;
    fn snd_pcm_add_chmap_ctls(
        pcm: *mut snd_pcm,
        stream: c_int,
        chmap: *const c_void,
        max_channels: c_int,
        private_value: c_long,
        info_ret: *mut *mut snd_pcm_chmap,
    ) -> c_int;
    fn WARN_ON(condition: bool_) -> c_int;
}

const AC_VERB_SET_HDMI_CHAN_SLOT: c_uint = 0;
const AC_VERB_GET_HDMI_CHAN_SLOT: c_uint = 0;
const AC_VERB_GET_CVT_CHAN_COUNT: c_uint = 0;
const AC_VERB_SET_CVT_CHAN_COUNT: c_uint = 0;
const SND_PRINT_CHANNEL_ALLOCATION_ADVISED_BUFSIZE: usize = 80;
const SNDRV_CHMAP_LAST: c_long = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 0;
const SNDRV_CTL_TLVT_CHMAP_VAR: c_int = 0;
const SNDRV_CTL_TLVT_CONTAINER: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_WRITE: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STATE_OPEN: c_int = 0;
const SNDRV_PCM_STATE_SETUP: c_int = 1;
const SNDRV_PCM_STATE_PREPARED: c_int = 2;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;

const SNDRV_CHMAP_FL: u8 = 1;
const SNDRV_CHMAP_FR: u8 = 2;
const SNDRV_CHMAP_RL: u8 = 3;
const SNDRV_CHMAP_RR: u8 = 4;
const SNDRV_CHMAP_LFE: u8 = 5;
const SNDRV_CHMAP_FC: u8 = 6;
const SNDRV_CHMAP_RLC: u8 = 7;
const SNDRV_CHMAP_RRC: u8 = 8;
const SNDRV_CHMAP_RC: u8 = 9;
const SNDRV_CHMAP_FLC: u8 = 10;
const SNDRV_CHMAP_FRC: u8 = 11;
const SNDRV_CHMAP_TFL: u8 = 12;
const SNDRV_CHMAP_TFR: u8 = 13;
const SNDRV_CHMAP_FLW: u8 = 14;
const SNDRV_CHMAP_FRW: u8 = 15;
const SNDRV_CHMAP_TC: u8 = 16;
const SNDRV_CHMAP_TFC: u8 = 17;

/*
 * CEA speaker placement:
 *
 *        FLH       FCH        FRH
 *  FLW    FL  FLC   FC   FRC   FR   FRW
 *
 *                                  LFE
 *                     TC
 *
 *          RL  RLC   RC   RRC   RR
 *
 * The Left/Right Surround channel _notions_ LS/RS in SMPTE 320M corresponds to
 * CEA RL/RR; The SMPTE channel _assignment_ C/LFE is swapped to CEA LFE/FC.
 */
const FL: c_int = 1 << 0; /* Front Left           */
const FC: c_int = 1 << 1; /* Front Center         */
const FR: c_int = 1 << 2; /* Front Right          */
const FLC: c_int = 1 << 3; /* Front Left Center    */
const FRC: c_int = 1 << 4; /* Front Right Center   */
const RL: c_int = 1 << 5; /* Rear Left            */
const RC: c_int = 1 << 6; /* Rear Center          */
const RR: c_int = 1 << 7; /* Rear Right           */
const RLC: c_int = 1 << 8; /* Rear Left Center     */
const RRC: c_int = 1 << 9; /* Rear Right Center    */
const LFE: c_int = 1 << 10; /* Low Frequency Effect */
const FLW: c_int = 1 << 11; /* Front Left Wide      */
const FRW: c_int = 1 << 12; /* Front Right Wide     */
const FLH: c_int = 1 << 13; /* Front Left High      */
const FCH: c_int = 1 << 14; /* Front Center High    */
const FRH: c_int = 1 << 15; /* Front Right High     */
const TC: c_int = 1 << 16; /* Top Center           */

static cea_speaker_allocation_names: [*const c_char; 11] = [
    b"FL/FR\0".as_ptr() as *const c_char,
    b"LFE\0".as_ptr() as *const c_char,
    b"FC\0".as_ptr() as *const c_char,
    b"RL/RR\0".as_ptr() as *const c_char,
    b"RC\0".as_ptr() as *const c_char,
    b"FLC/FRC\0".as_ptr() as *const c_char,
    b"RLC/RRC\0".as_ptr() as *const c_char,
    b"FLW/FRW\0".as_ptr() as *const c_char,
    b"FLH/FRH\0".as_ptr() as *const c_char,
    b"TC\0".as_ptr() as *const c_char,
    b"FCH\0".as_ptr() as *const c_char,
];

/*
 * ELD SA bits in the CEA Speaker Allocation data block
 */
static eld_speaker_allocation_bits: [c_int; 11] = [
    FL | FR,
    LFE,
    FC,
    RL | RR,
    RC,
    FLC | FRC,
    RLC | RRC,
    /* the following are not defined in ELD yet */
    FLW | FRW,
    FLH | FRH,
    TC,
    FCH,
];

/*
 * ALSA sequence is:
 *
 *       surround40   surround41   surround50   surround51   surround71
 * ch0   front left   =            =            =            =
 * ch1   front right  =            =            =            =
 * ch2   rear left    =            =            =            =
 * ch3   rear right   =            =            =            =
 * ch4                LFE          center       center       center
 * ch5                                          LFE          LFE
 * ch6                                                       side left
 * ch7                                                       side right
 *
 * surround71 = {FL, FR, RLC, RRC, FC, LFE, RL, RR}
 */
static mut hdmi_channel_mapping: [[c_int; 8]; 0x32] = {
    let mut m = [[0; 8]; 0x32];
    m[0x00] = [0x00, 0x11, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7];
    m[0x01] = [0x00, 0x11, 0x22, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7];
    m[0x02] = [0x00, 0x11, 0x23, 0xf2, 0xf4, 0xf5, 0xf6, 0xf7];
    m[0x08] = [0x00, 0x11, 0x24, 0x35, 0xf3, 0xf2, 0xf6, 0xf7];
    m[0x03] = [0x00, 0x11, 0x23, 0x32, 0x44, 0xf5, 0xf6, 0xf7];
    m[0x09] = [0x00, 0x11, 0x24, 0x35, 0x42, 0xf3, 0xf6, 0xf7];
    m[0x0a] = [0x00, 0x11, 0x24, 0x35, 0x43, 0xf2, 0xf6, 0xf7];
    m[0x0b] = [0x00, 0x11, 0x24, 0x35, 0x43, 0x52, 0xf6, 0xf7];
    m[0x13] = [0x00, 0x11, 0x26, 0x37, 0x43, 0x52, 0x64, 0x75];
    m
};

/*
 * This is an ordered list!
 *
 * The preceding ones have better chances to be selected by
 * hdmi_channel_allocation().
 */
static mut channel_allocations: [hdac_cea_channel_speaker_allocation; 50] = [
    hdac_cea_channel_speaker_allocation { ca_index: 0x00, speakers: [0, 0, 0, 0, 0, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x01, speakers: [0, 0, 0, 0, 0, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x02, speakers: [0, 0, 0, 0, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x08, speakers: [0, 0, RR, RL, 0, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x09, speakers: [0, 0, RR, RL, 0, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x0a, speakers: [0, 0, RR, RL, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x0b, speakers: [0, 0, RR, RL, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x0f, speakers: [0, RC, RR, RL, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x13, speakers: [RRC, RLC, RR, RL, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x03, speakers: [0, 0, 0, 0, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x04, speakers: [0, 0, 0, RC, 0, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x05, speakers: [0, 0, 0, RC, 0, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x06, speakers: [0, 0, 0, RC, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x07, speakers: [0, 0, 0, RC, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x0c, speakers: [0, RC, RR, RL, 0, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x0d, speakers: [0, RC, RR, RL, 0, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x0e, speakers: [0, RC, RR, RL, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x10, speakers: [RRC, RLC, RR, RL, 0, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x11, speakers: [RRC, RLC, RR, RL, 0, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x12, speakers: [RRC, RLC, RR, RL, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x14, speakers: [FRC, FLC, 0, 0, 0, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x15, speakers: [FRC, FLC, 0, 0, 0, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x16, speakers: [FRC, FLC, 0, 0, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x17, speakers: [FRC, FLC, 0, 0, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x18, speakers: [FRC, FLC, 0, RC, 0, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x19, speakers: [FRC, FLC, 0, RC, 0, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x1a, speakers: [FRC, FLC, 0, RC, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x1b, speakers: [FRC, FLC, 0, RC, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x1c, speakers: [FRC, FLC, RR, RL, 0, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x1d, speakers: [FRC, FLC, RR, RL, 0, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x1e, speakers: [FRC, FLC, RR, RL, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x1f, speakers: [FRC, FLC, RR, RL, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x20, speakers: [0, FCH, RR, RL, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x21, speakers: [0, FCH, RR, RL, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x22, speakers: [TC, 0, RR, RL, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x23, speakers: [TC, 0, RR, RL, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x24, speakers: [FRH, FLH, RR, RL, 0, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x25, speakers: [FRH, FLH, RR, RL, 0, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x26, speakers: [FRW, FLW, RR, RL, 0, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x27, speakers: [FRW, FLW, RR, RL, 0, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x28, speakers: [TC, RC, RR, RL, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x29, speakers: [TC, RC, RR, RL, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x2a, speakers: [FCH, RC, RR, RL, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x2b, speakers: [FCH, RC, RR, RL, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x2c, speakers: [TC, FCH, RR, RL, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x2d, speakers: [TC, FCH, RR, RL, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x2e, speakers: [FRH, FLH, RR, RL, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x2f, speakers: [FRH, FLH, RR, RL, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x30, speakers: [FRW, FLW, RR, RL, FC, 0, FR, FL], channels: 0, spk_mask: 0 },
    hdac_cea_channel_speaker_allocation { ca_index: 0x31, speakers: [FRW, FLW, RR, RL, FC, LFE, FR, FL], channels: 0, spk_mask: 0 },
];

unsafe extern "C" fn hdmi_pin_set_slot_channel(
    codec: *mut hdac_device,
    pin_nid: hda_nid_t,
    asp_slot: c_int,
    channel: c_int,
) -> c_int {
    snd_hdac_codec_write(
        codec,
        pin_nid,
        0,
        AC_VERB_SET_HDMI_CHAN_SLOT,
        ((channel << 4) | asp_slot) as c_uint,
    )
}

unsafe extern "C" fn hdmi_pin_get_slot_channel(
    codec: *mut hdac_device,
    pin_nid: hda_nid_t,
    asp_slot: c_int,
) -> c_int {
    ((snd_hdac_codec_read(codec, pin_nid, 0, AC_VERB_GET_HDMI_CHAN_SLOT, asp_slot as c_uint) & 0xf0) >> 4) as c_int
}

unsafe extern "C" fn hdmi_get_channel_count(codec: *mut hdac_device, cvt_nid: hda_nid_t) -> c_int {
    1 + snd_hdac_codec_read(codec, cvt_nid, 0, AC_VERB_GET_CVT_CHAN_COUNT, 0) as c_int
}

unsafe extern "C" fn hdmi_set_channel_count(codec: *mut hdac_device, cvt_nid: hda_nid_t, chs: c_int) {
    if chs != hdmi_get_channel_count(codec, cvt_nid) {
        snd_hdac_codec_write(codec, cvt_nid, 0, AC_VERB_SET_CVT_CHAN_COUNT, (chs - 1) as c_uint);
    }
}

/*
 * Channel mapping routines
 */

/*
 * Compute derived values in channel_allocations[].
 */
unsafe fn init_channel_allocations() {
    let mut i = 0usize;
    while i < channel_allocations.len() {
        channel_allocations[i].channels = 0;
        channel_allocations[i].spk_mask = 0;
        let mut j = 0usize;
        while j < channel_allocations[i].speakers.len() {
            if channel_allocations[i].speakers[j] != 0 {
                channel_allocations[i].channels += 1;
                channel_allocations[i].spk_mask |= channel_allocations[i].speakers[j];
            }
            j += 1;
        }
        i += 1;
    }
}

unsafe fn get_channel_allocation_order(ca: c_int) -> c_int {
    let mut i = 0usize;
    while i < channel_allocations.len() {
        if channel_allocations[i].ca_index == ca {
            break;
        }
        i += 1;
    }
    i as c_int
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_print_channel_allocation(spk_alloc: c_int, buf: *mut c_char, buflen: c_int) {
    let mut i = 0usize;
    let mut j: c_int = 0;
    while i < cea_speaker_allocation_names.len() {
        if (spk_alloc & (1 << i)) != 0 {
            j += scnprintf(
                buf.offset(j as isize),
                (buflen - j) as usize,
                b" %s\0".as_ptr() as *const c_char,
                cea_speaker_allocation_names[i],
            );
        }
        i += 1;
    }
    *buf.offset(j as isize) = 0; /* necessary when j == 0 */
}

/*
 * The transformation takes two steps:
 *
 *	eld->spk_alloc => (eld_speaker_allocation_bits[]) => spk_mask
 *	      spk_mask => (channel_allocations[])         => ai->CA
 *
 * TODO: it could select the wrong CA from multiple candidates.
*/
unsafe fn hdmi_channel_allocation_spk_alloc_blk(
    codec: *mut hdac_device,
    spk_alloc: c_int,
    channels: c_int,
) -> c_int {
    let mut i: usize;
    let mut ca = 0;
    let mut spk_mask = 0;
    let mut buf = [0 as c_char; SND_PRINT_CHANNEL_ALLOCATION_ADVISED_BUFSIZE];

    /*
     * CA defaults to 0 for basic stereo audio
     */
    if channels <= 2 {
        return 0;
    }

    /*
     * expand ELD's speaker allocation mask
     *
     * ELD tells the speaker mask in a compact(paired) form,
     * expand ELD's notions to match the ones used by Audio InfoFrame.
     */
    i = 0;
    while i < eld_speaker_allocation_bits.len() {
        if (spk_alloc & (1 << i)) != 0 {
            spk_mask |= eld_speaker_allocation_bits[i];
        }
        i += 1;
    }

    /* search for the first working match in the CA table */
    i = 0;
    while i < channel_allocations.len() {
        if channels == channel_allocations[i].channels
            && (spk_mask & channel_allocations[i].spk_mask) == channel_allocations[i].spk_mask
        {
            ca = channel_allocations[i].ca_index;
            break;
        }
        i += 1;
    }

    if ca == 0 {
        /*
         * if there was no match, select the regular ALSA channel
         * allocation with the matching number of channels
         */
        i = 0;
        while i < channel_allocations.len() {
            if channels == channel_allocations[i].channels {
                ca = channel_allocations[i].ca_index;
                break;
            }
            i += 1;
        }
    }

    snd_hdac_print_channel_allocation(spk_alloc, buf.as_mut_ptr(), buf.len() as c_int);
    dev_dbg(
        &(*codec).dev as *const device,
        b"HDMI: select CA 0x%x for %d-channel allocation: %s\n\0".as_ptr() as *const c_char,
        ca,
        channels,
        buf.as_ptr(),
    );

    ca
}

unsafe fn hdmi_debug_channel_mapping(chmap: *mut hdac_chmap, pin_nid: hda_nid_t) {
    // CONFIG_SND_DEBUG_VERBOSE conditional code from C source.
    #[cfg(CONFIG_SND_DEBUG_VERBOSE)]
    {
        let mut i = 0;
        while i < 8 {
            let channel = ((*chmap).ops.pin_get_slot_channel.unwrap())((*chmap).hdac, pin_nid, i);
            dev_dbg(
                &(*(*chmap).hdac).dev as *const device,
                b"HDMI: ASP channel %d => slot %d\n\0".as_ptr() as *const c_char,
                channel,
                i,
            );
            i += 1;
        }
    }
}

unsafe fn hdmi_std_setup_channel_mapping(
    chmap: *mut hdac_chmap,
    pin_nid: hda_nid_t,
    non_pcm: bool_,
    ca: c_int,
) {
    let mut i: c_int;
    let mut err: c_int;
    let order = get_channel_allocation_order(ca);
    let ch_alloc = &mut channel_allocations[order as usize] as *mut hdac_cea_channel_speaker_allocation;
    let mut non_pcm_mapping = [0; 8];

    if hdmi_channel_mapping[ca as usize][1] == 0 {
        let mut hdmi_slot = 0;
        /* fill actual channel mappings in ALSA channel (i) order */
        i = 0;
        while i < (*ch_alloc).channels && hdmi_slot < 8 {
            while (*ch_alloc).speakers[(7 - hdmi_slot) as usize] == 0 {
                /* skip zero slots */
                hdmi_slot += 1;
                if hdmi_slot >= 8 {
                    break;
                }
            }
            if hdmi_slot >= 8 {
                break;
            }

            hdmi_channel_mapping[ca as usize][i as usize] = (i << 4) | hdmi_slot;
            hdmi_slot += 1;
            i += 1;
        }
        /* fill the rest of the slots with ALSA channel 0xf */
        hdmi_slot = 0;
        while hdmi_slot < 8 {
            if (*ch_alloc).speakers[(7 - hdmi_slot) as usize] == 0 {
                hdmi_channel_mapping[ca as usize][i as usize] = (0xf << 4) | hdmi_slot;
                i += 1;
            }
            hdmi_slot += 1;
        }
    }

    if non_pcm {
        i = 0;
        while i < (*ch_alloc).channels {
            non_pcm_mapping[i as usize] = (i << 4) | i;
            i += 1;
        }
        while i < 8 {
            non_pcm_mapping[i as usize] = (0xf << 4) | i;
            i += 1;
        }
    }

    i = 0;
    while i < 8 {
        let slotsetup = if non_pcm { non_pcm_mapping[i as usize] } else { hdmi_channel_mapping[ca as usize][i as usize] };
        let hdmi_slot = slotsetup & 0x0f;
        let channel = (slotsetup & 0xf0) >> 4;

        err = ((*chmap).ops.pin_set_slot_channel.unwrap())((*chmap).hdac, pin_nid, hdmi_slot, channel);
        if err != 0 {
            dev_dbg(&(*(*chmap).hdac).dev as *const device, b"HDMI: channel mapping failed\n\0".as_ptr() as *const c_char);
            break;
        }
        i += 1;
    }
}

#[repr(C)]
pub struct channel_map_table {
    pub map: u8, /* ALSA API channel map position */
    pub spk_mask: c_int, /* speaker position bit mask */
}

static mut map_tables: [channel_map_table; 18] = [
    channel_map_table { map: SNDRV_CHMAP_FL, spk_mask: FL },
    channel_map_table { map: SNDRV_CHMAP_FR, spk_mask: FR },
    channel_map_table { map: SNDRV_CHMAP_RL, spk_mask: RL },
    channel_map_table { map: SNDRV_CHMAP_RR, spk_mask: RR },
    channel_map_table { map: SNDRV_CHMAP_LFE, spk_mask: LFE },
    channel_map_table { map: SNDRV_CHMAP_FC, spk_mask: FC },
    channel_map_table { map: SNDRV_CHMAP_RLC, spk_mask: RLC },
    channel_map_table { map: SNDRV_CHMAP_RRC, spk_mask: RRC },
    channel_map_table { map: SNDRV_CHMAP_RC, spk_mask: RC },
    channel_map_table { map: SNDRV_CHMAP_FLC, spk_mask: FLC },
    channel_map_table { map: SNDRV_CHMAP_FRC, spk_mask: FRC },
    channel_map_table { map: SNDRV_CHMAP_TFL, spk_mask: FLH },
    channel_map_table { map: SNDRV_CHMAP_TFR, spk_mask: FRH },
    channel_map_table { map: SNDRV_CHMAP_FLW, spk_mask: FLW },
    channel_map_table { map: SNDRV_CHMAP_FRW, spk_mask: FRW },
    channel_map_table { map: SNDRV_CHMAP_TC, spk_mask: TC },
    channel_map_table { map: SNDRV_CHMAP_TFC, spk_mask: FCH },
    channel_map_table { map: 0, spk_mask: 0 }, /* terminator */
];

/* from ALSA API channel position to speaker bit mask */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_chmap_to_spk_mask(c: u8) -> c_int {
    let mut idx = 0usize;
    while map_tables[idx].map != 0 {
        if map_tables[idx].map == c {
            return map_tables[idx].spk_mask;
        }
        idx += 1;
    }
    0
}

/* from ALSA API channel position to CEA slot */
unsafe fn to_cea_slot(ordered_ca: c_int, pos: u8) -> c_int {
    let mask = snd_hdac_chmap_to_spk_mask(pos);
    let mut i = 0usize;

    /* Add sanity check to pass klockwork check.
     * This should never happen.
     */
    if ordered_ca as usize >= channel_allocations.len() {
        return -1;
    }

    if mask != 0 {
        while i < 8 {
            if channel_allocations[ordered_ca as usize].speakers[7 - i] == mask {
                return i as c_int;
            }
            i += 1;
        }
    }

    -1
}

/* from speaker bit mask to ALSA API channel position */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_spk_to_chmap(spk: c_int) -> c_int {
    let mut idx = 0usize;
    while map_tables[idx].map != 0 {
        if map_tables[idx].spk_mask == spk {
            return map_tables[idx].map as c_int;
        }
        idx += 1;
    }
    0
}

/* from CEA slot to ALSA API channel position */
unsafe fn from_cea_slot(ordered_ca: c_int, slot: u8) -> c_int {
    let mask: c_int;

    /* Add sanity check to pass klockwork check.
     * This should never happen.
     */
    if slot >= 8 {
        return 0;
    }

    mask = channel_allocations[ordered_ca as usize].speakers[7 - slot as usize];

    snd_hdac_spk_to_chmap(mask)
}

/* get the CA index corresponding to the given ALSA API channel map */
unsafe fn hdmi_manual_channel_allocation(chs: c_int, map: *mut u8) -> c_int {
    let mut i: usize;
    let mut spks = 0;
    let mut spk_mask = 0;

    i = 0;
    while i < chs as usize {
        let mask = snd_hdac_chmap_to_spk_mask(*map.add(i));

        if mask != 0 {
            spk_mask |= mask;
            spks += 1;
        }
        i += 1;
    }

    i = 0;
    while i < channel_allocations.len() {
        if (chs == channel_allocations[i].channels || spks == channel_allocations[i].channels)
            && (spk_mask & channel_allocations[i].spk_mask) == channel_allocations[i].spk_mask
        {
            return channel_allocations[i].ca_index;
        }
        i += 1;
    }
    -1
}

/* set up the channel slots for the given ALSA API channel map */
unsafe fn hdmi_manual_setup_channel_mapping(
    chmap: *mut hdac_chmap,
    pin_nid: hda_nid_t,
    chs: c_int,
    map: *mut u8,
    ca: c_int,
) -> c_int {
    let ordered_ca = get_channel_allocation_order(ca);
    let mut alsa_pos = 0;
    let mut hdmi_slot: c_int;
    let mut assignments = [0xf; 8];

    while alsa_pos < chs {
        hdmi_slot = to_cea_slot(ordered_ca, *map.offset(alsa_pos as isize));

        if hdmi_slot < 0 {
            alsa_pos += 1;
            continue; /* unassigned channel */
        }

        assignments[hdmi_slot as usize] = alsa_pos;
        alsa_pos += 1;
    }

    hdmi_slot = 0;
    while hdmi_slot < 8 {
        let err = ((*chmap).ops.pin_set_slot_channel.unwrap())(
            (*chmap).hdac,
            pin_nid,
            hdmi_slot,
            assignments[hdmi_slot as usize],
        );
        if err != 0 {
            return -EINVAL;
        }
        hdmi_slot += 1;
    }
    0
}

/* store ALSA API channel map from the current default map */
unsafe fn hdmi_setup_fake_chmap(map: *mut u8, ca: c_int) {
    let mut i = 0usize;
    let ordered_ca = get_channel_allocation_order(ca);

    while i < 8 {
        if (ordered_ca as usize) < channel_allocations.len() && (i as c_int) < channel_allocations[ordered_ca as usize].channels {
            *map.add(i) = from_cea_slot(ordered_ca, (hdmi_channel_mapping[ca as usize][i] & 0x0f) as u8) as u8;
        } else {
            *map.add(i) = 0;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_setup_channel_mapping(
    chmap: *mut hdac_chmap,
    pin_nid: hda_nid_t,
    non_pcm: bool_,
    ca: c_int,
    channels: c_int,
    map: *mut u8,
    chmap_set: bool_,
) {
    if !non_pcm && chmap_set {
        hdmi_manual_setup_channel_mapping(chmap, pin_nid, channels, map, ca);
    } else {
        hdmi_std_setup_channel_mapping(chmap, pin_nid, non_pcm, ca);
        hdmi_setup_fake_chmap(map, ca);
    }

    hdmi_debug_channel_mapping(chmap, pin_nid);
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_get_active_channels(ca: c_int) -> c_int {
    let mut ordered_ca = get_channel_allocation_order(ca);

    /* Add sanity check to pass klockwork check.
     * This should never happen.
     */
    if ordered_ca as usize >= channel_allocations.len() {
        ordered_ca = 0;
    }

    channel_allocations[ordered_ca as usize].channels
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_get_ch_alloc_from_ca(ca: c_int) -> *mut hdac_cea_channel_speaker_allocation {
    &mut channel_allocations[get_channel_allocation_order(ca) as usize]
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_channel_allocation(
    hdac: *mut hdac_device,
    spk_alloc: c_int,
    channels: c_int,
    chmap_set: bool_,
    non_pcm: bool_,
    map: *mut u8,
) -> c_int {
    let mut ca: c_int;

    if !non_pcm && chmap_set {
        ca = hdmi_manual_channel_allocation(channels, map);
    } else {
        ca = hdmi_channel_allocation_spk_alloc_blk(hdac, spk_alloc, channels);
    }

    if ca < 0 {
        ca = 0;
    }

    ca
}

/*
 * ALSA API channel-map control callbacks
 */
unsafe extern "C" fn hdmi_chmap_ctl_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let info = snd_kcontrol_chip(kcontrol);
    let chmap = (*info).private_data;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = (*chmap).channels_max;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = SNDRV_CHMAP_LAST;
    0
}

unsafe extern "C" fn hdmi_chmap_cea_alloc_validate_get_type(
    _chmap: *mut hdac_chmap,
    cap: *mut hdac_cea_channel_speaker_allocation,
    channels: c_int,
) -> c_int {
    /* If the speaker allocation matches the channel count, it is OK.*/
    if (*cap).channels != channels {
        return -1;
    }

    /* all channels are remappable freely */
    SNDRV_CTL_TLVT_CHMAP_VAR
}

unsafe extern "C" fn hdmi_cea_alloc_to_tlv_chmap(
    _hchmap: *mut hdac_chmap,
    cap: *mut hdac_cea_channel_speaker_allocation,
    chmap: *mut c_uint,
    channels: c_int,
) {
    let mut count = 0;
    let mut c = 7;

    while c >= 0 {
        let spk = (*cap).speakers[c as usize];

        if spk != 0 {
            *chmap.add(count as usize) = snd_hdac_spk_to_chmap(spk) as c_uint;
            count += 1;
        }
        if c == 0 {
            break;
        }
        c -= 1;
    }

    WARN_ON(count != channels);
}

unsafe fn spk_mask_from_spk_alloc(spk_alloc: c_int) -> c_int {
    let mut i = 0usize;
    let mut spk_mask = eld_speaker_allocation_bits[0];

    while i < eld_speaker_allocation_bits.len() {
        if (spk_alloc & (1 << i)) != 0 {
            spk_mask |= eld_speaker_allocation_bits[i];
        }
        i += 1;
    }

    spk_mask
}

unsafe extern "C" fn hdmi_chmap_ctl_tlv(
    kcontrol: *mut snd_kcontrol,
    _op_flag: c_int,
    mut size: c_uint,
    tlv: *mut c_uint,
) -> c_int {
    let info = snd_kcontrol_chip(kcontrol);
    let chmap = (*info).private_data;
    let pcm_idx = (*kcontrol).private_value as c_int;
    let mut dst: *mut c_uint;
    let mut chs: c_int;
    let mut count = 0;
    let max_chs: c_ulong;
    let mut type_: c_int;
    let spk_alloc: c_int;
    let spk_mask: c_int;

    if size < 8 {
        return -ENOMEM;
    }
    if put_user(SNDRV_CTL_TLVT_CONTAINER, tlv) != 0 {
        return -EFAULT;
    }
    size -= 8;
    dst = tlv.add(2);

    spk_alloc = ((*chmap).ops.get_spk_alloc.unwrap())((*chmap).hdac, pcm_idx);
    spk_mask = spk_mask_from_spk_alloc(spk_alloc);

    max_chs = hweight_long(spk_mask as c_ulong);

    chs = 2;
    while (chs as c_ulong) <= max_chs {
        let mut i = 0usize;
        let mut cap = channel_allocations.as_mut_ptr();
        while i < channel_allocations.len() {
            let chs_bytes = chs * 4;
            let mut tlv_chmap = [0u32; 8];

            if (*cap).channels != chs {
                i += 1;
                cap = cap.add(1);
                continue;
            }

            if !((*cap).spk_mask == (spk_mask & (*cap).spk_mask)) {
                i += 1;
                cap = cap.add(1);
                continue;
            }

            type_ = ((*chmap).ops.chmap_cea_alloc_validate_get_type.unwrap())(chmap, cap, chs);
            if type_ < 0 {
                return -ENODEV;
            }
            if size < 8 {
                return -ENOMEM;
            }

            if put_user(type_ as c_uint, dst) != 0 || put_user(chs_bytes as c_uint, dst.add(1)) != 0 {
                return -EFAULT;
            }

            dst = dst.add(2);
            size -= 8;
            count += 8;

            if size < chs_bytes as c_uint {
                return -ENOMEM;
            }

            size -= chs_bytes as c_uint;
            count += chs_bytes;
            ((*chmap).ops.cea_alloc_to_tlv_chmap.unwrap())(chmap, cap, tlv_chmap.as_mut_ptr(), chs);

            if copy_to_user(dst as *mut c_void, tlv_chmap.as_ptr() as *const c_void, chs_bytes as c_ulong) != 0 {
                return -EFAULT;
            }
            dst = dst.add(chs as usize);
            i += 1;
            cap = cap.add(1);
        }
        chs += 1;
    }

    if put_user(count as c_uint, tlv.add(1)) != 0 {
        return -EFAULT;
    }

    0
}

unsafe extern "C" fn hdmi_chmap_ctl_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let info = snd_kcontrol_chip(kcontrol);
    let chmap = (*info).private_data;
    let pcm_idx = (*kcontrol).private_value as c_int;
    let mut pcm_chmap = [0u8; 8];
    let mut i = 0usize;

    memset(pcm_chmap.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&pcm_chmap));
    ((*chmap).ops.get_chmap.unwrap())((*chmap).hdac, pcm_idx, pcm_chmap.as_mut_ptr());

    while i < pcm_chmap.len() {
        (*ucontrol).value.integer.value[i] = pcm_chmap[i] as c_long;
        i += 1;
    }

    0
}

/* a simple sanity check for input values to chmap kcontrol */
unsafe fn chmap_value_check(hchmap: *mut hdac_chmap, ucontrol: *const snd_ctl_elem_value) -> c_int {
    let mut i = 0usize;

    while i < (*hchmap).channels_max as usize {
        if (*ucontrol).value.integer.value[i] < 0 || (*ucontrol).value.integer.value[i] > SNDRV_CHMAP_LAST {
            return -EINVAL;
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn hdmi_chmap_ctl_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let info = snd_kcontrol_chip(kcontrol);
    let hchmap = (*info).private_data;
    let pcm_idx = (*kcontrol).private_value as c_int;
    let ctl_idx: c_uint;
    let substream: *mut snd_pcm_substream;
    let mut chmap = [0u8; 8];
    let mut per_pin_chmap = [0u8; 8];
    let mut i: usize;
    let mut err: c_int;
    let ca: c_int;
    let mut prepared = false;

    err = chmap_value_check(hchmap, ucontrol);
    if err < 0 {
        return err;
    }

    /* No monitor is connected in dyn_pcm_assign.
     * It's invalid to setup the chmap
     */
    if !((*hchmap).ops.is_pcm_attached.unwrap())((*hchmap).hdac, pcm_idx) {
        return 0;
    }

    ctl_idx = snd_ctl_get_ioffidx(kcontrol, &(*ucontrol).id as *const snd_ctl_elem_id);
    substream = snd_pcm_chmap_substream(info, ctl_idx);
    if substream.is_null() || (*substream).runtime.is_null() {
        return 0; /* just for avoiding error from alsactl restore */
    }
    match (*(*substream).runtime).state {
        SNDRV_PCM_STATE_OPEN | SNDRV_PCM_STATE_SETUP => {}
        SNDRV_PCM_STATE_PREPARED => {
            prepared = true;
        }
        _ => {
            return -EBUSY;
        }
    }
    memset(chmap.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&chmap));
    i = 0;
    while i < chmap.len() {
        chmap[i] = (*ucontrol).value.integer.value[i] as u8;
        i += 1;
    }

    ((*hchmap).ops.get_chmap.unwrap())((*hchmap).hdac, pcm_idx, per_pin_chmap.as_mut_ptr());
    if memcmp(
        chmap.as_ptr() as *const c_void,
        per_pin_chmap.as_ptr() as *const c_void,
        core::mem::size_of_val(&chmap),
    ) == 0
    {
        return 0;
    }
    ca = hdmi_manual_channel_allocation(chmap.len() as c_int, chmap.as_mut_ptr());
    if ca < 0 {
        return -EINVAL;
    }
    if let Some(chmap_validate) = (*hchmap).ops.chmap_validate {
        err = chmap_validate(hchmap, ca, chmap.len() as c_int, chmap.as_mut_ptr());
        if err != 0 {
            return err;
        }
    }

    ((*hchmap).ops.set_chmap.unwrap())((*hchmap).hdac, pcm_idx, chmap.as_mut_ptr(), prepared);

    0
}

static chmap_ops: hdac_chmap_ops = hdac_chmap_ops {
    chmap_cea_alloc_validate_get_type: Some(hdmi_chmap_cea_alloc_validate_get_type),
    cea_alloc_to_tlv_chmap: Some(hdmi_cea_alloc_to_tlv_chmap),
    pin_get_slot_channel: Some(hdmi_pin_get_slot_channel),
    pin_set_slot_channel: Some(hdmi_pin_set_slot_channel),
    set_channel_count: Some(hdmi_set_channel_count),
    get_spk_alloc: None,
    get_chmap: None,
    set_chmap: None,
    is_pcm_attached: None,
    chmap_validate: None,
};

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_register_chmap_ops(hdac: *mut hdac_device, chmap: *mut hdac_chmap) {
    (*chmap).ops = chmap_ops;
    (*chmap).hdac = hdac;
    init_channel_allocations();
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_add_chmap_ctls(
    pcm: *mut snd_pcm,
    pcm_idx: c_int,
    hchmap: *mut hdac_chmap,
) -> c_int {
    let mut chmap: *mut snd_pcm_chmap = core::ptr::null_mut();
    let kctl: *mut snd_kcontrol;
    let mut err: c_int;
    let mut i: usize;

    err = snd_pcm_add_chmap_ctls(
        pcm,
        SNDRV_PCM_STREAM_PLAYBACK,
        core::ptr::null(),
        0,
        pcm_idx as c_long,
        &mut chmap as *mut *mut snd_pcm_chmap,
    );
    if err < 0 {
        return err;
    }
    /* override handlers */
    (*chmap).private_data = hchmap;
    kctl = (*chmap).kctl;
    i = 0;
    while i < (*kctl).count as usize {
        (*(*kctl).vd.add(i)).access |= SNDRV_CTL_ELEM_ACCESS_WRITE;
        i += 1;
    }
    (*kctl).info = Some(hdmi_chmap_ctl_info);
    (*kctl).get = Some(hdmi_chmap_ctl_get);
    (*kctl).put = Some(hdmi_chmap_ctl_put);
    (*kctl).tlv.c = Some(hdmi_chmap_ctl_tlv);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
