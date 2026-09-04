// SPDX-License-Identifier: GPL-2.0
/*
 *   Focusrite Scarlett 2 Protocol Driver for ALSA
 *   (including Scarlett 2nd Gen, 3rd Gen, 4th Gen, Clarett USB, and
 *   Clarett+ series products)
 *
 *   Supported models:
 *   - 6i6/18i8/18i20 Gen 2
 *   - Solo/2i2/4i4/8i6/18i8/18i20 Gen 3
 *   - Solo/2i2/4i4 Gen 4
 *   - Clarett 2Pre/4Pre/8Pre USB
 *   - Clarett+ 2Pre/4Pre/8Pre
 *   - Vocaster One/Two
 *
 *   Copyright (c) 2018-2025 by Geoffrey D. Bennett <g at b4.vu>
 *   Copyright (c) 2020-2021 by Vladimir Sadovnikov <sadko4u@gmail.com>
 *   Copyright (c) 2022 by Christian Colglazier <christian@cacolglazier.com>
 *
 *   Based on the Scarlett (Gen 1) Driver for ALSA:
 *
 *   Copyright (c) 2013 by Tobias Hoffmann
 *   Copyright (c) 2013 by Robin Gareus <robin at gareus.org>
 *   Copyright (c) 2002 by Takashi Iwai <tiwai at suse.de>
 *   Copyright (c) 2014 by Chris J Arges <chris.j.arges at canonical.com>
 *
 *   Many codes borrowed from audio.c by
 *     Alan Cox (alan at lxorguk.ukuu.org.uk)
 *     Thomas Sailer (sailer at ife.ee.ethz.ch)
 *
 *   Code cleanup:
 *   David Henningsson <david.henningsson at canonical.com>
 */

/* The protocol was reverse engineered by looking at the communication
 * between Focusrite Control 2.3.4 and the Focusrite(R) Scarlett 18i20
 * (firmware 1083) using usbmon in July-August 2018.
 *
 * Scarlett 18i8 support added in April 2019.
 *
 * Scarlett 6i6 support added in June 2019 (thanks to Martin Wittmann
 * for providing usbmon output and testing).
 *
 * Scarlett 4i4/8i6 Gen 3 support added in May 2020 (thanks to Laurent
 * Debricon for donating a 4i4 and to Fredrik Unger for providing 8i6
 * usbmon output and testing).
 *
 * Scarlett 18i8/18i20 Gen 3 support added in June 2020 (thanks to
 * Darren Jaeckel, Alex Sedlack, and Clovis Lunel for providing usbmon
 * output, protocol traces and testing).
 *
 * Support for loading mixer volume and mux configuration from the
 * interface during driver initialisation added in May 2021 (thanks to
 * Vladimir Sadovnikov for figuring out how).
 *
 * Support for Solo/2i2 Gen 3 added in May 2021 (thanks to Alexander
 * Vorona for 2i2 protocol traces).
 *
 * Support for phantom power, direct monitoring, speaker switching,
 * and talkback added in May-June 2021.
 *
 * Support for Clarett+ 8Pre added in Aug 2022 by Christian
 * Colglazier.
 *
 * Support for Clarett 8Pre USB added in Sep 2023 (thanks to Philippe
 * Perrot for confirmation).
 *
 * Support for Clarett+ 4Pre and 2Pre added in Sep 2023 (thanks to
 * Gregory Rozzo for donating a 4Pre, and David Sherwood and Patrice
 * Peterson for usbmon output).
 *
 * Support for Clarett 2Pre and 4Pre USB added in Oct 2023.
 *
 * Support for firmware updates added in Dec 2023.
 *
 * Support for Scarlett Solo/2i2/4i4 Gen 4 added in Dec 2023 (thanks
 * to many LinuxMusicians people and to Focusrite for hardware
 * donations).
 *
 * Support for Vocaster One and Two added in Mar 2024 (thanks to many
 * LinuxMusicians people and to Focusrite for hardware donations).
 *
 * This ALSA mixer gives access to (model-dependent):
 *  - input, output, mixer-matrix muxes
 *  - mixer-matrix gain stages
 *  - gain/volume/mute controls
 *  - level meters
 *  - line/inst level, pad, and air controls
 *  - phantom power, direct monitor, speaker switching, and talkback
 *    controls
 *  - disable/enable MSD mode
 *  - disable/enable standalone mode
 *  - input mute, gain, autogain, safe mode
 *  - direct monitor mixes
 *  - compressor and EQ
 *  - Bluetooth volume
 *
 * Gen 3/4 devices have a Mass Storage Device (MSD) mode where a small
 * disk with registration and driver download information is presented
 * to the host. To access the full functionality of the device without
 * proprietary software, MSD mode can be disabled by:
 * - holding down the 48V button for five seconds while powering on
 *   the device, or
 * - using this driver and alsamixer to change the "MSD Mode" setting
 *   to Off and power-cycling the device
 */

