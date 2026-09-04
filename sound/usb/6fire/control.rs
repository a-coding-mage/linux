// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux driver for TerraTec DMX 6Fire USB
 *
 * Mixer control
 *
 * Author:	Torsten Schenk <torsten.schenk@zoho.com>
 * Created:	Jan 01, 2011
 * Copyright:	(C) Torsten Schenk
 *
 * Thanks to:
 * - Holger Ruckdeschel: he found out how to control individual channel
 *   volumes and introduced mute switch
 */

// Dependencies from kernel headers: <linux/interrupt.h>, <sound/control.h>, <sound/tlv.h>
// Local dependencies: "control.h", "comm.h", "chip.h"

static OPT_COAX_TEXTS: &[&str] = &["Optical", "Coax"];
static LINE_PHONO_TEXTS: &[&str] = &["Line", "Phono"];

/*
 * data that needs to be sent to device. sets up card internal stuff.
 * values dumped from windows driver and filtered by trial'n'error.
 */
struct InitData {
    r#type: u8,
    reg: u8,
    value: u8,
}

static INIT_DATA: &[InitData] = &[
    InitData { r#type: 0x22, reg: 0x00, value: 0x00 },
    InitData { r#type: 0x20, reg: 0x00, value: 0x08 },
    InitData { r#type: 0x22, reg: 0x01, value: 0x01 },
    InitData { r#type: 0x20, reg: 0x01, value: 0x08 },
    InitData { r#type: 0x22, reg: 0x02, value: 0x00 },
    InitData { r#type: 0x20, reg: 0x02, value: 0x08 },
    InitData { r#type: 0x22, reg: 0x03, value: 0x00 },
    InitData { r#type: 0x20, reg: 0x03, value: 0x08 },
    InitData { r#type: 0x22, reg: 0x04, value: 0x00 },
    InitData { r#type: 0x20, reg: 0x04, value: 0x08 },
    InitData { r#type: 0x22, reg: 0x05, value: 0x01 },
    InitData { r#type: 0x20, reg: 0x05, value: 0x08 },
    InitData { r#type: 0x22, reg: 0x04, value: 0x01 },
    InitData { r#type: 0x12, reg: 0x04, value: 0x00 },
    InitData { r#type: 0x12, reg: 0x05, value: 0x00 },
    InitData { r#type: 0x12, reg: 0x0d, value: 0x38 },
    InitData { r#type: 0x12, reg: 0x21, value: 0x82 },
    InitData { r#type: 0x12, reg: 0x22, value: 0x80 },
    InitData { r#type: 0x12, reg: 0x23, value: 0x00 },
    InitData { r#type: 0x12, reg: 0x06, value: 0x02 },
    InitData { r#type: 0x12, reg: 0x03, value: 0x00 },
    InitData { r#type: 0x12, reg: 0x02, value: 0x00 },
    InitData { r#type: 0x22, reg: 0x03, value: 0x01 },
];

static RATES_ALTSETTING: &[i32] = &[1, 1, 2, 2, 3, 3];
// values to write to soundcard register for all samplerates
static RATES_6FIRE_VL: &[u16] = &[0x00, 0x01, 0x00, 0x01, 0x00, 0x01];
static RATES_6FIRE_VH: &[u16] = &[0x11, 0x11, 0x10, 0x10, 0x00, 0x00];

// TLV declarations - preserved from kernel macros DECLARE_TLV_DB_MINMAX
// tlv_output: -9000 dB to 0 dB
// tlv_input: -1500 dB to 1500 dB

const DIGITAL_THRU_ONLY_SAMPLERATE: i32 = 3;

// External type references from dependencies
extern "C" {
    type control_runtime;
    type comm_runtime;
    type sfire_chip;
    type usb_device;
    type snd_kcontrol;
    type snd_ctl_elem_info;
    type snd_ctl_elem_value;
    type snd_card;
    type snd_kcontrol_new;

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut control_runtime;
    fn snd_ctl_make_virtual_master(name: *const u8, tlv: *const u32) -> *mut snd_kcontrol;
    fn snd_ctl_new1(elem: *const snd_kcontrol_new, data: *mut core::ffi::c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> i32;
    fn snd_ctl_add_follower(master: *mut snd_kcontrol, follower: *mut snd_kcontrol) -> i32;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: u32, items: u32, names: *const *const u8) -> i32;
    fn snd_ctl_boolean_stereo_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn usb_set_interface(dev: *mut usb_device, ifnum: i32, alternate: i32) -> i32;
    fn dev_err(dev: *const core::ffi::c_void, fmt: *const u8, ...);
}

macro_rules! kzalloc_obj {
    ($ty:ty) => {{
        kzalloc(core::mem::size_of::<$ty>(), 0) as *mut $ty
    }};
}

unsafe fn usb6fire_control_output_vol_update(rt: *mut control_runtime) {
    let rt = &mut *rt;
    let comm_rt = (*rt).chip as *mut _ as *mut comm_runtime;
    let comm_rt = &*(comm_rt);

    if !comm_rt.is_null() {
        for i in 0..6 {
            if ((*rt).ovol_updated & (1 << i)) == 0 {
                (*comm_rt).write8(comm_rt as *mut _, 0x12, (0x0f + i) as u8,
                    (180 - (*rt).output_vol[i]) as u8);
                (*rt).ovol_updated |= 1 << i;
            }
        }
    }
}

unsafe fn usb6fire_control_output_mute_update(rt: *mut control_runtime) {
    let rt = &mut *rt;
    let comm_rt = (*rt).chip as *mut _ as *mut comm_runtime;
    let comm_rt = &*(comm_rt);

    if !comm_rt.is_null() {
        (*comm_rt).write8(comm_rt as *mut _, 0x12, 0x0e, !(*rt).output_mute);
    }
}

unsafe fn usb6fire_control_input_vol_update(rt: *mut control_runtime) {
    let rt = &mut *rt;
    let comm_rt = (*rt).chip as *mut _ as *mut comm_runtime;
    let comm_rt = &*(comm_rt);

    if !comm_rt.is_null() {
        for i in 0..2 {
            if ((*rt).ivol_updated & (1 << i)) == 0 {
                (*comm_rt).write8(comm_rt as *mut _, 0x12, (0x1c + i) as u8,
                    ((*rt).input_vol[i] & 0x3f) as u8);
                (*rt).ivol_updated |= 1 << i;
            }
        }
    }
}

unsafe fn usb6fire_control_line_phono_update(rt: *mut control_runtime) {
    let rt = &mut *rt;
    let comm_rt = (*rt).chip as *mut _ as *mut comm_runtime;
    let comm_rt = &*(comm_rt);

    if !comm_rt.is_null() {
        (*comm_rt).write8(comm_rt as *mut _, 0x22, 0x02, (*rt).line_phono_switch);
        (*comm_rt).write8(comm_rt as *mut _, 0x21, 0x02, (*rt).line_phono_switch);
    }
}

unsafe fn usb6fire_control_opt_coax_update(rt: *mut control_runtime) {
    let rt = &mut *rt;
    let comm_rt = (*rt).chip as *mut _ as *mut comm_runtime;
    let comm_rt = &*(comm_rt);

    if !comm_rt.is_null() {
        (*comm_rt).write8(comm_rt as *mut _, 0x22, 0x00, (*rt).opt_coax_switch);
        (*comm_rt).write8(comm_rt as *mut _, 0x21, 0x00, (*rt).opt_coax_switch);
    }
}

unsafe fn usb6fire_control_set_rate(rt: *mut control_runtime, rate: i32) -> i32 {
    let rt = &mut *rt;
    let device = (*rt).chip as *mut _ as *mut usb_device;
    let comm_rt = (*rt).chip as *mut _ as *mut comm_runtime;
    let comm_rt = &*(comm_rt);

    const CONTROL_N_RATES: i32 = 6;

    if rate < 0 || rate >= CONTROL_N_RATES {
        return -22; // -EINVAL
    }

    let ret = usb_set_interface(device, 1, RATES_ALTSETTING[rate as usize]);
    if ret < 0 {
        return ret;
    }

    // set soundcard clock
    let ret = (*comm_rt).write16(comm_rt as *mut _, 0x02, 0x01,
        RATES_6FIRE_VL[rate as usize],
        RATES_6FIRE_VH[rate as usize]);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe fn usb6fire_control_set_channels(
    rt: *mut control_runtime,
    n_analog_out: i32,
    n_analog_in: i32,
    spdif_out: bool,
    spdif_in: bool) -> i32 {
    let rt = &mut *rt;
    let comm_rt = (*rt).chip as *mut _ as *mut comm_runtime;
    let comm_rt = &*(comm_rt);

    // enable analog inputs and outputs
    // (one bit per stereo-channel)
    let ret = (*comm_rt).write16(comm_rt as *mut _, 0x02, 0x02,
        ((1 << (n_analog_out / 2)) - 1) as u16,
        ((1 << (n_analog_in / 2)) - 1) as u16);
    if ret < 0 {
        return ret;
    }

    // disable digital inputs and outputs
    // TODO: use spdif_x to enable/disable digital channels
    let ret = (*comm_rt).write16(comm_rt as *mut _, 0x02, 0x03, 0x00, 0x00);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe fn usb6fire_control_streaming_update(rt: *mut control_runtime) -> i32 {
    let rt = &mut *rt;
    let comm_rt = (*rt).chip as *mut _ as *mut comm_runtime;
    let comm_rt = &*(comm_rt);

    if !comm_rt.is_null() {
        if !(*rt).usb_streaming && (*rt).digital_thru_switch {
            usb6fire_control_set_rate(rt, DIGITAL_THRU_ONLY_SAMPLERATE);
        }
        return (*comm_rt).write16(comm_rt as *mut _, 0x02, 0x00, 0x00,
            if (*rt).usb_streaming { 0x01 } else { 0x00 } |
            if (*rt).digital_thru_switch { 0x08 } else { 0x00 });
    }
    -22 // -EINVAL
}

unsafe fn usb6fire_control_output_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = 2; // SNDRV_CTL_ELEM_TYPE_INTEGER
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 180;
    0
}

unsafe fn usb6fire_control_output_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &mut *rt;
    let ch = (*kcontrol).private_value as usize;
    let mut changed = 0;

    if ch > 4 {
        dev_err(&(*rt).chip as *const _ as *const core::ffi::c_void,
            b"Invalid channel in volume control.\0".as_ptr());
        return -22; // -EINVAL
    }

    if (*rt).output_vol[ch] != (*ucontrol).value.integer.value[0] as u8 {
        (*rt).output_vol[ch] = (*ucontrol).value.integer.value[0] as u8;
        (*rt).ovol_updated &= !(1 << ch);
        changed = 1;
    }
    if (*rt).output_vol[ch + 1] != (*ucontrol).value.integer.value[1] as u8 {
        (*rt).output_vol[ch + 1] = (*ucontrol).value.integer.value[1] as u8;
        (*rt).ovol_updated &= !(2 << ch);
        changed = 1;
    }

    if changed != 0 {
        usb6fire_control_output_vol_update(rt);
    }

    changed
}

unsafe fn usb6fire_control_output_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &*rt;
    let ch = (*kcontrol).private_value as usize;

    if ch > 4 {
        dev_err(&(*rt).chip as *const _ as *const core::ffi::c_void,
            b"Invalid channel in volume control.\0".as_ptr());
        return -22; // -EINVAL
    }

    (*ucontrol).value.integer.value[0] = (*rt).output_vol[ch] as i64;
    (*ucontrol).value.integer.value[1] = (*rt).output_vol[ch + 1] as i64;
    0
}

unsafe fn usb6fire_control_output_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &mut *rt;
    let ch = (*kcontrol).private_value as usize;
    let old = (*rt).output_mute;
    let mut value: u8 = 0;

    if ch > 4 {
        dev_err(&(*rt).chip as *const _ as *const core::ffi::c_void,
            b"Invalid channel in volume control.\0".as_ptr());
        return -22; // -EINVAL
    }

    (*rt).output_mute &= !(3 << ch) as u8;
    if (*ucontrol).value.integer.value[0] != 0 {
        value |= 1;
    }
    if (*ucontrol).value.integer.value[1] != 0 {
        value |= 2;
    }
    (*rt).output_mute |= value << ch as u8;

    if (*rt).output_mute != old {
        usb6fire_control_output_mute_update(rt);
    }

    if (*rt).output_mute != old { 1 } else { 0 }
}

unsafe fn usb6fire_control_output_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &*rt;
    let ch = (*kcontrol).private_value as usize;
    let mut value = (*rt).output_mute >> ch as u8;

    if ch > 4 {
        dev_err(&(*rt).chip as *const _ as *const core::ffi::c_void,
            b"Invalid channel in volume control.\0".as_ptr());
        return -22; // -EINVAL
    }

    (*ucontrol).value.integer.value[0] = (1 & value) as i64;
    value >>= 1;
    (*ucontrol).value.integer.value[1] = (1 & value) as i64;

    0
}

unsafe fn usb6fire_control_input_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = 2; // SNDRV_CTL_ELEM_TYPE_INTEGER
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 30;
    0
}

unsafe fn usb6fire_control_input_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &mut *rt;
    let vol0 = (*ucontrol).value.integer.value[0] - 15;
    let vol1 = (*ucontrol).value.integer.value[1] - 15;
    let mut changed = 0;

    if (*rt).input_vol[0] != vol0 as i8 {
        (*rt).input_vol[0] = vol0 as i8;
        (*rt).ivol_updated &= !(1 << 0);
        changed = 1;
    }
    if (*rt).input_vol[1] != vol1 as i8 {
        (*rt).input_vol[1] = vol1 as i8;
        (*rt).ivol_updated &= !(1 << 1);
        changed = 1;
    }

    if changed != 0 {
        usb6fire_control_input_vol_update(rt);
    }

    changed
}

unsafe fn usb6fire_control_input_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &*rt;

    (*ucontrol).value.integer.value[0] = ((*rt).input_vol[0] + 15) as i64;
    (*ucontrol).value.integer.value[1] = ((*rt).input_vol[1] + 15) as i64;

    0
}

unsafe fn usb6fire_control_line_phono_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info) -> i32 {
    snd_ctl_enum_info(uinfo, 1, 2, LINE_PHONO_TEXTS.as_ptr())
}

unsafe fn usb6fire_control_line_phono_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &mut *rt;
    let mut changed = 0;

    if (*rt).line_phono_switch != (*ucontrol).value.integer.value[0] as u8 {
        (*rt).line_phono_switch = (*ucontrol).value.integer.value[0] as u8;
        usb6fire_control_line_phono_update(rt);
        changed = 1;
    }
    changed
}

unsafe fn usb6fire_control_line_phono_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &*rt;

    (*ucontrol).value.integer.value[0] = (*rt).line_phono_switch as i64;
    0
}

unsafe fn usb6fire_control_opt_coax_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info) -> i32 {
    snd_ctl_enum_info(uinfo, 1, 2, OPT_COAX_TEXTS.as_ptr())
}

unsafe fn usb6fire_control_opt_coax_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &mut *rt;
    let mut changed = 0;

    if (*rt).opt_coax_switch != (*ucontrol).value.enumerated.item[0] as u8 {
        (*rt).opt_coax_switch = (*ucontrol).value.enumerated.item[0] as u8;
        usb6fire_control_opt_coax_update(rt);
        changed = 1;
    }
    changed
}

unsafe fn usb6fire_control_opt_coax_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &*rt;

    (*ucontrol).value.enumerated.item[0] = (*rt).opt_coax_switch as u32;
    0
}

unsafe fn usb6fire_control_digital_thru_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &mut *rt;
    let mut changed = 0;

    if (*rt).digital_thru_switch != ((*ucontrol).value.integer.value[0] != 0) {
        (*rt).digital_thru_switch = (*ucontrol).value.integer.value[0] != 0;
        usb6fire_control_streaming_update(rt);
        changed = 1;
    }
    changed
}

unsafe fn usb6fire_control_digital_thru_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let rt = snd_kcontrol_chip(kcontrol);
    let rt = &*rt;

    (*ucontrol).value.integer.value[0] = if (*rt).digital_thru_switch { 1 } else { 0 };
    0
}

// Struct definitions for control elements - these reference kernel ALSA structures
// The actual structure layout must match the kernel definitions (represented here with field names)
// This is a placeholder showing the logical structure expected by the code
#[repr(C)]
pub struct SndKcontrolNew {
    pub iface: u32,
    pub name: *const u8,
    pub index: u32,
    pub access: u32,
    pub private_value: usize,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32>,
    pub tlv_p: *const u32,
}

// SNDRV_CTL_ELEM_IFACE_MIXER
const SNDRV_CTL_ELEM_IFACE_MIXER: u32 = 2;
// SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ
const ACCESS_RW_TLV: u32 = (1 << 0) | (1 << 1) | (1 << 4);
// SNDRV_CTL_ELEM_ACCESS_READWRITE
const ACCESS_RW: u32 = (1 << 0) | (1 << 1);

static VOL_ELEMENTS: &[SndKcontrolNew] = &[
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Analog Playback Volume\0".as_ptr(),
        index: 0,
        private_value: 0,
        access: ACCESS_RW_TLV,
        info: Some(usb6fire_control_output_vol_info),
        get: Some(usb6fire_control_output_vol_get),
        put: Some(usb6fire_control_output_vol_put),
        tlv_p: core::ptr::null(),
    },
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Analog Playback Volume\0".as_ptr(),
        index: 1,
        private_value: 2,
        access: ACCESS_RW_TLV,
        info: Some(usb6fire_control_output_vol_info),
        get: Some(usb6fire_control_output_vol_get),
        put: Some(usb6fire_control_output_vol_put),
        tlv_p: core::ptr::null(),
    },
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Analog Playback Volume\0".as_ptr(),
        index: 2,
        private_value: 4,
        access: ACCESS_RW_TLV,
        info: Some(usb6fire_control_output_vol_info),
        get: Some(usb6fire_control_output_vol_get),
        put: Some(usb6fire_control_output_vol_put),
        tlv_p: core::ptr::null(),
    },
];

static MUTE_ELEMENTS: &[SndKcontrolNew] = &[
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Analog Playback Switch\0".as_ptr(),
        index: 0,
        private_value: 0,
        access: ACCESS_RW,
        info: Some(snd_ctl_boolean_stereo_info),
        get: Some(usb6fire_control_output_mute_get),
        put: Some(usb6fire_control_output_mute_put),
        tlv_p: core::ptr::null(),
    },
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Analog Playback Switch\0".as_ptr(),
        index: 1,
        private_value: 2,
        access: ACCESS_RW,
        info: Some(snd_ctl_boolean_stereo_info),
        get: Some(usb6fire_control_output_mute_get),
        put: Some(usb6fire_control_output_mute_put),
        tlv_p: core::ptr::null(),
    },
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Analog Playback Switch\0".as_ptr(),
        index: 2,
        private_value: 4,
        access: ACCESS_RW,
        info: Some(snd_ctl_boolean_stereo_info),
        get: Some(usb6fire_control_output_mute_get),
        put: Some(usb6fire_control_output_mute_put),
        tlv_p: core::ptr::null(),
    },
];

static ELEMENTS: &[SndKcontrolNew] = &[
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Line/Phono Capture Route\0".as_ptr(),
        index: 0,
        access: ACCESS_RW,
        private_value: 0,
        info: Some(usb6fire_control_line_phono_info),
        get: Some(usb6fire_control_line_phono_get),
        put: Some(usb6fire_control_line_phono_put),
        tlv_p: core::ptr::null(),
    },
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Opt/Coax Capture Route\0".as_ptr(),
        index: 0,
        access: ACCESS_RW,
        private_value: 0,
        info: Some(usb6fire_control_opt_coax_info),
        get: Some(usb6fire_control_opt_coax_get),
        put: Some(usb6fire_control_opt_coax_put),
        tlv_p: core::ptr::null(),
    },
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Digital Thru Playback Route\0".as_ptr(),
        index: 0,
        access: ACCESS_RW,
        private_value: 0,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(usb6fire_control_digital_thru_get),
        put: Some(usb6fire_control_digital_thru_put),
        tlv_p: core::ptr::null(),
    },
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Analog Capture Volume\0".as_ptr(),
        index: 0,
        access: ACCESS_RW_TLV,
        private_value: 0,
        info: Some(usb6fire_control_input_vol_info),
        get: Some(usb6fire_control_input_vol_get),
        put: Some(usb6fire_control_input_vol_put),
        tlv_p: core::ptr::null(),
    },
];

