// SPDX-License-Identifier: GPL-2.0-or-later
//
// Realtek ALC861 codec
//

use crate::*;

unsafe fn alc861_parse_auto_config(codec: *mut hda_codec) -> c_int {
    static ALC861_IGNORE: [hda_nid_t; 2] = [0x1d, 0];
    static ALC861_SSIDS: [hda_nid_t; 4] = [0x0e, 0x0f, 0x0b, 0];

    unsafe { alc_parse_auto_config(codec, ALC861_IGNORE.as_ptr(), ALC861_SSIDS.as_ptr()) }
}

/* Pin config fixes */
const ALC861_FIXUP_FSC_AMILO_PI1505: usize = 0;
const ALC861_FIXUP_AMP_VREF_0F: usize = 1;
const ALC861_FIXUP_NO_JACK_DETECT: usize = 2;
const ALC861_FIXUP_ASUS_A6RP: usize = 3;
const ALC660_FIXUP_ASUS_W7J: usize = 4;

/* On some laptops, VREF of pin 0x0f is abused for controlling the main amp */
unsafe extern "C" fn alc861_fixup_asus_amp_vref_0f(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: c_int,
) {
    let spec: *mut alc_spec = unsafe { (*codec).spec as *mut alc_spec };
    let mut val: c_uint;

    if action != HDA_FIXUP_ACT_INIT {
        return;
    }
    val = unsafe { snd_hda_codec_get_pin_target(codec, 0x0f) };
    if (val & (AC_PINCTL_IN_EN | AC_PINCTL_OUT_EN)) == 0 {
        val |= AC_PINCTL_IN_EN;
    }
    val |= AC_PINCTL_VREF_50;
    unsafe {
        snd_hda_set_pin_ctl(codec, 0x0f, val);
        (*spec).gen.keep_vref_in_automute = 1;
    }
}

static ALC861_FIXUP_FSC_AMILO_PI1505_PINS: [hda_pintbl; 3] = [
    hda_pintbl {
        nid: 0x0b,
        cfg: 0x0221101f,
    }, /* HP */
    hda_pintbl {
        nid: 0x0f,
        cfg: 0x90170310,
    }, /* speaker */
    hda_pintbl { nid: 0, cfg: 0 },
];

static ALC660_FIXUP_ASUS_W7J_VERBS: [hda_verb; 2] = [
    /*
     * ASUS W7J needs a magic pin setup on unused NID 0x10
     * for enabling outputs
     */
    hda_verb {
        nid: 0x10,
        verb: AC_VERB_SET_PIN_WIDGET_CONTROL,
        param: 0x24,
    },
    hda_verb {
        nid: 0,
        verb: 0,
        param: 0,
    },
];

static ALC861_FIXUPS: [hda_fixup; 5] = [
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup__bindgen_ty_1 {
            pins: ALC861_FIXUP_FSC_AMILO_PI1505_PINS.as_ptr(),
        },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup__bindgen_ty_1 {
            func: Some(alc861_fixup_asus_amp_vref_0f),
        },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup__bindgen_ty_1 {
            func: Some(alc_fixup_no_jack_detect),
        },
        chained: false,
        chain_id: 0,
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup__bindgen_ty_1 {
            func: Some(alc861_fixup_asus_amp_vref_0f),
        },
        chained: true,
        chain_id: ALC861_FIXUP_NO_JACK_DETECT as c_int,
    },
    hda_fixup {
        type_: HDA_FIXUP_VERBS,
        v: hda_fixup__bindgen_ty_1 {
            verbs: ALC660_FIXUP_ASUS_W7J_VERBS.as_ptr(),
        },
        chained: false,
        chain_id: 0,
    },
];

static ALC861_FIXUP_TBL: [hda_quirk; 8] = [
    SND_PCI_QUIRK!(0x1043, 0x1253, "ASUS W7J", ALC660_FIXUP_ASUS_W7J),
    SND_PCI_QUIRK!(0x1043, 0x1263, "ASUS Z35HL", ALC660_FIXUP_ASUS_W7J),
    SND_PCI_QUIRK!(0x1043, 0x1393, "ASUS A6Rp", ALC861_FIXUP_ASUS_A6RP),
    SND_PCI_QUIRK_VENDOR!(0x1043, "ASUS laptop", ALC861_FIXUP_AMP_VREF_0F),
    SND_PCI_QUIRK!(0x1462, 0x7254, "HP DX2200", ALC861_FIXUP_NO_JACK_DETECT),
    SND_PCI_QUIRK_VENDOR!(0x1584, "Haier/Uniwill", ALC861_FIXUP_AMP_VREF_0F),
    SND_PCI_QUIRK!(
        0x1734,
        0x10c7,
        "FSC Amilo Pi1505",
        ALC861_FIXUP_FSC_AMILO_PI1505
    ),
    hda_quirk::default(),
];

/*
 */
unsafe extern "C" fn alc861_probe(codec: *mut hda_codec, _id: *const hda_device_id) -> c_int {
    let spec: *mut alc_spec;
    let mut err: c_int;

    err = unsafe { alc_alloc_spec(codec, 0x15) };
    if err < 0 {
        return err;
    }

    spec = unsafe { (*codec).spec as *mut alc_spec };
    if unsafe { has_cdefine_beep(codec) } {
        unsafe {
            (*spec).gen.beep_nid = 0x23;
        }
    }

    unsafe {
        (*spec).power_hook = Some(alc_power_eapd);
    }

    unsafe {
        alc_pre_init(codec);
    }

    unsafe {
        snd_hda_pick_fixup(
            codec,
            core::ptr::null(),
            ALC861_FIXUP_TBL.as_ptr(),
            ALC861_FIXUPS.as_ptr(),
        );
        snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);
    }

    /* automatic parse from the BIOS config */
    err = unsafe { alc861_parse_auto_config(codec) };
    if err < 0 {
        unsafe {
            snd_hda_gen_remove(codec);
        }
        return err;
    }

    if unsafe { !(*spec).gen.no_analog } {
        err = unsafe { set_beep_amp(spec, 0x23, 0, HDA_OUTPUT) };
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

static ALC861_CODEC_OPS: hda_codec_ops = hda_codec_ops {
    probe: Some(alc861_probe),
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
static SND_HDA_ID_ALC861: [hda_device_id; 3] = [
    HDA_CODEC_ID_REV!(0x10ec0861, 0x100340, "ALC660"),
    HDA_CODEC_ID!(0x10ec0861, "ALC861"),
    hda_device_id::default(), /* terminator */
];

// MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_alc861);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Realtek ALC861 HD-audio codec");
// MODULE_IMPORT_NS("SND_HDA_CODEC_REALTEK");

static mut ALC861_DRIVER: hda_codec_driver = hda_codec_driver {
    id: SND_HDA_ID_ALC861.as_ptr(),
    ops: &ALC861_CODEC_OPS,
};

module_hda_codec_driver!(ALC861_DRIVER);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
