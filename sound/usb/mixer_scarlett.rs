// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Scarlett Driver for ALSA
 *
 *   Copyright (c) 2013 by Tobias Hoffmann
 *   Copyright (c) 2013 by Robin Gareus <robin at gareus.org>
 *   Copyright (c) 2002 by Takashi Iwai <tiwai at suse.de>
 *   Copyright (c) 2014 by Chris J Arges <chris.j.arges at canonical.com>
 *
 *   Many codes borrowed from audio.c by
 *	    Alan Cox (alan at lxorguk.ukuu.org.uk)
 *	    Thomas Sailer (sailer at ife.ee.ethz.ch)
 *
 *   Code cleanup:
 *   David Henningsson <david.henningsson at canonical.com>
 */

/*
 * Rewritten and extended to support more models, e.g. Scarlett 18i8.
 *
 * Auto-detection via UAC2 is not feasible to properly discover the vast
 * majority of features. It's related to both Linux/ALSA's UAC2 as well as
 * Focusrite's implementation of it. Eventually quirks may be sufficient but
 * right now it's a major headache to work around these things.
 *
 * NB. Neither the OSX nor the win driver provided by Focusrite performs
 * discovery, they seem to operate the same as this driver.
 */

/* Mixer Interface for the Focusrite Scarlett 18i6 audio interface.
 *
 * The protocol was reverse engineered by looking at communication between
 * Scarlett MixControl (v 1.2.128.0) and the Focusrite(R) Scarlett 18i6
 * (firmware v305) using wireshark and usbmon in January 2013.
 * Extended in July 2013.
 *
 * this mixer gives complete access to all features of the device:
 *  - change Impedance of inputs (Line-in, Mic / Instrument, Hi-Z)
 *  - select clock source
 *  - dynamic input to mixer-matrix assignment
 *  - 18 x 6 mixer-matrix gain stages
 *  - bus routing & volume control
 *  - automatic re-initialization on connect if device was power-cycled
 *
 * USB URB commands overview (bRequest = 0x01 = UAC2_CS_CUR)
 * wIndex
 * 0x01 Analog Input line/instrument impedance switch, wValue=0x0901 +
 *      channel, data=Line/Inst (2bytes)
 *      pad (-10dB) switch, wValue=0x0b01 + channel, data=Off/On (2bytes)
 *      ?? wValue=0x0803/04, ?? (2bytes)
 * 0x0a Master Volume, wValue=0x0200+bus[0:all + only 1..4?] data(2bytes)
 *      Bus Mute/Unmute wValue=0x0100+bus[0:all + only 1..4?], data(2bytes)
 * 0x28 Clock source, wValue=0x0100, data={1:int,2:spdif,3:adat} (1byte)
 * 0x29 Set Sample-rate, wValue=0x0100, data=sample-rate(4bytes)
 * 0x32 Mixer mux, wValue=0x0600 + mixer-channel, data=input-to-connect(2bytes)
 * 0x33 Output mux, wValue=bus, data=input-to-connect(2bytes)
 * 0x34 Capture mux, wValue=0...18, data=input-to-connect(2bytes)
 * 0x3c Matrix Mixer gains, wValue=mixer-node  data=gain(2bytes)
 *      ?? [sometimes](4bytes, e.g 0x000003be 0x000003bf ...03ff)
 *
 * USB reads: (i.e. actually issued by original software)
 * 0x01 wValue=0x0901+channel (1byte!!), wValue=0x0b01+channed (1byte!!)
 * 0x29 wValue=0x0100 sample-rate(4bytes)
 *      wValue=0x0200 ?? 1byte (only once)
 * 0x2a wValue=0x0100 ?? 4bytes, sample-rate2 ??
 *
 * USB reads with bRequest = 0x03 = UAC2_CS_MEM
 * 0x3c wValue=0x0002 1byte: sync status (locked=1)
 *      wValue=0x0000 18*2byte: peak meter (inputs)
 *      wValue=0x0001 8(?)*2byte: peak meter (mix)
 *      wValue=0x0003 6*2byte: peak meter (pcm/daw)
 *
 * USB write with bRequest = 0x03
 * 0x3c Save settings to hardware: wValue=0x005a, data=0xa5
 *
 *
 * <ditaa>
 *  /--------------\    18chn            6chn    /--------------\
 *  | Hardware  in +--+-------\        /------+--+ ALSA PCM out |
 *  \--------------/  |       |        |      |  \--------------/
 *                    |       |        |      |
 *                    |       v        v      |
 *                    |   +---------------+   |
 *                    |    \ Matrix  Mux /    |
 *                    |     +-----+-----+     |
 *                    |           |           |
 *                    |           | 18chn     |
 *                    |           v           |
 *                    |     +-----------+     |
 *                    |     | Mixer     |     |
 *                    |     |    Matrix |     |
 *                    |     |           |     |
 *                    |     | 18x6 Gain |     |
 *                    |     |   stages  |     |
 *                    |     +-----+-----+     |
 *                    |           |           |
 *                    |           |           |
 *                    | 18chn     | 6chn      | 6chn
 *                    v           v           v
 *                    =========================
 *             +---------------+     +--—------------+
 *              \ Output  Mux /       \ Capture Mux /
 *               +-----+-----+         +-----+-----+
 *                     |                     |
 *                     | 6chn                |
 *                     v                     |
 *              +-------------+              |
 *              | Master Gain |              |
 *              +------+------+              |
 *                     |                     |
 *                     | 6chn                | 18chn
 *                     | (3 stereo pairs)    |
 *  /--------------\   |                     |   /--------------\
 *  | Hardware out |<--/                     \-->| ALSA PCM  in |
 *  \--------------/                             \--------------/
 * </ditaa>
 *
 */

