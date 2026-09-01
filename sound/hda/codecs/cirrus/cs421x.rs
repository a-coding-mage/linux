// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cirrus Logic CS421x HD-audio codec
 */

// Dependencies from the original C file:
// linux/init.h, linux/slab.h, linux/module.h, sound/core.h, linux/pci.h,
// sound/tlv.h, sound/hda_codec.h, hda_local.h, hda_auto_parser.h,
// hda_jack.h, ../generic.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::zeroed;
use core::ptr::{null, null_mut};

type hda_nid_t = c_uint;

#[repr(C)]
pub struct hda_gen_spec {
    pub autocfg: auto_pin_cfg,
    pub master_mute: bool,
    pub automute_speaker: bool,
    pub hp_jack_present: bool,
    pub automute_hook: Option<unsafe extern "C" fn(*mut hda_codec)>,
}

#[repr(C)]
pub struct auto_pin_cfg {
    pub dig_outs: c_int,
    pub dig_out_pins: [hda_nid_t; 16],
    pub speaker_outs: c_int,
}

#[repr(C)]
pub struct hda_codec {
    pub spec: *mut c_void,
    pub power_save_node: c_int,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
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
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_data,
}

#[repr(C)]
pub union snd_ctl_elem_value_data {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub struct hda_model_fixup {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct hda_quirk {
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub name: *const c_char,
    pub value: c_int,
}

#[repr(C)]
pub struct hda_pintbl {
    pub nid: hda_nid_t,
    pub val: c_uint,
}

#[repr(C)]
pub struct hda_fixup {
    pub type_: c_int,
    pub v: hda_fixup_v,
    pub chained: bool,
    pub chain_id: c_int,
}

#[repr(C)]
pub union hda_fixup_v {
    pub pins: *const hda_pintbl,
    pub func: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_fixup, c_int)>,
}

#[repr(C)]
pub struct hda_verb {
    pub nid: hda_nid_t,
    pub verb: c_uint,
    pub param: c_uint,
}

#[repr(C)]
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub access: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_tlv,
}

#[repr(C)]
pub struct hda_jack_callback {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_device_id {
    pub vendor_id: c_uint,
    pub rev_id: c_uint,
    pub api_version: c_uint,
    pub name: *const c_char,
    pub driver_data: usize,
}

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, c_uint)>,
    pub suspend: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub stream_pm: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, bool)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

#[repr(C)]
pub struct cs_spec {
    pub gen: hda_gen_spec,

    pub gpio_mask: c_uint,
    pub gpio_dir: c_uint,
    pub gpio_data: c_uint,
    pub gpio_eapd_hp: c_uint,      /* EAPD GPIO bit for headphones */
    pub gpio_eapd_speaker: c_uint, /* EAPD GPIO bit for speakers */

    /* CS421x */
    pub spdif_detect: c_uint,
    pub spdif_present: c_uint,
    pub sense_b: c_uint,
    pub vendor_nid: hda_nid_t,

