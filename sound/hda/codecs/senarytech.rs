// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * HD audio codec driver for Senary HDA audio codec
 *
 * Initially based on conexant.c
 */

// C includes translated as dependency intent:
// linux/init.h, linux/delay.h, linux/slab.h, linux/module.h
// sound/core.h, sound/jack.h, sound/hda_codec.h
// hda_local.h, hda_auto_parser.h, hda_beep.h, hda_jack.h, generic.h

#[repr(C)]
pub struct senary_spec {
    pub gen: hda_gen_spec,

    /* extra EAPD pins */
    pub num_eapds: ::core::ffi::c_uint,
    pub eapds: [hda_nid_t; 4],
    pub dynamic_eapd: bool,
    pub mute_led_eapd: hda_nid_t,

    pub parse_flags: ::core::ffi::c_uint, /* flag for snd_hda_parse_pin_defcfg() */

    pub mute_led_polarity: ::core::ffi::c_int,
    pub gpio_led: ::core::ffi::c_uint,
    pub gpio_mute_led_mask: ::core::ffi::c_uint,
    pub gpio_mic_led_mask: ::core::ffi::c_uint,
}

pub const SENARY_FIXUP_PINCFG_DEFAULT: ::core::ffi::c_int = 0;

pub static senary_pincfg_default: [hda_pintbl; 7] = [
    hda_pintbl {
        nid: 0x16,
        val: 0x02211020,
    }, /* Headphone */
    hda_pintbl {
        nid: 0x17,
        val: 0x40f001f0,
    }, /* Not used */
    hda_pintbl {
        nid: 0x18,
        val: 0x05a1904d,
    }, /* Mic */
    hda_pintbl {
        nid: 0x19,
        val: 0x02a1104e,
    }, /* Headset Mic */
    hda_pintbl {
        nid: 0x1a,
        val: 0x01819030,
    }, /* Line-in */
    hda_pintbl {
        nid: 0x1d,
        val: 0x01014010,
    }, /* Line-out */
    hda_pintbl {
        nid: 0,
        val: 0,
    },
];

pub static senary_fixups: [hda_fixup; 1] = [hda_fixup {
    type_: HDA_FIXUP_PINS,
    v: hda_fixup__bindgen_ty_1 {
        pins: senary_pincfg_default.as_ptr(),
    },
}];

/* Quirk table for specific machines can be added here */
pub static sn6186_fixups: [hda_quirk; 1] = [hda_quirk {
    codec: 0,
    subvendor: 0,
    name: 0 as *const ::core::ffi::c_char,
    value: 0,
}];

// CONFIG_SND_HDA_INPUT_BEEP conditional support.
#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
/* additional beep mixers; private_value will be overwritten */
pub static senary_beep_mixer: [snd_kcontrol_new; 2] = [
    HDA_CODEC_VOLUME_MONO!(
        "Beep Playback Volume",
        0,
        1,
        0,
        HDA_OUTPUT
    ),
    HDA_CODEC_MUTE_BEEP_MONO!(
        "Beep Playback Switch",
        0,
        1,
        0,
        HDA_OUTPUT
    ),
];

#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
pub unsafe fn set_beep_amp(
    spec: *mut senary_spec,
    nid: hda_nid_t,
    idx: ::core::ffi::c_int,
    dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut knew: *mut snd_kcontrol_new;
    let beep_amp: ::core::ffi::c_uint = HDA_COMPOSE_AMP_VAL(nid, 1, idx, dir);
    let mut i: ::core::ffi::c_int = 0;

    while i < senary_beep_mixer.len() as ::core::ffi::c_int {
        knew = snd_hda_gen_add_kctl(
            &mut (*spec).gen,
            0 as *const ::core::ffi::c_char,
            &senary_beep_mixer[i as usize],
        );
        if knew.is_null() {
            return -ENOMEM;
        }
        (*knew).private_value = beep_amp as _;
        i += 1;
    }

    (*spec).gen.beep_nid = nid;
    0
}

#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
pub unsafe fn senary_auto_parse_beep(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let spec: *mut senary_spec = (*codec).spec as *mut senary_spec;
    let mut nid: hda_nid_t = 0;

    while nid < (*codec).core.afg + (*codec).core.mfg {
        if get_wcaps_type(get_wcaps(codec, nid)) == AC_WID_BEEP
            && (get_wcaps(codec, nid) & (AC_WCAP_OUT_AMP | AC_WCAP_AMP_OVRD)) != 0
        {
            return set_beep_amp(spec, nid, 0, HDA_OUTPUT);
        }
        nid += 1;
    }
    0
}

