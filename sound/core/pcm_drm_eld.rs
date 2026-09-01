// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PCM DRM helpers
 *
 * Translated from C. Kernel/header-provided items are declared here as
 * external dependencies for the surrounding repository.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_rule {
    pub var: c_uint,
    pub private: *mut c_void,
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
    pub openmin: c_uint,
    pub openmax: c_uint,
    pub integer: c_uint,
    pub empty: c_uint,
}

#[repr(C)]
pub struct snd_cea_sad {
    pub channels: c_int,
    pub format: c_int,
    pub rates: c_int,
    pub sample_bits: c_int,
    pub max_bitrate: c_int,
    pub profile: c_int,
}

#[repr(C)]
pub struct snd_parsed_hdmi_eld {
    pub monitor_name: [c_char; ELD_MAX_MNL + 1],
    pub eld_ver: c_int,
    pub baseline_len: c_int,
    pub cea_edid_ver: c_int,
    pub support_hdcp: c_int,
    pub support_ai: c_int,
    pub conn_type: c_int,
    pub sad_count: c_int,
    pub aud_synch_delay: c_int,
    pub spk_alloc: c_int,
    pub port_id: u64,
    pub manufacture_id: u16,
    pub product_id: u16,
    pub sad: [snd_cea_sad; ELD_MAX_SAD],
}

unsafe extern "C" {
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_interval;
    fn hw_param_interval_c(params: *mut snd_pcm_hw_params, var: c_uint) -> *const snd_interval;
    fn drm_eld_sad(eld: *const u8) -> *const u8;
    fn drm_eld_sad_count(eld: *const u8) -> c_uint;
    fn snd_interval_list(
        i: *mut snd_interval,
        count: c_uint,
        list: *const c_uint,
        mask: c_uint,
    ) -> c_int;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
        private: *mut c_void,
        ...
    ) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const u8, count: usize) -> isize;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

unsafe extern "C" {
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static HDMI_AUDIO_CODING_TYPE_PCM: c_uint;
    static HDMI_AUDIO_CODING_TYPE_AC3: c_uint;
    static HDMI_AUDIO_CODING_TYPE_DTS: c_uint;
    static HDMI_AUDIO_CODING_TYPE_EAC3: c_uint;
    static HDMI_AUDIO_CODING_TYPE_DTS_HD: c_uint;
    static HDMI_AUDIO_CODING_TYPE_MLP: c_uint;
    static AUDIO_CODING_TYPE_REF_STREAM_HEADER: c_int;
    static AUDIO_CODING_TYPE_LPCM: c_int;
    static AUDIO_CODING_TYPE_AC3: c_int;
    static AUDIO_CODING_TYPE_MPEG1: c_int;
    static AUDIO_CODING_TYPE_MP3: c_int;
    static AUDIO_CODING_TYPE_MPEG2: c_int;
    static AUDIO_CODING_TYPE_AACLC: c_int;
    static AUDIO_CODING_TYPE_DTS: c_int;
    static AUDIO_CODING_TYPE_ATRAC: c_int;
    static AUDIO_CODING_TYPE_SACD: c_int;
    static AUDIO_CODING_TYPE_EAC3: c_int;
    static AUDIO_CODING_TYPE_DTS_HD: c_int;
    static AUDIO_CODING_TYPE_MLP: c_int;
    static AUDIO_CODING_TYPE_DST: c_int;
    static AUDIO_CODING_TYPE_WMAPRO: c_int;
    static AUDIO_CODING_TYPE_REF_CXT: c_int;
    static AUDIO_CODING_XTYPE_HE_REF_CT: c_int;
    static AUDIO_CODING_XTYPE_FIRST_RESERVED: c_int;
    static AUDIO_CODING_TYPE_HE_AAC: c_int;
    static AUDIO_CODING_XTYPE_HE_AAC: c_int;
    static ELD_VER_CEA_861D: c_int;
    static ELD_VER_PARTIAL: c_int;
    static ELD_PCM_BITS_8: c_int;
    static ELD_PCM_BITS_16: c_int;
    static ELD_PCM_BITS_20: c_int;
    static ELD_PCM_BITS_24: c_int;
    static SNDRV_PCM_RATE_32000: c_int;
    static SNDRV_PCM_RATE_44100: c_int;
    static SNDRV_PCM_RATE_48000: c_int;
    static SNDRV_PCM_RATE_88200: c_int;
    static SNDRV_PCM_RATE_96000: c_int;
    static SNDRV_PCM_RATE_176400: c_int;
    static SNDRV_PCM_RATE_192000: c_int;
}

