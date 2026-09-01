// SPDX-License-Identifier: GPL-2.0-or-later
//
// Realtek ALC882/883/885/888/889 codec support
//
// ALC882 is almost identical with ALC880 but has cleaner and more flexible
// configuration.  Each pin widget can choose any input DACs and a mixer.
// Each ADC is connected from a mixer of all inputs.  This makes possible
// 6-channel independent captures.
//
// In addition, an independent DAC for the multi-playback (not used in this
// driver yet).
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint};

type hda_nid_t = u16;

#[repr(C)]
pub struct hda_codec {
    pub core: hda_codec_core,
    pub spec: *mut alc_spec,
}

#[repr(C)]
pub struct hda_codec_core {
    pub vendor_id: c_uint,
}

#[repr(C)]
pub struct alc_spec {
    pub gen: hda_gen_spec,
    pub gpio_write_delay: bool,
}

#[repr(C)]
pub struct hda_gen_spec {
    pub keep_vref_in_automute: c_int,
    pub no_primary_hp: c_int,
    pub no_multi_io: c_int,
    pub hp_jack_present: c_int,
    pub hp_automute_hook: Option<unsafe extern "C" fn(*mut hda_codec, *mut hda_jack_callback)>,
    pub beep_nid: hda_nid_t,
    pub no_analog: c_int,
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
    pub verbs: *const hda_verb,
    pub func: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_fixup, c_int)>,
}

#[repr(C)]
pub struct hda_pintbl {
    pub nid: hda_nid_t,
    pub val: c_uint,
}

#[repr(C)]
pub struct hda_verb {
    pub nid: hda_nid_t,
    pub verb: c_uint,
    pub param: c_uint,
}

#[repr(C)]
pub struct coef_fw {
    pub nid: c_uint,
    pub mask: c_uint,
    pub val: c_uint,
}

#[repr(C)]
pub struct hda_quirk {
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub name: *const c_char,
    pub value: c_int,
}

