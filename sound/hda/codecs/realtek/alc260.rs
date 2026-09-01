// SPDX-License-Identifier: GPL-2.0-or-later
//
// Realtek ALC260 codec
//

// C dependencies: <linux/init.h>, <linux/module.h>, "realtek.h"

pub type hda_nid_t = u16;

#[repr(C)]
pub struct hda_codec {
    pub spec: *mut alc_spec,
}

#[repr(C)]
pub struct hda_device_id {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct alc_spec {
    pub gen: hda_gen_spec,
    pub init_amp: i32,
    pub shutup: Option<unsafe extern "C" fn(*mut hda_codec)>,
}

#[repr(C)]
pub struct hda_gen_spec {
    pub hp_jack_present: bool,
    pub automute_hook: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub detect_hp: i32,
    pub automute_speaker: i32,
    pub autocfg: auto_pin_cfg,
    pub add_jack_modes: i32,
    pub hp_mic: i32,
    pub prefer_hp_amp: i32,
    pub beep_nid: hda_nid_t,
    pub no_analog: bool,
}

#[repr(C)]
pub struct auto_pin_cfg {
    pub hp_pins: [hda_nid_t; 32],
}

#[repr(C)]
pub struct hda_pintbl {
    pub nid: hda_nid_t,
    pub val: u32,
}

#[repr(C)]
pub struct hda_verb {
    pub nid: hda_nid_t,
    pub verb: u32,
    pub param: u32,
}

#[repr(C)]
pub union hda_fixup_v {
    pub pins: *const hda_pintbl,
    pub verbs: *const hda_verb,
    pub func: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_fixup, i32)>,
}

#[repr(C)]
pub struct hda_fixup {
    pub type_: i32,
    pub v: hda_fixup_v,
    pub chained: bool,
    pub chain_id: i32,
}

#[repr(C)]
pub struct hda_quirk {
    pub subvendor: u32,
    pub subdevice: u32,
    pub name: *const u8,
    pub value: i32,
}

#[repr(C)]
pub struct hda_model_fixup {
    pub id: i32,
    pub name: *const u8,
}

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> i32>,
    pub build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> i32>,
    pub init: Option<unsafe extern "C" fn(*mut hda_codec) -> i32>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, u32)>,
    pub resume: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub suspend: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub check_power_status: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t) -> i32>,
    pub stream_pm: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, bool)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

pub const HDA_FIXUP_ACT_PRE_PROBE: i32 = 0;
pub const HDA_FIXUP_ACT_PROBE: i32 = 1;
pub const HDA_FIXUP_PINS: i32 = 0;
pub const HDA_FIXUP_VERBS: i32 = 1;
pub const HDA_FIXUP_FUNC: i32 = 2;
pub const AC_VERB_SET_COEF_INDEX: u32 = 0;
pub const AC_VERB_SET_PROC_COEF: u32 = 0;
pub const ALC_INIT_NONE: i32 = 0;
pub const HDA_INPUT: i32 = 0;

extern "C" {
    fn alc_parse_auto_config(
        codec: *mut hda_codec,
        ignore: *const hda_nid_t,
        ssids: *const hda_nid_t,
    ) -> i32;
    fn alc_update_gpio_data(codec: *mut hda_codec, mask: u32, data: bool);
    fn snd_hda_jack_detect_enable_callback(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        cb: Option<unsafe extern "C" fn(*mut hda_codec)>,
    );
    fn snd_hda_gen_hp_automute(codec: *mut hda_codec);
    fn alc_setup_gpio(codec: *mut hda_codec, mask: u32);
    fn snd_hda_apply_pincfgs(codec: *mut hda_codec, cfg: *const hda_pintbl);
    fn alc_fixup_gpio1(codec: *mut hda_codec, fix: *const hda_fixup, action: i32);
    fn alc_alloc_spec(codec: *mut hda_codec, mixer_nid: hda_nid_t) -> i32;
    fn alc_eapd_shutup(codec: *mut hda_codec);
    fn alc_pre_init(codec: *mut hda_codec);
    fn snd_hda_pick_fixup(
        codec: *mut hda_codec,
        models: *const hda_model_fixup,
        tbl: *const hda_quirk,
        fixups: *const hda_fixup,
    );
    fn snd_hda_apply_fixup(codec: *mut hda_codec, action: i32);
    fn set_beep_amp(spec: *mut alc_spec, nid: hda_nid_t, idx: u32, dir: i32) -> i32;
    fn snd_hda_gen_remove(codec: *mut hda_codec);
    fn alc_build_controls(codec: *mut hda_codec) -> i32;
    fn snd_hda_gen_build_pcms(codec: *mut hda_codec) -> i32;
    fn alc_init(codec: *mut hda_codec) -> i32;
    fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: u32);
    fn alc_resume(codec: *mut hda_codec);
    fn alc_suspend(codec: *mut hda_codec);
    fn snd_hda_gen_check_power_status(codec: *mut hda_codec, nid: hda_nid_t) -> i32;
    fn snd_hda_gen_stream_pm(codec: *mut hda_codec, nid: hda_nid_t, on: bool);
}