use core::ffi::{c_char, c_void};

#[link(name = "c")]
extern "C" {}

// Device setup value to allow turning MSD mode back on
pub const SCARLETT2_MSD_ENABLE: u32 = 0x02;

// Device setup value to disable this mixer driver
pub const SCARLETT2_DISABLE: u32 = 0x04;

// Device setup value to use the FCP driver instead
pub const SCARLETT2_USE_FCP_DRIVER: u32 = 0x08;

// Some GUI mixers can't handle negative ctl values
pub const SCARLETT2_VOLUME_BIAS: u32 = 127;

// Maximum preamp input gain value (the corresponding value in dB is per-device)
pub const SCARLETT2_MAX_GAIN_VALUE: u32 = 70;

// Maximum Bluetooth volume value
pub const SCARLETT2_MAX_BLUETOOTH_VOLUME: u32 = 30;

// Maximum front-panel sleep time in seconds (24 hours)
pub const SCARLETT2_MAX_FP_SLEEP_TIME: u32 = 86400;

// Mixer range from -80dB to +12dB in 0.5dB steps
pub const SCARLETT2_MIXER_MIN_DB: i32 = -80;
pub const SCARLETT2_MIXER_BIAS: u32 = (-SCARLETT2_MIXER_MIN_DB as u32) * 2;
pub const SCARLETT2_MIXER_MAX_DB: i32 = 12;
pub const SCARLETT2_MIXER_MAX_VALUE: u32 = ((SCARLETT2_MIXER_MAX_DB - SCARLETT2_MIXER_MIN_DB) as u32) * 2;
pub const SCARLETT2_MIXER_VALUE_COUNT: usize = (SCARLETT2_MIXER_MAX_VALUE as usize) + 1;

// Map from (dB + 80) * 2 to mixer value
// for dB in 0 .. 184: int(8192 * pow(10, ((dB - 160) / 2 / 20)))
pub static SCARLETT2_MIXER_VALUES: [u16; SCARLETT2_MIXER_VALUE_COUNT] = [
	0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
	2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 8, 8,
	9, 9, 10, 10, 11, 12, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
	23, 24, 25, 27, 29, 30, 32, 34, 36, 38, 41, 43, 46, 48, 51,
	54, 57, 61, 65, 68, 73, 77, 81, 86, 91, 97, 103, 109, 115,
	122, 129, 137, 145, 154, 163, 173, 183, 194, 205, 217, 230,
	244, 259, 274, 290, 307, 326, 345, 365, 387, 410, 434, 460,
	487, 516, 547, 579, 614, 650, 689, 730, 773, 819, 867, 919,
	973, 1031, 1092, 1157, 1225, 1298, 1375, 1456, 1543, 1634,
	1731, 1833, 1942, 2057, 2179, 2308, 2445, 2590, 2744, 2906,
	3078, 3261, 3454, 3659, 3876, 4105, 4349, 4606, 4879, 5168,
	5475, 5799, 6143, 6507, 6892, 7301, 7733, 8192, 8677, 9191,
	9736, 10313, 10924, 11571, 12257, 12983, 13752, 14567, 15430,
	16345, 17313, 18339, 19426, 20577, 21796, 23088, 24456, 25905,
	27440, 29066, 30788, 32612
];

// Maximum number of analogue outputs
pub const SCARLETT2_ANALOGUE_MAX: usize = 10;

// Maximum number of various input controls
pub const SCARLETT2_LEVEL_SWITCH_MAX: usize = 2;
pub const SCARLETT2_PAD_SWITCH_MAX: usize = 8;
pub const SCARLETT2_AIR_SWITCH_MAX: usize = 8;
pub const SCARLETT2_DSP_SWITCH_MAX: usize = 2;
pub const SCARLETT2_INPUT_MUTE_SWITCH_MAX: usize = 2;
pub const SCARLETT2_PHANTOM_SWITCH_MAX: usize = 2;
pub const SCARLETT2_INPUT_GAIN_MAX: usize = 2;

// Maximum number of inputs to the mixer
pub const SCARLETT2_INPUT_MIX_MAX: usize = 25;

// Maximum number of outputs from the mixer
pub const SCARLETT2_OUTPUT_MIX_MAX: usize = 12;

// Maximum number of mixer gain controls
pub const SCARLETT2_MIX_MAX: usize = SCARLETT2_INPUT_MIX_MAX * SCARLETT2_OUTPUT_MIX_MAX;

// Maximum number of direct monitor mixer gain controls
// 1 (Solo) or 2 (2i2) direct monitor selections (Mono & Stereo)
// 2 Mix outputs (A/Left & B/Right)
// 4 Mix inputs
pub const SCARLETT2_MONITOR_MIX_MAX: usize = 2 * 2 * 4;