unsafe fn usb6fire_control_add_virtual(
    rt: *mut control_runtime,
    card: *mut snd_card,
    name: *const u8,
    elems: *const SndKcontrolNew) -> i32 {
    let vmaster = snd_ctl_make_virtual_master(name, core::ptr::null());
    if vmaster.is_null() {
        return -12; // -ENOMEM
    }

    let mut ret = snd_ctl_add(card, vmaster);
    if ret < 0 {
        return ret;
    }

    let mut i = 0;
    loop {
        if (*elems.add(i)).name.is_null() {
            break;
        }

        let control = snd_ctl_new1(elems.add(i), rt as *mut core::ffi::c_void);
        if control.is_null() {
            return -12; // -ENOMEM
        }

        ret = snd_ctl_add(card, control);
        if ret < 0 {
            return ret;
        }

        ret = snd_ctl_add_follower(vmaster, control);
        if ret < 0 {
            return ret;
        }

        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn usb6fire_control_init(chip: *mut sfire_chip) -> i32 {
    let chip = &mut *chip;
    let rt = kzalloc_obj!(control_runtime) as *mut control_runtime;
    let comm_rt = chip.comm;

    if rt.is_null() {
        return -12; // -ENOMEM
    }

    let rt_ref = &mut *rt;
    rt_ref.chip = chip as *mut sfire_chip;
    rt_ref.update_streaming = Some(usb6fire_control_streaming_update);
    rt_ref.set_rate = Some(usb6fire_control_set_rate);
    rt_ref.set_channels = Some(usb6fire_control_set_channels);

    let mut i = 0;
    loop {
        if INIT_DATA[i].r#type == 0 {
            break;
        }
        (*comm_rt).write8(
            comm_rt,
            INIT_DATA[i].r#type,
            INIT_DATA[i].reg,
            INIT_DATA[i].value);
        i += 1;
    }

    usb6fire_control_opt_coax_update(rt);
    usb6fire_control_line_phono_update(rt);
    usb6fire_control_output_vol_update(rt);
    usb6fire_control_output_mute_update(rt);
    usb6fire_control_input_vol_update(rt);
    usb6fire_control_streaming_update(rt);

    let mut ret = usb6fire_control_add_virtual(
        rt,
        chip.card,
        b"Master Playback Volume\0".as_ptr(),
        VOL_ELEMENTS.as_ptr() as *const SndKcontrolNew);
    if ret != 0 {
        dev_err(
            &chip.dev as *const _ as *const core::ffi::c_void,
            b"cannot add control.\n\0".as_ptr());
        goto_free_rt(rt);
        return ret;
    }

    ret = usb6fire_control_add_virtual(
        rt,
        chip.card,
        b"Master Playback Switch\0".as_ptr(),
        MUTE_ELEMENTS.as_ptr() as *const SndKcontrolNew);
    if ret != 0 {
        dev_err(
            &chip.dev as *const _ as *const core::ffi::c_void,
            b"cannot add control.\n\0".as_ptr());
        goto_free_rt(rt);
        return ret;
    }

    let mut i = 0;
    loop {
        if ELEMENTS[i].name.is_null() {
            break;
        }

        ret = snd_ctl_add(chip.card,
            snd_ctl_new1(&ELEMENTS[i], rt as *mut core::ffi::c_void));
        if ret < 0 {
            dev_err(
                &chip.dev as *const _ as *const core::ffi::c_void,
                b"cannot add control.\n\0".as_ptr());
            goto_free_rt(rt);
            return ret;
        }

        i += 1;
    }

    chip.control = rt;
    0
}

unsafe fn goto_free_rt(rt: *mut control_runtime) {
    kfree(rt as *mut core::ffi::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn usb6fire_control_abort(_chip: *mut sfire_chip) {
}

#[no_mangle]
pub unsafe extern "C" fn usb6fire_control_destroy(chip: *mut sfire_chip) {
    let chip = &mut *chip;
    kfree(chip.control as *mut core::ffi::c_void);
    chip.control = core::ptr::null_mut();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