// Corresponds to: #include <linux/slab.h>, <linux/usb.h>, <linux/usb/audio-v2.h>
// #include <sound/core.h>, <sound/control.h>, <sound/tlv.h>
// #include "usbaudio.h", "mixer.h", "helper.h", "power.h", "mixer_scarlett.h"
// These are external kernel driver dependencies, declared in module scope.

const SND_SCARLETT_LEVEL_BIAS: usize = 128;
const SND_SCARLETT_MATRIX_IN_MAX: usize = 18;
const SND_SCARLETT_CONTROLS_MAX: usize = 14;
const SND_SCARLETT_OFFSETS_MAX: usize = 5;

#[repr(u32)]
pub enum ControlType {
    ScarlettOutputs = 0,
    ScarlettSwitchImpedance = 1,
    ScarlettSwitchPad = 2,
    ScarlettSwitchGain = 3,
    ForteInputSource = 4,
    ForteInputHpf = 5,
    ForteInputPhantom = 6,
    ForteInputPhase = 7,
    ForteInputPad = 8,
    ForteInputGain = 9,
}

#[repr(u32)]
pub enum ScarlettOffset {
    Pcm = 0,
    Analog = 1,
    Spdif = 2,
    Adat = 3,
    Mix = 4,
}

#[repr(C)]
pub struct ScarlettMixerElemEnumInfo {
    pub start: i32,
    pub len: i32,
    pub offsets: [i32; SND_SCARLETT_OFFSETS_MAX],
    pub names: *const *const u8,
}

#[repr(C)]
pub struct ScarlettMixerControl {
    pub num: u8,
    pub control_type: u8,
    pub name: *const u8,
}

#[repr(C)]
pub struct ScarlettDeviceInfo {
    pub matrix_in: i32,
    pub matrix_out: i32,
    pub input_len: i32,
    pub output_len: i32,
    pub has_output_source_routing: bool,
    pub opt_master: ScarlettMixerElemEnumInfo,
    pub opt_matrix: ScarlettMixerElemEnumInfo,
    pub matrix_mux_init: [i32; SND_SCARLETT_MATRIX_IN_MAX],
    pub num_controls: i32,
    pub controls: [ScarlettMixerControl; SND_SCARLETT_CONTROLS_MAX],
}