    /* for MBP SPDIF control */
    pub spdif_sw_put:
        Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

/* CS421x boards */
const CS421X_CDB4210: c_int = 0;
const CS421X_SENSE_B: c_int = 1;
const CS421X_STUMPY: c_int = 2;

/* Vendor-specific processing widget */
const CS_DIG_OUT1_PIN_NID: c_uint = 0x10;
const CS_DIG_OUT2_PIN_NID: c_uint = 0x15;
const CS_DMIC1_PIN_NID: c_uint = 0x0e;
const CS_DMIC2_PIN_NID: c_uint = 0x12;

/* coef indices */
const IDX_SPDIF_STAT: c_uint = 0x0000;
const IDX_SPDIF_CTL: c_uint = 0x0001;
const IDX_ADC_CFG: c_uint = 0x0002;
/* SZC bitmask, 4 modes below:
 * 0 = immediate,
 * 1 = digital immediate, analog zero-cross
 * 2 = digtail & analog soft-ramp
 * 3 = digital soft-ramp, analog zero-cross
 */
const CS_COEF_ADC_SZC_MASK: c_uint = 3 << 0;
const CS_COEF_ADC_MIC_SZC_MODE: c_uint = 3 << 0; /* SZC setup for mic */
const CS_COEF_ADC_LI_SZC_MODE: c_uint = 3 << 0; /* SZC setup for line-in */
/* PGA mode: 0 = differential, 1 = signle-ended */
const CS_COEF_ADC_MIC_PGA_MODE: c_uint = 1 << 5; /* PGA setup for mic */
const CS_COEF_ADC_LI_PGA_MODE: c_uint = 1 << 6; /* PGA setup for line-in */
const IDX_DAC_CFG: c_uint = 0x0003;
/* SZC bitmask, 4 modes below:
 * 0 = Immediate
 * 1 = zero-cross
 * 2 = soft-ramp
 * 3 = soft-ramp on zero-cross
 */
const CS_COEF_DAC_HP_SZC_MODE: c_uint = 3 << 0; /* nid 0x02 */
const CS_COEF_DAC_LO_SZC_MODE: c_uint = 3 << 2; /* nid 0x03 */
const CS_COEF_DAC_SPK_SZC_MODE: c_uint = 3 << 4; /* nid 0x04 */

const IDX_BEEP_CFG: c_uint = 0x0004;
/* 0x0008 - test reg key */
/* 0x0009 - 0x0014 -> 12 test regs */
/* 0x0015 - visibility reg */

/*
 * Cirrus Logic CS4210
 *
 * 1 DAC => HP(sense) / Speakers,
 * 1 ADC <= LineIn(sense) / MicIn / DMicIn,
 * 1 SPDIF OUT => SPDIF Transmitter(sense)
 */
const CS4210_DAC_NID: hda_nid_t = 0x02;
const CS4210_ADC_NID: hda_nid_t = 0x03;
const CS4210_VENDOR_NID: hda_nid_t = 0x0B;
const CS421X_DMIC_PIN_NID: hda_nid_t = 0x09; /* Port E */
const CS421X_SPDIF_PIN_NID: hda_nid_t = 0x0A; /* Port H */

const CS421X_IDX_DEV_CFG: c_uint = 0x01;
const CS421X_IDX_ADC_CFG: c_uint = 0x02;
const CS421X_IDX_DAC_CFG: c_uint = 0x03;
const CS421X_IDX_SPK_CTL: c_uint = 0x04;

/* Cirrus Logic CS4213 is like CS4210 but does not have SPDIF input/output */
const CS4213_VENDOR_NID: hda_nid_t = 0x09;

const AC_VERB_SET_COEF_INDEX: c_uint = 0;
const AC_VERB_GET_PROC_COEF: c_uint = 0;
const AC_VERB_SET_PROC_COEF: c_uint = 0;
const AC_VERB_SET_GPIO_DATA: c_uint = 0;
const AC_VERB_SET_PROC_STATE: c_uint = 0;
const AC_VERB_SET_POWER_STATE: c_uint = 0;
const AC_PWRST_D3: c_uint = 0;
const AC_JACK_PORT_NONE: c_uint = 0;
const AC_DEFCFG_PORT_CONN: c_uint = 0;
const AC_DEFCFG_PORT_CONN_SHIFT: c_uint = 0;
const AC_WCAP_UNSOL_CAP: c_uint = 0;
const PIN_OUT: c_uint = 0;
const HDA_OUTPUT: c_uint = 0;
const AC_AMPCAP_NUM_STEPS_SHIFT: c_uint = 0;
const AC_AMPCAP_OFFSET_SHIFT: c_uint = 0;
const HDA_FIXUP_ACT_PRE_PROBE: c_int = 0;
const HDA_FIXUP_ACT_PROBE: c_int = 0;
const HDA_FIXUP_PINS: c_int = 0;
const HDA_FIXUP_FUNC: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0;
const ENOMEM: c_int = 12;

unsafe extern "C" {
    fn snd_hda_codec_write(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    ) -> c_uint;
    fn snd_hda_codec_read(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    ) -> c_uint;
    fn snd_hda_gen_update_outputs(codec: *mut hda_codec);
    fn snd_hda_codec_get_pincfg(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn get_defcfg_connect(def_conf: c_uint) -> c_uint;
    fn snd_hda_gen_spec_init(spec: *mut hda_gen_spec);
    fn snd_hda_codec_set_pincfg(codec: *mut hda_codec, nid: hda_nid_t, cfg: c_uint);
    fn snd_hda_jack_detect(codec: *mut hda_codec, nid: hda_nid_t) -> bool;
    fn snd_hda_set_pin_ctl(codec: *mut hda_codec, nid: hda_nid_t, val: c_uint);
    fn get_wcaps(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn snd_hda_jack_detect_enable_callback(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        cb: Option<unsafe extern "C" fn(*mut hda_codec, *mut hda_jack_callback)>,
    );
    fn snd_hda_sequence_write(codec: *mut hda_codec, seq: *const hda_verb);
    fn snd_hda_gen_init(codec: *mut hda_codec);
    fn snd_hda_codec_set_gpio(
        codec: *mut hda_codec,
        mask: c_uint,
        dir: c_uint,
        data: c_uint,
        send: c_int,
    );
    fn query_amp_caps(codec: *mut hda_codec, nid: hda_nid_t, direction: c_uint) -> c_uint;
    fn snd_hda_override_amp_caps(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        direction: c_uint,
        caps: c_uint,
    );
    fn snd_hda_parse_pin_defcfg(
        codec: *mut hda_codec,
        cfg: *mut auto_pin_cfg,
        ignore_nids: *const hda_nid_t,
        cond_flags: c_uint,
    ) -> c_int;
    fn snd_hda_gen_parse_auto_config(codec: *mut hda_codec, cfg: *mut auto_pin_cfg) -> c_int;
    fn snd_hda_gen_add_kctl(
        spec: *mut hda_gen_spec,
        name: *const c_char,
        knew: *const snd_kcontrol_new,
    ) -> *mut snd_kcontrol;
    fn snd_hda_shutup_pins(codec: *mut hda_codec);
    fn snd_hda_pick_fixup(
        codec: *mut hda_codec,
        models: *const hda_model_fixup,
        quirks: *const hda_quirk,
        fixups: *const hda_fixup,
    );
    fn snd_hda_apply_fixup(codec: *mut hda_codec, action: c_int);
    fn snd_hda_gen_remove(codec: *mut hda_codec);
    fn snd_hda_gen_build_controls(codec: *mut hda_codec) -> c_int;
    fn snd_hda_gen_build_pcms(codec: *mut hda_codec) -> c_int;
    fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: c_uint);
    fn snd_hda_gen_stream_pm(codec: *mut hda_codec, nid: hda_nid_t, on: bool);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut hda_codec;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
}

unsafe fn cs_vendor_coef_get(codec: *mut hda_codec, idx: c_uint) -> c_int {
    let spec = (*codec).spec as *mut cs_spec;

    snd_hda_codec_write(codec, (*spec).vendor_nid, 0, AC_VERB_SET_COEF_INDEX, idx);
    snd_hda_codec_read(codec, (*spec).vendor_nid, 0, AC_VERB_GET_PROC_COEF, 0) as c_int
}

unsafe fn cs_vendor_coef_set(codec: *mut hda_codec, idx: c_uint, coef: c_uint) {
    let spec = (*codec).spec as *mut cs_spec;

    snd_hda_codec_write(codec, (*spec).vendor_nid, 0, AC_VERB_SET_COEF_INDEX, idx);
    snd_hda_codec_write(codec, (*spec).vendor_nid, 0, AC_VERB_SET_PROC_COEF, coef);
}

/*
 * auto-mute and auto-mic switching
 * CS421x auto-output redirecting
 * HP/SPK/SPDIF
 */

unsafe extern "C" fn cs_automute(codec: *mut hda_codec) {
    let spec = (*codec).spec as *mut cs_spec;

    /* mute HPs if spdif jack (SENSE_B) is present */
    (*spec).gen.master_mute = (*spec).spdif_present != 0 && (*spec).sense_b != 0;

    snd_hda_gen_update_outputs(codec);

    if (*spec).gpio_eapd_hp != 0 || (*spec).gpio_eapd_speaker != 0 {
        if (*spec).gen.automute_speaker {
            (*spec).gpio_data = if (*spec).gen.hp_jack_present {
                (*spec).gpio_eapd_hp
            } else {
                (*spec).gpio_eapd_speaker
            };
        } else {
            (*spec).gpio_data = (*spec).gpio_eapd_hp | (*spec).gpio_eapd_speaker;
        }
        snd_hda_codec_write(codec, 0x01, 0, AC_VERB_SET_GPIO_DATA, (*spec).gpio_data);
    }
}

unsafe fn is_active_pin(codec: *mut hda_codec, nid: hda_nid_t) -> bool {
    let val: c_uint;

    val = snd_hda_codec_get_pincfg(codec, nid);
    get_defcfg_connect(val) != AC_JACK_PORT_NONE
}

unsafe fn cs_alloc_spec(codec: *mut hda_codec, vendor_nid: c_int) -> *mut cs_spec {
    let spec: *mut cs_spec;

    spec = kzalloc(core::mem::size_of::<cs_spec>(), 0) as *mut cs_spec;
    if spec.is_null() {
        return null_mut();
    }
    (*codec).spec = spec as *mut c_void;
    (*spec).vendor_nid = vendor_nid as hda_nid_t;
    (*codec).power_save_node = 1;
    snd_hda_gen_spec_init(&mut (*spec).gen);

    spec
}

/*
 * Cirrus Logic CS4210
 *
 * 1 DAC => HP(sense) / Speakers,
 * 1 ADC <= LineIn(sense) / MicIn / DMicIn,
 * 1 SPDIF OUT => SPDIF Transmitter(sense)
 */

/* CS4210 board names */
static cs421x_models: [hda_model_fixup; 3] = [
    hda_model_fixup { id: CS421X_CDB4210, name: c"cdb4210".as_ptr() },
    hda_model_fixup { id: CS421X_STUMPY, name: c"stumpy".as_ptr() },
    hda_model_fixup { id: 0, name: null() },
];

static cs421x_fixup_tbl: [hda_quirk; 2] = [
    /* Test Intel board + CDB2410  */
    hda_quirk {
        subvendor: 0x8086,
        subdevice: 0x5001,
        name: c"DP45SG/CDB4210".as_ptr(),
        value: CS421X_CDB4210,
    },
    hda_quirk { subvendor: 0, subdevice: 0, name: null(), value: 0 }, /* terminator */
];

/* CS4210 board pinconfigs */
/* Default CS4210 (CDB4210)*/
static cdb4210_pincfgs: [hda_pintbl; 7] = [
    hda_pintbl { nid: 0x05, val: 0x0321401f },
    hda_pintbl { nid: 0x06, val: 0x90170010 },
    hda_pintbl { nid: 0x07, val: 0x03813031 },
    hda_pintbl { nid: 0x08, val: 0xb7a70037 },
    hda_pintbl { nid: 0x09, val: 0xb7a6003e },
    hda_pintbl { nid: 0x0a, val: 0x034510f0 },
    hda_pintbl { nid: 0, val: 0 }, /* terminator */
];

/* Stumpy ChromeBox */
static stumpy_pincfgs: [hda_pintbl; 7] = [
    hda_pintbl { nid: 0x05, val: 0x022120f0 },
    hda_pintbl { nid: 0x06, val: 0x901700f0 },
    hda_pintbl { nid: 0x07, val: 0x02a120f0 },
    hda_pintbl { nid: 0x08, val: 0x77a70037 },
    hda_pintbl { nid: 0x09, val: 0x77a6003e },
    hda_pintbl { nid: 0x0a, val: 0x434510f0 },
    hda_pintbl { nid: 0, val: 0 }, /* terminator */
];

/* Setup GPIO/SENSE for each board (if used) */
unsafe extern "C" fn cs421x_fixup_sense_b(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: c_int,
) {
    let spec = (*codec).spec as *mut cs_spec;

    if action == HDA_FIXUP_ACT_PRE_PROBE {
        (*spec).sense_b = 1;
    }
}

static cs421x_fixups: [hda_fixup; 3] = [
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_v { pins: cdb4210_pincfgs.as_ptr() },
        chained: true,
        chain_id: CS421X_SENSE_B,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_v { func: Some(cs421x_fixup_sense_b) },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_v { pins: stumpy_pincfgs.as_ptr() },
        chained: false,
        chain_id: 0,
    },
];

static cs421x_coef_init_verbs: [hda_verb; 8] = [
    hda_verb { nid: 0x0B, verb: AC_VERB_SET_PROC_STATE, param: 1 },
    hda_verb { nid: 0x0B, verb: AC_VERB_SET_COEF_INDEX, param: CS421X_IDX_DEV_CFG },
    /*
     *  Disable Coefficient Index Auto-Increment(DAI)=1,
     *  PDREF=0
     */
    hda_verb { nid: 0x0B, verb: AC_VERB_SET_PROC_COEF, param: 0x0001 },

    hda_verb { nid: 0x0B, verb: AC_VERB_SET_COEF_INDEX, param: CS421X_IDX_ADC_CFG },
    /* ADC SZCMode = Digital Soft Ramp */
    hda_verb { nid: 0x0B, verb: AC_VERB_SET_PROC_COEF, param: 0x0002 },

    hda_verb { nid: 0x0B, verb: AC_VERB_SET_COEF_INDEX, param: CS421X_IDX_DAC_CFG },
    hda_verb {
        nid: 0x0B,
        verb: AC_VERB_SET_PROC_COEF,
        param: 0x0002 /* DAC SZCMode = Digital Soft Ramp */
            | 0x0004 /* Mute DAC on FIFO error */
            | 0x0008, /* Enable DAC High Pass Filter */
    },
    hda_verb { nid: 0, verb: 0, param: 0 }, /* terminator */
];

/* Errata: CS4210 rev A1 Silicon
 *
 * http://www.cirrus.com/en/pubs/errata/
 *
 * Description:
 * 1. Performance degredation is present in the ADC.
 * 2. Speaker output is not completely muted upon HP detect.
 * 3. Noise is present when clipping occurs on the amplified
 *    speaker outputs.
 *
 * Workaround:
 * The following verb sequence written to the registers during
 * initialization will correct the issues listed above.
 */

static cs421x_coef_init_verbs_A1_silicon_fixes: [hda_verb; 11] = [
    hda_verb { nid: 0x0B, verb: AC_VERB_SET_PROC_STATE, param: 0x01 }, /* VPW: processing on */

    hda_verb { nid: 0x0B, verb: AC_VERB_SET_COEF_INDEX, param: 0x0006 },
    hda_verb { nid: 0x0B, verb: AC_VERB_SET_PROC_COEF, param: 0x9999 }, /* Test mode: on */

    hda_verb { nid: 0x0B, verb: AC_VERB_SET_COEF_INDEX, param: 0x000A },
    hda_verb { nid: 0x0B, verb: AC_VERB_SET_PROC_COEF, param: 0x14CB }, /* Chop double */

    hda_verb { nid: 0x0B, verb: AC_VERB_SET_COEF_INDEX, param: 0x0011 },
    hda_verb { nid: 0x0B, verb: AC_VERB_SET_PROC_COEF, param: 0xA2D0 }, /* Increase ADC current */

    hda_verb { nid: 0x0B, verb: AC_VERB_SET_COEF_INDEX, param: 0x001A },
    hda_verb { nid: 0x0B, verb: AC_VERB_SET_PROC_COEF, param: 0x02A9 }, /* Mute speaker */

    hda_verb { nid: 0x0B, verb: AC_VERB_SET_COEF_INDEX, param: 0x001B },
    hda_verb { nid: 0x0B, verb: AC_VERB_SET_PROC_COEF, param: 0X1006 }, /* Remove noise */

    hda_verb { nid: 0, verb: 0, param: 0 }, /* terminator */
];

/* Speaker Amp Gain is controlled by the vendor widget's coef 4 */
static cs421x_speaker_boost_db_scale: [c_uint; 4] = [0, 900, 300, 0];

unsafe extern "C" fn cs421x_boost_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 3;
    0
}

