// SPDX-License-Identifier: GPL-2.0-or-later

// Rust translation of hda/codecs/realtek/alc268.c.
// Includes from the original C source:
//   <linux/init.h>
//   <linux/module.h>
//   "realtek.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

type hda_nid_t = u16;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_codec {
    pub spec: *mut c_void,
    pub control_mutex: mutex,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_gen_spec_autocfg {
    pub speaker_pins: [hda_nid_t; 1],
}

#[repr(C)]
pub struct hda_gen_spec {
    pub beep_nid: hda_nid_t,
    pub no_analog: bool,
    pub autocfg: hda_gen_spec_autocfg,
}

#[repr(C)]
pub struct alc_spec {
    pub gen: hda_gen_spec,
    pub shutup: Option<unsafe extern "C" fn(*mut hda_codec)>,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub subdevice: c_int,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut c_void) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct hda_verb {
    pub nid: hda_nid_t,
    pub verb: u32,
    pub param: u32,
}

#[repr(C)]
pub struct hda_pintbl {
    pub nid: hda_nid_t,
    pub val: u32,
}

#[repr(C)]
pub union hda_fixup_v {
    pub func: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_fixup, c_int)>,
    pub verbs: *const hda_verb,
    pub pins: *const hda_pintbl,
}

#[repr(C)]
pub struct hda_fixup {
    pub type_: c_int,
    pub v: hda_fixup_v,
}

#[repr(C)]
pub struct hda_model_fixup {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct hda_quirk {
    pub subvendor: u32,
    pub subdevice: u32,
    pub name: *const c_char,
    pub value: c_int,
}

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, u32)>,
    pub resume: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub check_power_status: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t) -> c_int>,
    pub stream_pm: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, bool)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut hda_codec;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn snd_hda_mixer_amp_switch_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn snd_hda_mixer_amp_switch_info(kcontrol: *mut snd_kcontrol, uinfo: *mut c_void) -> c_int;
    fn snd_hda_mixer_amp_switch_get(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn alc_fixup_inv_dmic(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
    fn alc_parse_auto_config(
        codec: *mut hda_codec,
        cfg: *const c_void,
        ssids: *const hda_nid_t,
    ) -> c_int;
    fn alc_alloc_spec(codec: *mut hda_codec, mixer_nid: c_int) -> c_int;
    fn has_cdefine_beep(codec: *mut hda_codec) -> bool;
    fn alc_eapd_shutup(codec: *mut hda_codec);
    fn alc_pre_init(codec: *mut hda_codec);
    fn snd_hda_pick_fixup(
        codec: *mut hda_codec,
        models: *const hda_model_fixup,
        tbl: *const hda_quirk,
        fixups: *const hda_fixup,
    );
    fn snd_hda_apply_fixup(codec: *mut hda_codec, action: c_int);
    fn snd_hda_gen_add_kctl(
        spec: *mut hda_gen_spec,
        name: *const c_char,
        knew: *const snd_kcontrol_new,
    ) -> *mut snd_kcontrol;
    fn snd_hda_add_verbs(codec: *mut hda_codec, list: *const hda_verb);
    fn query_amp_caps(codec: *mut hda_codec, nid: hda_nid_t, direction: c_int) -> u32;
    fn snd_hda_override_amp_caps(codec: *mut hda_codec, nid: hda_nid_t, dir: c_int, caps: u32);
    fn snd_hda_gen_remove(codec: *mut hda_codec);
    fn alc_build_controls(codec: *mut hda_codec) -> c_int;
    fn snd_hda_gen_build_pcms(codec: *mut hda_codec) -> c_int;
    fn alc_init(codec: *mut hda_codec) -> c_int;
    fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: u32);
    fn alc_resume(codec: *mut hda_codec) -> c_int;
    fn alc_suspend(codec: *mut hda_codec);
    fn snd_hda_gen_check_power_status(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    fn snd_hda_gen_stream_pm(codec: *mut hda_codec, nid: hda_nid_t, on: bool);
}

const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const HDA_SUBDEV_AMP_FLAG: c_int = 1;
const HDA_INPUT: c_int = 0;
const AC_VERB_SET_AMP_GAIN_MUTE: u32 = 0x300;
const AC_VERB_SET_EAPD_BTLENABLE: u32 = 0x70c;
const HDA_FIXUP_FUNC: c_int = 0;
const HDA_FIXUP_VERBS: c_int = 1;
const HDA_FIXUP_PINS: c_int = 2;
const HDA_FIXUP_ACT_PRE_PROBE: c_int = 0;
const HDA_FIXUP_ACT_PROBE: c_int = 1;
const AC_AMPCAP_OFFSET_SHIFT: u32 = 0;
const AC_AMPCAP_NUM_STEPS_SHIFT: u32 = 8;
const AC_AMPCAP_STEP_SIZE_SHIFT: u32 = 16;
const AC_AMPCAP_MUTE_SHIFT: u32 = 31;
const ENOMEM: c_int = 12;

const fn HDA_COMPOSE_AMP_VAL(nid: u32, channel: u32, index: u32, direction: u32) -> c_ulong {
    (nid | (channel << 16) | (index << 19) | (direction << 22)) as c_ulong
}

const fn AMP_IN_UNMUTE(index: u32) -> u32 {
    index << 8
}

const fn AMP_IN_MUTE(index: u32) -> u32 {
    (index << 8) | 0x80
}

const fn HDA_CODEC_ID(_id: u32, _name: *const c_char) -> hda_device_id {
    hda_device_id { _private: [] }
}

const fn SND_PCI_QUIRK(subvendor: u32, subdevice: u32, name: *const c_char, value: c_int) -> hda_quirk {
    hda_quirk {
        subvendor,
        subdevice,
        name,
        value,
    }
}

/* bind Beep switches of both NID 0x0f and 0x10 */
unsafe extern "C" fn alc268_beep_switch_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec: *mut hda_codec = snd_kcontrol_chip(kcontrol);
    let pval: c_ulong;
    let mut err: c_int;

    mutex_lock(&mut (*codec).control_mutex);
    pval = (*kcontrol).private_value;
    (*kcontrol).private_value = (pval & !0xff) | 0x0f;
    err = snd_hda_mixer_amp_switch_put(kcontrol, ucontrol);
    if err >= 0 {
        (*kcontrol).private_value = (pval & !0xff) | 0x10;
        err = snd_hda_mixer_amp_switch_put(kcontrol, ucontrol);
    }
    (*kcontrol).private_value = pval;
    mutex_unlock(&mut (*codec).control_mutex);
    err
}