// Enum Strings

static OPT_PAD: ScarlettMixerElemEnumInfo = ScarlettMixerElemEnumInfo {
    start: 0,
    len: 2,
    offsets: [0, 0, 0, 0, 0],
    names: &[b"0dB\0" as *const u8, b"-10dB\0" as *const u8] as *const [*const u8; 2] as *const *const u8,
};

static OPT_GAIN: ScarlettMixerElemEnumInfo = ScarlettMixerElemEnumInfo {
    start: 0,
    len: 2,
    offsets: [0, 0, 0, 0, 0],
    names: &[b"Lo\0" as *const u8, b"Hi\0" as *const u8] as *const [*const u8; 2] as *const *const u8,
};

static OPT_IMPEDANCE: ScarlettMixerElemEnumInfo = ScarlettMixerElemEnumInfo {
    start: 0,
    len: 2,
    offsets: [0, 0, 0, 0, 0],
    names: &[b"Line\0" as *const u8, b"Hi-Z\0" as *const u8] as *const [*const u8; 2] as *const *const u8,
};

static OPT_CLOCK: ScarlettMixerElemEnumInfo = ScarlettMixerElemEnumInfo {
    start: 1,
    len: 3,
    offsets: [0, 0, 0, 0, 0],
    names: &[b"Internal\0" as *const u8, b"SPDIF\0" as *const u8, b"ADAT\0" as *const u8] as *const [*const u8; 3] as *const *const u8,
};

static OPT_SYNC: ScarlettMixerElemEnumInfo = ScarlettMixerElemEnumInfo {
    start: 0,
    len: 2,
    offsets: [0, 0, 0, 0, 0],
    names: &[b"No Lock\0" as *const u8, b"Locked\0" as *const u8] as *const [*const u8; 2] as *const *const u8,
};

static OPT_FORTE_SOURCE: ScarlettMixerElemEnumInfo = ScarlettMixerElemEnumInfo {
    start: 0,
    len: 3,
    offsets: [0, 0, 0, 0, 0],
    names: &[b"Mic\0" as *const u8, b"Line\0" as *const u8, b"Inst\0" as *const u8] as *const [*const u8; 3] as *const *const u8,
};

const FORTE_INPUT_GAIN_MAX: i32 = 42;

/*
 * Forte-specific USB control functions
 * Forte input controls use bRequest=0x03 (UAC2_CS_MEM) instead of 0x01
 * wValue = (control_code << 8) | channel
 * wIndex = interface | (0x3c << 8) like Scarlett meter/matrix controls
 */
unsafe fn forte_set_ctl_value(elem: *mut core::ffi::c_void, value: i32) -> i32 {
    // TODO: Requires external USB mixer interface definitions and snd_usb_* functions
    0
}

unsafe fn forte_get_ctl_value(elem: *mut core::ffi::c_void, value: *mut i32) -> i32 {
    // TODO: Requires external USB mixer interface definitions
    if !value.is_null() {
        *value = 0;
    }
    0
}

/*
 * Forte Input Gain control functions
 * Gain range is 0-42 (0x00-0x2a) which maps to approximately:
 * - Mic: 0 to +75dB (~1.8dB per step)
 * - Instrument: +14 to +68dB
 * - Line: -12 to +42dB
 * We use a TLV scale of 0 to 7500 centidB (0 to 75dB) in ~179 cB steps
 */

unsafe fn forte_input_gain_info(
    kctl: *mut core::ffi::c_void,
    uinfo: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires snd_ctl_elem_info structure and SNDRV_CTL_ELEM_TYPE_INTEGER
    0
}

unsafe fn forte_input_gain_get(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires usb_mixer_elem_info and snd_ctl_elem_value structures
    0
}

