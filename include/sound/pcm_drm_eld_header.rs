/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uchar, c_void};

#[repr(i32)]
pub enum eld_versions {
    ELD_VER_CEA_861D = 2,
    ELD_VER_PARTIAL = 31,
}

#[repr(i32)]
pub enum cea_audio_coding_types {
    AUDIO_CODING_TYPE_REF_STREAM_HEADER = 0,
    AUDIO_CODING_TYPE_LPCM = 1,
    AUDIO_CODING_TYPE_AC3 = 2,
    AUDIO_CODING_TYPE_MPEG1 = 3,
    AUDIO_CODING_TYPE_MP3 = 4,
    AUDIO_CODING_TYPE_MPEG2 = 5,
    AUDIO_CODING_TYPE_AACLC = 6,
    AUDIO_CODING_TYPE_DTS = 7,
    AUDIO_CODING_TYPE_ATRAC = 8,
    AUDIO_CODING_TYPE_SACD = 9,
    AUDIO_CODING_TYPE_EAC3 = 10,
    AUDIO_CODING_TYPE_DTS_HD = 11,
    AUDIO_CODING_TYPE_MLP = 12,
    AUDIO_CODING_TYPE_DST = 13,
    AUDIO_CODING_TYPE_WMAPRO = 14,
    AUDIO_CODING_TYPE_REF_CXT = 15,
    /* also include valid xtypes below */
    AUDIO_CODING_TYPE_HE_AAC = 15,
    AUDIO_CODING_TYPE_HE_AAC2 = 16,
    AUDIO_CODING_TYPE_MPEG_SURROUND = 17,
}

#[repr(i32)]
pub enum cea_audio_coding_xtypes {
    AUDIO_CODING_XTYPE_HE_REF_CT = 0,
    AUDIO_CODING_XTYPE_HE_AAC = 1,
    AUDIO_CODING_XTYPE_HE_AAC2 = 2,
    AUDIO_CODING_XTYPE_MPEG_SURROUND = 3,
    AUDIO_CODING_XTYPE_FIRST_RESERVED = 4,
}

/* CEA Short Audio Descriptor data */
#[repr(C)]
pub struct snd_cea_sad {
    pub channels: c_int,
    pub format: c_int, /* (format == 0) indicates invalid SAD */
    pub rates: c_int,
    pub sample_bits: c_int, /* for LPCM */
    pub max_bitrate: c_int, /* for AC3...ATRAC */
    pub profile: c_int, /* for WMAPRO */
}

pub const ELD_FIXED_BYTES: usize = 20;
pub const ELD_MAX_SIZE: usize = 256;
pub const ELD_MAX_MNL: usize = 16;
pub const ELD_MAX_SAD: usize = 16;

pub const ELD_PCM_BITS_8: u32 = 1 << 0;
pub const ELD_PCM_BITS_16: u32 = 1 << 1;
pub const ELD_PCM_BITS_20: u32 = 1 << 2;
pub const ELD_PCM_BITS_24: u32 = 1 << 3;
pub const ELD_PCM_BITS_32: u32 = 1 << 4;

/* ELD: EDID Like Data */
#[repr(C)]
pub struct snd_parsed_hdmi_eld {
    /* all fields will be cleared before updating ELD */
    pub baseline_len: c_int,
    pub eld_ver: c_int,
    pub cea_edid_ver: c_int,
    pub monitor_name: [c_char; ELD_MAX_MNL + 1],
    pub manufacture_id: c_int,
    pub product_id: c_int,
    pub port_id: u64,
    pub support_hdcp: c_int,
    pub support_ai: c_int,
    pub conn_type: c_int,
    pub aud_synch_delay: c_int,
    pub spk_alloc: c_int,
    pub sad_count: c_int,
    pub sad: [snd_cea_sad; ELD_MAX_SAD],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

extern "C" {
    pub fn snd_pcm_hw_constraint_eld(runtime: *mut snd_pcm_runtime, eld: *mut c_void) -> c_int;
    pub fn snd_parse_eld(
        dev: *mut device,
        e: *mut snd_parsed_hdmi_eld,
        buf: *const c_uchar,
        size: c_int,
    ) -> c_int;
    pub fn snd_show_eld(dev: *mut device, e: *mut snd_parsed_hdmi_eld);

    /* Preserved from the source's CONFIG_SND_PROC_FS conditional. */
    #[cfg(feature = "CONFIG_SND_PROC_FS")]
    pub fn snd_print_eld_info(eld: *mut snd_parsed_hdmi_eld, buffer: *mut snd_info_buffer);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
