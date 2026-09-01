// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CM9825 HD-audio codec
 *
 * Translated from C implementation source. External Linux/HDA symbols are
 * declared here as dependencies supplied by the surrounding repository.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type hda_nid_t = c_uint;

const QUIRK_CM_STD: c_uint = 0x0;
const QUIRK_GENE_TWL7_SSID: c_uint = 0x160dc000;
const QUIRK_IBP_SSID: c_uint = 0x15bd3275;

/* CM9825 Offset Definitions */

const CM9825_VERB_SET_HPF_1: c_uint = 0x781;
const CM9825_VERB_SET_HPF_2: c_uint = 0x785;
const CM9825_VERB_SET_PLL: c_uint = 0x7a0;
const CM9825_VERB_SET_NEG: c_uint = 0x7a1;
const CM9825_VERB_SET_ADCL: c_uint = 0x7a2;
const CM9825_VERB_SET_DACL: c_uint = 0x7a3;
const CM9825_VERB_SET_MBIAS: c_uint = 0x7a4;
const CM9825_VERB_SET_VNEG: c_uint = 0x7a8;
const CM9825_VERB_SET_D2S: c_uint = 0x7a9;
const CM9825_VERB_SET_DACTRL: c_uint = 0x7aa;
const CM9825_VERB_SET_P3BCP: c_uint = 0x7ab;
const CM9825_VERB_SET_PDNEG: c_uint = 0x7ac;
const CM9825_VERB_SET_VDO: c_uint = 0x7ad;
const CM9825_VERB_SET_CDALR: c_uint = 0x7b0;
const CM9825_VERB_SET_MTCBA: c_uint = 0x7b1;
const CM9825_VERB_SET_OTP: c_uint = 0x7b2;
const CM9825_VERB_SET_OCP: c_uint = 0x7b3;
const CM9825_VERB_SET_GAD: c_uint = 0x7b4;
const CM9825_VERB_SET_TMOD: c_uint = 0x7b5;
const CM9825_VERB_SET_SNR: c_uint = 0x7b6;
const CM9825_VERB_SET_OMTP: c_uint = 0x7ef;
const CM9825_VERB_READ_OMTP: c_uint = 0xfec;

extern "C" {
    static AC_VERB_SET_EAPD_BTLENABLE: c_uint;
    static AC_VERB_SET_AMP_GAIN_MUTE: c_uint;
    static AC_AMP_SET_OUTPUT: c_uint;
    static AC_AMP_SET_RIGHT: c_uint;
    static AC_AMP_SET_LEFT: c_uint;
    static AC_VERB_SET_PIN_WIDGET_CONTROL: c_uint;
    static AC_VERB_SET_CONNECT_SEL: c_uint;
    static HDA_GEN_PCM_ACT_PREPARE: c_int;
    static HDA_GEN_PCM_ACT_CLEANUP: c_int;
    static HDA_FIXUP_ACT_INIT: c_int;
    static ENOMEM: c_int;
    static ENXIO: c_int;
}

const AUTO_CFG_MAX_INS: usize = 8;

#[repr(C)]
pub struct hda_verb {
    pub nid: hda_nid_t,
    pub verb: c_uint,
    pub param: c_uint,
}

#[repr(C)]
pub struct auto_pin_input {
    pub pin: hda_nid_t,
}

#[repr(C)]
pub struct auto_pin_cfg {
    pub num_inputs: c_int,
    pub inputs: [auto_pin_input; AUTO_CFG_MAX_INS],
    pub line_out_pins: [hda_nid_t; 5],
    pub hp_pins: [hda_nid_t; 5],
}

#[repr(C)]
pub struct hda_gen_spec {
    pub autocfg: auto_pin_cfg,
    pub pcm_playback_hook: Option<
        unsafe extern "C" fn(
            *mut hda_pcm_stream,
            *mut hda_codec,
            *mut snd_pcm_substream,
            c_int,
        ),
    >,
}