// Maximum size of the data in the USB mux assignment message:
// 20 inputs, 20 outputs, 25 matrix inputs, 12 spare
pub const SCARLETT2_MUX_MAX: usize = 77;

// Maximum number of sources (sum of input port counts)
pub const SCARLETT2_MAX_SRCS: usize = 52;

// Maximum number of meters (sum of output port counts)
pub const SCARLETT2_MAX_METERS: usize = 65;

// Compressor parameter data
// The compressor parameters are 32-bit fixed point values with 24
// bits of fraction. Integer values are sufficient for the parameters
// except for ratio which we can set in 0.5:1 steps.
#[repr(C)]
pub struct CompressorParam {
    pub name: *const c_char,
    pub typ: u32, // snd_ctl_elem_type_t
    pub min: i32,
    pub max: i32,
    pub scale_bits: i32,
}

// The available compressor parameters on the Vocaster:
// - Enable: Off, On
// - Threshold: -40dB to 0dB
// - Ratio: 1:1 to 50:1 in 0.5:1 steps
// - Knee Width: 0dB to 10dB
// - Attack: 30ms to 127ms
// - Release: 30ms to 127ms
// - Makeup Gain: 0dB to 24dB
pub static COMPRESSOR_PARAMS: [CompressorParam; 7] = [
    CompressorParam {
        name: b"Enable\0" as *const u8 as *const c_char,
        typ: 1, // SNDRV_CTL_ELEM_TYPE_BOOLEAN
        min: 0,
        max: 1,
        scale_bits: 0,
    },
    CompressorParam {
        name: b"Threshold\0" as *const u8 as *const c_char,
        typ: 2, // SNDRV_CTL_ELEM_TYPE_INTEGER
        min: -40,
        max: 0,
        scale_bits: 24,
    },
    CompressorParam {
        name: b"Ratio\0" as *const u8 as *const c_char,
        typ: 2, // SNDRV_CTL_ELEM_TYPE_INTEGER
        min: 2,
        max: 100,
        scale_bits: 23,
    },
    CompressorParam {
        name: b"Knee Width\0" as *const u8 as *const c_char,
        typ: 2, // SNDRV_CTL_ELEM_TYPE_INTEGER
        min: 0,
        max: 10,
        scale_bits: 24,
    },
    CompressorParam {
        name: b"Attack\0" as *const u8 as *const c_char,
        typ: 2, // SNDRV_CTL_ELEM_TYPE_INTEGER
        min: 30,
        max: 127,
        scale_bits: 24,
    },
    CompressorParam {
        name: b"Release\0" as *const u8 as *const c_char,
        typ: 2, // SNDRV_CTL_ELEM_TYPE_INTEGER
        min: 30,
        max: 127,
        scale_bits: 24,
    },
    CompressorParam {
        name: b"Makeup Gain\0" as *const u8 as *const c_char,
        typ: 2, // SNDRV_CTL_ELEM_TYPE_INTEGER
        min: 0,
        max: 24,
        scale_bits: 24,
    },
];

pub const SCARLETT2_COMPRESSOR_PARAM_COUNT: usize = 7;
pub const SCARLETT2_COMPRESSOR_CTLS_MAX: usize = SCARLETT2_COMPRESSOR_PARAM_COUNT * SCARLETT2_DSP_SWITCH_MAX;

// Maximum number of filter controls
pub const SCARLETT2_PRECOMP_FLT_CTLS_MAX: usize = 2 * SCARLETT2_DSP_SWITCH_MAX;
pub const SCARLETT2_PEQ_FLT_CTLS_MAX: usize = 3 * SCARLETT2_DSP_SWITCH_MAX;

// Number of biquad filter coefficients
pub const SCARLETT2_BIQUAD_COEFFS: usize = 5;

// Maximum number of filter coefficient values
pub const SCARLETT2_PRECOMP_FLT_VALUES_MAX: usize = SCARLETT2_PRECOMP_FLT_CTLS_MAX * SCARLETT2_BIQUAD_COEFFS;
pub const SCARLETT2_PEQ_FLT_VALUES_MAX: usize = SCARLETT2_PEQ_FLT_CTLS_MAX * SCARLETT2_BIQUAD_COEFFS;

// Maximum number of PEQ filter slots
pub const SCARLETT2_PEQ_FLT_SLOTS_MAX: usize = 4;

// Hardware port types:
// - None (no input to mux)
// - Analogue I/O
// - S/PDIF I/O
// - ADAT I/O
// - Mixer I/O
// - PCM I/O
pub const SCARLETT2_PORT_TYPE_NONE: u8 = 0;
pub const SCARLETT2_PORT_TYPE_ANALOGUE: u8 = 1;
pub const SCARLETT2_PORT_TYPE_SPDIF: u8 = 2;
pub const SCARLETT2_PORT_TYPE_ADAT: u8 = 3;
pub const SCARLETT2_PORT_TYPE_MIX: u8 = 4;
pub const SCARLETT2_PORT_TYPE_PCM: u8 = 5;
pub const SCARLETT2_PORT_TYPE_COUNT: usize = 6;