#[cfg(not(CONFIG_SND_HDA_INPUT_BEEP))]
pub unsafe fn senary_auto_parse_beep(_codec: *mut hda_codec) -> ::core::ffi::c_int {
    0
}

/* parse EAPDs */
pub unsafe fn senary_auto_parse_eapd(codec: *mut hda_codec) {
    let spec: *mut senary_spec = (*codec).spec as *mut senary_spec;
    let mut nid: hda_nid_t = 0;

    while nid < (*codec).core.afg + (*codec).core.mfg {
        if get_wcaps_type(get_wcaps(codec, nid)) != AC_WID_PIN {
            nid += 1;
            continue;
        }
        if (snd_hda_query_pin_caps(codec, nid) & AC_PINCAP_EAPD) == 0 {
            nid += 1;
            continue;
        }
        (*spec).eapds[(*spec).num_eapds as usize] = nid;
        (*spec).num_eapds += 1;
        if (*spec).num_eapds as usize >= (*spec).eapds.len() {
            break;
        }
        nid += 1;
    }
}

/* Hardware specific initialization verbs */
pub unsafe fn senary_init_verb(codec: *mut hda_codec) {
    /* Vendor specific init sequence */
    snd_hda_codec_write(codec, 0x1b, 0x0, 0x05a, 0xaa);
    snd_hda_codec_write(codec, 0x1b, 0x0, 0x059, 0x48);
    snd_hda_codec_write(codec, 0x1b, 0x0, 0x01b, 0x00);
    snd_hda_codec_write(codec, 0x1b, 0x0, 0x01c, 0x00);

    /* Override pin caps for headset mic */
    snd_hda_override_pin_caps(codec, 0x19, 0x2124);
}

pub unsafe fn senary_auto_turn_eapd(
    codec: *mut hda_codec,
    num_pins: ::core::ffi::c_int,
    pins: *const hda_nid_t,
    on: bool,
) {
    let mut i: ::core::ffi::c_int = 0;

    while i < num_pins {
        snd_hda_codec_write(
            codec,
            *pins.offset(i as isize),
            0,
            AC_VERB_SET_EAPD_BTLENABLE,
            if on { 0x02 } else { 0 },
        );
        i += 1;
    }
}

/* turn on/off EAPD according to Master switch */
pub unsafe extern "C" fn senary_auto_vmaster_hook(
    private_data: *mut ::core::ffi::c_void,
    enabled: ::core::ffi::c_int,
) {
    let codec: *mut hda_codec = private_data as *mut hda_codec;
    let spec: *mut senary_spec = (*codec).spec as *mut senary_spec;

    senary_auto_turn_eapd(
        codec,
        (*spec).num_eapds as ::core::ffi::c_int,
        (*spec).eapds.as_ptr(),
        enabled != 0,
    );
}

pub unsafe fn senary_init_gpio_led(codec: *mut hda_codec) {
    let spec: *mut senary_spec = (*codec).spec as *mut senary_spec;
    let mask: ::core::ffi::c_uint = (*spec).gpio_mute_led_mask | (*spec).gpio_mic_led_mask;

    if mask != 0 {
        snd_hda_codec_set_gpio(codec, mask, mask, (*spec).gpio_led, 0);
    }
}

pub unsafe extern "C" fn senary_init(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let spec: *mut senary_spec = (*codec).spec as *mut senary_spec;

    snd_hda_gen_init(codec);
    senary_init_gpio_led(codec);
    senary_init_verb(codec);
    if !(*spec).dynamic_eapd {
        senary_auto_turn_eapd(
            codec,
            (*spec).num_eapds as ::core::ffi::c_int,
            (*spec).eapds.as_ptr(),
            true,
        );
    }
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_INIT);

    0
}

pub unsafe fn senary_shutdown(codec: *mut hda_codec) {
    let spec: *mut senary_spec = (*codec).spec as *mut senary_spec;

    /* Turn the problematic codec into D3 to avoid spurious noises
     * from the internal speaker during (and after) reboot
     */
    senary_auto_turn_eapd(
        codec,
        (*spec).num_eapds as ::core::ffi::c_int,
        (*spec).eapds.as_ptr(),
        false,
    );
}