static alc268_beep_mixer: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Beep Playback Volume\0".as_ptr() as *const c_char,
        subdevice: HDA_SUBDEV_AMP_FLAG,
        info: None,
        get: None,
        put: None,
        private_value: HDA_COMPOSE_AMP_VAL(0x1d, 0x0, 0, HDA_INPUT as u32),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Beep Playback Switch\0".as_ptr() as *const c_char,
        subdevice: HDA_SUBDEV_AMP_FLAG,
        info: Some(snd_hda_mixer_amp_switch_info),
        get: Some(snd_hda_mixer_amp_switch_get),
        put: Some(alc268_beep_switch_put),
        private_value: HDA_COMPOSE_AMP_VAL(0x0f, 3, 1, HDA_INPUT as u32),
    },
];

/* set PCBEEP vol = 0, mute connections */
static alc268_beep_init_verbs: [hda_verb; 4] = [
    hda_verb {
        nid: 0x1d,
        verb: AC_VERB_SET_AMP_GAIN_MUTE,
        param: AMP_IN_UNMUTE(0),
    },
    hda_verb {
        nid: 0x0f,
        verb: AC_VERB_SET_AMP_GAIN_MUTE,
        param: AMP_IN_MUTE(1),
    },
    hda_verb {
        nid: 0x10,
        verb: AC_VERB_SET_AMP_GAIN_MUTE,
        param: AMP_IN_MUTE(1),
    },
    hda_verb {
        nid: 0,
        verb: 0,
        param: 0,
    },
];

const ALC268_FIXUP_INV_DMIC: c_int = 0;
const ALC268_FIXUP_HP_EAPD: c_int = 1;
const ALC268_FIXUP_SPDIF: c_int = 2;

static ALC268_FIXUP_HP_EAPD_VERBS: [hda_verb; 2] = [
    hda_verb {
        nid: 0x15,
        verb: AC_VERB_SET_EAPD_BTLENABLE,
        param: 0,
    },
    hda_verb {
        nid: 0,
        verb: 0,
        param: 0,
    },
];

static ALC268_FIXUP_SPDIF_PINS: [hda_pintbl; 2] = [
    hda_pintbl {
        nid: 0x1e,
        val: 0x014b1180,
    }, /* enable SPDIF out */
    hda_pintbl { nid: 0, val: 0 },
];

static alc268_fixups: [hda_fixup; 3] = [
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_v {
            func: Some(alc_fixup_inv_dmic),
        },
    },
    hda_fixup {
        type_: HDA_FIXUP_VERBS,
        v: hda_fixup_v {
            verbs: ALC268_FIXUP_HP_EAPD_VERBS.as_ptr(),
        },
    },
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_v {
            pins: ALC268_FIXUP_SPDIF_PINS.as_ptr(),
        },
    },
];

static alc268_fixup_models: [hda_model_fixup; 4] = [
    hda_model_fixup {
        id: ALC268_FIXUP_INV_DMIC,
        name: b"inv-dmic\0".as_ptr() as *const c_char,
    },
    hda_model_fixup {
        id: ALC268_FIXUP_HP_EAPD,
        name: b"hp-eapd\0".as_ptr() as *const c_char,
    },
    hda_model_fixup {
        id: ALC268_FIXUP_SPDIF,
        name: b"spdif\0".as_ptr() as *const c_char,
    },
    hda_model_fixup {
        id: 0,
        name: core::ptr::null(),
    },
];

