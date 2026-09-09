/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Translated from hdspm.h. */

pub const HDSPM_MAX_CHANNELS: usize = 64;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdspm_io_type {
    MADI,
    MADIface,
    AIO,
    AES32,
    RayDAT,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdspm_speed {
    ss,
    ds,
    qs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_peak_rms {
    pub input_peaks: [u32; 64],
    pub playback_peaks: [u32; 64],
    pub output_peaks: [u32; 64],
    pub input_rms: [u64; 64],
    pub playback_rms: [u64; 64],
    pub output_rms: [u64; 64],
    pub speed: u8,
    pub status2: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_config {
    pub pref_sync_ref: u8,
    pub wordclock_sync_check: u8,
    pub madi_sync_check: u8,
    pub system_sample_rate: u32,
    pub autosync_sample_rate: u32,
    pub system_clock_mode: u8,
    pub clock_source: u8,
    pub autosync_ref: u8,
    pub line_out: u8,
    pub passthru: u32,
    pub analog_out: u32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdspm_ltc_format { format_invalid, fps_24, fps_25, fps_2997, fps_30 }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdspm_ltc_frame { frame_invalid, drop_frame, full_frame }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdspm_ltc_input_format { ntsc, pal, no_video }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_ltc {
    pub ltc: u32,
    pub format: hdspm_ltc_format,
    pub frame: hdspm_ltc_frame,
    pub input_format: hdspm_ltc_input_format,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdspm_sync {
    hdspm_sync_no_lock = 0,
    hdspm_sync_lock = 1,
    hdspm_sync_sync = 2,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdspm_madi_input { hdspm_input_optical = 0, hdspm_input_coax = 1 }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdspm_madi_channel_format { hdspm_format_ch_64 = 0, hdspm_format_ch_56 = 1 }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdspm_madi_frame_format { hdspm_frame_48 = 0, hdspm_frame_96 = 1 }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdspm_syncsource {
    syncsource_wc = 0,
    syncsource_madi = 1,
    syncsource_tco = 2,
    syncsource_sync = 3,
    syncsource_none = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_status_madi {
    pub sync_wc: u8,
    pub sync_madi: u8,
    pub sync_tco: u8,
    pub sync_in: u8,
    pub madi_input: u8,
    pub channel_format: u8,
    pub frame_format: u8,
}

#[repr(C)]
pub union hdspm_status_card_specific { pub madi: hdspm_status_madi }

#[repr(C)]
pub struct hdspm_status {
    pub card_type: u8,
    pub autosync_source: hdspm_syncsource,
    pub card_clock: u64,
    pub master_period: u32,
    pub card_specific: hdspm_status_card_specific,
}

pub const HDSPM_ADDON_TCO: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_version {
    pub card_type: u8,
    pub cardname: [core::ffi::c_char; 20],
    pub serial: u32,
    pub firmware_rev: u16,
    pub addons: i32,
}

pub const HDSPM_MIXER_CHANNELS: usize = HDSPM_MAX_CHANNELS;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_channelfader {
    pub in_: [u32; HDSPM_MIXER_CHANNELS],
    pub pb: [u32; HDSPM_MIXER_CHANNELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_mixer { pub ch: [hdspm_channelfader; HDSPM_MIXER_CHANNELS] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_mixer_ioctl { pub mixer: *mut hdspm_mixer }

/* IOCTL values use the platform _IOR macro from the original header:
 * GET_PEAK_RMS (0x42), GET_CONFIG (0x41), GET_LTC (0x46), GET_STATUS
 * (0x47), GET_VERSION (0x48), and GET_MIXER (0x44). */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
