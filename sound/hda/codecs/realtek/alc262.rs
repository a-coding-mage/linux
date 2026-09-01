// SPDX-License-Identifier: GPL-2.0-or-later
//
// Realtek ALC262 codec
//

// C dependencies: <linux/init.h>, <linux/module.h>, "realtek.h"

unsafe fn alc262_parse_auto_config(codec: *mut hda_codec) -> c_int {
    static ALC262_IGNORE: [hda_nid_t; 2] = [0x1d, 0];
    static ALC262_SSIDS: [hda_nid_t; 4] = [0x15, 0x1b, 0x14, 0];

    unsafe { alc_parse_auto_config(codec, ALC262_IGNORE.as_ptr(), ALC262_SSIDS.as_ptr()) }
}

/*
 * Pin config fixes
 */
const ALC262_FIXUP_FSC_H270: c_int = 0;
const ALC262_FIXUP_FSC_S7110: c_int = 1;
const ALC262_FIXUP_HP_Z200: c_int = 2;
const ALC262_FIXUP_TYAN: c_int = 3;
const ALC262_FIXUP_LENOVO_3000: c_int = 4;
const ALC262_FIXUP_BENQ: c_int = 5;
const ALC262_FIXUP_BENQ_T31: c_int = 6;
const ALC262_FIXUP_INV_DMIC: c_int = 7;
const ALC262_FIXUP_INTEL_BAYLEYBAY: c_int = 8;

static ALC262_FIXUP_FSC_H270_PINS: [hda_pintbl; 4] = [
    hda_pintbl {
        nid: 0x14,
        val: 0x99130110,
    }, /* speaker */
    hda_pintbl {
        nid: 0x15,
        val: 0x0221142f,
    }, /* front HP */
    hda_pintbl {
        nid: 0x1b,
        val: 0x0121141f,
    }, /* rear HP */
    hda_pintbl { nid: 0, val: 0 },
];

static ALC262_FIXUP_FSC_S7110_PINS: [hda_pintbl; 2] = [
    hda_pintbl {
        nid: 0x15,
        val: 0x90170110,
    }, /* speaker */
    hda_pintbl { nid: 0, val: 0 },
];

static ALC262_FIXUP_HP_Z200_PINS: [hda_pintbl; 2] = [
    hda_pintbl {
        nid: 0x16,
        val: 0x99130120,
    }, /* internal speaker */
    hda_pintbl { nid: 0, val: 0 },
];

static ALC262_FIXUP_TYAN_PINS: [hda_pintbl; 2] = [
    hda_pintbl {
        nid: 0x14,
        val: 0x1993e1f0,
    }, /* int AUX */
    hda_pintbl { nid: 0, val: 0 },
];

static ALC262_FIXUP_LENOVO_3000_PINS: [hda_pintbl; 2] = [
    hda_pintbl {
        nid: 0x19,
        val: PIN_VREF50,
    },
    hda_pintbl { nid: 0, val: 0 },
];

static ALC262_FIXUP_BENQ_VERBS: [hda_verb; 3] = [
    hda_verb {
        nid: 0x20,
        verb: AC_VERB_SET_COEF_INDEX,
        param: 0x07,
    },
    hda_verb {
        nid: 0x20,
        verb: AC_VERB_SET_PROC_COEF,
        param: 0x3070,
    },
    hda_verb {
        nid: 0,
        verb: 0,
        param: 0,
    },
];

static ALC262_FIXUP_BENQ_T31_VERBS: [hda_verb; 3] = [
    hda_verb {
        nid: 0x20,
        verb: AC_VERB_SET_COEF_INDEX,
        param: 0x07,
    },
    hda_verb {
        nid: 0x20,
        verb: AC_VERB_SET_PROC_COEF,
        param: 0x3050,
    },
    hda_verb {
        nid: 0,
        verb: 0,
        param: 0,
    },
];

static ALC262_FIXUPS: [hda_fixup; 9] = [
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup__bindgen_ty_1 {
            pins: ALC262_FIXUP_FSC_H270_PINS.as_ptr(),
        },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup__bindgen_ty_1 {
            pins: ALC262_FIXUP_FSC_S7110_PINS.as_ptr(),
        },
        chained: true,
        chain_id: ALC262_FIXUP_BENQ,
    },
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup__bindgen_ty_1 {
            pins: ALC262_FIXUP_HP_Z200_PINS.as_ptr(),
        },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup__bindgen_ty_1 {
            pins: ALC262_FIXUP_TYAN_PINS.as_ptr(),
        },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_PINCTLS,
        v: hda_fixup__bindgen_ty_1 {
            pins: ALC262_FIXUP_LENOVO_3000_PINS.as_ptr(),
        },
        chained: true,
        chain_id: ALC262_FIXUP_BENQ,
    },
    hda_fixup {
        type_: HDA_FIXUP_VERBS,
        v: hda_fixup__bindgen_ty_1 {
            verbs: ALC262_FIXUP_BENQ_VERBS.as_ptr(),
        },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_VERBS,
        v: hda_fixup__bindgen_ty_1 {
            verbs: ALC262_FIXUP_BENQ_T31_VERBS.as_ptr(),
        },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup__bindgen_ty_1 {
            func: Some(alc_fixup_inv_dmic),
        },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup__bindgen_ty_1 {
            func: Some(alc_fixup_no_depop_delay),
        },
        chained: false,
        chain_id: 0,
    },
];