unsafe fn forte_input_gain_put(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures and clamp function
    0
}

unsafe fn forte_input_gain_resume(list: *mut core::ffi::c_void) -> i32 {
    // TODO: Requires mixer_elem_list_to_info conversion
    0
}

/*
 * Forte-specific enum control functions (for Source selection)
 * Uses bRequest=0x03 (UAC2_CS_MEM) instead of standard 0x01
 */

unsafe fn forte_ctl_enum_get(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures
    0
}

unsafe fn forte_ctl_enum_put(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures
    0
}

unsafe fn forte_ctl_enum_resume(list: *mut core::ffi::c_void) -> i32 {
    // TODO: Requires mixer_elem_list_to_info conversion
    0
}

/*
 * Forte-specific switch control functions (for HPF, 48V, Phase, Pad)
 * Uses bRequest=0x03 (UAC2_CS_MEM) instead of standard 0x01
 */

unsafe fn forte_ctl_switch_get(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures
    0
}

unsafe fn forte_ctl_switch_put(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures
    0
}

unsafe fn forte_ctl_switch_resume(list: *mut core::ffi::c_void) -> i32 {
    // TODO: Requires mixer_elem_list_to_info conversion
    0
}

unsafe fn scarlett_ctl_switch_info(
    kctl: *mut core::ffi::c_void,
    uinfo: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures and SNDRV_CTL_ELEM_TYPE_BOOLEAN
    0
}

unsafe fn scarlett_ctl_switch_get(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures and snd_usb_get_cur_mix_value
    0
}

unsafe fn scarlett_ctl_switch_put(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures
    0
}

unsafe fn scarlett_ctl_resume(list: *mut core::ffi::c_void) -> i32 {
    // TODO: Requires mixer_elem_list_to_info and snd_usb_set_cur_mix_value
    0
}

unsafe fn scarlett_ctl_info(
    kctl: *mut core::ffi::c_void,
    uinfo: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures
    0
}

unsafe fn scarlett_ctl_get(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures
    0
}

unsafe fn scarlett_ctl_put(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures
    0
}

fn scarlett_generate_name(
    i: i32,
    dst: *mut u8,
    size: usize,
    offsets: &[i32; 5],
) {
    // This would require snprintf/scnprintf from the kernel or FFI
    // Placeholder for name generation logic
    if i > offsets[ScarlettOffset::Mix as usize] {
        // Mix %c
    } else if i > offsets[ScarlettOffset::Adat as usize] {
        // ADAT %d
    } else if i > offsets[ScarlettOffset::Spdif as usize] {
        // SPDIF %d
    } else if i > offsets[ScarlettOffset::Analog as usize] {
        // Analog %d
    } else if i > offsets[ScarlettOffset::Pcm as usize] {
        // PCM %d
    } else {
        // Off
    }
}

unsafe fn scarlett_ctl_enum_dynamic_info(
    kctl: *mut core::ffi::c_void,
    uinfo: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures
    0
}

unsafe fn scarlett_ctl_enum_info(
    kctl: *mut core::ffi::c_void,
    uinfo: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires snd_ctl_enum_info
    0
}

unsafe fn scarlett_ctl_enum_get(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures
    0
}

unsafe fn scarlett_ctl_enum_put(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures
    0
}

unsafe fn scarlett_ctl_enum_resume(list: *mut core::ffi::c_void) -> i32 {
    // TODO: Requires mixer_elem_list_to_info
    0
}

unsafe fn scarlett_ctl_meter_get(
    kctl: *mut core::ffi::c_void,
    ucontrol: *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires external structures and USB control message
    0
}

// Control structure definitions - external snd_kcontrol_new structures would be defined here
// Placeholder for static control definitions