// I/O count of each port type kept in struct scarlett2_ports
pub const SCARLETT2_PORT_IN: usize = 0;
pub const SCARLETT2_PORT_OUT: usize = 1;
pub const SCARLETT2_PORT_DIRNS: usize = 2;

// Dim/Mute buttons on the 18i20
pub const SCARLETT2_BUTTON_MUTE: usize = 0;
pub const SCARLETT2_BUTTON_DIM: usize = 1;
pub const SCARLETT2_DIM_MUTE_COUNT: usize = 2;

// Autogain target values
pub const SCARLETT2_AG_TARGET_MIN: i32 = -30;

pub const SCARLETT2_AG_HOT_TARGET: usize = 0;
pub const SCARLETT2_AG_MEAN_TARGET: usize = 1;
pub const SCARLETT2_AG_PEAK_TARGET: usize = 2;
pub const SCARLETT2_AG_TARGET_COUNT: usize = 3;

// Flash Write State
pub const SCARLETT2_FLASH_WRITE_STATE_IDLE: u8 = 0;
pub const SCARLETT2_FLASH_WRITE_STATE_SELECTED: u8 = 1;
pub const SCARLETT2_FLASH_WRITE_STATE_ERASING: u8 = 2;
pub const SCARLETT2_FLASH_WRITE_STATE_WRITE: u8 = 3;

pub static SCARLETT2_DIM_MUTE_NAMES: [*const c_char; SCARLETT2_DIM_MUTE_COUNT] = [
    b"Mute Playback Switch\0" as *const u8 as *const c_char,
    b"Dim Playback Switch\0" as *const u8 as *const c_char,
];

// Vocaster One speaker/headphone mute names
pub static VOCASTER_ONE_SP_HP_MUTE_NAMES: [*const c_char; 3] = [
    b"Speaker Mute Playback Switch\0" as *const u8 as *const c_char,
    b"Headphones Mute Playback Switch\0" as *const u8 as *const c_char,
    core::ptr::null(),
];

// Vocaster Two speaker/headphone mute names
pub static VOCASTER_TWO_SP_HP_MUTE_NAMES: [*const c_char; 4] = [
    b"Speaker Mute Playback Switch\0" as *const u8 as *const c_char,
    b"Headphones 1 Mute Playback Switch\0" as *const u8 as *const c_char,
    b"Headphones 2 Mute Playback Switch\0" as *const u8 as *const c_char,
    core::ptr::null(),
];

// The autogain_status is set based on the autogain_switch and
// raw_autogain_status values.
//
// If autogain_switch is set, autogain_status is set to 0 (Running).
// The other status values are from the raw_autogain_status value + 1.
pub static SCARLETT2_AUTOGAIN_STATUS_GEN4: [*const c_char; 11] = [
    b"Running\0" as *const u8 as *const c_char,
    b"Success\0" as *const u8 as *const c_char,
    b"SuccessDRover\0" as *const u8 as *const c_char,
    b"WarnMinGainLimit\0" as *const u8 as *const c_char,
    b"FailDRunder\0" as *const u8 as *const c_char,
    b"FailMaxGainLimit\0" as *const u8 as *const c_char,
    b"FailClipped\0" as *const u8 as *const c_char,
    b"Cancelled\0" as *const u8 as *const c_char,
    b"Root\0" as *const u8 as *const c_char,
    b"Invalid\0" as *const u8 as *const c_char,
    core::ptr::null(),
];

pub static SCARLETT2_AUTOGAIN_STATUS_VOCASTER: [*const c_char; 9] = [
    b"Running\0" as *const u8 as *const c_char,
    b"Success\0" as *const u8 as *const c_char,
    b"FailPG\0" as *const u8 as *const c_char,
    b"FailRange\0" as *const u8 as *const c_char,
    b"WarnMaxCap\0" as *const u8 as *const c_char,
    b"WarnMinCap\0" as *const u8 as *const c_char,
    b"Cancelled\0" as *const u8 as *const c_char,
    b"Invalid\0" as *const u8 as *const c_char,
    core::ptr::null(),
];

// Power Status Values
pub const SCARLETT2_POWER_STATUS_EXT: usize = 0;
pub const SCARLETT2_POWER_STATUS_BUS: usize = 1;
pub const SCARLETT2_POWER_STATUS_FAIL: usize = 2;
pub const SCARLETT2_POWER_STATUS_COUNT: usize = 3;