unsafe fn alc260_parse_auto_config(codec: *mut hda_codec) -> i32 {
    static ALC260_IGNORE: [hda_nid_t; 2] = [0x17, 0];
    static ALC260_SSIDS: [hda_nid_t; 4] = [0x10, 0x15, 0x0f, 0];
    unsafe { alc_parse_auto_config(codec, ALC260_IGNORE.as_ptr(), ALC260_SSIDS.as_ptr()) }
}

/*
 * Pin config fixes
 */
const ALC260_FIXUP_HP_DC5750: i32 = 0;
const ALC260_FIXUP_HP_PIN_0F: i32 = 1;
const ALC260_FIXUP_COEF: i32 = 2;
const ALC260_FIXUP_GPIO1: i32 = 3;
const ALC260_FIXUP_GPIO1_TOGGLE: i32 = 4;
const ALC260_FIXUP_REPLACER: i32 = 5;
const ALC260_FIXUP_HP_B1900: i32 = 6;
const ALC260_FIXUP_KN1: i32 = 7;
const ALC260_FIXUP_FSC_S7020: i32 = 8;
const ALC260_FIXUP_FSC_S7020_JWSE: i32 = 9;
const ALC260_FIXUP_VAIO_PINS: i32 = 10;

unsafe extern "C" fn alc260_gpio1_automute(codec: *mut hda_codec) {
    let spec = unsafe { (*codec).spec };

    unsafe { alc_update_gpio_data(codec, 0x01, (*spec).gen.hp_jack_present) };
}

unsafe extern "C" fn alc260_fixup_gpio1_toggle(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: i32,
) {
    let spec = unsafe { (*codec).spec };
    if action == HDA_FIXUP_ACT_PROBE {
        /* although the machine has only one output pin, we need to
         * toggle GPIO1 according to the jack state
         */
        unsafe {
            (*spec).gen.automute_hook = Some(alc260_gpio1_automute);
            (*spec).gen.detect_hp = 1;
            (*spec).gen.automute_speaker = 1;
            (*spec).gen.autocfg.hp_pins[0] = 0x0f; /* copy it for automute */
            snd_hda_jack_detect_enable_callback(codec, 0x0f, Some(snd_hda_gen_hp_automute));
            alc_setup_gpio(codec, 0x01);
        }
    }
}

unsafe extern "C" fn alc260_fixup_kn1(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: i32,
) {
    let spec = unsafe { (*codec).spec };
    static PINCFGS: [hda_pintbl; 12] = [
        hda_pintbl { nid: 0x0f, val: 0x02214000 }, /* HP/speaker */
        hda_pintbl { nid: 0x12, val: 0x90a60160 }, /* int mic */
        hda_pintbl { nid: 0x13, val: 0x02a19000 }, /* ext mic */
        hda_pintbl { nid: 0x18, val: 0x01446000 }, /* SPDIF out */
        /* disable bogus I/O pins */
        hda_pintbl { nid: 0x10, val: 0x411111f0 },
        hda_pintbl { nid: 0x11, val: 0x411111f0 },
        hda_pintbl { nid: 0x14, val: 0x411111f0 },
        hda_pintbl { nid: 0x15, val: 0x411111f0 },
        hda_pintbl { nid: 0x16, val: 0x411111f0 },
        hda_pintbl { nid: 0x17, val: 0x411111f0 },
        hda_pintbl { nid: 0x19, val: 0x411111f0 },
        hda_pintbl { nid: 0, val: 0 },
    ];

    match action {
        HDA_FIXUP_ACT_PRE_PROBE => {
            unsafe {
                snd_hda_apply_pincfgs(codec, PINCFGS.as_ptr());
                (*spec).init_amp = ALC_INIT_NONE;
            }
        }
        _ => {}
    }
}

unsafe extern "C" fn alc260_fixup_fsc_s7020(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: i32,
) {
    let spec = unsafe { (*codec).spec };
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        unsafe { (*spec).init_amp = ALC_INIT_NONE };
    }
}

unsafe extern "C" fn alc260_fixup_fsc_s7020_jwse(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: i32,
) {
    let spec = unsafe { (*codec).spec };
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        unsafe {
            (*spec).gen.add_jack_modes = 1;
            (*spec).gen.hp_mic = 1;
        }
    }
}