const EINVAL: c_int = 22;
const ELD_MAX_MNL: usize = 16;
const ELD_FIXED_BYTES: c_int = 20;
const ELD_MAX_SAD: usize = 16;

const fn bit(n: c_uint) -> c_uint {
    1u32 << n
}

const fn genmask(high: c_uint, low: c_uint) -> c_uint {
    ((!0u32) << low) & ((!0u32) >> (31 - high))
}

fn field_get(mask: c_uint, reg: c_uint) -> c_uint {
    (reg & mask) >> mask.trailing_zeros()
}

const SAD0_CHANNELS_MASK: c_uint = genmask(2, 0); /* max number of channels - 1 */
const SAD0_FORMAT_MASK: c_uint = genmask(6, 3); /* audio format */

const SAD1_RATE_MASK: c_uint = genmask(6, 0); /* bitfield of supported rates */
const SAD1_RATE_32000_MASK: c_uint = bit(0);
const SAD1_RATE_44100_MASK: c_uint = bit(1);
const SAD1_RATE_48000_MASK: c_uint = bit(2);
const SAD1_RATE_88200_MASK: c_uint = bit(3);
const SAD1_RATE_96000_MASK: c_uint = bit(4);
const SAD1_RATE_176400_MASK: c_uint = bit(5);
const SAD1_RATE_192000_MASK: c_uint = bit(6);

static eld_rates: [c_uint; 7] = [32000, 44100, 48000, 88200, 96000, 176400, 192000];

unsafe fn map_rate_families(
    sad: *const u8,
    mask_32000: c_uint,
    mask_44100: c_uint,
    mask_48000: c_uint,
) -> c_uint {
    let mut rate_mask: c_uint = 0;

    if (*sad.add(1) as c_uint & SAD1_RATE_32000_MASK) != 0 {
        rate_mask |= mask_32000;
    }
    if (*sad.add(1) as c_uint
        & (SAD1_RATE_44100_MASK | SAD1_RATE_88200_MASK | SAD1_RATE_176400_MASK))
        != 0
    {
        rate_mask |= mask_44100;
    }
    if (*sad.add(1) as c_uint
        & (SAD1_RATE_48000_MASK | SAD1_RATE_96000_MASK | SAD1_RATE_192000_MASK))
        != 0
    {
        rate_mask |= mask_48000;
    }
    rate_mask
}

unsafe fn sad_rate_mask(sad: *const u8) -> c_uint {
    match field_get(SAD0_FORMAT_MASK, *sad.add(0) as c_uint) {
        x if x == HDMI_AUDIO_CODING_TYPE_PCM => *sad.add(1) as c_uint & SAD1_RATE_MASK,
        x if x == HDMI_AUDIO_CODING_TYPE_AC3 || x == HDMI_AUDIO_CODING_TYPE_DTS => {
            map_rate_families(
                sad,
                SAD1_RATE_32000_MASK,
                SAD1_RATE_44100_MASK,
                SAD1_RATE_48000_MASK,
            )
        }
        x if x == HDMI_AUDIO_CODING_TYPE_EAC3
            || x == HDMI_AUDIO_CODING_TYPE_DTS_HD
            || x == HDMI_AUDIO_CODING_TYPE_MLP =>
        {
            map_rate_families(sad, 0, SAD1_RATE_176400_MASK, SAD1_RATE_192000_MASK)
        }
        _ => {
            /* TODO adjust for other compressed formats as well */
            *sad.add(1) as c_uint & SAD1_RATE_MASK
        }
    }
}