unsafe extern "C" fn cs421x_boost_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);

    (*ucontrol).value.integer.value[0] =
        (cs_vendor_coef_get(codec, CS421X_IDX_SPK_CTL) & 0x0003) as i64;
    0
}

unsafe extern "C" fn cs421x_boost_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);

    let vol: c_uint = (*ucontrol).value.integer.value[0] as c_uint;
    let mut coef: c_uint = cs_vendor_coef_get(codec, CS421X_IDX_SPK_CTL) as c_uint;
    let original_coef: c_uint = coef;

    coef &= !0x0003;
    coef |= vol & 0x0003;
    if original_coef != coef {
        cs_vendor_coef_set(codec, CS421X_IDX_SPK_CTL, coef);
        return 1;
    }

    0
}

static cs421x_speaker_boost_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    name: c"Speaker Boost Playback Volume".as_ptr(),
    info: Some(cs421x_boost_vol_info),
    get: Some(cs421x_boost_vol_get),
    put: Some(cs421x_boost_vol_put),
    tlv: snd_kcontrol_tlv {
        p: cs421x_speaker_boost_db_scale.as_ptr(),
    },
};

unsafe fn cs4210_pinmux_init(codec: *mut hda_codec) {
    let spec = (*codec).spec as *mut cs_spec;
    let mut def_conf: c_uint;
    let mut coef: c_uint;

    /* GPIO, DMIC_SCL, DMIC_SDA and SENSE_B are multiplexed */
    coef = cs_vendor_coef_get(codec, CS421X_IDX_DEV_CFG) as c_uint;

    if (*spec).gpio_mask != 0 {
        coef |= 0x0008; /* B1,B2 are GPIOs */
    } else {
        coef &= !0x0008;
    }

    if (*spec).sense_b != 0 {
        coef |= 0x0010; /* B2 is SENSE_B, not inverted  */
    } else {
        coef &= !0x0010;
    }

    cs_vendor_coef_set(codec, CS421X_IDX_DEV_CFG, coef);

    if ((*spec).gpio_mask != 0 || (*spec).sense_b != 0)
        && is_active_pin(codec, CS421X_DMIC_PIN_NID)
    {
        /*
         *  GPIO or SENSE_B forced - disconnect the DMIC pin.
         */
        def_conf = snd_hda_codec_get_pincfg(codec, CS421X_DMIC_PIN_NID);
        def_conf &= !AC_DEFCFG_PORT_CONN;
        def_conf |= AC_JACK_PORT_NONE << AC_DEFCFG_PORT_CONN_SHIFT;
        snd_hda_codec_set_pincfg(codec, CS421X_DMIC_PIN_NID, def_conf);
    }
}