static ALC260_FIXUP_HP_DC5750_PINS: [hda_pintbl; 2] = [
    hda_pintbl { nid: 0x11, val: 0x90130110 }, /* speaker */
    hda_pintbl { nid: 0, val: 0 },
];

static ALC260_FIXUP_HP_PIN_0F_PINS: [hda_pintbl; 2] = [
    hda_pintbl { nid: 0x0f, val: 0x01214000 }, /* HP */
    hda_pintbl { nid: 0, val: 0 },
];

static ALC260_FIXUP_COEF_VERBS: [hda_verb; 3] = [
    hda_verb { nid: 0x1a, verb: AC_VERB_SET_COEF_INDEX, param: 0x07 },
    hda_verb { nid: 0x1a, verb: AC_VERB_SET_PROC_COEF, param: 0x3040 },
    hda_verb { nid: 0, verb: 0, param: 0 },
];

static ALC260_FIXUP_REPLACER_VERBS: [hda_verb; 3] = [
    hda_verb { nid: 0x1a, verb: AC_VERB_SET_COEF_INDEX, param: 0x07 },
    hda_verb { nid: 0x1a, verb: AC_VERB_SET_PROC_COEF, param: 0x3050 },
    hda_verb { nid: 0, verb: 0, param: 0 },
];

static ALC260_FIXUP_VAIO_PINS_PINS: [hda_pintbl; 12] = [
    /* Pin configs are missing completely on some VAIOs */
    hda_pintbl { nid: 0x0f, val: 0x01211020 },
    hda_pintbl { nid: 0x10, val: 0x0001003f },
    hda_pintbl { nid: 0x11, val: 0x411111f0 },
    hda_pintbl { nid: 0x12, val: 0x01a15930 },
    hda_pintbl { nid: 0x13, val: 0x411111f0 },
    hda_pintbl { nid: 0x14, val: 0x411111f0 },
    hda_pintbl { nid: 0x15, val: 0x411111f0 },
    hda_pintbl { nid: 0x16, val: 0x411111f0 },
    hda_pintbl { nid: 0x17, val: 0x411111f0 },
    hda_pintbl { nid: 0x18, val: 0x411111f0 },
    hda_pintbl { nid: 0x19, val: 0x411111f0 },
    hda_pintbl { nid: 0, val: 0 },
];

static ALC260_FIXUPS: [hda_fixup; 11] = [
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_v { pins: ALC260_FIXUP_HP_DC5750_PINS.as_ptr() },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_v { pins: ALC260_FIXUP_HP_PIN_0F_PINS.as_ptr() },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_VERBS,
        v: hda_fixup_v { verbs: ALC260_FIXUP_COEF_VERBS.as_ptr() },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_v { func: Some(alc_fixup_gpio1) },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_v { func: Some(alc260_fixup_gpio1_toggle) },
        chained: true,
        chain_id: ALC260_FIXUP_HP_PIN_0F,
    },
    hda_fixup {
        type_: HDA_FIXUP_VERBS,
        v: hda_fixup_v { verbs: ALC260_FIXUP_REPLACER_VERBS.as_ptr() },
        chained: true,
        chain_id: ALC260_FIXUP_GPIO1_TOGGLE,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_v { func: Some(alc260_fixup_gpio1_toggle) },
        chained: true,
        chain_id: ALC260_FIXUP_COEF,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_v { func: Some(alc260_fixup_kn1) },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_v { func: Some(alc260_fixup_fsc_s7020) },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_v { func: Some(alc260_fixup_fsc_s7020_jwse) },
        chained: true,
        chain_id: ALC260_FIXUP_FSC_S7020,
    },
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_v { pins: ALC260_FIXUP_VAIO_PINS_PINS.as_ptr() },
        chained: false,
        chain_id: 0,
    },
];