unsafe fn sad_max_channels(sad: *const u8) -> c_uint {
    match field_get(SAD0_FORMAT_MASK, *sad.add(0) as c_uint) {
        x if x == HDMI_AUDIO_CODING_TYPE_PCM => 1 + field_get(SAD0_CHANNELS_MASK, *sad.add(0) as c_uint),
        x if x == HDMI_AUDIO_CODING_TYPE_AC3
            || x == HDMI_AUDIO_CODING_TYPE_DTS
            || x == HDMI_AUDIO_CODING_TYPE_EAC3 =>
        {
            2
        }
        x if x == HDMI_AUDIO_CODING_TYPE_DTS_HD || x == HDMI_AUDIO_CODING_TYPE_MLP => 8,
        _ => {
            /* TODO adjust for other compressed formats as well */
            1 + field_get(SAD0_CHANNELS_MASK, *sad.add(0) as c_uint)
        }
    }
}

unsafe extern "C" fn eld_limit_rates(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let r = hw_param_interval(params, (*rule).var);
    let mut rate_mask: c_uint = 7;
    let eld = (*rule).private as *const u8;

    let mut sad = drm_eld_sad(eld);
    if !sad.is_null() {
        let c = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_CHANNELS as c_uint);
        let mut i = drm_eld_sad_count(eld);

        while i > 0 {
            let max_channels = sad_max_channels(sad);

            /*
             * Exclude SADs which do not include the
             * requested number of channels.
             */
            if (*c).min <= max_channels {
                rate_mask |= sad_rate_mask(sad);
            }
            i -= 1;
            sad = sad.add(3);
        }
    }

    snd_interval_list(r, eld_rates.len() as c_uint, eld_rates.as_ptr(), rate_mask)
}