unsafe fn add_new_ctl(
    mixer: *mut core::ffi::c_void,
    ncontrol: *const core::ffi::c_void,
    resume: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    index: i32,
    offset: i32,
    num: i32,
    val_type: i32,
    channels: i32,
    name: *const u8,
    opt: *const ScarlettMixerElemEnumInfo,
    elem_ret: *mut *mut core::ffi::c_void,
) -> i32 {
    // TODO: Requires kzalloc, snd_ctl_new1, snd_usb_mixer_add_control
    0
}

unsafe fn add_output_ctls(
    mixer: *mut core::ffi::c_void,
    index: i32,
    name: *const u8,
    info: *const ScarlettDeviceInfo,
) -> i32 {
    // TODO: Requires snprintf and add_new_ctl
    0
}

// Device-specific configurations

static FORTE_INFO: ScarlettDeviceInfo = ScarlettDeviceInfo {
    matrix_in: 6,
    matrix_out: 4,
    input_len: 2,
    output_len: 4,
    has_output_source_routing: false,
    opt_master: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 13,
        offsets: [0, 4, 6, 6, 6],
        names: core::ptr::null(),
    },
    opt_matrix: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 7,
        offsets: [0, 4, 6, 6, 6],
        names: core::ptr::null(),
    },
    matrix_mux_init: [0; SND_SCARLETT_MATRIX_IN_MAX],
    num_controls: 14,
    controls: [
        ScarlettMixerControl {
            num: 0,
            control_type: 0,
            name: b"Line Out\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 0,
            name: b"Headphone\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 9,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 4,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 5,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 6,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 7,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 8,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 9,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 4,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 5,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 6,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 7,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 8,
            name: core::ptr::null(),
        },
    ],
};

// untested...
static S6I6_INFO: ScarlettDeviceInfo = ScarlettDeviceInfo {
    matrix_in: 18,
    matrix_out: 8,
    input_len: 6,
    output_len: 6,
    has_output_source_routing: true,
    opt_master: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 27,
        offsets: [0, 12, 16, 18, 18],
        names: core::ptr::null(),
    },
    opt_matrix: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 19,
        offsets: [0, 12, 16, 18, 18],
        names: core::ptr::null(),
    },
    matrix_mux_init: [
        12, 13, 14, 15, 16, 17, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
    ],
    num_controls: 9,
    controls: [
        ScarlettMixerControl {
            num: 0,
            control_type: 0,
            name: b"Monitor\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 0,
            name: b"Headphone\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 0,
            name: b"SPDIF\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 1,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 2,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 1,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 2,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 3,
            control_type: 3,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 4,
            control_type: 3,
            name: core::ptr::null(),
        },
    ],
};

// untested...
static S8I6_INFO: ScarlettDeviceInfo = ScarlettDeviceInfo {
    matrix_in: 18,
    matrix_out: 6,
    input_len: 8,
    output_len: 6,
    has_output_source_routing: true,
    opt_master: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 25,
        offsets: [0, 12, 16, 18, 18],
        names: core::ptr::null(),
    },
    opt_matrix: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 19,
        offsets: [0, 12, 16, 18, 18],
        names: core::ptr::null(),
    },
    matrix_mux_init: [
        12, 13, 14, 15, 16, 17, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
    ],
    num_controls: 7,
    controls: [
        ScarlettMixerControl {
            num: 0,
            control_type: 0,
            name: b"Monitor\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 0,
            name: b"Headphone\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 0,
            name: b"SPDIF\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 1,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 1,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 3,
            control_type: 2,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 4,
            control_type: 2,
            name: core::ptr::null(),
        },
    ],
};

static S18I6_INFO: ScarlettDeviceInfo = ScarlettDeviceInfo {
    matrix_in: 18,
    matrix_out: 6,
    input_len: 18,
    output_len: 6,
    has_output_source_routing: true,
    opt_master: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 31,
        offsets: [0, 6, 14, 16, 24],
        names: core::ptr::null(),
    },
    opt_matrix: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 25,
        offsets: [0, 6, 14, 16, 24],
        names: core::ptr::null(),
    },
    matrix_mux_init: [6, 7, 8, 9, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 14, 15, 0, 1],
    num_controls: 5,
    controls: [
        ScarlettMixerControl {
            num: 0,
            control_type: 0,
            name: b"Monitor\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 0,
            name: b"Headphone\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 0,
            name: b"SPDIF\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 1,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 1,
            name: core::ptr::null(),
        },
    ],
};