static ALC260_FIXUP_TBL: [hda_quirk; 13] = [
    hda_quirk { subvendor: 0x1025, subdevice: 0x007b, name: c"Acer C20x".as_ptr() as *const u8, value: ALC260_FIXUP_GPIO1 },
    hda_quirk { subvendor: 0x1025, subdevice: 0x007f, name: c"Acer Aspire 9500".as_ptr() as *const u8, value: ALC260_FIXUP_COEF },
    hda_quirk { subvendor: 0x1025, subdevice: 0x008f, name: c"Acer".as_ptr() as *const u8, value: ALC260_FIXUP_GPIO1 },
    hda_quirk { subvendor: 0x103c, subdevice: 0x280a, name: c"HP dc5750".as_ptr() as *const u8, value: ALC260_FIXUP_HP_DC5750 },
    hda_quirk { subvendor: 0x103c, subdevice: 0x30ba, name: c"HP Presario B1900".as_ptr() as *const u8, value: ALC260_FIXUP_HP_B1900 },
    hda_quirk { subvendor: 0x104d, subdevice: 0x81bb, name: c"Sony VAIO".as_ptr() as *const u8, value: ALC260_FIXUP_VAIO_PINS },
    hda_quirk { subvendor: 0x104d, subdevice: 0x81e2, name: c"Sony VAIO TX".as_ptr() as *const u8, value: ALC260_FIXUP_HP_PIN_0F },
    hda_quirk { subvendor: 0x10cf, subdevice: 0x1326, name: c"FSC LifeBook S7020".as_ptr() as *const u8, value: ALC260_FIXUP_FSC_S7020 },
    hda_quirk { subvendor: 0x1509, subdevice: 0x4540, name: c"Favorit 100XS".as_ptr() as *const u8, value: ALC260_FIXUP_GPIO1 },
    hda_quirk { subvendor: 0x152d, subdevice: 0x0729, name: c"Quanta KN1".as_ptr() as *const u8, value: ALC260_FIXUP_KN1 },
    hda_quirk { subvendor: 0x161f, subdevice: 0x2057, name: c"Replacer 672V".as_ptr() as *const u8, value: ALC260_FIXUP_REPLACER },
    hda_quirk { subvendor: 0x1631, subdevice: 0xc017, name: c"PB V7900".as_ptr() as *const u8, value: ALC260_FIXUP_COEF },
    hda_quirk { subvendor: 0, subdevice: 0, name: core::ptr::null(), value: 0 },
];

static ALC260_FIXUP_MODELS: [hda_model_fixup; 5] = [
    hda_model_fixup { id: ALC260_FIXUP_GPIO1, name: c"gpio1".as_ptr() as *const u8 },
    hda_model_fixup { id: ALC260_FIXUP_COEF, name: c"coef".as_ptr() as *const u8 },
    hda_model_fixup { id: ALC260_FIXUP_FSC_S7020, name: c"fujitsu".as_ptr() as *const u8 },
    hda_model_fixup { id: ALC260_FIXUP_FSC_S7020_JWSE, name: c"fujitsu-jwse".as_ptr() as *const u8 },
    hda_model_fixup { id: 0, name: core::ptr::null() },
];

/*
 */
unsafe extern "C" fn alc260_probe(codec: *mut hda_codec, _id: *const hda_device_id) -> i32 {
    let spec: *mut alc_spec;
    let mut err: i32;

    err = unsafe { alc_alloc_spec(codec, 0x07) };
    if err < 0 {
        return err;
    }

    spec = unsafe { (*codec).spec };
    /* as quite a few machines require HP amp for speaker outputs,
     * it's easier to enable it unconditionally; even if it's unneeded,
     * it's almost harmless.
     */
    unsafe {
        (*spec).gen.prefer_hp_amp = 1;
        (*spec).gen.beep_nid = 0x01;

        (*spec).shutup = Some(alc_eapd_shutup);

        alc_pre_init(codec);

        snd_hda_pick_fixup(
            codec,
            ALC260_FIXUP_MODELS.as_ptr(),
            ALC260_FIXUP_TBL.as_ptr(),
            ALC260_FIXUPS.as_ptr(),
        );
        snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

        /* automatic parse from the BIOS config */
        err = alc260_parse_auto_config(codec);
    }
    if err < 0 {
        unsafe {
            snd_hda_gen_remove(codec);
        }
        return err;
    }

    if unsafe { !(*spec).gen.no_analog } {
        err = unsafe { set_beep_amp(spec, 0x07, 0x05, HDA_INPUT) };
        if err < 0 {
            unsafe {
                snd_hda_gen_remove(codec);
            }
            return err;
        }
    }

    unsafe { snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE) };

    0
}

static ALC260_CODEC_OPS: hda_codec_ops = hda_codec_ops {
    probe: Some(alc260_probe),
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
#[repr(C)]
pub struct hda_device_id_init {
    pub id: u32,
    pub name: *const u8,
}

static SND_HDA_ID_ALC260: [hda_device_id_init; 2] = [
    hda_device_id_init { id: 0x10ec0260, name: c"ALC260".as_ptr() as *const u8 },
    hda_device_id_init { id: 0, name: core::ptr::null() }, /* terminator */
];
/* MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_alc260); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("Realtek ALC260 HD-audio codec"); */
/* MODULE_IMPORT_NS("SND_HDA_CODEC_REALTEK"); */

static mut ALC260_DRIVER: hda_codec_driver = hda_codec_driver {
    id: SND_HDA_ID_ALC260.as_ptr() as *const hda_device_id,
    ops: &ALC260_CODEC_OPS,
};

/* module_hda_codec_driver(alc260_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