// Notification callback functions
#[repr(C)]
pub struct Scarlett2Notification {
    pub mask: u32,
    pub func: Option<unsafe extern "C" fn(*mut c_void)>,
}

// Forward declarations for notification functions (external dependencies)
extern "C" {
    fn scarlett2_notify_ack(mixer: *mut c_void);
    fn scarlett2_notify_sync(mixer: *mut c_void);
    fn scarlett2_notify_dim_mute(mixer: *mut c_void);
    fn scarlett2_notify_monitor(mixer: *mut c_void);
    fn scarlett2_notify_volume(mixer: *mut c_void);
    fn scarlett2_notify_input_level(mixer: *mut c_void);
    fn scarlett2_notify_input_pad(mixer: *mut c_void);
    fn scarlett2_notify_input_air(mixer: *mut c_void);
    fn scarlett2_notify_input_dsp(mixer: *mut c_void);
    fn scarlett2_notify_input_mute(mixer: *mut c_void);
    fn scarlett2_notify_input_phantom(mixer: *mut c_void);
    fn scarlett2_notify_input_other(mixer: *mut c_void);
    fn scarlett2_notify_input_select(mixer: *mut c_void);
    fn scarlett2_notify_input_gain(mixer: *mut c_void);
    fn scarlett2_notify_autogain(mixer: *mut c_void);
    fn scarlett2_notify_input_safe(mixer: *mut c_void);
    fn scarlett2_notify_monitor_other(mixer: *mut c_void);
    fn scarlett2_notify_direct_monitor(mixer: *mut c_void);
    fn scarlett2_notify_power_status(mixer: *mut c_void);
    fn scarlett2_notify_pcm_input_switch(mixer: *mut c_void);
    fn scarlett2_notify_bluetooth(mixer: *mut c_void);
}

// Configuration parameters that can be read and written
pub const SCARLETT2_CONFIG_DIM_MUTE: usize = 0;
pub const SCARLETT2_CONFIG_LINE_OUT_VOLUME: usize = 1;
pub const SCARLETT2_CONFIG_MUTE_SWITCH: usize = 2;
pub const SCARLETT2_CONFIG_SW_HW_SWITCH: usize = 3;
pub const SCARLETT2_CONFIG_MASTER_VOLUME: usize = 4;
pub const SCARLETT2_CONFIG_HEADPHONE_VOLUME: usize = 5;
pub const SCARLETT2_CONFIG_LEVEL_SWITCH: usize = 6;
pub const SCARLETT2_CONFIG_PAD_SWITCH: usize = 7;
pub const SCARLETT2_CONFIG_MSD_SWITCH: usize = 8;
pub const SCARLETT2_CONFIG_AIR_SWITCH: usize = 9;
pub const SCARLETT2_CONFIG_DSP_SWITCH: usize = 10;
pub const SCARLETT2_CONFIG_COMPRESSOR_PARAMS: usize = 11;
pub const SCARLETT2_CONFIG_PRECOMP_FLT_SWITCH: usize = 12;
pub const SCARLETT2_CONFIG_PRECOMP_FLT_PARAMS: usize = 13;
pub const SCARLETT2_CONFIG_PEQ_FLT_SWITCH: usize = 14;
pub const SCARLETT2_CONFIG_PEQ_FLT_PARAMS: usize = 15;
pub const SCARLETT2_CONFIG_INPUT_MUTE_SWITCH: usize = 16;
pub const SCARLETT2_CONFIG_STANDALONE_SWITCH: usize = 17;
pub const SCARLETT2_CONFIG_PHANTOM_SWITCH: usize = 18;
pub const SCARLETT2_CONFIG_PHANTOM_PERSISTENCE: usize = 19;
pub const SCARLETT2_CONFIG_DIRECT_MONITOR: usize = 20;
pub const SCARLETT2_CONFIG_MONITOR_OTHER_SWITCH: usize = 21;
pub const SCARLETT2_CONFIG_MONITOR_OTHER_ENABLE: usize = 22;
pub const SCARLETT2_CONFIG_TALKBACK_MAP: usize = 23;
pub const SCARLETT2_CONFIG_AUTOGAIN_SWITCH: usize = 24;
pub const SCARLETT2_CONFIG_AUTOGAIN_STATUS: usize = 25;
pub const SCARLETT2_CONFIG_AG_HOT_TARGET: usize = 26;
pub const SCARLETT2_CONFIG_AG_MEAN_TARGET: usize = 27;
pub const SCARLETT2_CONFIG_AG_PEAK_TARGET: usize = 28;
pub const SCARLETT2_CONFIG_INPUT_GAIN: usize = 29;
pub const SCARLETT2_CONFIG_SAFE_SWITCH: usize = 30;
pub const SCARLETT2_CONFIG_INPUT_SELECT_SWITCH: usize = 31;
pub const SCARLETT2_CONFIG_INPUT_LINK_SWITCH: usize = 32;
pub const SCARLETT2_CONFIG_POWER_EXT: usize = 33;
pub const SCARLETT2_CONFIG_POWER_LOW: usize = 34;
pub const SCARLETT2_CONFIG_PCM_INPUT_SWITCH: usize = 35;
pub const SCARLETT2_CONFIG_DIRECT_MONITOR_GAIN: usize = 36;
pub const SCARLETT2_CONFIG_BLUETOOTH_VOLUME: usize = 37;
pub const SCARLETT2_CONFIG_SPDIF_MODE: usize = 38;
pub const SCARLETT2_CONFIG_SP_HP_MUTE: usize = 39;
pub const SCARLETT2_CONFIG_FP_BRIGHTNESS: usize = 40;
pub const SCARLETT2_CONFIG_FP_SLEEP_TIME: usize = 41;
pub const SCARLETT2_CONFIG_COUNT: usize = 42;

