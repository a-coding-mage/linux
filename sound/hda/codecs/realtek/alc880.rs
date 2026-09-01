// SPDX-License-Identifier: GPL-2.0-or-later
//
// Realtek ALC880 codec
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type hda_nid_t = u16;

// Dependencies from linux/init.h, linux/module.h and realtek.h.
#[repr(C)]
pub struct hda_codec {
    pub spec: *mut alc_spec,
}

#[repr(C)]
pub struct hda_device_id {
    pub vendor_id: u32,
    pub name: *const c_char,
}

#[repr(C)]
pub struct alc_spec {
    pub gen: hda_gen_spec,
}

#[repr(C)]
pub struct hda_gen_spec {
    pub need_dac_fix: c_int,
    pub beep_nid: hda_nid_t,
    pub no_analog: bool,
}

#[repr(C)]
pub struct hda_verb {
    pub nid: hda_nid_t,
    pub verb: c_uint,
    pub param: c_uint,
}

#[repr(C)]
pub struct hda_pintbl {
    pub nid: hda_nid_t,
    pub cfg: c_uint,
}

#[repr(C)]
pub union hda_fixup_v {
    pub verbs: *const hda_verb,
    pub pins: *const hda_pintbl,
    pub func: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_fixup, c_int)>,
}

#[repr(C)]
pub struct hda_fixup {
    pub type_: c_int,
    pub v: hda_fixup_v,
    pub chained: bool,
    pub chained_before: bool,
    pub chain_id: c_int,
}

#[repr(C)]
pub struct hda_quirk {
    pub subvendor: u32,
    pub subdevice: u32,
    pub name: *const c_char,
    pub value: c_int,
}

#[repr(C)]
pub struct hda_model_fixup {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, c_uint)>,
    pub resume: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub suspend: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub check_power_status: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t) -> c_int>,
    pub stream_pm: Option<unsafe extern "C" fn(*mut hda_codec, c_int)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