#[repr(C)]
pub struct hda_codec_core {
    pub subsystem_id: c_uint,
    pub chip_name: *const c_char,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hda_codec_core,
    pub spec: *mut cmi_spec,
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_jack_tbl {
    pub block_report: c_int,
}

#[repr(C)]
pub struct hda_jack_callback {
    pub nid: hda_nid_t,
}

#[repr(C)]
pub struct hda_pcm_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_device_id {
    pub vendor_id: c_uint,
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
    pub suspend: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub check_power_status: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t) -> c_int>,
    pub stream_pm: Option<unsafe extern "C" fn(*mut hda_codec, *mut hda_pcm_stream, bool_)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

#[repr(C)]
pub struct cmi_spec {
    pub gen: hda_gen_spec,
    pub chip_d0_verbs: *const hda_verb,
    pub chip_d3_verbs: *const hda_verb,
    pub chip_playback_start_verbs: *const hda_verb,
    pub chip_playback_stop_verbs: *const hda_verb,
    pub chip_hp_present_verbs: *const hda_verb,
    pub chip_hp_remove_verbs: *const hda_verb,
    pub chip_lineout_present_verbs: *const hda_verb,
    pub chip_lineout_remove_verbs: *const hda_verb,
    pub codec: *mut hda_codec,
    pub unsol_inputs_work: delayed_work,
    pub unsol_lineout_work: delayed_work,
    pub unsol_hp_work: delayed_work,
    pub jd_cap_hp: hda_nid_t,
    pub jd_cap_lineout: hda_nid_t,
    pub jd_cap_inputs: [hda_nid_t; AUTO_CFG_MAX_INS],
    pub quirk: c_int,
}

macro_rules! hda_verb {
    () => {
        hda_verb {
            nid: 0,
            verb: 0,
            param: 0,
        }
    };
    ($nid:expr, $verb:expr, $param:expr) => {
        hda_verb {
            nid: $nid,
            verb: $verb,
            param: $param,
        }
    };
}

static cm9825_std_d3_verbs: &[hda_verb] = &[
    /* chip sleep verbs */
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0x62), /* depop */
    hda_verb!(0x43, CM9825_VERB_SET_PLL, 0x01), /* PLL set */
    hda_verb!(0x43, CM9825_VERB_SET_NEG, 0xc2), /* NEG set */
    hda_verb!(0x43, CM9825_VERB_SET_ADCL, 0x00), /* ADC */
    hda_verb!(0x43, CM9825_VERB_SET_DACL, 0x02), /* DACL */
    hda_verb!(0x43, CM9825_VERB_SET_VNEG, 0x50), /* VOL NEG */
    hda_verb!(0x43, CM9825_VERB_SET_MBIAS, 0x00), /* MBIAS */
    hda_verb!(0x43, CM9825_VERB_SET_PDNEG, 0x04), /* SEL OSC */
    hda_verb!(0x43, CM9825_VERB_SET_CDALR, 0xf6), /* Class D */
    hda_verb!(0x43, CM9825_VERB_SET_OTP, 0xcd), /* OTP set */
    hda_verb!(),
];

static cm9825_std_d0_verbs: &[hda_verb] = &[
    /* chip init verbs */
    hda_verb!(0x34, unsafe { AC_VERB_SET_EAPD_BTLENABLE }, 0x02), /* EAPD set */
    hda_verb!(0x43, CM9825_VERB_SET_SNR, 0x30), /* SNR set */
    hda_verb!(0x43, CM9825_VERB_SET_PLL, 0x00), /* PLL set */
    hda_verb!(0x43, CM9825_VERB_SET_ADCL, 0x00), /* ADC */
    hda_verb!(0x43, CM9825_VERB_SET_DACL, 0x02), /* DACL */
    hda_verb!(0x43, CM9825_VERB_SET_MBIAS, 0x00), /* MBIAS */
    hda_verb!(0x43, CM9825_VERB_SET_VNEG, 0x56), /* VOL NEG */
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0x62), /* depop */
    hda_verb!(0x43, CM9825_VERB_SET_DACTRL, 0x00), /* DACTRL set */
    hda_verb!(0x43, CM9825_VERB_SET_PDNEG, 0x0c), /* SEL OSC */
    hda_verb!(0x43, CM9825_VERB_SET_VDO, 0x80), /* VDO set */
    hda_verb!(0x43, CM9825_VERB_SET_CDALR, 0xf4), /* Class D */
    hda_verb!(0x43, CM9825_VERB_SET_OTP, 0xcd), /* OTP set */
    hda_verb!(0x43, CM9825_VERB_SET_MTCBA, 0x61), /* SR set */
    hda_verb!(0x43, CM9825_VERB_SET_OCP, 0x33), /* OTP set */
    hda_verb!(0x43, CM9825_VERB_SET_GAD, 0x07), /* ADC -3db */
    hda_verb!(0x43, CM9825_VERB_SET_TMOD, 0x26), /* Class D clk */
    hda_verb!(0x3c, unsafe { AC_VERB_SET_AMP_GAIN_MUTE | AC_AMP_SET_OUTPUT | AC_AMP_SET_RIGHT }, 0x2d), /* Gain set */
    hda_verb!(0x3c, unsafe { AC_VERB_SET_AMP_GAIN_MUTE | AC_AMP_SET_OUTPUT | AC_AMP_SET_LEFT }, 0x2d), /* Gain set */
    hda_verb!(0x43, CM9825_VERB_SET_HPF_1, 0x40), /* HPF set */
    hda_verb!(0x43, CM9825_VERB_SET_HPF_2, 0x40), /* HPF set */
    hda_verb!(),
];

static cm9825_hp_present_verbs: &[hda_verb] = &[
    hda_verb!(0x42, unsafe { AC_VERB_SET_PIN_WIDGET_CONTROL }, 0x00), /* PIN off */
    hda_verb!(0x43, CM9825_VERB_SET_ADCL, 0x88), /* ADC */
    hda_verb!(0x43, CM9825_VERB_SET_DACL, 0xaa), /* DACL */
    hda_verb!(0x43, CM9825_VERB_SET_MBIAS, 0x10), /* MBIAS */
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0xf2), /* depop */
    hda_verb!(0x43, CM9825_VERB_SET_DACTRL, 0x00), /* DACTRL set */
    hda_verb!(0x43, CM9825_VERB_SET_VDO, 0xc4), /* VDO set */
    hda_verb!(),
];