static ALC262_FIXUP_TBL: [hda_quirk; 11] = [
    SND_PCI_QUIRK(0x103c, 0x170b, c"HP Z200".as_ptr(), ALC262_FIXUP_HP_Z200),
    SND_PCI_QUIRK(
        0x10cf,
        0x1397,
        c"Fujitsu Lifebook S7110".as_ptr(),
        ALC262_FIXUP_FSC_S7110,
    ),
    SND_PCI_QUIRK(
        0x10cf,
        0x142d,
        c"Fujitsu Lifebook E8410".as_ptr(),
        ALC262_FIXUP_BENQ,
    ),
    SND_PCI_QUIRK(
        0x10f1,
        0x2915,
        c"Tyan Thunder n6650W".as_ptr(),
        ALC262_FIXUP_TYAN,
    ),
    SND_PCI_QUIRK(
        0x1734,
        0x1141,
        c"FSC ESPRIMO U9210".as_ptr(),
        ALC262_FIXUP_FSC_H270,
    ),
    SND_PCI_QUIRK(
        0x1734,
        0x1147,
        c"FSC Celsius H270".as_ptr(),
        ALC262_FIXUP_FSC_H270,
    ),
    SND_PCI_QUIRK(
        0x17aa,
        0x384e,
        c"Lenovo 3000".as_ptr(),
        ALC262_FIXUP_LENOVO_3000,
    ),
    SND_PCI_QUIRK(0x17ff, 0x0560, c"Benq ED8".as_ptr(), ALC262_FIXUP_BENQ),
    SND_PCI_QUIRK(
        0x17ff,
        0x058d,
        c"Benq T31-16".as_ptr(),
        ALC262_FIXUP_BENQ_T31,
    ),
    SND_PCI_QUIRK(
        0x8086,
        0x7270,
        c"BayleyBay".as_ptr(),
        ALC262_FIXUP_INTEL_BAYLEYBAY,
    ),
    hda_quirk::default(),
];

static ALC262_FIXUP_MODELS: [hda_model_fixup; 10] = [
    hda_model_fixup {
        id: ALC262_FIXUP_INV_DMIC,
        name: c"inv-dmic".as_ptr(),
    },
    hda_model_fixup {
        id: ALC262_FIXUP_FSC_H270,
        name: c"fsc-h270".as_ptr(),
    },
    hda_model_fixup {
        id: ALC262_FIXUP_FSC_S7110,
        name: c"fsc-s7110".as_ptr(),
    },
    hda_model_fixup {
        id: ALC262_FIXUP_HP_Z200,
        name: c"hp-z200".as_ptr(),
    },
    hda_model_fixup {
        id: ALC262_FIXUP_TYAN,
        name: c"tyan".as_ptr(),
    },
    hda_model_fixup {
        id: ALC262_FIXUP_LENOVO_3000,
        name: c"lenovo-3000".as_ptr(),
    },
    hda_model_fixup {
        id: ALC262_FIXUP_BENQ,
        name: c"benq".as_ptr(),
    },
    hda_model_fixup {
        id: ALC262_FIXUP_BENQ_T31,
        name: c"benq-t31".as_ptr(),
    },
    hda_model_fixup {
        id: ALC262_FIXUP_INTEL_BAYLEYBAY,
        name: c"bayleybay".as_ptr(),
    },
    hda_model_fixup {
        id: 0,
        name: core::ptr::null(),
    },
];

/*
 */
unsafe fn alc262_probe(codec: *mut hda_codec, id: *const hda_device_id) -> c_int {
    let mut spec: *mut alc_spec;
    let mut err: c_int;

    err = unsafe { alc_alloc_spec(codec, 0x0b) };
    if err < 0 {
        return err;
    }

    spec = unsafe { (*codec).spec as *mut alc_spec };
    unsafe {
        (*spec).gen.shared_mic_vref_pin = 0x18;

        (*spec).shutup = Some(alc_eapd_shutup);
    }

    /*
     * Original C disabled block:
     *
     * pshou 07/11/05  set a zero PCM sample to DAC when FIFO is
     * under-run
     *
     * alc_update_coefex_idx(codec, 0x1a, 7, 0, 0x80);
     */
    unsafe {
        alc_fix_pll_init(codec, 0x20, 0x0a, 10);

        alc_pre_init(codec);

        snd_hda_pick_fixup(
            codec,
            ALC262_FIXUP_MODELS.as_ptr(),
            ALC262_FIXUP_TBL.as_ptr(),
            ALC262_FIXUPS.as_ptr(),
        );
        snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

        alc_auto_parse_customize_define(codec);
    }

    if unsafe { has_cdefine_beep(codec) } {
        unsafe {
            (*spec).gen.beep_nid = 0x01;
        }
    }

    /* automatic parse from the BIOS config */
    err = unsafe { alc262_parse_auto_config(codec) };
    if err < 0 {
        unsafe {
            snd_hda_gen_remove(codec);
        }
        return err;
    }

    if unsafe { !(*spec).gen.no_analog && (*spec).gen.beep_nid != 0 } {
        err = unsafe { set_beep_amp(spec, 0x0b, 0x05, HDA_INPUT) };
        if err < 0 {
            unsafe {
                snd_hda_gen_remove(codec);
            }
            return err;
        }
    }

    unsafe {
        snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);
    }

    0
}

static ALC262_CODEC_OPS: hda_codec_ops = hda_codec_ops {
    probe: Some(alc262_probe),
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
static SND_HDA_ID_ALC262: [hda_device_id; 2] = [
    HDA_CODEC_ID(0x10ec0262, c"ALC262".as_ptr()),
    hda_device_id::default(), /* terminator */
];

// MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_alc262);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Realtek ALC262 HD-audio codec");
// MODULE_IMPORT_NS("SND_HDA_CODEC_REALTEK");

static mut ALC262_DRIVER: hda_codec_driver = hda_codec_driver {
    id: SND_HDA_ID_ALC262.as_ptr(),
    ops: &ALC262_CODEC_OPS,
};

module_hda_codec_driver!(ALC262_DRIVER);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