static alc268_fixup_tbl: [hda_quirk; 4] = [
    SND_PCI_QUIRK(
        0x1025,
        0x0139,
        b"Acer TravelMate 6293\0".as_ptr() as *const c_char,
        ALC268_FIXUP_SPDIF,
    ),
    SND_PCI_QUIRK(
        0x1025,
        0x015b,
        b"Acer AOA 150 (ZG5)\0".as_ptr() as *const c_char,
        ALC268_FIXUP_INV_DMIC,
    ),
    /*
     * below is codec SSID since multiple Toshiba laptops have the
     * same PCI SSID 1179:ff00
     */
    SND_PCI_QUIRK(
        0x1179,
        0xff06,
        b"Toshiba P200\0".as_ptr() as *const c_char,
        ALC268_FIXUP_HP_EAPD,
    ),
    hda_quirk {
        subvendor: 0,
        subdevice: 0,
        name: core::ptr::null(),
        value: 0,
    },
];

/*
 * BIOS auto configuration
 */
unsafe extern "C" fn alc268_parse_auto_config(codec: *mut hda_codec) -> c_int {
    static alc268_ssids: [hda_nid_t; 4] = [0x15, 0x1b, 0x14, 0];
    alc_parse_auto_config(codec, core::ptr::null(), alc268_ssids.as_ptr())
}

/*
 */
unsafe extern "C" fn alc268_probe(codec: *mut hda_codec, _id: *const hda_device_id) -> c_int {
    let spec: *mut alc_spec;
    let mut i: usize;
    let mut err: c_int;

    /* ALC268 has no aa-loopback mixer */
    err = alc_alloc_spec(codec, 0);
    if err < 0 {
        return err;
    }

    spec = (*codec).spec as *mut alc_spec;
    if has_cdefine_beep(codec) {
        (*spec).gen.beep_nid = 0x01;
    }

    (*spec).shutup = Some(alc_eapd_shutup);

    alc_pre_init(codec);

    snd_hda_pick_fixup(
        codec,
        alc268_fixup_models.as_ptr(),
        alc268_fixup_tbl.as_ptr(),
        alc268_fixups.as_ptr(),
    );
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

    /* automatic parse from the BIOS config */
    err = alc268_parse_auto_config(codec);
    if err < 0 {
        snd_hda_gen_remove(codec);
        return err;
    }

    if err > 0 && !(*spec).gen.no_analog && (*spec).gen.autocfg.speaker_pins[0] != 0x1d {
        i = 0;
        while i < alc268_beep_mixer.len() {
            if snd_hda_gen_add_kctl(
                &mut (*spec).gen,
                core::ptr::null(),
                &alc268_beep_mixer[i],
            )
            .is_null()
            {
                err = -ENOMEM;
                snd_hda_gen_remove(codec);
                return err;
            }
            i += 1;
        }
        snd_hda_add_verbs(codec, alc268_beep_init_verbs.as_ptr());
        if query_amp_caps(codec, 0x1d, HDA_INPUT) == 0 {
            /* override the amp caps for beep generator */
            snd_hda_override_amp_caps(
                codec,
                0x1d,
                HDA_INPUT,
                (0x0c << AC_AMPCAP_OFFSET_SHIFT)
                    | (0x0c << AC_AMPCAP_NUM_STEPS_SHIFT)
                    | (0x07 << AC_AMPCAP_STEP_SIZE_SHIFT)
                    | (0 << AC_AMPCAP_MUTE_SHIFT),
            );
        }
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);

    0
}

static alc268_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(alc268_probe),
    remove: Some(snd_hda_gen_remove),
    build_controls: Some(alc_build_controls),
    build_pcms: Some(snd_hda_gen_build_pcms),
    init: Some(alc_init),
    unsol_event: Some(snd_hda_jack_unsol_event),
    resume: Some(alc_resume),
    suspend: Some(alc_suspend),
    check_power_status: Some(snd_hda_gen_check_power_status),
    stream_pm: Some(snd_hda_gen_stream_pm),
};

/*
 * driver entries
 */
static snd_hda_id_alc268: [hda_device_id; 3] = [
    HDA_CODEC_ID(0x10ec0267, b"ALC267\0".as_ptr() as *const c_char),
    HDA_CODEC_ID(0x10ec0268, b"ALC268\0".as_ptr() as *const c_char),
    hda_device_id { _private: [] }, /* terminator */
];

// MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_alc268);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Realtek ALC267/268 HD-audio codec");
// MODULE_IMPORT_NS("SND_HDA_CODEC_REALTEK");

static mut alc268_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_alc268.as_ptr(),
    ops: &alc268_codec_ops,
};

// module_hda_codec_driver(alc268_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