// Autogain target configuration parameters and names
pub static SCARLETT2_AG_TARGET_CONFIGS: [i32; SCARLETT2_AG_TARGET_COUNT] = [
    SCARLETT2_CONFIG_AG_HOT_TARGET as i32,
    SCARLETT2_CONFIG_AG_MEAN_TARGET as i32,
    SCARLETT2_CONFIG_AG_PEAK_TARGET as i32,
];

pub static SCARLETT2_AG_TARGET_NAMES: [*const c_char; SCARLETT2_AG_TARGET_COUNT] = [
    b"Hot\0" as *const u8 as *const c_char,
    b"Mean\0" as *const u8 as *const c_char,
    b"Peak\0" as *const u8 as *const c_char,
];

// Location, size, and activation command number for the configuration
// parameters. Size is in bits and may be 1, 8, 16, or 32.
//
// Vocaster and 4th Gen devices have a parameter buffer to set certain
// configuration parameters. When pbuf is set, rather than writing to
// the given offset, the channel and value are written to the
// parameter buffer and the activate command is sent to the device.
//
// Some Gen 4 configuration parameters are written with 0x02 for a
// desired value of 0x01, and 0x03 for 0x00. These are indicated with
// mute set to 1. 0x02 and 0x03 are temporary values while the device
// makes the change and the channel and/or corresponding DSP channel
// output is muted.
#[repr(C)]
pub struct Scarlett2Config {
    pub offset: u16,
    pub size: u8,
    pub activate: u8,
    pub pbuf: u8,
    pub mute: u8,
}

#[repr(C)]
pub struct Scarlett2ConfigSet {
    pub notifications: *const Scarlett2Notification,
    pub param_buf_addr: u16,
    pub input_gain_tlv: *const u32,
    pub autogain_status_texts: *const *const c_char,
    pub items: *const [Scarlett2Config; SCARLETT2_CONFIG_COUNT],
}

// Map firmware versions to config sets per-device.
//
// Each device lists one or more entries, sorted in ascending order of
// from_firmware_version. At probe time the running firmware version
// is looked up against this list and the last entry whose
// from_firmware_version is <= the running version is selected.
//
// The list is terminated by a sentinel entry with config_set == NULL.
#[repr(C)]
pub struct Scarlett2ConfigSetEntry {
    pub from_firmware_version: u16,
    pub config_set: *const Scarlett2ConfigSet,
}

// Input gain TLV dB ranges - external definitions
extern "C" {
    static db_scale_vocaster_gain: [u32; 4];
    static db_scale_gen4_gain: [u32; 4];
}

// Placeholder declarations for device info structures (defined in C code)
pub struct Scarlett2Port {
    pub id: u16,
    pub src_descr: *const c_char,
    pub src_num_offset: i32,
    pub dst_descr: *const c_char,
    pub dsp_src_descr: *const c_char,
    pub dsp_dst_descr: *const c_char,
}