unsafe extern "C" fn cs4210_spdif_automute(
    codec: *mut hda_codec,
    _tbl: *mut hda_jack_callback,
) {
    let spec = (*codec).spec as *mut cs_spec;
    let mut spdif_present: bool = false;
    let spdif_pin: hda_nid_t = (*spec).gen.autocfg.dig_out_pins[0];

    /* detect on spdif is specific to CS4210 */
    if (*spec).spdif_detect == 0 || (*spec).vendor_nid != CS4210_VENDOR_NID {
        return;
    }

    spdif_present = snd_hda_jack_detect(codec, spdif_pin);
    if spdif_present == ((*spec).spdif_present != 0) {
        return;
    }

    (*spec).spdif_present = spdif_present as c_uint;
    /* SPDIF TX on/off */
    snd_hda_set_pin_ctl(
        codec,
        spdif_pin,
        if spdif_present { PIN_OUT } else { 0 },
    );

    cs_automute(codec);
}

unsafe fn parse_cs421x_digital(codec: *mut hda_codec) {
    let spec = (*codec).spec as *mut cs_spec;
    let cfg = &mut (*spec).gen.autocfg as *mut auto_pin_cfg;
    let mut i: c_int;

    i = 0;
    while i < (*cfg).dig_outs {
        let nid: hda_nid_t = (*cfg).dig_out_pins[i as usize];

        if get_wcaps(codec, nid) & AC_WCAP_UNSOL_CAP != 0 {
            (*spec).spdif_detect = 1;
            snd_hda_jack_detect_enable_callback(codec, nid, Some(cs4210_spdif_automute));
        }
        i += 1;
    }
}