static S18I8_INFO: ScarlettDeviceInfo = ScarlettDeviceInfo {
    matrix_in: 18,
    matrix_out: 8,
    input_len: 18,
    output_len: 8,
    has_output_source_routing: true,
    opt_master: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 35,
        offsets: [0, 8, 16, 18, 26],
        names: core::ptr::null(),
    },
    opt_matrix: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 27,
        offsets: [0, 8, 16, 18, 26],
        names: core::ptr::null(),
    },
    matrix_mux_init: [8, 9, 10, 11, 12, 13, 14, 15, 18, 19, 20, 21, 22, 23, 16, 17, 0, 1],
    num_controls: 10,
    controls: [
        ScarlettMixerControl {
            num: 0,
            control_type: 0,
            name: b"Monitor\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 0,
            name: b"Headphone 1\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 0,
            name: b"Headphone 2\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 3,
            control_type: 0,
            name: b"SPDIF\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 1,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 2,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 1,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 2,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 3,
            control_type: 2,
            name: core::ptr::null(),
        },
        ScarlettMixerControl {
            num: 4,
            control_type: 2,
            name: core::ptr::null(),
        },
    ],
};

static S18I20_INFO: ScarlettDeviceInfo = ScarlettDeviceInfo {
    matrix_in: 18,
    matrix_out: 8,
    input_len: 18,
    output_len: 20,
    has_output_source_routing: true,
    opt_master: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 47,
        offsets: [0, 20, 28, 30, 38],
        names: core::ptr::null(),
    },
    opt_matrix: ScarlettMixerElemEnumInfo {
        start: -1,
        len: 39,
        offsets: [0, 20, 28, 30, 38],
        names: core::ptr::null(),
    },
    matrix_mux_init: [
        20, 21, 22, 23, 24, 25, 26, 27, 30, 31, 32, 33, 34, 35, 28, 29, 0, 1,
    ],
    num_controls: 10,
    controls: [
        ScarlettMixerControl {
            num: 0,
            control_type: 0,
            name: b"Monitor\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 1,
            control_type: 0,
            name: b"Line 3/4\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 2,
            control_type: 0,
            name: b"Line 5/6\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 3,
            control_type: 0,
            name: b"Line 7/8\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 4,
            control_type: 0,
            name: b"Line 9/10\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 5,
            control_type: 0,
            name: b"SPDIF\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 6,
            control_type: 0,
            name: b"ADAT 1/2\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 7,
            control_type: 0,
            name: b"ADAT 3/4\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 8,
            control_type: 0,
            name: b"ADAT 5/6\0" as *const u8,
        },
        ScarlettMixerControl {
            num: 9,
            control_type: 0,
            name: b"ADAT 7/8\0" as *const u8,
        },
    ],
};

unsafe fn scarlett_controls_create_generic(
    mixer: *mut core::ffi::c_void,
    info: *const ScarlettDeviceInfo,
) -> i32 {
    // TODO: Requires add_new_ctl, scnprintf, and external structures
    0
}

/*
 * Create and initialize a mixer for the Focusrite(R) Scarlett
 */
pub unsafe extern "C" fn snd_scarlett_controls_create(mixer: *mut core::ffi::c_void) -> i32 {
    // TODO: Requires external USB mixer interface and device ID matching
    0
}

/*
 * Create and initialize a mixer for the Focusrite(R) Forte
 */
pub unsafe extern "C" fn snd_forte_controls_create(mixer: *mut core::ffi::c_void) -> i32 {
    // TODO: Requires external USB mixer interface and device ID matching
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