pub static SCARLETT2_PORTS: [Scarlett2Port; SCARLETT2_PORT_TYPE_COUNT] = [
    Scarlett2Port {
        id: 0x000,
        src_descr: b"Off\0" as *const u8 as *const c_char,
        src_num_offset: 0,
        dst_descr: core::ptr::null(),
        dsp_src_descr: core::ptr::null(),
        dsp_dst_descr: core::ptr::null(),
    },
    Scarlett2Port {
        id: 0x080,
        src_descr: b"Analogue %d\0" as *const u8 as *const c_char,
        src_num_offset: 1,
        dst_descr: b"Analogue Output %02d Playback\0" as *const u8 as *const c_char,
        dsp_src_descr: core::ptr::null(),
        dsp_dst_descr: core::ptr::null(),
    },
    Scarlett2Port {
        id: 0x180,
        src_descr: b"S/PDIF %d\0" as *const u8 as *const c_char,
        src_num_offset: 1,
        dst_descr: b"S/PDIF Output %d Playback\0" as *const u8 as *const c_char,
        dsp_src_descr: core::ptr::null(),
        dsp_dst_descr: core::ptr::null(),
    },
    Scarlett2Port {
        id: 0x200,
        src_descr: b"ADAT %d\0" as *const u8 as *const c_char,
        src_num_offset: 1,
        dst_descr: b"ADAT Output %d Playback\0" as *const u8 as *const c_char,
        dsp_src_descr: core::ptr::null(),
        dsp_dst_descr: core::ptr::null(),
    },
    Scarlett2Port {
        id: 0x300,
        src_descr: b"Mix %c\0" as *const u8 as *const c_char,
        src_num_offset: 65, // 'A'
        dst_descr: b"Mixer Input %02d Capture\0" as *const u8 as *const c_char,
        dsp_src_descr: b"DSP %d\0" as *const u8 as *const c_char,
        dsp_dst_descr: b"DSP Input %d Capture\0" as *const u8 as *const c_char,
    },
    Scarlett2Port {
        id: 0x600,
        src_descr: b"PCM %d\0" as *const u8 as *const c_char,
        src_num_offset: 1,
        dst_descr: b"PCM %02d Capture\0" as *const u8 as *const c_char,
        dsp_src_descr: core::ptr::null(),
        dsp_dst_descr: core::ptr::null(),
    },
];

// Number of mux tables: one for each band of sample rates
// (44.1/48kHz, 88.2/96kHz, and 176.4/176kHz)
pub const SCARLETT2_MUX_TABLES: usize = 3;

// Maximum number of entries in a mux table
pub const SCARLETT2_MAX_MUX_ENTRIES: usize = 10;

// One entry within mux_assignment defines the port type and range of
// ports to add to the set_mux message. The end of the list is marked
// with count == 0.
#[repr(C)]
pub struct Scarlett2MuxEntry {
    pub port_type: u8,
    pub start: u8,
    pub count: u8,
}

// Maximum number of entries in a mux table
pub const SCARLETT2_MAX_METER_ENTRIES: usize = 9;

// One entry within meter_assignment defines the range of mux outputs
// that consecutive meter entries are mapped to. The end of the list
// is marked with count == 0.
#[repr(C)]
pub struct Scarlett2MeterEntry {
    pub start: u8,
    pub count: u8,
}

// Configuration struct for device info - external definition
pub struct Scarlett2DeviceInfo {
    pub config_sets: *const Scarlett2ConfigSetEntry,
    pub has_devmap: u8,
    pub has_speaker_switching: u8,
    pub has_talkback: u8,
    pub level_input_count: u8,
    pub level_input_first: u8,
    pub pad_input_count: u8,
    pub air_input_count: u8,
    pub air_input_first: u8,
    pub air_option: u8,
    pub dsp_input_count: u8,
    pub precomp_flt_count: u8,
    pub peq_flt_count: u8,
    pub peq_flt_total_count: u8,
    pub mute_input_count: u8,
    pub phantom_count: u8,
    pub phantom_first: u8,
    pub inputs_per_phantom: u8,
    pub gain_input_count: u8,
    pub safe_input_count: u8,
    pub direct_monitor: u8,
    pub dsp_count: u8,
    pub has_bluetooth: u8,
    pub spdif_mode_control_name: *const c_char,
    pub spdif_mode_values: *const u8,
    pub spdif_mode_texts: *const *const c_char,
    pub line_out_remap_enable: u8,
    pub line_out_remap: [u8; SCARLETT2_ANALOGUE_MAX],
    pub line_out_unmap: [u8; SCARLETT2_ANALOGUE_MAX],
    pub line_out_descrs: [*const c_char; SCARLETT2_ANALOGUE_MAX],
    pub sp_hp_mute_names: *const *const c_char,
    pub port_count: [[i32; SCARLETT2_PORT_DIRNS]; SCARLETT2_PORT_TYPE_COUNT],
    pub mux_assignment: [[Scarlett2MuxEntry; SCARLETT2_MAX_MUX_ENTRIES]; SCARLETT2_MUX_TABLES],
    pub meter_map: [Scarlett2MeterEntry; SCARLETT2_MAX_METER_ENTRIES],
}