static cm9825_hp_remove_verbs: &[hda_verb] = &[
    hda_verb!(0x43, CM9825_VERB_SET_ADCL, 0x00), /* ADC */
    hda_verb!(0x43, CM9825_VERB_SET_DACL, 0x56), /* DACL */
    hda_verb!(0x43, CM9825_VERB_SET_MBIAS, 0x00), /* MBIAS */
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0x62), /* depop */
    hda_verb!(0x43, CM9825_VERB_SET_DACTRL, 0xe0), /* DACTRL set */
    hda_verb!(0x43, CM9825_VERB_SET_VDO, 0x80), /* VDO set */
    hda_verb!(0x42, unsafe { AC_VERB_SET_PIN_WIDGET_CONTROL }, 0x40), /* PIN on */
    hda_verb!(),
];

/*
 * To save power, AD/CLK is turned off.
 */
static cm9825_gene_twl7_d3_verbs: &[hda_verb] = &[
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0x62),
    hda_verb!(0x43, CM9825_VERB_SET_PLL, 0x01),
    hda_verb!(0x43, CM9825_VERB_SET_NEG, 0xc2),
    hda_verb!(0x43, CM9825_VERB_SET_ADCL, 0x00),
    hda_verb!(0x43, CM9825_VERB_SET_DACL, 0x02),
    hda_verb!(0x43, CM9825_VERB_SET_MBIAS, 0x00),
    hda_verb!(0x43, CM9825_VERB_SET_VNEG, 0x50),
    hda_verb!(0x43, CM9825_VERB_SET_PDNEG, 0x04),
    hda_verb!(0x43, CM9825_VERB_SET_CDALR, 0xf6),
    hda_verb!(0x43, CM9825_VERB_SET_OTP, 0xcd),
    hda_verb!(),
];

/*
 * These settings are required to properly enable the PLL, clock, ADC and
 * DAC paths, and to select the correct analog input routing. Without
 * these explicit configurations, the ADC does not start correctly and
 * recording does not work reliably on this hardware.
 *
 * D0 configuration: enable PLL/CLK/ADC/DAC and optimize performance
 */
static cm9825_gene_twl7_d0_verbs: &[hda_verb] = &[
    hda_verb!(0x34, unsafe { AC_VERB_SET_EAPD_BTLENABLE }, 0x02),
    hda_verb!(0x43, CM9825_VERB_SET_SNR, 0x38),
    hda_verb!(0x43, CM9825_VERB_SET_PLL, 0x00),
    hda_verb!(0x43, CM9825_VERB_SET_ADCL, 0xcf),
    hda_verb!(0x43, CM9825_VERB_SET_DACL, 0xaa),
    hda_verb!(0x43, CM9825_VERB_SET_MBIAS, 0x1c),
    hda_verb!(0x43, CM9825_VERB_SET_VNEG, 0x56),
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0x62),
    hda_verb!(0x43, CM9825_VERB_SET_DACTRL, 0x00),
    hda_verb!(0x43, CM9825_VERB_SET_PDNEG, 0x0c),
    hda_verb!(0x43, CM9825_VERB_SET_CDALR, 0xf4),
    hda_verb!(0x43, CM9825_VERB_SET_OTP, 0xcd),
    hda_verb!(0x43, CM9825_VERB_SET_MTCBA, 0x61),
    hda_verb!(0x43, CM9825_VERB_SET_OCP, 0x33),
    hda_verb!(0x43, CM9825_VERB_SET_GAD, 0x07),
    hda_verb!(0x43, CM9825_VERB_SET_TMOD, 0x26),
    hda_verb!(0x43, CM9825_VERB_SET_HPF_1, 0x40),
    hda_verb!(0x43, CM9825_VERB_SET_HPF_2, 0x40),
    hda_verb!(0x40, unsafe { AC_VERB_SET_CONNECT_SEL }, 0x00),
    hda_verb!(0x3d, unsafe { AC_VERB_SET_CONNECT_SEL }, 0x01),
    hda_verb!(0x46, CM9825_VERB_SET_P3BCP, 0x20),
    hda_verb!(),
];

/*
 * Enable DAC to start playback.
 */
static cm9825_gene_twl7_playback_start_verbs: &[hda_verb] = &[
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0xf2),
    hda_verb!(0x43, CM9825_VERB_SET_VDO, 0xd4),
    hda_verb!(0x43, CM9825_VERB_SET_SNR, 0x30),
    hda_verb!(),
];

/*
 * Disable DAC and enable de-pop noise mechanism.
 */
static cm9825_gene_twl7_playback_stop_verbs: &[hda_verb] = &[
    hda_verb!(0x43, CM9825_VERB_SET_VDO, 0xc0),
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0x62),
    hda_verb!(0x43, CM9825_VERB_SET_VDO, 0xd0),
    hda_verb!(0x43, CM9825_VERB_SET_SNR, 0x38),
    hda_verb!(),
];

/*
 * To save power, AD/CLK is turned off.
 */
static cm9825_ibp_d3_verbs: &[hda_verb] = &[
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0x62),
    hda_verb!(0x43, CM9825_VERB_SET_PLL, 0x01),
    hda_verb!(0x43, CM9825_VERB_SET_NEG, 0xc2),
    hda_verb!(0x43, CM9825_VERB_SET_ADCL, 0x00),
    hda_verb!(0x43, CM9825_VERB_SET_DACL, 0x02),
    hda_verb!(0x43, CM9825_VERB_SET_VNEG, 0x50),
    hda_verb!(0x43, CM9825_VERB_SET_MBIAS, 0x00),
    hda_verb!(0x43, CM9825_VERB_SET_PDNEG, 0x04),
    hda_verb!(0x43, CM9825_VERB_SET_CDALR, 0xf6),
    hda_verb!(0x43, CM9825_VERB_SET_OTP, 0xcd),
    hda_verb!(),
];

