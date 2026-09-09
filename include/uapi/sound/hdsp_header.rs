/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/*
 *   Copyright (C) 2003 Thomas Charbonnel (thomas@undata.org)
 */

// The Linux __u32/__u64 types and ioctl encoding macros are supplied by the
// surrounding API environment.

pub const HDSP_MATRIX_MIXER_SIZE: usize = 2048;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HDSP_IO_Type {
    Digiface,
    Multiface,
    H9652,
    H9632,
    RPM,
    Undefined,
}

#[repr(C)]
pub struct hdsp_peak_rms {
    pub input_peaks: [__u32; 26],
    pub playback_peaks: [__u32; 26],
    pub output_peaks: [__u32; 28],
    pub input_rms: [__u64; 26],
    pub playback_rms: [__u64; 26],
    /* These are only used for H96xx cards */
    pub output_rms: [__u64; 26],
}

pub const SNDRV_HDSP_IOCTL_GET_PEAK_RMS: u64 =
    _IOR!(b'H', 0x40, hdsp_peak_rms);

#[repr(C)]
pub struct hdsp_config_info {
    pub pref_sync_ref: u8,
    pub wordclock_sync_check: u8,
    pub spdif_sync_check: u8,
    pub adatsync_sync_check: u8,
    pub adat_sync_check: [u8; 3],
    pub spdif_in: u8,
    pub spdif_out: u8,
    pub spdif_professional: u8,
    pub spdif_emphasis: u8,
    pub spdif_nonaudio: u8,
    pub spdif_sample_rate: c_uint,
    pub system_sample_rate: c_uint,
    pub autosync_sample_rate: c_uint,
    pub system_clock_mode: u8,
    pub clock_source: u8,
    pub autosync_ref: u8,
    pub line_out: u8,
    pub passthru: u8,
    pub da_gain: u8,
    pub ad_gain: u8,
    pub phone_gain: u8,
    pub xlr_breakout_cable: u8,
    pub analog_extension_board: u8,
}

pub const SNDRV_HDSP_IOCTL_GET_CONFIG_INFO: u64 =
    _IOR!(b'H', 0x41, hdsp_config_info);

#[repr(C)]
pub struct hdsp_firmware {
    pub firmware_data: *mut core::ffi::c_void, /* 24413 x 4 bytes */
}

pub const SNDRV_HDSP_IOCTL_UPLOAD_FIRMWARE: u64 =
    _IOW!(b'H', 0x42, hdsp_firmware);

#[repr(C)]
pub struct hdsp_version {
    pub io_type: HDSP_IO_Type,
    pub firmware_rev: u16,
}

pub const SNDRV_HDSP_IOCTL_GET_VERSION: u64 =
    _IOR!(b'H', 0x43, hdsp_version);

#[repr(C)]
pub struct hdsp_mixer {
    pub matrix: [u16; HDSP_MATRIX_MIXER_SIZE],
}

pub const SNDRV_HDSP_IOCTL_GET_MIXER: u64 =
    _IOR!(b'H', 0x44, hdsp_mixer);

#[repr(C)]
pub struct hdsp_9632_aeb {
    pub aebi: i32,
    pub aebo: i32,
}

pub const SNDRV_HDSP_IOCTL_GET_9632_AEB: u64 =
    _IOR!(b'H', 0x45, hdsp_9632_aeb);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