// Data struct that holds runtime state
pub struct Scarlett2Data {
    pub mixer: *mut c_void,
    pub usb_mutex: *mut c_void, // prevent sending concurrent USB requests
    pub cmd_done: *mut c_void,
    pub urb: *mut c_void, // notification endpoint
    pub data_mutex: *mut c_void, // lock access to this data
    pub running: u8,
    pub hwdep_in_use: u8,
    pub selected_flash_segment_id: u8,
    pub flash_write_state: u8,
    pub work: *mut c_void,
    pub info: *const Scarlett2DeviceInfo,
    pub config_set: *const Scarlett2ConfigSet,
    pub series_name: *const c_char,
    pub b_interface_number: u8,
    pub b_endpoint_address: u8,
    pub w_max_packet_size: u16,
    pub b_interval: u8,
    pub num_mux_srcs: u8,
    pub num_mux_dsts: u8,
    pub num_mix_in: u8,
    pub num_mix_out: u8,
    pub num_line_out: u8,
    pub num_monitor_mix_ctls: u8,
    pub num_autogain_status_texts: u8,
    pub firmware_version: u32,
    pub flash_segment_nums: [u8; 4],
    pub flash_segment_blocks: [u8; 4],
    pub scarlett2_seq: u16,
    pub sync_updated: u8,
    pub vol_updated: u8,
    pub dim_mute_updated: u8,
    pub input_level_updated: u8,
    pub input_pad_updated: u8,
    pub input_air_updated: u8,
    pub input_dsp_updated: u8,
    pub input_mute_updated: u8,
    pub input_phantom_updated: u8,
    pub input_select_updated: u8,
    pub input_gain_updated: u8,
    pub autogain_updated: u8,
    pub input_safe_updated: u8,
    pub pcm_input_switch_updated: u8,
    pub monitor_other_updated: u8,
    pub direct_monitor_updated: u8,
    pub mux_updated: u8,
    pub mix_updated: u8,
    pub speaker_switching_switched: u8,
    pub power_status_updated: u8,
    pub bluetooth_updated: u8,
    pub sync: u8,
    pub master_vol: u8,
    pub headphone_vol: u8,
    pub vol: [u8; SCARLETT2_ANALOGUE_MAX],
    pub vol_sw_hw_switch: [u8; SCARLETT2_ANALOGUE_MAX],
    pub mute_switch: [u8; SCARLETT2_ANALOGUE_MAX],
    pub level_switch: [u8; SCARLETT2_LEVEL_SWITCH_MAX],
    pub pad_switch: [u8; SCARLETT2_PAD_SWITCH_MAX],
    pub dim_mute: [u8; SCARLETT2_DIM_MUTE_COUNT],
    pub sp_hp_mute: u8,
    pub air_switch: [u8; SCARLETT2_AIR_SWITCH_MAX],
    pub dsp_switch: [u8; SCARLETT2_DSP_SWITCH_MAX],
    pub compressor_values: [i32; SCARLETT2_COMPRESSOR_CTLS_MAX],
    pub precomp_flt_values: [i32; SCARLETT2_PRECOMP_FLT_VALUES_MAX],
    pub peq_flt_values: [i32; SCARLETT2_PEQ_FLT_VALUES_MAX],
    pub precomp_flt_switch: [u8; SCARLETT2_DSP_SWITCH_MAX],
    pub peq_flt_switch: [u8; SCARLETT2_DSP_SWITCH_MAX],
    pub input_mute_switch: [u8; SCARLETT2_INPUT_MUTE_SWITCH_MAX],
    pub phantom_switch: [u8; SCARLETT2_PHANTOM_SWITCH_MAX],
    pub phantom_persistence: u8,
    pub input_select_switch: u8,
    pub input_link_switch: [u8; SCARLETT2_INPUT_GAIN_MAX],
    pub gain: [u8; SCARLETT2_INPUT_GAIN_MAX],
    pub autogain_switch: [u8; SCARLETT2_INPUT_GAIN_MAX],
    pub autogain_status: [u8; SCARLETT2_INPUT_GAIN_MAX],
    pub ag_targets: [i8; SCARLETT2_AG_TARGET_COUNT],
    pub safe_switch: [u8; SCARLETT2_INPUT_GAIN_MAX],
    pub pcm_input_switch: u8,
    pub direct_monitor_switch: u8,
    pub speaker_switching_switch: u8,
    pub talkback_switch: u8,
    pub talkback_map: [u8; SCARLETT2_OUTPUT_MIX_MAX],
    pub msd_switch: u8,
    pub standalone_switch: u8,
    pub power_status: u8,
    pub bluetooth_volume: u8,
    pub spdif_mode: u8,
    pub meter_level_map: [u8; SCARLETT2_MAX_METERS],
}

// Segment ID constants - external definition
pub const SCARLETT2_SEGMENT_ID_COUNT: usize = 4;

// Placeholder for external function declarations
extern "C" {
    pub fn snd_scarlett2_init(mixer: *mut c_void) -> i32;
    pub fn snd_fcp_init(mixer: *mut c_void) -> i32;
    pub fn snd_scarlett2_controls_create(mixer: *mut c_void, entry: *const c_void) -> i32;
    pub fn scarlett2_hwdep_init(mixer: *mut c_void) -> i32;
    pub fn scarlett2_devmap_init(mixer: *mut c_void) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