/*
 * D0 configuration: enable PLL/CLK/ADC/DAC and optimize performance
 */
static cm9825_ibp_d0_verbs: &[hda_verb] = &[
    hda_verb!(0x34, unsafe { AC_VERB_SET_EAPD_BTLENABLE }, 0x02),
    hda_verb!(0x43, CM9825_VERB_SET_SNR, 0x38),
    hda_verb!(0x43, CM9825_VERB_SET_PLL, 0x00),
    hda_verb!(0x43, CM9825_VERB_SET_ADCL, 0x00),
    hda_verb!(0x43, CM9825_VERB_SET_DACL, 0x02),
    hda_verb!(0x43, CM9825_VERB_SET_MBIAS, 0x00),
    hda_verb!(0x43, CM9825_VERB_SET_VNEG, 0x56),
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0x62),
    hda_verb!(0x43, CM9825_VERB_SET_DACTRL, 0x00),
    hda_verb!(0x43, CM9825_VERB_SET_PDNEG, 0x0c),
    hda_verb!(0x43, CM9825_VERB_SET_CDALR, 0xf4),
    hda_verb!(0x43, CM9825_VERB_SET_OTP, 0xcd),
    hda_verb!(0x43, CM9825_VERB_SET_MTCBA, 0x61),
    hda_verb!(0x43, CM9825_VERB_SET_OCP, 0x33),
    hda_verb!(0x43, CM9825_VERB_SET_GAD, 0x07),
    hda_verb!(0x43, CM9825_VERB_SET_TMOD, 0x26),
    hda_verb!(0x3c, unsafe { AC_VERB_SET_AMP_GAIN_MUTE | 0xa0 }, 0x2d),
    hda_verb!(0x3c, unsafe { AC_VERB_SET_AMP_GAIN_MUTE | 0x90 }, 0x2d),
    hda_verb!(0x43, CM9825_VERB_SET_HPF_1, 0x40),
    hda_verb!(0x43, CM9825_VERB_SET_HPF_2, 0x40),
    hda_verb!(),
];

/*
 * Enable mbias, ADC.
 */
static cm9825_ibp_lineout_present_verbs: &[hda_verb] = &[
    hda_verb!(0x43, CM9825_VERB_SET_ADCL, 0x8c),
    hda_verb!(0x43, CM9825_VERB_SET_MBIAS, 0x10),
    hda_verb!(),
];

/*
 * Disable mbias, ADC.
 */
static cm9825_ibp_lineout_remove_verbs: &[hda_verb] = &[
    hda_verb!(0x43, CM9825_VERB_SET_ADCL, 0x00),
    hda_verb!(0x43, CM9825_VERB_SET_MBIAS, 0x00),
    hda_verb!(),
];

/*
 * Turn on the DAC (widget NID 0x43) in the playback path by writing
 * a sequence of vendor-specific verbs.
 */
static cm9825_ibp_playback_start_verbs: &[hda_verb] = &[
    hda_verb!(0x43, CM9825_VERB_SET_DACL, 0xaa),
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0xf2),
    hda_verb!(0x43, CM9825_VERB_SET_VDO, 0xc4),
    hda_verb!(0x43, CM9825_VERB_SET_SNR, 0x30),
    hda_verb!(),
];

/*
 * Shut down the playback path. The order differs slightly from the
 * enable sequence, likely to avoid audible pop noise when powering
 * down the output stage.
 */
static cm9825_ibp_playback_stop_verbs: &[hda_verb] = &[
    hda_verb!(0x43, CM9825_VERB_SET_VDO, 0xc0),
    hda_verb!(0x43, CM9825_VERB_SET_DACL, 0x02),
    hda_verb!(0x43, CM9825_VERB_SET_D2S, 0x62),
    hda_verb!(0x43, CM9825_VERB_SET_VDO, 0x80),
    hda_verb!(0x43, CM9825_VERB_SET_SNR, 0x38),
    hda_verb!(),
];