unsafe extern "C" fn cs421x_init(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut cs_spec;

    if (*spec).vendor_nid == CS4210_VENDOR_NID {
        snd_hda_sequence_write(codec, cs421x_coef_init_verbs.as_ptr());
        snd_hda_sequence_write(codec, cs421x_coef_init_verbs_A1_silicon_fixes.as_ptr());
        cs4210_pinmux_init(codec);
    }

    snd_hda_gen_init(codec);

    if (*spec).gpio_mask != 0 {
        snd_hda_codec_set_gpio(
            codec,
            (*spec).gpio_mask,
            (*spec).gpio_dir,
            (*spec).gpio_data,
            0,
        );
    }

    cs4210_spdif_automute(codec, null_mut());

    0
}

unsafe fn fix_volume_caps(codec: *mut hda_codec, dac: hda_nid_t) {
    let mut caps: c_uint;

    /* set the upper-limit for mixer amp to 0dB */
    caps = query_amp_caps(codec, dac, HDA_OUTPUT);
    caps &= !(0x7f << AC_AMPCAP_NUM_STEPS_SHIFT);
    caps |= ((caps >> AC_AMPCAP_OFFSET_SHIFT) & 0x7f) << AC_AMPCAP_NUM_STEPS_SHIFT;
    snd_hda_override_amp_caps(codec, dac, HDA_OUTPUT, caps);
}