#[repr(C)]
pub struct hda_model_fixup {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_hda_pin_quirk {
    pub codec: c_uint,
    pub subvendor: c_uint,
    pub name: *const c_char,
    pub value: c_int,
    pub pins: *const hda_pintbl,
}

#[repr(C)]
pub struct hda_device_id {
    pub vendor_id: c_uint,
    pub rev_id: c_uint,
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
    pub resume: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub check_power_status: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t) -> c_int>,
    pub stream_pm: Option<unsafe extern "C" fn(*mut hda_codec, bool)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

#[repr(C)]
pub struct hda_jack_callback {
    _private: [u8; 0],
}

const HDA_FIXUP_PINS: c_int = 0;
const HDA_FIXUP_VERBS: c_int = 1;
const HDA_FIXUP_PINCTLS: c_int = 2;
const HDA_FIXUP_FUNC: c_int = 3;

const HDA_FIXUP_ACT_PRE_PROBE: c_int = 0;
const HDA_FIXUP_ACT_PROBE: c_int = 1;
const HDA_FIXUP_ACT_INIT: c_int = 2;

extern "C" {
    fn alc_update_coef_idx(codec: *mut hda_codec, idx: c_uint, mask: c_uint, val: c_uint);
    fn alc_fixup_gpio1(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
    fn alc_fixup_gpio2(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
    fn alc_fixup_gpio3(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
    fn snd_hda_override_conn_list(codec: *mut hda_codec, nid: hda_nid_t, nums: c_int, list: *const hda_nid_t);
    fn snd_hda_codec_get_pincfg(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn get_defcfg_device(cfg: c_uint) -> c_uint;
    fn snd_hda_codec_get_pin_target(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn snd_hda_set_pin_ctl(codec: *mut hda_codec, nid: hda_nid_t, val: c_uint);
    fn alc_process_coef_fw(codec: *mut hda_codec, fw: *const coef_fw);
    fn alc_fixup_headset_mode_no_hp_mic(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
    fn snd_hda_set_pin_ctl_cache(codec: *mut hda_codec, nid: hda_nid_t, val: c_uint);
    fn snd_hda_gen_hp_automute(codec: *mut hda_codec, jack: *mut hda_jack_callback);
    fn alc_fixup_sku_ignore(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
    fn alc_fixup_inv_dmic(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
    fn alc_fixup_bass_chmap(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
    fn alc1220_fixup_gb_dual_codecs(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
    fn alc_parse_auto_config(codec: *mut hda_codec, ignore: *const hda_nid_t, ssids: *const hda_nid_t) -> c_int;
    fn alc_alloc_spec(codec: *mut hda_codec, mixer_nid: hda_nid_t) -> c_int;
    fn alc_fix_pll_init(codec: *mut hda_codec, nid: hda_nid_t, coef_idx: c_uint, coef_bit: c_uint);
    fn alc_pre_init(codec: *mut hda_codec);
    fn snd_hda_pick_fixup(codec: *mut hda_codec, models: *const hda_model_fixup, quirks: *const hda_quirk, fixups: *const hda_fixup);
    fn snd_hda_pick_pin_fixup(codec: *mut hda_codec, pin_quirks: *const snd_hda_pin_quirk, fixups: *const hda_fixup, match_all_pins: bool);
    fn snd_hda_apply_fixup(codec: *mut hda_codec, action: c_int);
    fn alc_auto_parse_customize_define(codec: *mut hda_codec);
    fn has_cdefine_beep(codec: *mut hda_codec) -> bool;
    fn set_beep_amp(spec: *mut alc_spec, nid: hda_nid_t, idx: c_uint, dir: c_uint) -> c_int;
    fn snd_hda_gen_remove(codec: *mut hda_codec);
    fn alc_build_controls(codec: *mut hda_codec) -> c_int;
    fn snd_hda_gen_build_pcms(codec: *mut hda_codec) -> c_int;
    fn alc_init(codec: *mut hda_codec) -> c_int;
    fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: c_uint);
    fn alc_resume(codec: *mut hda_codec) -> c_int;
    fn alc_suspend(codec: *mut hda_codec);
    fn snd_hda_gen_check_power_status(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    fn snd_hda_gen_stream_pm(codec: *mut hda_codec, enable: bool);
}

const AC_JACK_HP_OUT: c_uint = 0;
const AC_PINCTL_VREF_80: c_uint = 0;
const AC_PINCTL_VREF_50: c_uint = 0;
const AC_PINCTL_VREF_HIZ: c_uint = 0;
const AC_VERB_SET_COEF_INDEX: c_uint = 0;
const AC_VERB_SET_PROC_COEF: c_uint = 0;
const PIN_VREF80: c_uint = 0;
const PIN_VREF50: c_uint = 0;
const PIN_VREF100: c_uint = 0;
const PIN_HP: c_uint = 0;
const HDA_INPUT: c_uint = 0;

/*
 * Pin config fixes
 */
const ALC882_FIXUP_ABIT_AW9D_MAX: c_int = 0;
const ALC882_FIXUP_LENOVO_Y530: c_int = 1;
const ALC882_FIXUP_PB_M5210: c_int = 2;
const ALC882_FIXUP_ACER_ASPIRE_7736: c_int = 3;
const ALC882_FIXUP_ASUS_W90V: c_int = 4;
const ALC889_FIXUP_CD: c_int = 5;
const ALC889_FIXUP_FRONT_HP_NO_PRESENCE: c_int = 6;
const ALC889_FIXUP_VAIO_TT: c_int = 7;
const ALC888_FIXUP_EEE1601: c_int = 8;
const ALC886_FIXUP_EAPD: c_int = 9;
const ALC882_FIXUP_EAPD: c_int = 10;
const ALC883_FIXUP_EAPD: c_int = 11;
const ALC883_FIXUP_ACER_EAPD: c_int = 12;
const ALC882_FIXUP_GPIO1: c_int = 13;
const ALC882_FIXUP_GPIO2: c_int = 14;
const ALC882_FIXUP_GPIO3: c_int = 15;
const ALC889_FIXUP_COEF: c_int = 16;
const ALC882_FIXUP_ASUS_W2JC: c_int = 17;
const ALC882_FIXUP_ACER_ASPIRE_4930G: c_int = 18;
const ALC882_FIXUP_ACER_ASPIRE_8930G: c_int = 19;
const ALC882_FIXUP_ASPIRE_8930G_VERBS: c_int = 20;
const ALC885_FIXUP_MACPRO_GPIO: c_int = 21;
const ALC889_FIXUP_DAC_ROUTE: c_int = 22;
const ALC889_FIXUP_MBP_VREF: c_int = 23;
const ALC889_FIXUP_IMAC91_VREF: c_int = 24;
const ALC889_FIXUP_MBA11_VREF: c_int = 25;
const ALC889_FIXUP_MBA21_VREF: c_int = 26;
const ALC889_FIXUP_MP11_VREF: c_int = 27;
const ALC889_FIXUP_MP41_VREF: c_int = 28;
const ALC882_FIXUP_INV_DMIC: c_int = 29;
const ALC882_FIXUP_NO_PRIMARY_HP: c_int = 30;
const ALC887_FIXUP_ASUS_BASS: c_int = 31;
const ALC887_FIXUP_BASS_CHMAP: c_int = 32;
const ALC1220_FIXUP_GB_DUAL_CODECS: c_int = 33;
const ALC1220_FIXUP_GB_X570: c_int = 34;
const ALC1220_FIXUP_CLEVO_P950: c_int = 35;
const ALC1220_FIXUP_CLEVO_PB51ED: c_int = 36;
const ALC1220_FIXUP_CLEVO_PB51ED_PINS: c_int = 37;
const ALC887_FIXUP_ASUS_AUDIO: c_int = 38;
const ALC887_FIXUP_ASUS_HMIC: c_int = 39;
const ALCS1200A_FIXUP_MIC_VREF: c_int = 40;
const ALC888VD_FIXUP_MIC_100VREF: c_int = 41;
const ALC898_FIXUP_CLEVO_P775TM1: c_int = 42;

unsafe extern "C" fn alc889_fixup_coef(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    if action != HDA_FIXUP_ACT_INIT {
        return;
    }
    alc_update_coef_idx(codec, 7, 0, 0x2030);
}

/* set up GPIO at initialization */
unsafe extern "C" fn alc885_fixup_macpro_gpio(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int) {
    let spec = (*codec).spec;

    (*spec).gpio_write_delay = true;
    alc_fixup_gpio3(codec, fix, action);
}

/* Fix the connection of some pins for ALC889:
 * At least, Acer Aspire 5935 shows the connections to DAC3/4 don't
 * work correctly (bko#42740)
 */
unsafe extern "C" fn alc889_fixup_dac_route(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        /* fake the connections during parsing the tree */
        static conn1: [hda_nid_t; 2] = [0x0c, 0x0d];
        static conn2: [hda_nid_t; 2] = [0x0e, 0x0f];
        snd_hda_override_conn_list(codec, 0x14, conn1.len() as c_int, conn1.as_ptr());
        snd_hda_override_conn_list(codec, 0x15, conn1.len() as c_int, conn1.as_ptr());
        snd_hda_override_conn_list(codec, 0x18, conn2.len() as c_int, conn2.as_ptr());
        snd_hda_override_conn_list(codec, 0x1a, conn2.len() as c_int, conn2.as_ptr());
    } else if action == HDA_FIXUP_ACT_PROBE {
        /* restore the connections */
        static conn: [hda_nid_t; 5] = [0x0c, 0x0d, 0x0e, 0x0f, 0x26];
        snd_hda_override_conn_list(codec, 0x14, conn.len() as c_int, conn.as_ptr());
        snd_hda_override_conn_list(codec, 0x15, conn.len() as c_int, conn.as_ptr());
        snd_hda_override_conn_list(codec, 0x18, conn.len() as c_int, conn.as_ptr());
        snd_hda_override_conn_list(codec, 0x1a, conn.len() as c_int, conn.as_ptr());
    }
}

/* Set VREF on HP pin */
unsafe extern "C" fn alc889_fixup_mbp_vref(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    static nids: [hda_nid_t; 3] = [0x14, 0x15, 0x19];
    let spec = (*codec).spec;

    if action != HDA_FIXUP_ACT_INIT {
        return;
    }
    let mut i = 0usize;
    while i < nids.len() {
        let mut val = snd_hda_codec_get_pincfg(codec, nids[i]);
        if get_defcfg_device(val) != AC_JACK_HP_OUT {
            i += 1;
            continue;
        }
        val = snd_hda_codec_get_pin_target(codec, nids[i]);
        val |= AC_PINCTL_VREF_80;
        snd_hda_set_pin_ctl(codec, nids[i], val);
        (*spec).gen.keep_vref_in_automute = 1;
        break;
    }
}

unsafe extern "C" fn alc889_fixup_mac_pins(codec: *mut hda_codec, nids: *const hda_nid_t, num_nids: c_int) {
    let spec = (*codec).spec;
    let mut i = 0;

    while i < num_nids {
        let nid = *nids.offset(i as isize);
        let mut val = snd_hda_codec_get_pin_target(codec, nid);
        val |= AC_PINCTL_VREF_50;
        snd_hda_set_pin_ctl(codec, nid, val);
        i += 1;
    }
    (*spec).gen.keep_vref_in_automute = 1;
}

/* Set VREF on speaker pins on imac91 */
unsafe extern "C" fn alc889_fixup_imac91_vref(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    static nids: [hda_nid_t; 2] = [0x18, 0x1a];

    if action == HDA_FIXUP_ACT_INIT {
        alc889_fixup_mac_pins(codec, nids.as_ptr(), nids.len() as c_int);
    }
}

/* Set VREF on speaker pins on mba11 */
unsafe extern "C" fn alc889_fixup_mba11_vref(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    static nids: [hda_nid_t; 1] = [0x18];

    if action == HDA_FIXUP_ACT_INIT {
        alc889_fixup_mac_pins(codec, nids.as_ptr(), nids.len() as c_int);
    }
}

/* Set VREF on speaker pins on mba21 */
unsafe extern "C" fn alc889_fixup_mba21_vref(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    static nids: [hda_nid_t; 2] = [0x18, 0x19];

    if action == HDA_FIXUP_ACT_INIT {
        alc889_fixup_mac_pins(codec, nids.as_ptr(), nids.len() as c_int);
    }
}

/* Don't take HP output as primary
 * Strangely, the speaker output doesn't work on Vaio Z and some Vaio
 * all-in-one desktop PCs (for example VGC-LN51JGB) through DAC 0x05
 */
unsafe extern "C" fn alc882_fixup_no_primary_hp(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    let spec = (*codec).spec;
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        (*spec).gen.no_primary_hp = 1;
        (*spec).gen.no_multi_io = 1;
    }
}

unsafe extern "C" fn alc1220_fixup_gb_x570(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    static conn1: [hda_nid_t; 1] = [0x0c];
    static gb_x570_coefs: [coef_fw; 5] = [
        coef_fw { nid: 0x07, mask: 0, val: 0x03c0 },
        coef_fw { nid: 0x1a, mask: 0, val: 0x01c1 },
        coef_fw { nid: 0x1b, mask: 0, val: 0x0202 },
        coef_fw { nid: 0x43, mask: 0, val: 0x3005 },
        coef_fw { nid: 0, mask: 0, val: 0 },
    ];

    match action {
        HDA_FIXUP_ACT_PRE_PROBE => {
            snd_hda_override_conn_list(codec, 0x14, conn1.len() as c_int, conn1.as_ptr());
            snd_hda_override_conn_list(codec, 0x1b, conn1.len() as c_int, conn1.as_ptr());
        }
        HDA_FIXUP_ACT_INIT => {
            alc_process_coef_fw(codec, gb_x570_coefs.as_ptr());
        }
        _ => {}
    }
}

unsafe extern "C" fn alc1220_fixup_clevo_p950(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    static conn1: [hda_nid_t; 1] = [0x0c];

    if action != HDA_FIXUP_ACT_PRE_PROBE {
        return;
    }

    alc_update_coef_idx(codec, 0x7, 0, 0x3c3);
    /* We therefore want to make sure 0x14 (front headphone) and
     * 0x1b (speakers) use the stereo DAC 0x02
     */
    snd_hda_override_conn_list(codec, 0x14, conn1.len() as c_int, conn1.as_ptr());
    snd_hda_override_conn_list(codec, 0x1b, conn1.len() as c_int, conn1.as_ptr());
}

unsafe extern "C" fn alc1220_fixup_clevo_pb51ed(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int) {
    alc1220_fixup_clevo_p950(codec, fix, action);
    alc_fixup_headset_mode_no_hp_mic(codec, fix, action);
}

/* On Clevo P775TM1, VREF of pin 0x1b enables the external headphone amp */
unsafe extern "C" fn alc898_fixup_clevo_p775tm1(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    let spec = (*codec).spec;

    if action != HDA_FIXUP_ACT_PRE_PROBE {
        return;
    }

    snd_hda_set_pin_ctl_cache(codec, 0x1b, PIN_VREF80);
    (*spec).gen.keep_vref_in_automute = 1;
}

unsafe extern "C" fn alc887_asus_hp_automute_hook(codec: *mut hda_codec, jack: *mut hda_jack_callback) {
    let spec = (*codec).spec;
    let vref: c_uint;

    snd_hda_gen_hp_automute(codec, jack);

    if (*spec).gen.hp_jack_present != 0 {
        vref = AC_PINCTL_VREF_80;
    } else {
        vref = AC_PINCTL_VREF_HIZ;
    }
    snd_hda_set_pin_ctl(codec, 0x19, PIN_HP | vref);
}

unsafe extern "C" fn alc887_fixup_asus_jack(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    let spec = (*codec).spec;
    if action != HDA_FIXUP_ACT_PROBE {
        return;
    }
    snd_hda_set_pin_ctl_cache(codec, 0x1b, PIN_HP);
    (*spec).gen.hp_automute_hook = Some(alc887_asus_hp_automute_hook);
}

const fn pintbl(nid: hda_nid_t, val: c_uint) -> hda_pintbl {
    hda_pintbl { nid, val }
}

const fn verb(nid: hda_nid_t, verb: c_uint, param: c_uint) -> hda_verb {
    hda_verb { nid, verb, param }
}

const fn fixup_pins(pins: *const hda_pintbl, chained: bool, chain_id: c_int) -> hda_fixup {
    hda_fixup { type_: HDA_FIXUP_PINS, v: hda_fixup_v { pins }, chained, chain_id }
}

const fn fixup_pinctls(pins: *const hda_pintbl) -> hda_fixup {
    hda_fixup { type_: HDA_FIXUP_PINCTLS, v: hda_fixup_v { pins }, chained: false, chain_id: 0 }
}

const fn fixup_verbs(verbs: *const hda_verb, chained: bool, chain_id: c_int) -> hda_fixup {
    hda_fixup { type_: HDA_FIXUP_VERBS, v: hda_fixup_v { verbs }, chained, chain_id }
}

const fn fixup_func(func: unsafe extern "C" fn(*mut hda_codec, *const hda_fixup, c_int), chained: bool, chain_id: c_int) -> hda_fixup {
    hda_fixup { type_: HDA_FIXUP_FUNC, v: hda_fixup_v { func: Some(func) }, chained, chain_id }
}

static ALC882_FIXUP_ABIT_AW9D_MAX_PINS: [hda_pintbl; 4] = [pintbl(0x15, 0x01080104), pintbl(0x16, 0x01011012), pintbl(0x17, 0x01016011), pintbl(0, 0)];
static ALC882_FIXUP_LENOVO_Y530_PINS: [hda_pintbl; 3] = [pintbl(0x15, 0x99130112), pintbl(0x16, 0x99130111), pintbl(0, 0)];
static ALC882_FIXUP_PB_M5210_PINS: [hda_pintbl; 2] = [pintbl(0x19, PIN_VREF50), pintbl(0, 0)];
static ALC882_FIXUP_ASUS_W90V_PINS: [hda_pintbl; 2] = [pintbl(0x16, 0x99130110), pintbl(0, 0)];
static ALC889_FIXUP_CD_PINS: [hda_pintbl; 2] = [pintbl(0x1c, 0x993301f0), pintbl(0, 0)];
static ALC889_FIXUP_FRONT_HP_NO_PRESENCE_PINS: [hda_pintbl; 2] = [pintbl(0x1b, 0x02214120), pintbl(0, 0)];
static ALC889_FIXUP_VAIO_TT_PINS: [hda_pintbl; 2] = [pintbl(0x17, 0x90170111), pintbl(0, 0)];
static ALC888_FIXUP_EEE1601_VERBS: [hda_verb; 3] = [verb(0x20, AC_VERB_SET_COEF_INDEX, 0x0b), verb(0x20, AC_VERB_SET_PROC_COEF, 0x0838), verb(0, 0, 0)];
static ALC886_FIXUP_EAPD_VERBS: [hda_verb; 3] = [verb(0x20, AC_VERB_SET_COEF_INDEX, 0x07), verb(0x20, AC_VERB_SET_PROC_COEF, 0x0068), verb(0, 0, 0)];
static ALC882_FIXUP_EAPD_VERBS: [hda_verb; 3] = [verb(0x20, AC_VERB_SET_COEF_INDEX, 0x07), verb(0x20, AC_VERB_SET_PROC_COEF, 0x3060), verb(0, 0, 0)];
static ALC883_FIXUP_EAPD_VERBS: [hda_verb; 3] = [verb(0x20, AC_VERB_SET_COEF_INDEX, 0x07), verb(0x20, AC_VERB_SET_PROC_COEF, 0x3070), verb(0, 0, 0)];
static ALC883_FIXUP_ACER_EAPD_VERBS: [hda_verb; 3] = [verb(0x20, AC_VERB_SET_COEF_INDEX, 0x07), verb(0x20, AC_VERB_SET_PROC_COEF, 0x3050), verb(0, 0, 0)];
static ALC882_FIXUP_ACER_ASPIRE_4930G_PINS: [hda_pintbl; 3] = [pintbl(0x16, 0x99130111), pintbl(0x17, 0x99130112), pintbl(0, 0)];
static ALC882_FIXUP_ACER_ASPIRE_8930G_PINS: [hda_pintbl; 3] = [pintbl(0x16, 0x99130111), pintbl(0x1b, 0x99130112), pintbl(0, 0)];
static ALC882_FIXUP_ASPIRE_8930G_VERBS_DATA: [hda_verb; 9] = [
    verb(0x20, AC_VERB_SET_COEF_INDEX, 0x03),
    verb(0x20, AC_VERB_SET_PROC_COEF, 0x0000),
    verb(0x20, AC_VERB_SET_COEF_INDEX, 0x08),
    verb(0x20, AC_VERB_SET_PROC_COEF, 0x0000),
    verb(0x20, AC_VERB_SET_COEF_INDEX, 0x0b),
    verb(0x20, AC_VERB_SET_PROC_COEF, 0x0003),
    verb(0x20, AC_VERB_SET_COEF_INDEX, 0x07),
    verb(0x20, AC_VERB_SET_PROC_COEF, 0x3050),
    verb(0, 0, 0),
];
static ALC887_FIXUP_ASUS_BASS_PINS: [hda_pintbl; 2] = [pintbl(0x16, 0x99130130), pintbl(0, 0)];
static ALC1220_FIXUP_CLEVO_PB51ED_PINS_DATA: [hda_pintbl; 2] = [pintbl(0x19, 0x01a1913c), pintbl(0, 0)];
static ALC887_FIXUP_ASUS_AUDIO_PINS: [hda_pintbl; 3] = [pintbl(0x15, 0x02a14150), pintbl(0x19, 0x22219420), pintbl(0, 0)];
static ALCS1200A_FIXUP_MIC_VREF_PINS: [hda_pintbl; 3] = [pintbl(0x18, PIN_VREF50), pintbl(0x19, PIN_VREF50), pintbl(0, 0)];
static ALC888VD_FIXUP_MIC_100VREF_PINS: [hda_pintbl; 2] = [pintbl(0x18, PIN_VREF100), pintbl(0, 0)];

static alc882_fixups: [hda_fixup; 43] = [
    fixup_pins(ALC882_FIXUP_ABIT_AW9D_MAX_PINS.as_ptr(), false, 0),
    fixup_pins(ALC882_FIXUP_LENOVO_Y530_PINS.as_ptr(), false, 0),
    fixup_pinctls(ALC882_FIXUP_PB_M5210_PINS.as_ptr()),
    fixup_func(alc_fixup_sku_ignore, false, 0),
    fixup_pins(ALC882_FIXUP_ASUS_W90V_PINS.as_ptr(), false, 0),
    fixup_pins(ALC889_FIXUP_CD_PINS.as_ptr(), false, 0),
    fixup_pins(ALC889_FIXUP_FRONT_HP_NO_PRESENCE_PINS.as_ptr(), true, ALC889_FIXUP_CD),
    fixup_pins(ALC889_FIXUP_VAIO_TT_PINS.as_ptr(), false, 0),
    fixup_verbs(ALC888_FIXUP_EEE1601_VERBS.as_ptr(), false, 0),
    fixup_verbs(ALC886_FIXUP_EAPD_VERBS.as_ptr(), false, 0),
    fixup_verbs(ALC882_FIXUP_EAPD_VERBS.as_ptr(), false, 0),
    fixup_verbs(ALC883_FIXUP_EAPD_VERBS.as_ptr(), false, 0),
    fixup_verbs(ALC883_FIXUP_ACER_EAPD_VERBS.as_ptr(), false, 0),
    fixup_func(alc_fixup_gpio1, false, 0),
    fixup_func(alc_fixup_gpio2, false, 0),
    fixup_func(alc_fixup_gpio3, false, 0),
    fixup_func(alc889_fixup_coef, false, 0),
    fixup_func(alc_fixup_gpio1, true, ALC882_FIXUP_EAPD),
    fixup_pins(ALC882_FIXUP_ACER_ASPIRE_4930G_PINS.as_ptr(), true, ALC882_FIXUP_GPIO1),
    fixup_pins(ALC882_FIXUP_ACER_ASPIRE_8930G_PINS.as_ptr(), true, ALC882_FIXUP_ASPIRE_8930G_VERBS),
    fixup_verbs(ALC882_FIXUP_ASPIRE_8930G_VERBS_DATA.as_ptr(), true, ALC882_FIXUP_GPIO1),
    fixup_func(alc885_fixup_macpro_gpio, false, 0),
    fixup_func(alc889_fixup_dac_route, false, 0),
    fixup_func(alc889_fixup_mbp_vref, true, ALC882_FIXUP_GPIO1),
    fixup_func(alc889_fixup_imac91_vref, true, ALC882_FIXUP_GPIO1),
    fixup_func(alc889_fixup_mba11_vref, true, ALC889_FIXUP_MBP_VREF),
    fixup_func(alc889_fixup_mba21_vref, true, ALC889_FIXUP_MBP_VREF),
    fixup_func(alc889_fixup_mba11_vref, true, ALC885_FIXUP_MACPRO_GPIO),
    fixup_func(alc889_fixup_mbp_vref, true, ALC885_FIXUP_MACPRO_GPIO),
    fixup_func(alc_fixup_inv_dmic, false, 0),
    fixup_func(alc882_fixup_no_primary_hp, false, 0),
    fixup_pins(ALC887_FIXUP_ASUS_BASS_PINS.as_ptr(), true, ALC887_FIXUP_BASS_CHMAP),
    fixup_func(alc_fixup_bass_chmap, false, 0),
    fixup_func(alc1220_fixup_gb_dual_codecs, false, 0),
    fixup_func(alc1220_fixup_gb_x570, false, 0),
    fixup_func(alc1220_fixup_clevo_p950, false, 0),
    fixup_func(alc1220_fixup_clevo_pb51ed, false, 0),
    fixup_pins(ALC1220_FIXUP_CLEVO_PB51ED_PINS_DATA.as_ptr(), true, ALC1220_FIXUP_CLEVO_PB51ED),
    fixup_pins(ALC887_FIXUP_ASUS_AUDIO_PINS.as_ptr(), false, 0),
    fixup_func(alc887_fixup_asus_jack, true, ALC887_FIXUP_ASUS_AUDIO),
    fixup_pinctls(ALCS1200A_FIXUP_MIC_VREF_PINS.as_ptr()),
    fixup_pinctls(ALC888VD_FIXUP_MIC_100VREF_PINS.as_ptr()),
    fixup_func(alc898_fixup_clevo_p775tm1, true, ALC882_FIXUP_EAPD),
];

const fn quirk(subvendor: c_uint, subdevice: c_uint, name: *const c_char, value: c_int) -> hda_quirk {
    hda_quirk { subvendor, subdevice, name, value }
}

const fn quirk_vendor(subvendor: c_uint, name: *const c_char, value: c_int) -> hda_quirk {
    hda_quirk { subvendor, subdevice: 0xffffffff, name, value }
}

static alc882_fixup_tbl: &[hda_quirk] = &[
    quirk(0x1025, 0x006c, c"Acer Aspire 9810".as_ptr(), ALC883_FIXUP_ACER_EAPD),
    quirk(0x1025, 0x0090, c"Acer Aspire".as_ptr(), ALC883_FIXUP_ACER_EAPD),
    quirk(0x1025, 0x0107, c"Acer Aspire".as_ptr(), ALC883_FIXUP_ACER_EAPD),
    quirk(0x1025, 0x010a, c"Acer Ferrari 5000".as_ptr(), ALC883_FIXUP_ACER_EAPD),
    quirk(0x1025, 0x0110, c"Acer Aspire".as_ptr(), ALC883_FIXUP_ACER_EAPD),
    quirk(0x1025, 0x0112, c"Acer Aspire 9303".as_ptr(), ALC883_FIXUP_ACER_EAPD),
    quirk(0x1025, 0x0121, c"Acer Aspire 5920G".as_ptr(), ALC883_FIXUP_ACER_EAPD),
    quirk(0x1025, 0x013e, c"Acer Aspire 4930G".as_ptr(), ALC882_FIXUP_ACER_ASPIRE_4930G),
    quirk(0x1025, 0x013f, c"Acer Aspire 5930G".as_ptr(), ALC882_FIXUP_ACER_ASPIRE_4930G),
    quirk(0x1025, 0x0145, c"Acer Aspire 8930G".as_ptr(), ALC882_FIXUP_ACER_ASPIRE_8930G),
    quirk(0x1025, 0x0146, c"Acer Aspire 6935G".as_ptr(), ALC882_FIXUP_ACER_ASPIRE_8930G),
    quirk(0x1025, 0x0142, c"Acer Aspire 7730G".as_ptr(), ALC882_FIXUP_ACER_ASPIRE_4930G),
    quirk(0x1025, 0x0155, c"Packard-Bell M5120".as_ptr(), ALC882_FIXUP_PB_M5210),
    quirk(0x1025, 0x015e, c"Acer Aspire 6930G".as_ptr(), ALC882_FIXUP_ACER_ASPIRE_4930G),
    quirk(0x1025, 0x0166, c"Acer Aspire 6530G".as_ptr(), ALC882_FIXUP_ACER_ASPIRE_4930G),
    quirk(0x1025, 0x021e, c"Acer Aspire 5739G".as_ptr(), ALC882_FIXUP_ACER_ASPIRE_4930G),
    quirk(0x1025, 0x0259, c"Acer Aspire 5935".as_ptr(), ALC889_FIXUP_DAC_ROUTE),
    quirk(0x1025, 0x026b, c"Acer Aspire 8940G".as_ptr(), ALC882_FIXUP_ACER_ASPIRE_8930G),
    quirk(0x1025, 0x0296, c"Acer Aspire 7736z".as_ptr(), ALC882_FIXUP_ACER_ASPIRE_7736),
    quirk(0x1043, 0x13c2, c"Asus A7M".as_ptr(), ALC882_FIXUP_EAPD),
    quirk(0x1043, 0x1873, c"ASUS W90V".as_ptr(), ALC882_FIXUP_ASUS_W90V),
    quirk(0x1043, 0x1971, c"Asus W2JC".as_ptr(), ALC882_FIXUP_ASUS_W2JC),
    quirk(0x1043, 0x2390, c"Asus D700SA".as_ptr(), ALC887_FIXUP_ASUS_HMIC),
    quirk(0x1043, 0x835f, c"Asus Eee 1601".as_ptr(), ALC888_FIXUP_EEE1601),
    quirk(0x1043, 0x84bc, c"ASUS ET2700".as_ptr(), ALC887_FIXUP_ASUS_BASS),
    quirk(0x1043, 0x8691, c"ASUS ROG Ranger VIII".as_ptr(), ALC882_FIXUP_GPIO3),
    quirk(0x1043, 0x8797, c"ASUS TUF B550M-PLUS".as_ptr(), ALCS1200A_FIXUP_MIC_VREF),
    quirk(0x104d, 0x9043, c"Sony Vaio VGC-LN51JGB".as_ptr(), ALC882_FIXUP_NO_PRIMARY_HP),
    quirk(0x104d, 0x9044, c"Sony VAIO AiO".as_ptr(), ALC882_FIXUP_NO_PRIMARY_HP),
    quirk(0x104d, 0x9047, c"Sony Vaio TT".as_ptr(), ALC889_FIXUP_VAIO_TT),
    quirk(0x104d, 0x905a, c"Sony Vaio Z".as_ptr(), ALC882_FIXUP_NO_PRIMARY_HP),
    quirk(0x104d, 0x9060, c"Sony Vaio VPCL14M1R".as_ptr(), ALC882_FIXUP_NO_PRIMARY_HP),
    quirk(0x106b, 0x00a0, c"MacBookPro 3,1".as_ptr(), ALC889_FIXUP_MBP_VREF),
    quirk(0x106b, 0x00a1, c"Macbook".as_ptr(), ALC889_FIXUP_MBP_VREF),
    quirk(0x106b, 0x00a4, c"MacbookPro 4,1".as_ptr(), ALC889_FIXUP_MBP_VREF),
    quirk(0x106b, 0x0c00, c"Mac Pro".as_ptr(), ALC889_FIXUP_MP11_VREF),
    quirk(0x106b, 0x1000, c"iMac 24".as_ptr(), ALC885_FIXUP_MACPRO_GPIO),
    quirk(0x106b, 0x2800, c"AppleTV".as_ptr(), ALC885_FIXUP_MACPRO_GPIO),
    quirk(0x106b, 0x2c00, c"MacbookPro rev3".as_ptr(), ALC889_FIXUP_MBP_VREF),
    quirk(0x106b, 0x3000, c"iMac".as_ptr(), ALC889_FIXUP_MBP_VREF),
    quirk(0x106b, 0x3200, c"iMac 7,1 Aluminum".as_ptr(), ALC882_FIXUP_EAPD),
    quirk(0x106b, 0x3400, c"MacBookAir 1,1".as_ptr(), ALC889_FIXUP_MBA11_VREF),
    quirk(0x106b, 0x3500, c"MacBookAir 2,1".as_ptr(), ALC889_FIXUP_MBA21_VREF),
    quirk(0x106b, 0x3600, c"Macbook 3,1".as_ptr(), ALC889_FIXUP_MBP_VREF),
    quirk(0x106b, 0x3800, c"MacbookPro 4,1".as_ptr(), ALC889_FIXUP_MBP_VREF),
    quirk(0x106b, 0x3e00, c"iMac 24 Aluminum".as_ptr(), ALC885_FIXUP_MACPRO_GPIO),
    quirk(0x106b, 0x3f00, c"Macbook 5,1".as_ptr(), ALC889_FIXUP_IMAC91_VREF),
    quirk(0x106b, 0x4000, c"MacbookPro 5,1".as_ptr(), ALC889_FIXUP_IMAC91_VREF),
    quirk(0x106b, 0x4100, c"Macmini 3,1".as_ptr(), ALC889_FIXUP_IMAC91_VREF),
    quirk(0x106b, 0x4200, c"Mac Pro 4,1/5,1".as_ptr(), ALC889_FIXUP_MP41_VREF),
    quirk(0x106b, 0x4300, c"iMac 9,1".as_ptr(), ALC889_FIXUP_IMAC91_VREF),
    quirk(0x106b, 0x4600, c"MacbookPro 5,2".as_ptr(), ALC889_FIXUP_IMAC91_VREF),
    quirk(0x106b, 0x4900, c"iMac 9,1 Aluminum".as_ptr(), ALC889_FIXUP_IMAC91_VREF),
    quirk(0x106b, 0x4a00, c"Macbook 5,2".as_ptr(), ALC889_FIXUP_MBA11_VREF),
    quirk(0x1071, 0x8258, c"Evesham Voyaeger".as_ptr(), ALC882_FIXUP_EAPD),
    quirk(0x10ec, 0x12d8, c"iBase Elo Touch".as_ptr(), ALC888VD_FIXUP_MIC_100VREF),
    quirk(0x13fe, 0x1009, c"Advantech MIT-W101".as_ptr(), ALC886_FIXUP_EAPD),
    quirk(0x1458, 0xa002, c"Gigabyte EP45-DS3/Z87X-UD3H".as_ptr(), ALC889_FIXUP_FRONT_HP_NO_PRESENCE),
    quirk(0x1458, 0xa0b8, c"Gigabyte AZ370-Gaming".as_ptr(), ALC1220_FIXUP_GB_DUAL_CODECS),
    quirk(0x1458, 0xa0cd, c"Gigabyte X570 Aorus Master".as_ptr(), ALC1220_FIXUP_GB_X570),
    quirk(0x1458, 0xa0ce, c"Gigabyte X570 Aorus Xtreme".as_ptr(), ALC1220_FIXUP_GB_X570),
    quirk(0x1458, 0xa0d5, c"Gigabyte X570S Aorus Master".as_ptr(), ALC1220_FIXUP_GB_X570),
    quirk(0x1462, 0x11f7, c"MSI-GE63".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1462, 0x1228, c"MSI-GP63".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1462, 0x1229, c"MSI-GP73".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1462, 0x1275, c"MSI-GL63".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1462, 0x1276, c"MSI-GL73".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1462, 0x1293, c"MSI-GP65".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1462, 0x7350, c"MSI-7350".as_ptr(), ALC889_FIXUP_CD),
    quirk(0x1462, 0xcc34, c"MSI Godlike X570".as_ptr(), ALC1220_FIXUP_GB_DUAL_CODECS),
    quirk(0x1462, 0xda57, c"MSI Z270-Gaming".as_ptr(), ALC1220_FIXUP_GB_DUAL_CODECS),
    quirk_vendor(0x1462, c"MSI".as_ptr(), ALC882_FIXUP_GPIO3),
    quirk(0x147b, 0x107a, c"Abit AW9D-MAX".as_ptr(), ALC882_FIXUP_ABIT_AW9D_MAX),
    quirk(0x1558, 0x3702, c"Clevo X370SN[VW]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x50d3, c"Clevo PC50[ER][CDF]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x5802, c"Clevo X58[05]WN[RST]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x65d1, c"Clevo PB51[ER][CDF]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x65d2, c"Clevo PB51R[CDF]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x65e1, c"Clevo PB51[ED][DF]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x65e5, c"Clevo PC50D[PRS](?:-D|-G)?".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x65f1, c"Clevo PC50HS".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x65f5, c"Clevo PD50PN[NRT]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x66a2, c"Clevo PE60RNE".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x66a6, c"Clevo PE60SN[CDE]-[GS]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x67d1, c"Clevo PB71[ER][CDF]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x67e1, c"Clevo PB71[DE][CDF]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x67e5, c"Clevo PC70D[PRS](?:-D|-G)?".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x67f1, c"Clevo PC70H[PRS]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x67f5, c"Clevo PD70PN[NRT]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x70d1, c"Clevo PC70[ER][CDF]".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x7709, c"Clevo P775TM1".as_ptr(), ALC898_FIXUP_CLEVO_P775TM1),
    quirk(0x1558, 0x7714, c"Clevo X170SM".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk(0x1558, 0x7715, c"Clevo X170KM-G".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED),
    quirk(0x1558, 0x9501, c"Clevo P950HR".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0x9506, c"Clevo P955HQ".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0x950a, c"Clevo P955H[PR]".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0x95e1, c"Clevo P95xER".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0x95e2, c"Clevo P950ER".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0x95e3, c"Clevo P955[ER]T".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0x95e4, c"Clevo P955ER".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0x95e5, c"Clevo P955EE6".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0x95e6, c"Clevo P950R[CDF]".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0x96e1, c"Clevo P960[ER][CDFN]-K".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0x97e1, c"Clevo P970[ER][CDFN]".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0x97e2, c"Clevo P970RC-M".as_ptr(), ALC1220_FIXUP_CLEVO_P950),
    quirk(0x1558, 0xd502, c"Clevo PD50SNE".as_ptr(), ALC1220_FIXUP_CLEVO_PB51ED_PINS),
    quirk_vendor(0x1558, c"Clevo laptop".as_ptr(), ALC882_FIXUP_EAPD),
    quirk(0x161f, 0x2054, c"Medion laptop".as_ptr(), ALC883_FIXUP_EAPD),
    quirk(0x17aa, 0x3a0d, c"Lenovo Y530".as_ptr(), ALC882_FIXUP_LENOVO_Y530),
    quirk(0x8086, 0x0022, c"DX58SO".as_ptr(), ALC889_FIXUP_COEF),
    quirk(0, 0, core::ptr::null(), 0),
];

static alc882_fixup_models: [hda_model_fixup; 35] = [
    hda_model_fixup { id: ALC882_FIXUP_ABIT_AW9D_MAX, name: c"abit-aw9d".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_LENOVO_Y530, name: c"lenovo-y530".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_ACER_ASPIRE_7736, name: c"acer-aspire-7736".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_ASUS_W90V, name: c"asus-w90v".as_ptr() },
    hda_model_fixup { id: ALC889_FIXUP_CD, name: c"cd".as_ptr() },
    hda_model_fixup { id: ALC889_FIXUP_FRONT_HP_NO_PRESENCE, name: c"no-front-hp".as_ptr() },
    hda_model_fixup { id: ALC889_FIXUP_VAIO_TT, name: c"vaio-tt".as_ptr() },
    hda_model_fixup { id: ALC888_FIXUP_EEE1601, name: c"eee1601".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_EAPD, name: c"alc882-eapd".as_ptr() },
    hda_model_fixup { id: ALC883_FIXUP_EAPD, name: c"alc883-eapd".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_GPIO1, name: c"gpio1".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_GPIO2, name: c"gpio2".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_GPIO3, name: c"gpio3".as_ptr() },
    hda_model_fixup { id: ALC889_FIXUP_COEF, name: c"alc889-coef".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_ASUS_W2JC, name: c"asus-w2jc".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_ACER_ASPIRE_4930G, name: c"acer-aspire-4930g".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_ACER_ASPIRE_8930G, name: c"acer-aspire-8930g".as_ptr() },
    hda_model_fixup { id: ALC883_FIXUP_ACER_EAPD, name: c"acer-aspire".as_ptr() },
    hda_model_fixup { id: ALC885_FIXUP_MACPRO_GPIO, name: c"macpro-gpio".as_ptr() },
    hda_model_fixup { id: ALC889_FIXUP_DAC_ROUTE, name: c"dac-route".as_ptr() },
    hda_model_fixup { id: ALC889_FIXUP_MBP_VREF, name: c"mbp-vref".as_ptr() },
    hda_model_fixup { id: ALC889_FIXUP_IMAC91_VREF, name: c"imac91-vref".as_ptr() },
    hda_model_fixup { id: ALC889_FIXUP_MBA11_VREF, name: c"mba11-vref".as_ptr() },
    hda_model_fixup { id: ALC889_FIXUP_MBA21_VREF, name: c"mba21-vref".as_ptr() },
    hda_model_fixup { id: ALC889_FIXUP_MP11_VREF, name: c"mp11-vref".as_ptr() },
    hda_model_fixup { id: ALC889_FIXUP_MP41_VREF, name: c"mp41-vref".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_INV_DMIC, name: c"inv-dmic".as_ptr() },
    hda_model_fixup { id: ALC882_FIXUP_NO_PRIMARY_HP, name: c"no-primary-hp".as_ptr() },
    hda_model_fixup { id: ALC887_FIXUP_ASUS_BASS, name: c"asus-bass".as_ptr() },
    hda_model_fixup { id: ALC1220_FIXUP_GB_DUAL_CODECS, name: c"dual-codecs".as_ptr() },
    hda_model_fixup { id: ALC1220_FIXUP_GB_X570, name: c"gb-x570".as_ptr() },
    hda_model_fixup { id: ALC1220_FIXUP_CLEVO_P950, name: c"clevo-p950".as_ptr() },
    hda_model_fixup { id: ALC898_FIXUP_CLEVO_P775TM1, name: c"clevo-p775tm1".as_ptr() },
    hda_model_fixup { id: 0, name: core::ptr::null() },
];

static ALC882_PIN_QUIRK_0_PINS: [hda_pintbl; 8] = [pintbl(0x14, 0x01014010), pintbl(0x15, 0x01011012), pintbl(0x16, 0x01016011), pintbl(0x18, 0x01a19040), pintbl(0x19, 0x02a19050), pintbl(0x1a, 0x0181304f), pintbl(0x1b, 0x0221401f), pintbl(0x1e, 0x01456130)];
static ALC882_PIN_QUIRK_1_PINS: [hda_pintbl; 8] = [pintbl(0x14, 0x01015010), pintbl(0x15, 0x01011012), pintbl(0x16, 0x01011011), pintbl(0x18, 0x01a11040), pintbl(0x19, 0x02a19050), pintbl(0x1a, 0x0181104f), pintbl(0x1b, 0x0221401f), pintbl(0x1e, 0x01451130)];

static alc882_pin_fixup_tbl: [snd_hda_pin_quirk; 3] = [
    snd_hda_pin_quirk { codec: 0x10ec1220, subvendor: 0x1043, name: c"ASUS".as_ptr(), value: ALC1220_FIXUP_CLEVO_P950, pins: ALC882_PIN_QUIRK_0_PINS.as_ptr() },
    snd_hda_pin_quirk { codec: 0x10ec1220, subvendor: 0x1462, name: c"MS-7C35".as_ptr(), value: ALC1220_FIXUP_CLEVO_P950, pins: ALC882_PIN_QUIRK_1_PINS.as_ptr() },
    snd_hda_pin_quirk { codec: 0, subvendor: 0, name: core::ptr::null(), value: 0, pins: core::ptr::null() },
];

/*
 * BIOS auto configuration
 */
/* almost identical with ALC880 parser... */
unsafe extern "C" fn alc882_parse_auto_config(codec: *mut hda_codec) -> c_int {
    static alc882_ignore: [hda_nid_t; 2] = [0x1d, 0];
    static alc882_ssids: [hda_nid_t; 4] = [0x15, 0x1b, 0x14, 0];
    alc_parse_auto_config(codec, alc882_ignore.as_ptr(), alc882_ssids.as_ptr())
}

/*
 */
unsafe extern "C" fn alc882_probe(codec: *mut hda_codec, _id: *const hda_device_id) -> c_int {
    let spec: *mut alc_spec;
    let mut err: c_int;

    err = alc_alloc_spec(codec, 0x0b);
    if err < 0 {
        return err;
    }

    spec = (*codec).spec;

    match (*codec).core.vendor_id {
        0x10ec0882 | 0x10ec0885 | 0x10ec0900 | 0x10ec0b00 | 0x10ec1220 => {}
        _ => {
            /* ALC883 and variants */
            alc_fix_pll_init(codec, 0x20, 0x0a, 10);
        }
    }

    alc_pre_init(codec);

    snd_hda_pick_fixup(codec, alc882_fixup_models.as_ptr(), alc882_fixup_tbl.as_ptr(), alc882_fixups.as_ptr());
    snd_hda_pick_pin_fixup(codec, alc882_pin_fixup_tbl.as_ptr(), alc882_fixups.as_ptr(), true);
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

    alc_auto_parse_customize_define(codec);

    if has_cdefine_beep(codec) {
        (*spec).gen.beep_nid = 0x01;
    }

    /* automatic parse from the BIOS config */
    err = alc882_parse_auto_config(codec);
    if err < 0 {
        snd_hda_gen_remove(codec);
        return err;
    }

    if (*spec).gen.no_analog == 0 && (*spec).gen.beep_nid != 0 {
        err = set_beep_amp(spec, 0x0b, 0x05, HDA_INPUT);
        if err < 0 {
            snd_hda_gen_remove(codec);
            return err;
        }
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);

    0
}

static alc882_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(alc882_probe),
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

const fn codec_id(vendor_id: c_uint, name: *const c_char) -> hda_device_id {
    hda_device_id { vendor_id, rev_id: 0, name }
}

const fn codec_id_rev(vendor_id: c_uint, rev_id: c_uint, name: *const c_char) -> hda_device_id {
    hda_device_id { vendor_id, rev_id, name }
}

/*
 * driver entries
 */
static snd_hda_id_alc882: [hda_device_id; 16] = [
    codec_id_rev(0x10ec0662, 0x100002, c"ALC662 rev2".as_ptr()),
    codec_id(0x10ec0882, c"ALC882".as_ptr()),
    codec_id(0x10ec0883, c"ALC883".as_ptr()),
    codec_id_rev(0x10ec0885, 0x100101, c"ALC889A".as_ptr()),
    codec_id_rev(0x10ec0885, 0x100103, c"ALC889A".as_ptr()),
    codec_id(0x10ec0885, c"ALC885".as_ptr()),
    codec_id(0x10ec0887, c"ALC887".as_ptr()),
    codec_id_rev(0x10ec0888, 0x100101, c"ALC1200".as_ptr()),
    codec_id(0x10ec0888, c"ALC888".as_ptr()),
    codec_id(0x10ec0889, c"ALC889".as_ptr()),
    codec_id(0x10ec0899, c"ALC898".as_ptr()),
    codec_id(0x10ec0900, c"ALC1150".as_ptr()),
    codec_id(0x10ec0b00, c"ALCS1200A".as_ptr()),
    codec_id(0x10ec1168, c"ALC1220".as_ptr()),
    codec_id(0x10ec1220, c"ALC1220".as_ptr()),
    hda_device_id { vendor_id: 0, rev_id: 0, name: core::ptr::null() }, /* terminator */
];
/* MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_alc882); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("Realtek ALC882 and compatible HD-audio codecs"); */
/* MODULE_IMPORT_NS("SND_HDA_CODEC_REALTEK"); */

static mut alc882_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_alc882.as_ptr(),
    ops: &alc882_codec_ops,
};

/* module_hda_codec_driver(alc882_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