extern "C" {
    fn snd_hda_jack_detect(codec: *mut hda_codec, nid: hda_nid_t) -> bool_;
    fn snd_hda_jack_tbl_get(codec: *mut hda_codec, nid: hda_nid_t) -> *mut hda_jack_tbl;
    fn snd_hda_jack_report_sync(codec: *mut hda_codec);
    fn snd_hda_sequence_write(codec: *mut hda_codec, seq: *const hda_verb);
    fn snd_hda_codec_write(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    ) -> c_int;
    fn snd_hda_codec_read(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    ) -> c_uint;
    fn snd_hda_gen_init(codec: *mut hda_codec);
    fn snd_hda_apply_fixup(codec: *mut hda_codec, action: c_int);
    fn snd_hda_gen_remove(codec: *mut hda_codec);
    fn snd_hda_codec_init(codec: *mut hda_codec);
    fn snd_hda_regmap_sync(codec: *mut hda_codec);
    fn hda_call_check_power_status(codec: *mut hda_codec, nid: hda_nid_t);
    fn snd_hda_gen_spec_init(spec: *mut hda_gen_spec);
    fn snd_hda_codec_set_name(codec: *mut hda_codec, name: *const c_char);
    fn snd_hda_codec_set_pincfg(codec: *mut hda_codec, nid: hda_nid_t, cfg: c_uint);
    fn snd_hda_parse_pin_defcfg(
        codec: *mut hda_codec,
        cfg: *mut auto_pin_cfg,
        ignore_nids: *const hda_nid_t,
        cond_flags: c_uint,
    ) -> c_int;
    fn snd_hda_gen_parse_auto_config(codec: *mut hda_codec, cfg: *mut auto_pin_cfg) -> c_int;
    fn snd_hda_jack_detect_enable_callback(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        cb: unsafe extern "C" fn(*mut hda_codec, *mut hda_jack_callback),
    );
    fn snd_hda_gen_build_controls(codec: *mut hda_codec) -> c_int;
    fn snd_hda_gen_build_pcms(codec: *mut hda_codec) -> c_int;
    fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: c_uint);
    fn snd_hda_gen_check_power_status(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    fn snd_hda_gen_stream_pm(codec: *mut hda_codec, hinfo: *mut hda_pcm_stream, on: bool_);
    fn is_jack_detectable(codec: *mut hda_codec, nid: hda_nid_t) -> bool_;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_uint) -> bool_;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool_;
    fn msecs_to_jiffies(msecs: c_uint) -> c_uint;
    fn msleep(msecs: c_uint);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn to_delayed_work(work: *mut work_struct) -> *mut delayed_work;
    fn codec_dbg(codec: *mut hda_codec, fmt: *const c_char, ...);
    fn codec_info(codec: *mut hda_codec, fmt: *const c_char, ...);
}

const GFP_KERNEL: c_uint = 0;

unsafe fn container_of_delayed_work(
    delayed: *mut delayed_work,
    field: unsafe fn(*mut cmi_spec) -> *mut delayed_work,
) -> *mut cmi_spec {
    let base = ptr::null_mut::<cmi_spec>();
    let offset = (field(base) as usize).wrapping_sub(base as usize);
    (delayed as *mut u8).wrapping_sub(offset) as *mut cmi_spec
}

unsafe fn unsol_inputs_work_ptr(spec: *mut cmi_spec) -> *mut delayed_work {
    &mut (*spec).unsol_inputs_work
}

unsafe fn unsol_lineout_work_ptr(spec: *mut cmi_spec) -> *mut delayed_work {
    &mut (*spec).unsol_lineout_work
}

unsafe fn unsol_hp_work_ptr(spec: *mut cmi_spec) -> *mut delayed_work {
    &mut (*spec).unsol_hp_work
}

unsafe extern "C" fn cm9825_update_jk_plug_status(codec: *mut hda_codec, nid: hda_nid_t) {
    let spec = (*codec).spec;
    let jack_plugin: bool_;
    let jack: *mut hda_jack_tbl;

    jack_plugin = snd_hda_jack_detect((*spec).codec, nid);
    jack = snd_hda_jack_tbl_get((*spec).codec, nid);
    if !jack.is_null() {
        (*jack).block_report = 0;
        snd_hda_jack_report_sync((*spec).codec);
    }

    codec_dbg(
        (*spec).codec,
        b"%s, jack_plugin %d, nid 0x%X, line%d\n\0".as_ptr() as *const c_char,
        b"cm9825_update_jk_plug_status\0".as_ptr() as *const c_char,
        jack_plugin as c_int,
        nid,
        line!() as c_int,
    );
}

unsafe extern "C" fn cm9825_unsol_inputs_delayed(work: *mut work_struct) {
    let spec = container_of_delayed_work(to_delayed_work(work), unsol_inputs_work_ptr);
    let mut i: c_int;

    i = 0;
    while i < (*spec).gen.autocfg.num_inputs {
        if (*spec).jd_cap_inputs[i as usize] == 0 {
            i += 1;
            continue;
        }

        cm9825_update_jk_plug_status((*spec).codec, (*spec).gen.autocfg.inputs[i as usize].pin);
        i += 1;
    }
}

unsafe extern "C" fn cm9825_unsol_lineout_delayed(work: *mut work_struct) {
    let spec = container_of_delayed_work(to_delayed_work(work), unsol_lineout_work_ptr);
    let line_out_pin: hda_nid_t = (*spec).gen.autocfg.line_out_pins[0];
    let mut line_out_jack_plugin: bool_ = false;

    line_out_jack_plugin = snd_hda_jack_detect((*spec).codec, line_out_pin);

    codec_dbg(
        (*spec).codec,
        b"lineout_jack_plugin %d, lineout_pin 0x%X\n\0".as_ptr() as *const c_char,
        line_out_jack_plugin as c_int,
        line_out_pin,
    );

    if !line_out_jack_plugin {
        /* Jack plugout */
        snd_hda_sequence_write((*spec).codec, (*spec).chip_lineout_remove_verbs);
    } else {
        /* Jack plugin */
        snd_hda_sequence_write((*spec).codec, (*spec).chip_lineout_present_verbs);
    }

    cm9825_update_jk_plug_status((*spec).codec, (*spec).gen.autocfg.line_out_pins[0]);
}