pub unsafe extern "C" fn senary_remove(codec: *mut hda_codec) {
    senary_shutdown(codec);
    snd_hda_gen_remove(codec);
}

pub unsafe extern "C" fn senary_suspend(codec: *mut hda_codec) -> ::core::ffi::c_int {
    senary_shutdown(codec);
    0
}

pub unsafe extern "C" fn senary_probe(
    codec: *mut hda_codec,
    _id: *const hda_device_id,
) -> ::core::ffi::c_int {
    let spec: *mut senary_spec;
    let mut err: ::core::ffi::c_int;

    codec_info!(
        codec,
        "%s: BIOS auto-probing.\n",
        (*codec).core.chip_name
    );

    spec = kzalloc_obj::<senary_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    snd_hda_gen_spec_init(&mut (*spec).gen);
    (*codec).spec = spec as *mut ::core::ffi::c_void;

    senary_auto_parse_eapd(codec);
    (*spec).gen.own_eapd_ctl = 1;

    /* Setup fixups based on codec vendor ID */
    match (*codec).core.vendor_id {
        0x1fa86186 => {
            (*codec).pin_amp_workaround = 1;
            (*spec).gen.mixer_nid = 0x15;
            snd_hda_pick_fixup(
                codec,
                0 as *const hda_model_fixup,
                sn6186_fixups.as_ptr(),
                senary_fixups.as_ptr(),
            );

            /* If no specific quirk found, apply the default pin configuration */
            if (*codec).fixup_id == HDA_FIXUP_ID_NOT_SET {
                (*codec).fixup_id = SENARY_FIXUP_PINCFG_DEFAULT;
            }
        }
        _ => {
            snd_hda_pick_fixup(
                codec,
                0 as *const hda_model_fixup,
                sn6186_fixups.as_ptr(),
                senary_fixups.as_ptr(),
            );
        }
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

    /* Run hardware init verbs once during probe */
    senary_init_verb(codec);

    if (*spec).gen.vmaster_mute.hook.is_none() {
        (*spec).gen.vmaster_mute.hook = Some(senary_auto_vmaster_hook);
    }

    err = snd_hda_parse_pin_defcfg(
        codec,
        &mut (*spec).gen.autocfg,
        0 as *const hda_pintbl,
        (*spec).parse_flags,
    );
    if err < 0 {
        senary_remove(codec);
        return err;
    }

    err = senary_auto_parse_beep(codec);
    if err < 0 {
        senary_remove(codec);
        return err;
    }

    err = snd_hda_gen_parse_auto_config(codec, &mut (*spec).gen.autocfg);
    if err < 0 {
        senary_remove(codec);
        return err;
    }

    /* Some laptops with Senary chips show stalls in S3 resume,
     * which falls into the single-cmd mode.
     * Better to make reset, then.
     */
    if (*(*codec).bus).core.sync_write == 0 {
        codec_info!(codec, "Enable sync_write for stable communication\n");
        (*(*codec).bus).core.sync_write = 1;
        (*(*codec).bus).allow_bus_reset = 1;
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);

    0
}

pub static senary_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(senary_probe),
    remove: Some(senary_remove),
    build_controls: Some(snd_hda_gen_build_controls),
    build_pcms: Some(snd_hda_gen_build_pcms),
    init: Some(senary_init),
    unsol_event: Some(snd_hda_jack_unsol_event),
    suspend: Some(senary_suspend),
    check_power_status: Some(snd_hda_gen_check_power_status),
    stream_pm: Some(snd_hda_gen_stream_pm),
};

/*
 */

pub static snd_hda_id_senary: [hda_device_id; 2] = [
    HDA_CODEC_ID!(0x1fa86186, "SN6186"),
    hda_device_id {
        vendor_id: 0,
        rev_id: 0,
        name: 0 as *const ::core::ffi::c_char,
        driver_data: 0,
    }, /* terminator */
];
MODULE_DEVICE_TABLE!(hdaudio, snd_hda_id_senary);

MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("Senarytech HD-audio codec");

pub static mut senary_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_senary.as_ptr(),
    ops: &senary_codec_ops,
};

module_hda_codec_driver!(senary_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