unsafe extern "C" fn eld_limit_channels(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let c = hw_param_interval(params, (*rule).var);
    let mut t = snd_interval {
        min: 1,
        max: 2,
        openmin: 0,
        openmax: 0,
        integer: 1,
        empty: 0,
    };
    let eld = (*rule).private as *const u8;

    let mut sad = drm_eld_sad(eld);
    if !sad.is_null() {
        let mut rate_mask: c_uint = 0;

        /* Convert the rate interval to a mask */
        let r = hw_param_interval_c(params, SNDRV_PCM_HW_PARAM_RATE as c_uint);
        let mut i: usize = 0;
        while i < eld_rates.len() {
            if (*r).min <= eld_rates[i] && (*r).max >= eld_rates[i] {
                rate_mask |= bit(i as c_uint);
            }
            i += 1;
        }

        let mut n = drm_eld_sad_count(eld);
        while n > 0 {
            if (rate_mask & sad_rate_mask(sad)) != 0 {
                t.max = core::cmp::max(t.max, sad_max_channels(sad));
            }
            n -= 1;
            sad = sad.add(3);
        }
    }

    snd_interval_refine(c, &t)
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_constraint_eld(
    runtime: *mut snd_pcm_runtime,
    eld: *mut c_void,
) -> c_int {
    let mut ret: c_int;

    ret = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        eld_limit_rates,
        eld,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_pcm_hw_rule_add(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        eld_limit_channels,
        eld,
        SNDRV_PCM_HW_PARAM_RATE,
        -1,
    );

    ret
}
/* EXPORT_SYMBOL_GPL(snd_pcm_hw_constraint_eld); */

const SND_PRINT_RATES_ADVISED_BUFSIZE: usize = 80;
const SND_PRINT_BITS_ADVISED_BUFSIZE: usize = 16;
const SND_PRINT_CHANNEL_ALLOCATION_ADVISED_BUFSIZE: usize = 80;

static eld_connection_type_names: [*const c_char; 4] = [
    b"HDMI\0".as_ptr() as *const c_char,
    b"DisplayPort\0".as_ptr() as *const c_char,
    b"2-reserved\0".as_ptr() as *const c_char,
    b"3-reserved\0".as_ptr() as *const c_char,
];

static cea_audio_coding_type_names: [*const c_char; 18] = [
    b"undefined\0".as_ptr() as *const c_char,
    b"LPCM\0".as_ptr() as *const c_char,
    b"AC-3\0".as_ptr() as *const c_char,
    b"MPEG1\0".as_ptr() as *const c_char,
    b"MP3\0".as_ptr() as *const c_char,
    b"MPEG2\0".as_ptr() as *const c_char,
    b"AAC-LC\0".as_ptr() as *const c_char,
    b"DTS\0".as_ptr() as *const c_char,
    b"ATRAC\0".as_ptr() as *const c_char,
    b"DSD (One Bit Audio)\0".as_ptr() as *const c_char,
    b"E-AC-3/DD+ (Dolby Digital Plus)\0".as_ptr() as *const c_char,
    b"DTS-HD\0".as_ptr() as *const c_char,
    b"MLP (Dolby TrueHD)\0".as_ptr() as *const c_char,
    b"DST\0".as_ptr() as *const c_char,
    b"WMAPro\0".as_ptr() as *const c_char,
    b"HE-AAC\0".as_ptr() as *const c_char,
    b"HE-AACv2\0".as_ptr() as *const c_char,
    b"MPEG Surround\0".as_ptr() as *const c_char,
];

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
 * SS1:SS0 index => sample size
 */
unsafe fn cea_sample_sizes() -> [c_int; 4] {
    [
        0,               /* 0: Refer to Stream Header */
        ELD_PCM_BITS_16, /* 1: 16 bits */
        ELD_PCM_BITS_20, /* 2: 20 bits */
        ELD_PCM_BITS_24, /* 3: 24 bits */
    ]
}

/*
 * SF2:SF1:SF0 index => sampling frequency
 */
unsafe fn cea_sampling_frequencies() -> [c_int; 8] {
    [
        0,                    /* 0: Refer to Stream Header */
        SNDRV_PCM_RATE_32000, /* 1:  32000Hz */
        SNDRV_PCM_RATE_44100, /* 2:  44100Hz */
        SNDRV_PCM_RATE_48000, /* 3:  48000Hz */
        SNDRV_PCM_RATE_88200, /* 4:  88200Hz */
        SNDRV_PCM_RATE_96000, /* 5:  96000Hz */
        SNDRV_PCM_RATE_176400, /* 6: 176400Hz */
        SNDRV_PCM_RATE_192000, /* 7: 192000Hz */
    ]
}

fn GRAB_BITS(buf: *const u8, byte: usize, lowbit: c_uint, bits: c_uint) -> c_int {
    unsafe { ((*buf.add(byte) as c_uint >> lowbit) & ((1u32 << bits) - 1)) as c_int }
}

unsafe fn hdmi_update_short_audio_desc(
    dev: *mut device,
    a: *mut snd_cea_sad,
    buf: *const c_uchar,
) {
    let mut i: c_int;
    let mut val: c_int;

    val = GRAB_BITS(buf, 1, 0, 7);
    (*a).rates = 0;
    i = 0;
    while i < 7 {
        if (val & (1 << i)) != 0 {
            (*a).rates |= cea_sampling_frequencies()[(i + 1) as usize];
        }
        i += 1;
    }

    (*a).channels = GRAB_BITS(buf, 0, 0, 3);
    (*a).channels += 1;

    (*a).sample_bits = 0;
    (*a).max_bitrate = 0;

    (*a).format = GRAB_BITS(buf, 0, 3, 4);
    match (*a).format {
        x if x == AUDIO_CODING_TYPE_REF_STREAM_HEADER => {
            dev_info(dev, b"HDMI: audio coding type 0 not expected\n\0".as_ptr() as *const c_char);
        }
        x if x == AUDIO_CODING_TYPE_LPCM => {
            val = GRAB_BITS(buf, 2, 0, 3);
            i = 0;
            while i < 3 {
                if (val & (1 << i)) != 0 {
                    (*a).sample_bits |= cea_sample_sizes()[(i + 1) as usize];
                }
                i += 1;
            }
        }
        x if x == AUDIO_CODING_TYPE_AC3
            || x == AUDIO_CODING_TYPE_MPEG1
            || x == AUDIO_CODING_TYPE_MP3
            || x == AUDIO_CODING_TYPE_MPEG2
            || x == AUDIO_CODING_TYPE_AACLC
            || x == AUDIO_CODING_TYPE_DTS
            || x == AUDIO_CODING_TYPE_ATRAC =>
        {
            (*a).max_bitrate = GRAB_BITS(buf, 2, 0, 8);
            (*a).max_bitrate *= 8000;
        }
        x if x == AUDIO_CODING_TYPE_SACD => {}
        x if x == AUDIO_CODING_TYPE_EAC3 => {}
        x if x == AUDIO_CODING_TYPE_DTS_HD => {}
        x if x == AUDIO_CODING_TYPE_MLP => {}
        x if x == AUDIO_CODING_TYPE_DST => {}
        x if x == AUDIO_CODING_TYPE_WMAPRO => {
            (*a).profile = GRAB_BITS(buf, 2, 0, 3);
        }
        x if x == AUDIO_CODING_TYPE_REF_CXT => {
            (*a).format = GRAB_BITS(buf, 2, 3, 5);
            if (*a).format == AUDIO_CODING_XTYPE_HE_REF_CT
                || (*a).format >= AUDIO_CODING_XTYPE_FIRST_RESERVED
            {
                dev_info(
                    dev,
                    b"HDMI: audio coding xtype %d not expected\n\0".as_ptr() as *const c_char,
                    (*a).format,
                );
                (*a).format = 0;
            } else {
                (*a).format += AUDIO_CODING_TYPE_HE_AAC - AUDIO_CODING_XTYPE_HE_AAC;
            }
        }
        _ => {}
    }
}

type c_uchar = u8;

unsafe fn get_unaligned_le16(p: *const u8) -> u16 {
    u16::from_le_bytes([*p.add(0), *p.add(1)])
}

unsafe fn get_unaligned_le64(p: *const u8) -> u64 {
    u64::from_le_bytes([
        *p.add(0),
        *p.add(1),
        *p.add(2),
        *p.add(3),
        *p.add(4),
        *p.add(5),
        *p.add(6),
        *p.add(7),
    ])
}

/*
 * Be careful, ELD buf could be totally rubbish!
 */
#[no_mangle]
pub unsafe extern "C" fn snd_parse_eld(
    dev: *mut device,
    e: *mut snd_parsed_hdmi_eld,
    buf: *const c_uchar,
    size: c_int,
) -> c_int {
    let mnl: c_int;
    let mut i: c_int;

    ptr::write_bytes(e as *mut u8, 0, size_of::<snd_parsed_hdmi_eld>());
    (*e).eld_ver = GRAB_BITS(buf, 0, 3, 5);
    if (*e).eld_ver != ELD_VER_CEA_861D && (*e).eld_ver != ELD_VER_PARTIAL {
        dev_info_ratelimited(
            dev,
            b"HDMI: Unknown ELD version %d\n\0".as_ptr() as *const c_char,
            (*e).eld_ver,
        );
        return -EINVAL;
    }

    (*e).baseline_len = GRAB_BITS(buf, 2, 0, 8);
    mnl = GRAB_BITS(buf, 4, 0, 5);
    (*e).cea_edid_ver = GRAB_BITS(buf, 4, 5, 3);

    (*e).support_hdcp = GRAB_BITS(buf, 5, 0, 1);
    (*e).support_ai = GRAB_BITS(buf, 5, 1, 1);
    (*e).conn_type = GRAB_BITS(buf, 5, 2, 2);
    (*e).sad_count = GRAB_BITS(buf, 5, 4, 4);

    (*e).aud_synch_delay = GRAB_BITS(buf, 6, 0, 8) * 2;
    (*e).spk_alloc = GRAB_BITS(buf, 7, 0, 7);

    (*e).port_id = get_unaligned_le64(buf.add(8));

    /* not specified, but the spec's tendency is little endian */
    (*e).manufacture_id = get_unaligned_le16(buf.add(16));
    (*e).product_id = get_unaligned_le16(buf.add(18));

    if mnl > ELD_MAX_MNL as c_int {
        dev_info_ratelimited(
            dev,
            b"HDMI: MNL is reserved value %d\n\0".as_ptr() as *const c_char,
            mnl,
        );
        return -EINVAL;
    } else if ELD_FIXED_BYTES + mnl > size {
        dev_info(
            dev,
            b"HDMI: out of range MNL %d\n\0".as_ptr() as *const c_char,
            mnl,
        );
        return -EINVAL;
    } else {
        strscpy(
            (*e).monitor_name.as_mut_ptr(),
            buf.add(ELD_FIXED_BYTES as usize),
            (mnl + 1) as usize,
        );
    }

    i = 0;
    while i < (*e).sad_count {
        if ELD_FIXED_BYTES + mnl + 3 * (i + 1) > size {
            dev_info(
                dev,
                b"HDMI: out of range SAD %d\n\0".as_ptr() as *const c_char,
                i,
            );
            return -EINVAL;
        }
        hdmi_update_short_audio_desc(
            dev,
            (*e).sad.as_mut_ptr().add(i as usize),
            buf.add((ELD_FIXED_BYTES + mnl + 3 * i) as usize),
        );
        i += 1;
    }

    /*
     * HDMI sink's ELD info cannot always be retrieved for now, e.g.
     * in console or for audio devices. Assume the highest speakers
     * configuration, to _not_ prohibit multi-channel audio playback.
     */
    if (*e).spk_alloc == 0 && (*e).sad_count != 0 {
        (*e).spk_alloc = 0xffff;
    }

    0
}
/* EXPORT_SYMBOL_GPL(snd_parse_eld); */

/*
 * SNDRV_PCM_RATE_* and AC_PAR_PCM values don't match, print correct rates with
 * hdmi-specific routine.
 */
unsafe fn hdmi_print_pcm_rates(pcm: c_int, buf: *mut c_char, buflen: c_int) {
    static alsa_rates: [c_uint; 14] = [
        5512, 8000, 11025, 16000, 22050, 32000, 44100, 48000, 64000, 88200, 96000, 176400,
        192000, 384000,
    ];
    let mut i: usize = 0;
    let mut j: c_int = 0;

    while i < alsa_rates.len() {
        if (pcm & (1 << i)) != 0 {
            j += scnprintf(
                buf.add(j as usize),
                (buflen - j) as usize,
                b" %d\0".as_ptr() as *const c_char,
                alsa_rates[i],
            );
        }
        i += 1;
    }

    *buf.add(j as usize) = 0; /* necessary when j == 0 */
}

unsafe fn eld_print_pcm_bits(pcm: c_int, buf: *mut c_char, buflen: c_int) {
    static bits: [c_uint; 5] = [8, 16, 20, 24, 32];
    let mut i: usize = 0;
    let mut j: c_int = 0;

    while i < bits.len() {
        if (pcm & (ELD_PCM_BITS_8 << i)) != 0 {
            j += scnprintf(
                buf.add(j as usize),
                (buflen - j) as usize,
                b" %d\0".as_ptr() as *const c_char,
                bits[i],
            );
        }
        i += 1;
    }

    *buf.add(j as usize) = 0; /* necessary when j == 0 */
}

unsafe fn hdmi_show_short_audio_desc(dev: *mut device, a: *mut snd_cea_sad) {
    let mut buf: [c_char; SND_PRINT_RATES_ADVISED_BUFSIZE] = [0; SND_PRINT_RATES_ADVISED_BUFSIZE];
    let mut buf2: [c_char; 8 + SND_PRINT_BITS_ADVISED_BUFSIZE] =
        [0; 8 + SND_PRINT_BITS_ADVISED_BUFSIZE];
    ptr::copy_nonoverlapping(b", bits =\0".as_ptr() as *const c_char, buf2.as_mut_ptr(), 9);

    if (*a).format == 0 {
        return;
    }

    hdmi_print_pcm_rates((*a).rates, buf.as_mut_ptr(), buf.len() as c_int);

    if (*a).format == AUDIO_CODING_TYPE_LPCM {
        eld_print_pcm_bits(
            (*a).sample_bits,
            buf2.as_mut_ptr().add(8),
            (buf2.len() - 8) as c_int,
        );
    } else if (*a).max_bitrate != 0 {
        snprintf(
            buf2.as_mut_ptr(),
            buf2.len(),
            b", max bitrate = %d\0".as_ptr() as *const c_char,
            (*a).max_bitrate,
        );
    } else {
        buf2[0] = 0;
    }

    dev_dbg(
        dev,
        b"HDMI: supports coding type %s: channels = %d, rates =%s%s\n\0".as_ptr() as *const c_char,
        cea_audio_coding_type_names[(*a).format as usize],
        (*a).channels,
        buf.as_ptr(),
        buf2.as_ptr(),
    );
}

unsafe fn snd_eld_print_channel_allocation(spk_alloc: c_int, buf: *mut c_char, buflen: c_int) {
    let mut i: usize = 0;
    let mut j: c_int = 0;

    while i < cea_speaker_allocation_names.len() {
        if (spk_alloc & (1 << i)) != 0 {
            j += scnprintf(
                buf.add(j as usize),
                (buflen - j) as usize,
                b" %s\0".as_ptr() as *const c_char,
                cea_speaker_allocation_names[i],
            );
        }
        i += 1;
    }
    *buf.add(j as usize) = 0; /* necessary when j == 0 */
}

#[no_mangle]
pub unsafe extern "C" fn snd_show_eld(dev: *mut device, e: *mut snd_parsed_hdmi_eld) {
    let mut i: c_int;

    dev_dbg(
        dev,
        b"HDMI: detected monitor %s at connection type %s\n\0".as_ptr() as *const c_char,
        (*e).monitor_name.as_ptr(),
        eld_connection_type_names[(*e).conn_type as usize],
    );

    if (*e).spk_alloc != 0 {
        let mut buf: [c_char; SND_PRINT_CHANNEL_ALLOCATION_ADVISED_BUFSIZE] =
            [0; SND_PRINT_CHANNEL_ALLOCATION_ADVISED_BUFSIZE];

        snd_eld_print_channel_allocation((*e).spk_alloc, buf.as_mut_ptr(), buf.len() as c_int);
        dev_dbg(
            dev,
            b"HDMI: available speakers:%s\n\0".as_ptr() as *const c_char,
            buf.as_ptr(),
        );
    }

    i = 0;
    while i < (*e).sad_count {
        hdmi_show_short_audio_desc(dev, (*e).sad.as_mut_ptr().add(i as usize));
        i += 1;
    }
}
/* EXPORT_SYMBOL_GPL(snd_show_eld); */

/* CONFIG_SND_PROC_FS */
unsafe fn hdmi_print_sad_info(i: c_int, a: *mut snd_cea_sad, buffer: *mut snd_info_buffer) {
    let mut buf: [c_char; SND_PRINT_RATES_ADVISED_BUFSIZE] = [0; SND_PRINT_RATES_ADVISED_BUFSIZE];

    snd_iprintf(
        buffer,
        b"sad%d_coding_type\t[0x%x] %s\n\0".as_ptr() as *const c_char,
        i,
        (*a).format,
        cea_audio_coding_type_names[(*a).format as usize],
    );
    snd_iprintf(
        buffer,
        b"sad%d_channels\t\t%d\n\0".as_ptr() as *const c_char,
        i,
        (*a).channels,
    );

    hdmi_print_pcm_rates((*a).rates, buf.as_mut_ptr(), buf.len() as c_int);
    snd_iprintf(
        buffer,
        b"sad%d_rates\t\t[0x%x]%s\n\0".as_ptr() as *const c_char,
        i,
        (*a).rates,
        buf.as_ptr(),
    );

    if (*a).format == AUDIO_CODING_TYPE_LPCM {
        eld_print_pcm_bits((*a).sample_bits, buf.as_mut_ptr(), buf.len() as c_int);
        snd_iprintf(
            buffer,
            b"sad%d_bits\t\t[0x%x]%s\n\0".as_ptr() as *const c_char,
            i,
            (*a).sample_bits,
            buf.as_ptr(),
        );
    }

    if (*a).max_bitrate != 0 {
        snd_iprintf(
            buffer,
            b"sad%d_max_bitrate\t%d\n\0".as_ptr() as *const c_char,
            i,
            (*a).max_bitrate,
        );
    }

    if (*a).profile != 0 {
        snd_iprintf(
            buffer,
            b"sad%d_profile\t\t%d\n\0".as_ptr() as *const c_char,
            i,
            (*a).profile,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_print_eld_info(
    e: *mut snd_parsed_hdmi_eld,
    buffer: *mut snd_info_buffer,
) {
    let mut buf: [c_char; SND_PRINT_CHANNEL_ALLOCATION_ADVISED_BUFSIZE] =
        [0; SND_PRINT_CHANNEL_ALLOCATION_ADVISED_BUFSIZE];
    let mut i: c_int;
    let mut eld_version_names: [*const c_char; 32] = [b"reserved\0".as_ptr() as *const c_char; 32];
    eld_version_names[2] = b"CEA-861D or below\0".as_ptr() as *const c_char;
    eld_version_names[31] = b"partial\0".as_ptr() as *const c_char;
    let mut cea_edid_version_names: [*const c_char; 8] =
        [b"reserved\0".as_ptr() as *const c_char; 8];
    cea_edid_version_names[0] =
        b"no CEA EDID Timing Extension block present\0".as_ptr() as *const c_char;
    cea_edid_version_names[1] = b"CEA-861\0".as_ptr() as *const c_char;
    cea_edid_version_names[2] = b"CEA-861-A\0".as_ptr() as *const c_char;
    cea_edid_version_names[3] = b"CEA-861-B, C or D\0".as_ptr() as *const c_char;

    snd_iprintf(
        buffer,
        b"monitor_name\t\t%s\n\0".as_ptr() as *const c_char,
        (*e).monitor_name.as_ptr(),
    );
    snd_iprintf(
        buffer,
        b"connection_type\t\t%s\n\0".as_ptr() as *const c_char,
        eld_connection_type_names[(*e).conn_type as usize],
    );
    snd_iprintf(
        buffer,
        b"eld_version\t\t[0x%x] %s\n\0".as_ptr() as *const c_char,
        (*e).eld_ver,
        eld_version_names[(*e).eld_ver as usize],
    );
    snd_iprintf(
        buffer,
        b"edid_version\t\t[0x%x] %s\n\0".as_ptr() as *const c_char,
        (*e).cea_edid_ver,
        cea_edid_version_names[(*e).cea_edid_ver as usize],
    );
    snd_iprintf(
        buffer,
        b"manufacture_id\t\t0x%x\n\0".as_ptr() as *const c_char,
        (*e).manufacture_id as c_int,
    );
    snd_iprintf(
        buffer,
        b"product_id\t\t0x%x\n\0".as_ptr() as *const c_char,
        (*e).product_id as c_int,
    );
    snd_iprintf(
        buffer,
        b"port_id\t\t\t0x%llx\n\0".as_ptr() as *const c_char,
        (*e).port_id as i64,
    );
    snd_iprintf(
        buffer,
        b"support_hdcp\t\t%d\n\0".as_ptr() as *const c_char,
        (*e).support_hdcp,
    );
    snd_iprintf(
        buffer,
        b"support_ai\t\t%d\n\0".as_ptr() as *const c_char,
        (*e).support_ai,
    );
    snd_iprintf(
        buffer,
        b"audio_sync_delay\t%d\n\0".as_ptr() as *const c_char,
        (*e).aud_synch_delay,
    );

    snd_eld_print_channel_allocation((*e).spk_alloc, buf.as_mut_ptr(), buf.len() as c_int);
    snd_iprintf(
        buffer,
        b"speakers\t\t[0x%x]%s\n\0".as_ptr() as *const c_char,
        (*e).spk_alloc,
        buf.as_ptr(),
    );

    snd_iprintf(
        buffer,
        b"sad_count\t\t%d\n\0".as_ptr() as *const c_char,
        (*e).sad_count,
    );

    i = 0;
    while i < (*e).sad_count {
        hdmi_print_sad_info(i, (*e).sad.as_mut_ptr().add(i as usize), buffer);
        i += 1;
    }
}
/* EXPORT_SYMBOL_GPL(snd_print_eld_info); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