unsafe extern "C" fn cm9825_unsol_hp_delayed(work: *mut work_struct) {
    let spec = container_of_delayed_work(to_delayed_work(work), unsol_hp_work_ptr);
    let mut jack: *mut hda_jack_tbl;
    let hp_pin: hda_nid_t = (*spec).gen.autocfg.hp_pins[0];
    let mut hp_jack_plugin: bool_ = false;
    let mut err: c_int = 0;

    hp_jack_plugin = snd_hda_jack_detect((*spec).codec, hp_pin);

    codec_dbg(
        (*spec).codec,
        b"hp_jack_plugin %d, hp_pin 0x%X\n\0".as_ptr() as *const c_char,
        hp_jack_plugin as c_int,
        hp_pin,
    );

    if !hp_jack_plugin {
        err = snd_hda_codec_write(
            (*spec).codec,
            0x42,
            0,
            AC_VERB_SET_PIN_WIDGET_CONTROL,
            0x40,
        );
        if err != 0 {
            codec_dbg(
                (*spec).codec,
                b"codec_write err %d\n\0".as_ptr() as *const c_char,
                err,
            );
        }

        snd_hda_sequence_write((*spec).codec, (*spec).chip_hp_remove_verbs);
    } else {
        snd_hda_sequence_write((*spec).codec, (*spec).chip_hp_present_verbs);
    }

    jack = snd_hda_jack_tbl_get((*spec).codec, hp_pin);
    if !jack.is_null() {
        (*jack).block_report = 0;
        snd_hda_jack_report_sync((*spec).codec);
    }
}

unsafe extern "C" fn hp_callback(codec: *mut hda_codec, cb: *mut hda_jack_callback) {
    let spec = (*codec).spec;
    let tbl: *mut hda_jack_tbl;

    /* Delay enabling the HP amp, to let the mic-detection
     * state machine run.
     */

    codec_dbg(
        (*spec).codec,
        b"cb->nid 0x%X\n\0".as_ptr() as *const c_char,
        (*cb).nid,
    );

    tbl = snd_hda_jack_tbl_get(codec, (*cb).nid);
    if !tbl.is_null() {
        (*tbl).block_report = 1;
    }

    if (*cb).nid == (*spec).jd_cap_hp {
        schedule_delayed_work(&mut (*spec).unsol_hp_work, msecs_to_jiffies(200));
    } else if (*cb).nid == (*spec).jd_cap_lineout {
        schedule_delayed_work(&mut (*spec).unsol_lineout_work, msecs_to_jiffies(200));
    }

    let mut i: c_int = 0;
    while i < (*spec).gen.autocfg.num_inputs {
        if (*cb).nid == (*spec).jd_cap_inputs[i as usize] {
            schedule_delayed_work(&mut (*spec).unsol_inputs_work, msecs_to_jiffies(200));
        }
        i += 1;
    }
}

unsafe extern "C" fn cm9825_setup_unsol(codec: *mut hda_codec) {
    let spec = (*codec).spec;
    let mut i: c_int;

    let hp_pin: hda_nid_t = (*spec).gen.autocfg.hp_pins[0];

    let lineout_pin: hda_nid_t = (*spec).gen.autocfg.line_out_pins[0];

    if hp_pin != 0 {
        if is_jack_detectable(codec, hp_pin) {
            (*spec).jd_cap_hp = hp_pin;
            snd_hda_jack_detect_enable_callback(codec, hp_pin, hp_callback);
        } else {
            (*spec).jd_cap_hp = 0;
        }
    } else {
        (*spec).jd_cap_hp = 0;
    }

    if lineout_pin != 0 {
        if is_jack_detectable(codec, lineout_pin) {
            (*spec).jd_cap_lineout = lineout_pin;
            snd_hda_jack_detect_enable_callback(codec, lineout_pin, hp_callback);
        } else {
            (*spec).jd_cap_lineout = 0;
        }
    } else {
        (*spec).jd_cap_lineout = 0;
    }

    codec_dbg(
        codec,
        b"%s, jd_cap_hp 0x%02X, jd_cap_lineout 0x%02X, line%d\n\0".as_ptr() as *const c_char,
        b"cm9825_setup_unsol\0".as_ptr() as *const c_char,
        (*spec).jd_cap_hp,
        (*spec).jd_cap_lineout,
        line!() as c_int,
    );

    i = 0;
    while i < (*spec).gen.autocfg.num_inputs {
        if (*spec).gen.autocfg.inputs[i as usize].pin != 0 {
            if is_jack_detectable(codec, (*spec).gen.autocfg.inputs[i as usize].pin) {
                (*spec).jd_cap_inputs[i as usize] = (*spec).gen.autocfg.inputs[i as usize].pin;
                snd_hda_jack_detect_enable_callback(
                    codec,
                    (*spec).gen.autocfg.inputs[i as usize].pin,
                    hp_callback,
                );
            } else {
                (*spec).jd_cap_inputs[i as usize] = 0;
            }
        } else {
            (*spec).jd_cap_inputs[i as usize] = 0;
        }

        codec_dbg(
            codec,
            b"%s, input jd_cap_inputs[%d] 0x%02X, line%d\n\0".as_ptr() as *const c_char,
            b"cm9825_setup_unsol\0".as_ptr() as *const c_char,
            i,
            (*spec).jd_cap_inputs[i as usize],
            line!() as c_int,
        );
        i += 1;
    }
}