unsafe fn cs421x_parse_auto_config(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut cs_spec;
    let dac: hda_nid_t = CS4210_DAC_NID;
    let mut err: c_int;

    fix_volume_caps(codec, dac);

    err = snd_hda_parse_pin_defcfg(codec, &mut (*spec).gen.autocfg, null(), 0);
    if err < 0 {
        return err;
    }

    err = snd_hda_gen_parse_auto_config(codec, &mut (*spec).gen.autocfg);
    if err < 0 {
        return err;
    }

    parse_cs421x_digital(codec);

    if (*spec).gen.autocfg.speaker_outs != 0 && (*spec).vendor_nid == CS4210_VENDOR_NID {
        if snd_hda_gen_add_kctl(&mut (*spec).gen, null(), &cs421x_speaker_boost_ctl).is_null() {
            return -ENOMEM;
        }
    }

    0
}

/*
 *	Manage PDREF, when transitioning to D3hot
 *	(DAC,ADC) -> D3, PDREF=1, AFG->D3
 */
unsafe extern "C" fn cs421x_suspend(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut cs_spec;
    let mut coef: c_uint;

    snd_hda_shutup_pins(codec);

    snd_hda_codec_write(codec, CS4210_DAC_NID, 0, AC_VERB_SET_POWER_STATE, AC_PWRST_D3);
    snd_hda_codec_write(codec, CS4210_ADC_NID, 0, AC_VERB_SET_POWER_STATE, AC_PWRST_D3);

    if (*spec).vendor_nid == CS4210_VENDOR_NID {
        coef = cs_vendor_coef_get(codec, CS421X_IDX_DEV_CFG) as c_uint;
        coef |= 0x0004; /* PDREF */
        cs_vendor_coef_set(codec, CS421X_IDX_DEV_CFG, coef);
    }

    0
}