unsafe extern "C" {
    fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: c_uint);
    fn alc_parse_auto_config(
        codec: *mut hda_codec,
        ignore: *const hda_nid_t,
        ssids: *const hda_nid_t,
    ) -> c_int;
    fn snd_hda_jack_detect_enable_callback(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        callback: Option<unsafe extern "C" fn()>,
    );
    fn alc_update_knob_master();
    fn alc_fixup_gpio1(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
    fn alc_fixup_gpio2(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
    fn alc_alloc_spec(codec: *mut hda_codec, mixer_nid: hda_nid_t) -> c_int;
    fn alc_pre_init(codec: *mut hda_codec);
    fn snd_hda_pick_fixup(
        codec: *mut hda_codec,
        models: *const hda_model_fixup,
        quirks: *const hda_quirk,
        fixups: *const hda_fixup,
    );
    fn snd_hda_apply_fixup(codec: *mut hda_codec, action: c_int);
    fn set_beep_amp(spec: *mut alc_spec, nid: hda_nid_t, idx: c_int, dir: c_int) -> c_int;
    fn snd_hda_gen_remove(codec: *mut hda_codec);
    fn alc_build_controls(codec: *mut hda_codec) -> c_int;
    fn snd_hda_gen_build_pcms(codec: *mut hda_codec) -> c_int;
    fn alc_init(codec: *mut hda_codec) -> c_int;
    fn alc_resume(codec: *mut hda_codec);
    fn alc_suspend(codec: *mut hda_codec);
    fn snd_hda_gen_check_power_status(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    fn snd_hda_gen_stream_pm(codec: *mut hda_codec, state: c_int);
}

const HDA_FIXUP_FUNC: c_int = 0;
const HDA_FIXUP_VERBS: c_int = 1;
const HDA_FIXUP_PINS: c_int = 2;
const HDA_FIXUP_ACT_PRE_PROBE: c_int = 0;
const HDA_FIXUP_ACT_PROBE: c_int = 1;
const AC_VERB_SET_COEF_INDEX: c_uint = 0;
const AC_VERB_SET_PROC_COEF: c_uint = 0;
const HDA_INPUT: c_int = 0;

const ALC880_FIXUP_GPIO1: c_int = 0;
const ALC880_FIXUP_GPIO2: c_int = 1;
const ALC880_FIXUP_MEDION_RIM: c_int = 2;
const ALC880_FIXUP_LG: c_int = 3;
const ALC880_FIXUP_LG_LW25: c_int = 4;
const ALC880_FIXUP_W810: c_int = 5;
const ALC880_FIXUP_EAPD_COEF: c_int = 6;
const ALC880_FIXUP_TCL_S700: c_int = 7;
const ALC880_FIXUP_VOL_KNOB: c_int = 8;
const ALC880_FIXUP_FUJITSU: c_int = 9;
const ALC880_FIXUP_F1734: c_int = 10;
const ALC880_FIXUP_UNIWILL: c_int = 11;
const ALC880_FIXUP_UNIWILL_DIG: c_int = 12;
const ALC880_FIXUP_Z71V: c_int = 13;
const ALC880_FIXUP_ASUS_W5A: c_int = 14;
const ALC880_FIXUP_3ST_BASE: c_int = 15;
const ALC880_FIXUP_3ST: c_int = 16;
const ALC880_FIXUP_3ST_DIG: c_int = 17;
const ALC880_FIXUP_5ST_BASE: c_int = 18;
const ALC880_FIXUP_5ST: c_int = 19;
const ALC880_FIXUP_5ST_DIG: c_int = 20;
const ALC880_FIXUP_6ST_BASE: c_int = 21;
const ALC880_FIXUP_6ST: c_int = 22;
const ALC880_FIXUP_6ST_DIG: c_int = 23;
const ALC880_FIXUP_6ST_AUTOMUTE: c_int = 24;

const fn verb(nid: hda_nid_t, verb: c_uint, param: c_uint) -> hda_verb {
    hda_verb { nid, verb, param }
}

const fn pin(nid: hda_nid_t, cfg: c_uint) -> hda_pintbl {
    hda_pintbl { nid, cfg }
}

const fn fixup_func(
    func: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_fixup, c_int)>,
    chained: bool,
    chained_before: bool,
    chain_id: c_int,
) -> hda_fixup {
    hda_fixup { type_: HDA_FIXUP_FUNC, v: hda_fixup_v { func }, chained, chained_before, chain_id }
}

const fn fixup_verbs(
    verbs: *const hda_verb,
    chained: bool,
    chained_before: bool,
    chain_id: c_int,
) -> hda_fixup {
    hda_fixup { type_: HDA_FIXUP_VERBS, v: hda_fixup_v { verbs }, chained, chained_before, chain_id }
}

const fn fixup_pins(
    pins: *const hda_pintbl,
    chained: bool,
    chained_before: bool,
    chain_id: c_int,
) -> hda_fixup {
    hda_fixup { type_: HDA_FIXUP_PINS, v: hda_fixup_v { pins }, chained, chained_before, chain_id }
}

const fn quirk(subvendor: u32, subdevice: u32, name: *const c_char, value: c_int) -> hda_quirk {
    hda_quirk { subvendor, subdevice, name, value }
}

const fn quirk_vendor(subvendor: u32, name: *const c_char, value: c_int) -> hda_quirk {
    hda_quirk { subvendor, subdevice: 0xffff_ffff, name, value }
}

const fn model(id: c_int, name: *const c_char) -> hda_model_fixup {
    hda_model_fixup { id, name }
}

unsafe extern "C" fn alc880_unsol_event(codec: *mut hda_codec, res: c_uint) {
    /* For some reason, the res given from ALC880 is broken.
       Here we adjust it properly. */
    unsafe { snd_hda_jack_unsol_event(codec, res >> 2) };
}

unsafe extern "C" fn alc880_parse_auto_config(codec: *mut hda_codec) -> c_int {
    static alc880_ignore: [hda_nid_t; 2] = [0x1d, 0];
    static alc880_ssids: [hda_nid_t; 4] = [0x15, 0x1b, 0x14, 0];
    unsafe { alc_parse_auto_config(codec, alc880_ignore.as_ptr(), alc880_ssids.as_ptr()) }
}

/*
 * ALC880 fix-ups
 */

/* enable the volume-knob widget support on NID 0x21 */
unsafe extern "C" fn alc880_fixup_vol_knob(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: c_int,
) {
    if action == HDA_FIXUP_ACT_PROBE {
        unsafe {
            snd_hda_jack_detect_enable_callback(codec, 0x21, Some(core::mem::transmute(alc_update_knob_master as unsafe extern "C" fn())));
        }
    }
}

static ALC880_FIXUP_MEDION_RIM_VERBS: [hda_verb; 3] = [
    verb(0x20, AC_VERB_SET_COEF_INDEX, 0x07),
    verb(0x20, AC_VERB_SET_PROC_COEF, 0x3060),
    verb(0, 0, 0),
];
static ALC880_FIXUP_LG_PINS: [hda_pintbl; 4] = [
    /* disable bogus unused pins */
    pin(0x16, 0x411111f0),
    pin(0x18, 0x411111f0),
    pin(0x1a, 0x411111f0),
    pin(0, 0),
];
static ALC880_FIXUP_LG_LW25_PINS: [hda_pintbl; 3] = [
    pin(0x1a, 0x0181344f), /* line-in */
    pin(0x1b, 0x0321403f), /* headphone */
    pin(0, 0),
];
static ALC880_FIXUP_W810_PINS: [hda_pintbl; 2] = [
    /* disable bogus unused pins */
    pin(0x17, 0x411111f0),
    pin(0, 0),
];
static ALC880_FIXUP_EAPD_COEF_VERBS: [hda_verb; 3] = [
    /* change to EAPD mode */
    verb(0x20, AC_VERB_SET_COEF_INDEX, 0x07),
    verb(0x20, AC_VERB_SET_PROC_COEF, 0x3060),
    verb(0, 0, 0),
];
static ALC880_FIXUP_TCL_S700_VERBS: [hda_verb; 3] = [
    /* change to EAPD mode */
    verb(0x20, AC_VERB_SET_COEF_INDEX, 0x07),
    verb(0x20, AC_VERB_SET_PROC_COEF, 0x3070),
    verb(0, 0, 0),
];
static ALC880_FIXUP_FUJITSU_PINS: [hda_pintbl; 12] = [
    pin(0x14, 0x0121401f), /* HP */
    pin(0x15, 0x99030120), /* speaker */
    pin(0x16, 0x99030130), /* bass speaker */
    pin(0x17, 0x411111f0), /* N/A */
    pin(0x18, 0x411111f0), /* N/A */
    pin(0x19, 0x01a19950), /* mic-in */
    pin(0x1a, 0x411111f0), /* N/A */
    pin(0x1b, 0x411111f0), /* N/A */
    pin(0x1c, 0x411111f0), /* N/A */
    pin(0x1d, 0x411111f0), /* N/A */
    pin(0x1e, 0x01454140), /* SPDIF out */
    pin(0, 0),
];
static ALC880_FIXUP_F1734_PINS: [hda_pintbl; 12] = [
    pin(0x14, 0x0121401f), pin(0x15, 0x99030120), pin(0x16, 0x411111f0),
    pin(0x17, 0x411111f0), pin(0x18, 0x411111f0), pin(0x19, 0x01a19950),
    pin(0x1a, 0x411111f0), pin(0x1b, 0x411111f0), pin(0x1c, 0x411111f0),
    pin(0x1d, 0x411111f0), pin(0x1e, 0x411111f0), pin(0, 0),
];
static ALC880_FIXUP_UNIWILL_PINS: [hda_pintbl; 4] = [
    pin(0x14, 0x0121411f), pin(0x15, 0x99030120), pin(0x16, 0x99030130), pin(0, 0),
];
static ALC880_FIXUP_UNIWILL_DIG_PINS: [hda_pintbl; 5] = [
    /* disable bogus unused pins */
    pin(0x17, 0x411111f0), pin(0x19, 0x411111f0), pin(0x1b, 0x411111f0),
    pin(0x1f, 0x411111f0), pin(0, 0),
];
static ALC880_FIXUP_Z71V_PINS: [hda_pintbl; 12] = [
    pin(0x14, 0x99030120), pin(0x15, 0x0121411f), pin(0x16, 0x411111f0),
    pin(0x17, 0x411111f0), pin(0x18, 0x01a19950), pin(0x19, 0x411111f0),
    pin(0x1a, 0x01813031), pin(0x1b, 0x411111f0), pin(0x1c, 0x411111f0),
    pin(0x1d, 0x411111f0), pin(0x1e, 0x0144111e), pin(0, 0),
];
static ALC880_FIXUP_ASUS_W5A_PINS: [hda_pintbl; 12] = [
    pin(0x14, 0x0121411f), pin(0x15, 0x411111f0), pin(0x16, 0x411111f0),
    pin(0x17, 0x411111f0), pin(0x18, 0x90a60160), pin(0x19, 0x411111f0),
    pin(0x1a, 0x411111f0), pin(0x1b, 0x411111f0), pin(0x1c, 0x411111f0),
    pin(0x1d, 0x411111f0), pin(0x1e, 0xb743111e), pin(0, 0),
];
static ALC880_FIXUP_3ST_BASE_PINS: [hda_pintbl; 12] = [
    pin(0x14, 0x01014010), pin(0x15, 0x411111f0), pin(0x16, 0x411111f0),
    pin(0x17, 0x411111f0), pin(0x18, 0x01a19c30), pin(0x19, 0x0121411f),
    pin(0x1a, 0x01813031), pin(0x1b, 0x02a19c40), pin(0x1c, 0x411111f0),
    pin(0x1d, 0x411111f0), /* 0x1e is filled in below */
    pin(0x1f, 0x411111f0), pin(0, 0),
];
static ALC880_FIXUP_3ST_PINS: [hda_pintbl; 2] = [pin(0x1e, 0x411111f0), pin(0, 0)];
static ALC880_FIXUP_3ST_DIG_PINS: [hda_pintbl; 2] = [pin(0x1e, 0x0144111e), pin(0, 0)];
static ALC880_FIXUP_5ST_BASE_PINS: [hda_pintbl; 12] = [
    pin(0x14, 0x01014010), pin(0x15, 0x411111f0), pin(0x16, 0x01011411),
    pin(0x17, 0x01016412), pin(0x18, 0x01a19c30), pin(0x19, 0x0121411f),
    pin(0x1a, 0x01813031), pin(0x1b, 0x02a19c40), pin(0x1c, 0x411111f0),
    pin(0x1d, 0x411111f0), /* 0x1e is filled in below */
    pin(0x1f, 0x411111f0), pin(0, 0),
];
static ALC880_FIXUP_5ST_PINS: [hda_pintbl; 2] = [pin(0x1e, 0x411111f0), pin(0, 0)];
static ALC880_FIXUP_5ST_DIG_PINS: [hda_pintbl; 2] = [pin(0x1e, 0x0144111e), pin(0, 0)];
static ALC880_FIXUP_6ST_BASE_PINS: [hda_pintbl; 12] = [
    pin(0x14, 0x01014010), pin(0x15, 0x01016412), pin(0x16, 0x01011411),
    pin(0x17, 0x01012414), pin(0x18, 0x01a19c30), pin(0x19, 0x02a19c40),
    pin(0x1a, 0x01813031), pin(0x1b, 0x0121411f), pin(0x1c, 0x411111f0),
    pin(0x1d, 0x411111f0), /* 0x1e is filled in below */
    pin(0x1f, 0x411111f0), pin(0, 0),
];
static ALC880_FIXUP_6ST_PINS: [hda_pintbl; 2] = [pin(0x1e, 0x411111f0), pin(0, 0)];
static ALC880_FIXUP_6ST_DIG_PINS: [hda_pintbl; 2] = [pin(0x1e, 0x0144111e), pin(0, 0)];
static ALC880_FIXUP_6ST_AUTOMUTE_PINS: [hda_pintbl; 2] = [pin(0x1b, 0x0121401f), pin(0, 0)];

static alc880_fixups: [hda_fixup; 25] = [
    fixup_func(Some(alc_fixup_gpio1), false, false, 0),
    fixup_func(Some(alc_fixup_gpio2), false, false, 0),
    fixup_verbs(ALC880_FIXUP_MEDION_RIM_VERBS.as_ptr(), true, false, ALC880_FIXUP_GPIO2),
    fixup_pins(ALC880_FIXUP_LG_PINS.as_ptr(), false, false, 0),
    fixup_pins(ALC880_FIXUP_LG_LW25_PINS.as_ptr(), false, false, 0),
    fixup_pins(ALC880_FIXUP_W810_PINS.as_ptr(), true, false, ALC880_FIXUP_GPIO2),
    fixup_verbs(ALC880_FIXUP_EAPD_COEF_VERBS.as_ptr(), false, false, 0),
    fixup_verbs(ALC880_FIXUP_TCL_S700_VERBS.as_ptr(), true, false, ALC880_FIXUP_GPIO2),
    fixup_func(Some(alc880_fixup_vol_knob), false, false, 0),
    fixup_pins(ALC880_FIXUP_FUJITSU_PINS.as_ptr(), true, false, ALC880_FIXUP_VOL_KNOB),
    fixup_pins(ALC880_FIXUP_F1734_PINS.as_ptr(), true, false, ALC880_FIXUP_VOL_KNOB),
    fixup_pins(ALC880_FIXUP_UNIWILL_PINS.as_ptr(), false, false, 0),
    fixup_pins(ALC880_FIXUP_UNIWILL_DIG_PINS.as_ptr(), false, false, 0),
    fixup_pins(ALC880_FIXUP_Z71V_PINS.as_ptr(), false, false, 0),
    fixup_pins(ALC880_FIXUP_ASUS_W5A_PINS.as_ptr(), true, false, ALC880_FIXUP_GPIO1),
    fixup_pins(ALC880_FIXUP_3ST_BASE_PINS.as_ptr(), false, false, 0),
    fixup_pins(ALC880_FIXUP_3ST_PINS.as_ptr(), true, false, ALC880_FIXUP_3ST_BASE),
    fixup_pins(ALC880_FIXUP_3ST_DIG_PINS.as_ptr(), true, false, ALC880_FIXUP_3ST_BASE),
    fixup_pins(ALC880_FIXUP_5ST_BASE_PINS.as_ptr(), false, false, 0),
    fixup_pins(ALC880_FIXUP_5ST_PINS.as_ptr(), true, false, ALC880_FIXUP_5ST_BASE),
    fixup_pins(ALC880_FIXUP_5ST_DIG_PINS.as_ptr(), true, false, ALC880_FIXUP_5ST_BASE),
    fixup_pins(ALC880_FIXUP_6ST_BASE_PINS.as_ptr(), false, false, 0),
    fixup_pins(ALC880_FIXUP_6ST_PINS.as_ptr(), true, false, ALC880_FIXUP_6ST_BASE),
    fixup_pins(ALC880_FIXUP_6ST_DIG_PINS.as_ptr(), true, false, ALC880_FIXUP_6ST_BASE),
    fixup_pins(ALC880_FIXUP_6ST_AUTOMUTE_PINS.as_ptr(), false, true, ALC880_FIXUP_6ST_BASE),
];

static alc880_fixup_tbl: [hda_quirk; 64] = [
    quirk(0x1019, 0x0f69, c"Coeus G610P".as_ptr(), ALC880_FIXUP_W810),
    quirk(0x1043, 0x10c3, c"ASUS W5A".as_ptr(), ALC880_FIXUP_ASUS_W5A),
    quirk(0x1043, 0x1964, c"ASUS Z71V".as_ptr(), ALC880_FIXUP_Z71V),
    quirk_vendor(0x1043, c"ASUS".as_ptr(), ALC880_FIXUP_GPIO1),
    quirk(0x147b, 0x1045, c"ABit AA8XE".as_ptr(), ALC880_FIXUP_6ST_AUTOMUTE),
    quirk(0x1558, 0x5401, c"Clevo GPIO2".as_ptr(), ALC880_FIXUP_GPIO2),
    quirk_vendor(0x1558, c"Clevo".as_ptr(), ALC880_FIXUP_EAPD_COEF),
    quirk(0x1584, 0x9050, c"Uniwill".as_ptr(), ALC880_FIXUP_UNIWILL_DIG),
    quirk(0x1584, 0x9054, c"Uniwill".as_ptr(), ALC880_FIXUP_F1734),
    quirk(0x1584, 0x9070, c"Uniwill".as_ptr(), ALC880_FIXUP_UNIWILL),
    quirk(0x1584, 0x9077, c"Uniwill P53".as_ptr(), ALC880_FIXUP_VOL_KNOB),
    quirk(0x161f, 0x203d, c"W810".as_ptr(), ALC880_FIXUP_W810),
    quirk(0x161f, 0x205d, c"Medion Rim 2150".as_ptr(), ALC880_FIXUP_MEDION_RIM),
    quirk(0x1631, 0xe011, c"PB 13201056".as_ptr(), ALC880_FIXUP_6ST_AUTOMUTE),
    quirk(0x1734, 0x107c, c"FSC Amilo M1437".as_ptr(), ALC880_FIXUP_FUJITSU),
    quirk(0x1734, 0x1094, c"FSC Amilo M1451G".as_ptr(), ALC880_FIXUP_FUJITSU),
    quirk(0x1734, 0x10ac, c"FSC AMILO Xi 1526".as_ptr(), ALC880_FIXUP_F1734),
    quirk(0x1734, 0x10b0, c"FSC Amilo Pi1556".as_ptr(), ALC880_FIXUP_FUJITSU),
    quirk(0x1854, 0x003b, c"LG".as_ptr(), ALC880_FIXUP_LG),
    quirk(0x1854, 0x005f, c"LG P1 Express".as_ptr(), ALC880_FIXUP_LG),
    quirk(0x1854, 0x0068, c"LG w1".as_ptr(), ALC880_FIXUP_LG),
    quirk(0x1854, 0x0077, c"LG LW25".as_ptr(), ALC880_FIXUP_LG_LW25),
    quirk(0x19db, 0x4188, c"TCL S700".as_ptr(), ALC880_FIXUP_TCL_S700),
    /*
     * Below is the copied entries from alc880_quirks.c.
     * It's not quite sure whether BIOS sets the correct pin-config table
     * on these machines, thus they are kept to be compatible with
     * the old static quirks.  Once when it's confirmed to work without
     * these overrides, it'd be better to remove.
     */
    quirk(0x1019, 0xa880, c"ECS".as_ptr(), ALC880_FIXUP_5ST_DIG),
    quirk(0x1019, 0xa884, c"Acer APFV".as_ptr(), ALC880_FIXUP_6ST),
    quirk(0x1025, 0x0070, c"ULI".as_ptr(), ALC880_FIXUP_3ST_DIG),
    quirk(0x1025, 0x0077, c"ULI".as_ptr(), ALC880_FIXUP_6ST_DIG),
    quirk(0x1025, 0x0078, c"ULI".as_ptr(), ALC880_FIXUP_6ST_DIG),
    quirk(0x1025, 0x0087, c"ULI".as_ptr(), ALC880_FIXUP_6ST_DIG),
    quirk(0x1025, 0xe309, c"ULI".as_ptr(), ALC880_FIXUP_3ST_DIG),
    quirk(0x1025, 0xe310, c"ULI".as_ptr(), ALC880_FIXUP_3ST),
    quirk(0x1039, 0x1234, core::ptr::null(), ALC880_FIXUP_6ST_DIG),
    quirk(0x104d, 0x81a0, c"Sony".as_ptr(), ALC880_FIXUP_3ST),
    quirk(0x104d, 0x81d6, c"Sony".as_ptr(), ALC880_FIXUP_3ST),
    quirk(0x107b, 0x3032, c"Gateway".as_ptr(), ALC880_FIXUP_5ST),
    quirk(0x107b, 0x3033, c"Gateway".as_ptr(), ALC880_FIXUP_5ST),
    quirk(0x107b, 0x4039, c"Gateway".as_ptr(), ALC880_FIXUP_5ST),
    quirk(0x1297, 0xc790, c"Shuttle ST20G5".as_ptr(), ALC880_FIXUP_6ST_DIG),
    quirk(0x1458, 0xa102, c"Gigabyte K8".as_ptr(), ALC880_FIXUP_6ST_DIG),
    quirk(0x1462, 0x1150, c"MSI".as_ptr(), ALC880_FIXUP_6ST_DIG),
    quirk(0x1509, 0x925d, c"FIC P4M".as_ptr(), ALC880_FIXUP_6ST_DIG),
    quirk(0x1565, 0x8202, c"Biostar".as_ptr(), ALC880_FIXUP_5ST_DIG),
    quirk(0x1695, 0x400d, c"EPoX".as_ptr(), ALC880_FIXUP_5ST_DIG),
    quirk(0x1695, 0x4012, c"EPox EP-5LDA".as_ptr(), ALC880_FIXUP_5ST_DIG),
    quirk(0x2668, 0x8086, core::ptr::null(), ALC880_FIXUP_6ST_DIG), /* broken BIOS */
    quirk(0x8086, 0x2668, core::ptr::null(), ALC880_FIXUP_6ST_DIG),
    quirk(0x8086, 0xa100, c"Intel mobo".as_ptr(), ALC880_FIXUP_5ST_DIG),
    quirk(0x8086, 0xd400, c"Intel mobo".as_ptr(), ALC880_FIXUP_5ST_DIG),
    quirk(0x8086, 0xd401, c"Intel mobo".as_ptr(), ALC880_FIXUP_5ST_DIG),
    quirk(0x8086, 0xd402, c"Intel mobo".as_ptr(), ALC880_FIXUP_3ST_DIG),
    quirk(0x8086, 0xe224, c"Intel mobo".as_ptr(), ALC880_FIXUP_5ST_DIG),
    quirk(0x8086, 0xe305, c"Intel mobo".as_ptr(), ALC880_FIXUP_3ST_DIG),
    quirk(0x8086, 0xe308, c"Intel mobo".as_ptr(), ALC880_FIXUP_3ST_DIG),
    quirk(0x8086, 0xe400, c"Intel mobo".as_ptr(), ALC880_FIXUP_5ST_DIG),
    quirk(0x8086, 0xe401, c"Intel mobo".as_ptr(), ALC880_FIXUP_5ST_DIG),
    quirk(0x8086, 0xe402, c"Intel mobo".as_ptr(), ALC880_FIXUP_5ST_DIG),
    /* default Intel */
    quirk_vendor(0x8086, c"Intel mobo".as_ptr(), ALC880_FIXUP_3ST),
    quirk(0xa0a0, 0x0560, c"AOpen i915GMm-HFS".as_ptr(), ALC880_FIXUP_5ST_DIG),
    quirk(0xe803, 0x1019, core::ptr::null(), ALC880_FIXUP_6ST_DIG),
    quirk(0, 0, core::ptr::null(), 0),
];

static alc880_fixup_models: [hda_model_fixup; 8] = [
    model(ALC880_FIXUP_3ST, c"3stack".as_ptr()),
    model(ALC880_FIXUP_3ST_DIG, c"3stack-digout".as_ptr()),
    model(ALC880_FIXUP_5ST, c"5stack".as_ptr()),
    model(ALC880_FIXUP_5ST_DIG, c"5stack-digout".as_ptr()),
    model(ALC880_FIXUP_6ST, c"6stack".as_ptr()),
    model(ALC880_FIXUP_6ST_DIG, c"6stack-digout".as_ptr()),
    model(ALC880_FIXUP_6ST_AUTOMUTE, c"6stack-automute".as_ptr()),
    model(0, core::ptr::null()),
];

/*
 * OK, here we have finally the probe for ALC880
 */
unsafe extern "C" fn alc880_probe(codec: *mut hda_codec, _id: *const hda_device_id) -> c_int {
    let mut err: c_int;

    err = unsafe { alc_alloc_spec(codec, 0x0b) };
    if err < 0 {
        return err;
    }

    let spec = unsafe { (*codec).spec };
    unsafe {
        (*spec).gen.need_dac_fix = 1;
        (*spec).gen.beep_nid = 0x01;
    }

    unsafe { alc_pre_init(codec) };

    unsafe {
        snd_hda_pick_fixup(
            codec,
            alc880_fixup_models.as_ptr(),
            alc880_fixup_tbl.as_ptr(),
            alc880_fixups.as_ptr(),
        );
        snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);
    }

    /* automatic parse from the BIOS config */
    err = unsafe { alc880_parse_auto_config(codec) };
    if err < 0 {
        unsafe { snd_hda_gen_remove(codec) };
        return err;
    }

    if unsafe { !(*spec).gen.no_analog } {
        err = unsafe { set_beep_amp(spec, 0x0b, 0x05, HDA_INPUT) };
        if err < 0 {
            unsafe { snd_hda_gen_remove(codec) };
            return err;
        }
    }

    unsafe { snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE) };

    0
}

static alc880_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(alc880_probe),
    remove: Some(snd_hda_gen_remove),
    build_controls: Some(alc_build_controls),
    build_pcms: Some(snd_hda_gen_build_pcms),
    init: Some(alc_init),
    unsol_event: Some(alc880_unsol_event),
    resume: Some(alc_resume),
    suspend: Some(alc_suspend),
    check_power_status: Some(snd_hda_gen_check_power_status),
    stream_pm: Some(snd_hda_gen_stream_pm),
};

/*
 * driver entries
 */
static snd_hda_id_alc880: [hda_device_id; 2] = [
    hda_device_id { vendor_id: 0x10ec0880, name: c"ALC880".as_ptr() },
    hda_device_id { vendor_id: 0, name: core::ptr::null() }, /* terminator */
];
/* MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_alc880); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("Realtek ALC880 HD-audio codec"); */
/* MODULE_IMPORT_NS("SND_HDA_CODEC_REALTEK"); */

static mut alc880_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_alc880.as_ptr(),
    ops: &alc880_codec_ops,
};

/* module_hda_codec_driver(alc880_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