unsafe extern "C" fn cm9825_playback_pcm_hook(
    _hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    _substream: *mut snd_pcm_substream,
    action: c_int,
) {
    let spec = (*codec).spec;

    if action == HDA_GEN_PCM_ACT_PREPARE {
        snd_hda_sequence_write(codec, (*spec).chip_playback_start_verbs);
    } else if action == HDA_GEN_PCM_ACT_CLEANUP {
        snd_hda_sequence_write(codec, (*spec).chip_playback_stop_verbs);
    } else {
        return;
    }
}

unsafe extern "C" fn cm9825_init(codec: *mut hda_codec) -> c_int {
    snd_hda_gen_init(codec);
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_INIT);

    0
}

unsafe extern "C" fn cm9825_remove(codec: *mut hda_codec) {
    let spec = (*codec).spec;
    let mut i: c_int;

    if (*spec).jd_cap_hp != 0 {
        cancel_delayed_work_sync(&mut (*spec).unsol_hp_work);
    }

    if (*spec).jd_cap_lineout != 0 {
        cancel_delayed_work_sync(&mut (*spec).unsol_lineout_work);
    }

    i = 0;
    while i < (*spec).gen.autocfg.num_inputs {
        if (*spec).jd_cap_inputs[i as usize] != 0 {
            cancel_delayed_work_sync(&mut (*spec).unsol_inputs_work);
            break;
        }
        i += 1;
    }

    snd_hda_gen_remove(codec);
}

unsafe extern "C" fn cm9825_suspend(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec;
    let mut i: c_int;

    if (*spec).jd_cap_hp != 0 {
        cancel_delayed_work_sync(&mut (*spec).unsol_hp_work);
    }

    if (*spec).jd_cap_lineout != 0 {
        cancel_delayed_work_sync(&mut (*spec).unsol_lineout_work);
    }

    i = 0;
    while i < (*spec).gen.autocfg.num_inputs {
        if (*spec).jd_cap_inputs[i as usize] != 0 {
            cancel_delayed_work_sync(&mut (*spec).unsol_inputs_work);
            break;
        }
        i += 1;
    }

    snd_hda_sequence_write(codec, (*spec).chip_d3_verbs);

    0
}

unsafe extern "C" fn cm9825_cm_std_resume(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec;
    let mut hp_pin: hda_nid_t = 0;
    let mut hp_jack_plugin: bool_ = false;
    let mut err: c_int;

    err = snd_hda_codec_write(
        (*spec).codec,
        0x42,
        0,
        AC_VERB_SET_PIN_WIDGET_CONTROL,
        0x00,
    );
    if err != 0 {
        codec_dbg(
            codec,
            b"codec_write err %d\n\0".as_ptr() as *const c_char,
            err,
        );
    }

    msleep(150); /* for depop noise */

    snd_hda_codec_init(codec);

    snd_hda_sequence_write(codec, (*spec).chip_d0_verbs);

    hp_pin = (*spec).gen.autocfg.hp_pins[0];
    hp_jack_plugin = snd_hda_jack_detect((*spec).codec, hp_pin);

    codec_dbg(
        (*spec).codec,
        b"hp_jack_plugin %d, hp_pin 0x%X\n\0".as_ptr() as *const c_char,
        hp_jack_plugin as c_int,
        hp_pin,
    );

    if !hp_jack_plugin {
        err = snd_hda_codec_write(
            (*spec).codec,
            0x42,
            0,
            AC_VERB_SET_PIN_WIDGET_CONTROL,
            0x40,
        );

        if err != 0 {
            codec_dbg(
                codec,
                b"codec_write err %d\n\0".as_ptr() as *const c_char,
                err,
            );
        }

        snd_hda_sequence_write(codec, cm9825_hp_remove_verbs.as_ptr());
    }

    0
}

unsafe extern "C" fn cm9825_resume(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec;

    if (*codec).core.subsystem_id == QUIRK_CM_STD {
        cm9825_cm_std_resume(codec);
    } else if (*codec).core.subsystem_id == QUIRK_GENE_TWL7_SSID
        || (*codec).core.subsystem_id == QUIRK_IBP_SSID
    {
        snd_hda_codec_init(codec);
        snd_hda_sequence_write(codec, (*spec).chip_d0_verbs);
    }

    snd_hda_regmap_sync(codec);
    hda_call_check_power_status(codec, 0x01);

    0
}