unsafe extern "C" fn cs421x_probe(codec: *mut hda_codec, id: *const hda_device_id) -> c_int {
    let spec: *mut cs_spec;
    let mut err: c_int;

    spec = cs_alloc_spec(codec, (*id).driver_data as c_int);
    if spec.is_null() {
        return -ENOMEM;
    }

    (*spec).gen.automute_hook = Some(cs_automute);

    if (*spec).vendor_nid == CS4210_VENDOR_NID {
        snd_hda_pick_fixup(
            codec,
            cs421x_models.as_ptr(),
            cs421x_fixup_tbl.as_ptr(),
            cs421x_fixups.as_ptr(),
        );
        snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

        /*
         *  Update the GPIO/DMIC/SENSE_B pinmux before the configuration
         *   is auto-parsed. If GPIO or SENSE_B is forced, DMIC input
         *   is disabled.
         */
        cs4210_pinmux_init(codec);
    }

    err = cs421x_parse_auto_config(codec);
    if err < 0 {
        snd_hda_gen_remove(codec);
        return err;
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);

    0
}

static cs421x_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(cs421x_probe),
    remove: Some(snd_hda_gen_remove),
    build_controls: Some(snd_hda_gen_build_controls),
    build_pcms: Some(snd_hda_gen_build_pcms),
    init: Some(cs421x_init),
    unsol_event: Some(snd_hda_jack_unsol_event),
    suspend: Some(cs421x_suspend),
    stream_pm: Some(snd_hda_gen_stream_pm),
};

/*
 * driver entries
 */
static snd_hda_id_cs421x: [hda_device_id; 3] = [
    hda_device_id {
        vendor_id: 0x10134210,
        rev_id: 0,
        api_version: 0,
        name: c"CS4210".as_ptr(),
        driver_data: CS4210_VENDOR_NID as usize,
    },
    hda_device_id {
        vendor_id: 0x10134213,
        rev_id: 0,
        api_version: 0,
        name: c"CS4213".as_ptr(),
        driver_data: CS4213_VENDOR_NID as usize,
    },
    hda_device_id {
        vendor_id: 0,
        rev_id: 0,
        api_version: 0,
        name: null(),
        driver_data: 0,
    }, /* terminator */
];
/* MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_cs421x); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("Cirrus Logic CS421x HD-audio codec"); */

static mut cs421x_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_cs421x.as_ptr(),
    ops: &cs421x_codec_ops,
};

/* module_hda_codec_driver(cs421x_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