unsafe extern "C" fn cm9825_probe(codec: *mut hda_codec, _id: *const hda_device_id) -> c_int {
    let mut spec: *mut cmi_spec;
    let cfg: *mut auto_pin_cfg;
    let mut err: c_int = 0;
    let mut val: c_uint;

    spec = kzalloc(core::mem::size_of::<cmi_spec>(), GFP_KERNEL) as *mut cmi_spec;
    if spec.is_null() {
        return -ENOMEM;
    }

    codec_dbg(
        codec,
        b"chip_name: %s, ssid: 0x%X\n\0".as_ptr() as *const c_char,
        (*codec).core.chip_name,
        (*codec).core.subsystem_id,
    );

    (*codec).spec = spec;
    (*spec).codec = codec;
    cfg = &mut (*spec).gen.autocfg;
    snd_hda_gen_spec_init(&mut (*spec).gen);
    (*spec).chip_d0_verbs = cm9825_std_d0_verbs.as_ptr();
    (*spec).chip_d3_verbs = cm9825_std_d3_verbs.as_ptr();
    (*spec).chip_hp_present_verbs = cm9825_hp_present_verbs.as_ptr();
    (*spec).chip_hp_remove_verbs = cm9825_hp_remove_verbs.as_ptr();

    INIT_DELAYED_WORK(&mut (*spec).unsol_hp_work, cm9825_unsol_hp_delayed);
    INIT_DELAYED_WORK(
        &mut (*spec).unsol_inputs_work,
        cm9825_unsol_inputs_delayed,
    );
    INIT_DELAYED_WORK(
        &mut (*spec).unsol_lineout_work,
        cm9825_unsol_lineout_delayed,
    );

    match (*codec).core.subsystem_id {
        QUIRK_CM_STD => {
            snd_hda_codec_set_name(codec, b"CM9825 STD\0".as_ptr() as *const c_char);
            (*spec).chip_d0_verbs = cm9825_std_d0_verbs.as_ptr();
            (*spec).chip_d3_verbs = cm9825_std_d3_verbs.as_ptr();
            (*spec).chip_hp_present_verbs = cm9825_hp_present_verbs.as_ptr();
            (*spec).chip_hp_remove_verbs = cm9825_hp_remove_verbs.as_ptr();
        }
        QUIRK_GENE_TWL7_SSID => {
            snd_hda_codec_set_name(codec, b"CM9825 GENE_TWL7\0".as_ptr() as *const c_char);
            (*spec).chip_d0_verbs = cm9825_gene_twl7_d0_verbs.as_ptr();
            (*spec).chip_d3_verbs = cm9825_gene_twl7_d3_verbs.as_ptr();
            (*spec).gen.pcm_playback_hook = Some(cm9825_playback_pcm_hook);
            (*spec).chip_playback_start_verbs = cm9825_gene_twl7_playback_start_verbs.as_ptr();
            (*spec).chip_playback_stop_verbs = cm9825_gene_twl7_playback_stop_verbs.as_ptr();
            /* Internal fixed device, Rear, Mic-in, 3.5mm */
            snd_hda_codec_set_pincfg(codec, 0x37, 0x24A70100);
        }
        QUIRK_IBP_SSID => {
            snd_hda_codec_set_name(codec, b"CM9825 IBP\0".as_ptr() as *const c_char);
            (*spec).chip_d0_verbs = cm9825_ibp_d0_verbs.as_ptr();
            (*spec).chip_d3_verbs = cm9825_ibp_d3_verbs.as_ptr();
            (*spec).gen.pcm_playback_hook = Some(cm9825_playback_pcm_hook);
            (*spec).chip_lineout_present_verbs = cm9825_ibp_lineout_present_verbs.as_ptr();
            (*spec).chip_lineout_remove_verbs = cm9825_ibp_lineout_remove_verbs.as_ptr();
            (*spec).chip_playback_start_verbs = cm9825_ibp_playback_start_verbs.as_ptr();
            (*spec).chip_playback_stop_verbs = cm9825_ibp_playback_stop_verbs.as_ptr();

            /* OMTP */
            val = snd_hda_codec_read(codec, 0x46, 0, CM9825_VERB_READ_OMTP, 0x0);
            snd_hda_codec_write(
                codec,
                0x46,
                0,
                CM9825_VERB_SET_OMTP,
                (val >> 24) & 0x7f,
            );
        }
        _ => {
            err = -ENXIO;
        }
    }

    if err < 0 {
        cm9825_remove(codec);

        codec_info(
            codec,
            b"Enter err %d\n\0".as_ptr() as *const c_char,
            err,
        );

        return err;
    }

    snd_hda_sequence_write(codec, (*spec).chip_d0_verbs);

    err = snd_hda_parse_pin_defcfg(codec, cfg, ptr::null(), 0);
    if err < 0 {
        cm9825_remove(codec);

        codec_info(
            codec,
            b"Enter err %d\n\0".as_ptr() as *const c_char,
            err,
        );

        return err;
    }
    err = snd_hda_gen_parse_auto_config(codec, cfg);
    if err < 0 {
        cm9825_remove(codec);

        codec_info(
            codec,
            b"Enter err %d\n\0".as_ptr() as *const c_char,
            err,
        );

        return err;
    }

    cm9825_setup_unsol(codec);

    0
}

static cm9825_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(cm9825_probe),
    remove: Some(cm9825_remove),
    build_controls: Some(snd_hda_gen_build_controls),
    build_pcms: Some(snd_hda_gen_build_pcms),
    init: Some(cm9825_init),
    unsol_event: Some(snd_hda_jack_unsol_event),
    suspend: Some(cm9825_suspend),
    resume: Some(cm9825_resume),
    check_power_status: Some(snd_hda_gen_check_power_status),
    stream_pm: Some(snd_hda_gen_stream_pm),
};

/*
 * driver entries
 */
static snd_hda_id_cm9825: &[hda_device_id] = &[
    hda_device_id {
        vendor_id: 0x13f69825,
        name: b"CM9825\0".as_ptr() as *const c_char,
    },
    hda_device_id {
        vendor_id: 0,
        name: ptr::null(),
    }, /* terminator */
];
/* MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_cm9825); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("CM9825 HD-audio codec"); */

static mut cm9825_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_cm9825.as_ptr(),
    ops: &cm9825_codec_ops,
};

/* module_hda_codec_driver(cm9825_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
