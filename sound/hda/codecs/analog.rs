// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * HD audio codec driver for AD1882, AD1884, AD1981HD, AD1983, AD1984,
 *   AD1986A, AD1988
 *
 * Copyright (c) 2005-2007 Takashi Iwai <tiwai@suse.de>
 */

// C includes translated as dependency intent:
// <linux/init.h>, <linux/slab.h>, <linux/module.h>
// <sound/core.h>, <sound/hda_codec.h>
// "hda_local.h", "hda_auto_parser.h", "hda_beep.h", "hda_jack.h", "generic.h"

const MODEL_AD1882: i32 = 0;
const MODEL_AD1884: i32 = 1;
const MODEL_AD1981: i32 = 2;
const MODEL_AD1983: i32 = 3;
const MODEL_AD1986A: i32 = 4;
const MODEL_AD1988: i32 = 5;

#[repr(C)]
pub struct ad198x_spec {
    pub gen: hda_gen_spec,
    pub model: ::core::ffi::c_int,

    /* for auto parser */
    pub smux_paths: [::core::ffi::c_int; 4],
    pub cur_smux: ::core::ffi::c_uint,
    pub eapd_nid: hda_nid_t,

    pub beep_amp: ::core::ffi::c_uint, /* beep amp value, set via set_beep_amp() */
    pub num_smux_conns: ::core::ffi::c_int,

    pub gpio_data: ::core::ffi::c_uint,
}

// CONFIG_SND_HDA_INPUT_BEEP: additional beep mixers; the actual parameters are overwritten at build.
#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
static ad_beep_mixer: [snd_kcontrol_new; 3] = [
    HDA_CODEC_VOLUME!("Beep Playback Volume", 0, 0, HDA_OUTPUT),
    HDA_CODEC_MUTE_BEEP!("Beep Playback Switch", 0, 0, HDA_OUTPUT),
    snd_kcontrol_new::default(), /* end */
];

#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
unsafe fn set_beep_amp(spec: *mut ad198x_spec, nid: hda_nid_t, idx: ::core::ffi::c_int, dir: ::core::ffi::c_int) {
    (*spec).beep_amp = HDA_COMPOSE_AMP_VAL(nid, 1, idx, dir); /* mono */
}

#[cfg(not(CONFIG_SND_HDA_INPUT_BEEP))]
unsafe fn set_beep_amp(_spec: *mut ad198x_spec, _nid: hda_nid_t, _idx: ::core::ffi::c_int, _dir: ::core::ffi::c_int) {
    /* NOP */
}

#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
unsafe fn create_beep_ctls(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let spec = (*codec).spec as *mut ad198x_spec;
    let mut knew = ad_beep_mixer.as_ptr();

    if (*spec).beep_amp == 0 {
        return 0;
    }

    while !(*knew).name.is_null() {
        let err: ::core::ffi::c_int;
        let kctl: *mut snd_kcontrol;
        kctl = snd_ctl_new1(knew, codec as *mut ::core::ffi::c_void);
        if kctl.is_null() {
            return -ENOMEM;
        }
        (*kctl).private_value = (*spec).beep_amp as _;
        err = snd_hda_ctl_add(codec, 0, kctl);
        if err < 0 {
            return err;
        }
        knew = knew.add(1);
    }
    0
}

#[cfg(not(CONFIG_SND_HDA_INPUT_BEEP))]
unsafe fn create_beep_ctls(_codec: *mut hda_codec) -> ::core::ffi::c_int {
    0
}

unsafe fn ad198x_power_eapd_write(codec: *mut hda_codec, front: hda_nid_t, hp: hda_nid_t) {
    if snd_hda_query_pin_caps(codec, front) & AC_PINCAP_EAPD != 0 {
        snd_hda_codec_write(codec, front, 0, AC_VERB_SET_EAPD_BTLENABLE,
                            if (*codec).inv_eapd == 0 { 0x00 } else { 0x02 });
    }
    if snd_hda_query_pin_caps(codec, hp) & AC_PINCAP_EAPD != 0 {
        snd_hda_codec_write(codec, hp, 0, AC_VERB_SET_EAPD_BTLENABLE,
                            if (*codec).inv_eapd == 0 { 0x00 } else { 0x02 });
    }
}

unsafe fn ad198x_power_eapd(codec: *mut hda_codec) {
    /* We currently only handle front, HP */
    match (*codec).core.vendor_id {
        0x11d41882 | 0x11d4882a | 0x11d41884 | 0x11d41984 | 0x11d41883 |
        0x11d4184a | 0x11d4194a | 0x11d4194b | 0x11d41988 | 0x11d4198b |
        0x11d4989a | 0x11d4989b => ad198x_power_eapd_write(codec, 0x12, 0x11),
        0x11d41981 | 0x11d41983 => ad198x_power_eapd_write(codec, 0x05, 0x06),
        0x11d41986 => ad198x_power_eapd_write(codec, 0x1b, 0x1a),
        _ => {}
    }
}

unsafe fn ad_codec_suspend(codec: *mut hda_codec) -> ::core::ffi::c_int {
    snd_hda_shutup_pins(codec);
    ad198x_power_eapd(codec);
    0
}

/* follow EAPD via vmaster hook */
unsafe extern "C" fn ad_vmaster_eapd_hook(private_data: *mut ::core::ffi::c_void, mut enabled: ::core::ffi::c_int) {
    let codec = private_data as *mut hda_codec;
    let spec = (*codec).spec as *mut ad198x_spec;

    if (*spec).eapd_nid == 0 {
        return;
    }
    if (*codec).inv_eapd != 0 {
        enabled = !enabled;
    }
    snd_hda_codec_write_cache(codec, (*spec).eapd_nid, 0,
                              AC_VERB_SET_EAPD_BTLENABLE,
                              if enabled != 0 { 0x02 } else { 0x00 });
}

/*
 * Automatic parse of I/O pins from the BIOS configuration
 */

unsafe fn ad_codec_build_controls(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int;

    err = snd_hda_gen_build_controls(codec);
    if err < 0 {
        return err;
    }
    err = create_beep_ctls(codec);
    if err < 0 {
        return err;
    }
    0
}

unsafe fn ad198x_parse_auto_config(codec: *mut hda_codec, indep_hp: bool) -> ::core::ffi::c_int {
    let spec = (*codec).spec as *mut ad198x_spec;
    let cfg = &mut (*spec).gen.autocfg as *mut auto_pin_cfg;
    let mut err: ::core::ffi::c_int;

    (*codec).spdif_status_reset = 1;
    (*codec).no_trigger_sense = 1;
    (*codec).no_sticky_stream = 1;

    (*spec).gen.indep_hp = indep_hp;
    if (*spec).gen.add_stereo_mix_input == 0 {
        (*spec).gen.add_stereo_mix_input = HDA_HINT_STEREO_MIX_AUTO;
    }

    err = snd_hda_parse_pin_defcfg(codec, cfg, ::core::ptr::null_mut(), 0);
    if err < 0 {
        return err;
    }
    err = snd_hda_gen_parse_auto_config(codec, cfg);
    if err < 0 {
        return err;
    }

    0
}

/*
 * AD1986A specific
 */

unsafe fn alloc_ad_spec(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let spec: *mut ad198x_spec;

    spec = kzalloc_obj::<ad198x_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    (*codec).spec = spec as *mut ::core::ffi::c_void;
    snd_hda_gen_spec_init(&mut (*spec).gen);
    0
}

/*
 * AD1986A fixup codes
 */

/* Lenovo N100 seems to report the reversed bit for HP jack-sensing */
unsafe extern "C" fn ad_fixup_inv_jack_detect(codec: *mut hda_codec,
                                             _fix: *const hda_fixup,
                                             action: ::core::ffi::c_int) {
    let spec = (*codec).spec as *mut ad198x_spec;

    if action == HDA_FIXUP_ACT_PRE_PROBE {
        (*codec).inv_jack_detect = 1;
        (*spec).gen.keep_eapd_on = 1;
        (*spec).gen.vmaster_mute.hook = Some(ad_vmaster_eapd_hook);
        (*spec).eapd_nid = 0x1b;
    }
}

/* Toshiba Satellite L40 implements EAPD in a standard way unlike others */
unsafe extern "C" fn ad1986a_fixup_eapd(codec: *mut hda_codec,
                                        _fix: *const hda_fixup,
                                        action: ::core::ffi::c_int) {
    let spec = (*codec).spec as *mut ad198x_spec;

    if action == HDA_FIXUP_ACT_PRE_PROBE {
        (*codec).inv_eapd = 0;
        (*spec).gen.keep_eapd_on = 1;
        (*spec).eapd_nid = 0x1b;
    }
}

/* enable stereo-mix input for avoiding regression on KDE (bko#88251) */
unsafe extern "C" fn ad1986a_fixup_eapd_mix_in(codec: *mut hda_codec,
                                               fix: *const hda_fixup,
                                               action: ::core::ffi::c_int) {
    let spec = (*codec).spec as *mut ad198x_spec;

    if action == HDA_FIXUP_ACT_PRE_PROBE {
        ad1986a_fixup_eapd(codec, fix, action);
        (*spec).gen.add_stereo_mix_input = HDA_HINT_STEREO_MIX_ENABLE;
    }
}

const AD1986A_FIXUP_INV_JACK_DETECT: i32 = 0;
const AD1986A_FIXUP_ULTRA: i32 = 1;
const AD1986A_FIXUP_SAMSUNG: i32 = 2;
const AD1986A_FIXUP_3STACK: i32 = 3;
const AD1986A_FIXUP_LAPTOP: i32 = 4;
const AD1986A_FIXUP_LAPTOP_IMIC: i32 = 5;
const AD1986A_FIXUP_EAPD: i32 = 6;
const AD1986A_FIXUP_EAPD_MIX_IN: i32 = 7;
const AD1986A_FIXUP_EASYNOTE: i32 = 8;

static ad1986a_fixups: &[hda_fixup] = &[
    hda_fixup_func!(AD1986A_FIXUP_INV_JACK_DETECT, ad_fixup_inv_jack_detect),
    hda_fixup_pins!(AD1986A_FIXUP_ULTRA, &[
        hda_pintbl { nid: 0x1b, val: 0x90170110 }, /* speaker */
        hda_pintbl { nid: 0x1d, val: 0x90a7013e }, /* int mic */
        hda_pintbl::default(),
    ]),
    hda_fixup_pins!(AD1986A_FIXUP_SAMSUNG, &[
        hda_pintbl { nid: 0x1b, val: 0x90170110 }, /* speaker */
        hda_pintbl { nid: 0x1d, val: 0x90a7013e }, /* int mic */
        hda_pintbl { nid: 0x20, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x24, val: 0x411111f0 }, /* N/A */
        hda_pintbl::default(),
    ]),
    hda_fixup_pins!(AD1986A_FIXUP_3STACK, &[
        hda_pintbl { nid: 0x1a, val: 0x02214021 }, /* headphone */
        hda_pintbl { nid: 0x1b, val: 0x01014011 }, /* front */
        hda_pintbl { nid: 0x1c, val: 0x01813030 }, /* line-in */
        hda_pintbl { nid: 0x1d, val: 0x01a19020 }, /* rear mic */
        hda_pintbl { nid: 0x1e, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x1f, val: 0x02a190f0 }, /* mic */
        hda_pintbl { nid: 0x20, val: 0x411111f0 }, /* N/A */
        hda_pintbl::default(),
    ]),
    hda_fixup_pins!(AD1986A_FIXUP_LAPTOP, &[
        hda_pintbl { nid: 0x1a, val: 0x02214021 }, /* headphone */
        hda_pintbl { nid: 0x1b, val: 0x90170110 }, /* speaker */
        hda_pintbl { nid: 0x1c, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x1d, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x1e, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x1f, val: 0x02a191f0 }, /* mic */
        hda_pintbl { nid: 0x20, val: 0x411111f0 }, /* N/A */
        hda_pintbl::default(),
    ]),
    hda_fixup_pins_chained_before!(AD1986A_FIXUP_LAPTOP_IMIC, &[
        hda_pintbl { nid: 0x1d, val: 0x90a7013e }, /* int mic */
        hda_pintbl::default(),
    ], AD1986A_FIXUP_LAPTOP),
    hda_fixup_func!(AD1986A_FIXUP_EAPD, ad1986a_fixup_eapd),
    hda_fixup_func!(AD1986A_FIXUP_EAPD_MIX_IN, ad1986a_fixup_eapd_mix_in),
    hda_fixup_pins_chained!(AD1986A_FIXUP_EASYNOTE, &[
        hda_pintbl { nid: 0x1a, val: 0x0421402f }, /* headphone */
        hda_pintbl { nid: 0x1b, val: 0x90170110 }, /* speaker */
        hda_pintbl { nid: 0x1c, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x1d, val: 0x90a70130 }, /* int mic */
        hda_pintbl { nid: 0x1e, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x1f, val: 0x04a19040 }, /* mic */
        hda_pintbl { nid: 0x20, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x21, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x22, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x23, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x24, val: 0x411111f0 }, /* N/A */
        hda_pintbl { nid: 0x25, val: 0x411111f0 }, /* N/A */
        hda_pintbl::default(),
    ], AD1986A_FIXUP_EAPD_MIX_IN),
];

static ad1986a_fixup_tbl: &[hda_quirk] = &[
    SND_PCI_QUIRK!(0x103c, 0x30af, "HP B2800", AD1986A_FIXUP_LAPTOP_IMIC),
    SND_PCI_QUIRK!(0x1043, 0x1153, "ASUS M9V", AD1986A_FIXUP_LAPTOP_IMIC),
    SND_PCI_QUIRK!(0x1043, 0x1443, "ASUS Z99He", AD1986A_FIXUP_EAPD),
    SND_PCI_QUIRK!(0x1043, 0x1447, "ASUS A8JN", AD1986A_FIXUP_EAPD),
    SND_PCI_QUIRK_MASK!(0x1043, 0xff00, 0x8100, "ASUS P5", AD1986A_FIXUP_3STACK),
    SND_PCI_QUIRK_MASK!(0x1043, 0xff00, 0x8200, "ASUS M2", AD1986A_FIXUP_3STACK),
    SND_PCI_QUIRK!(0x10de, 0xcb84, "ASUS A8N-VM", AD1986A_FIXUP_3STACK),
    SND_PCI_QUIRK!(0x1179, 0xff40, "Toshiba Satellite L40", AD1986A_FIXUP_EAPD),
    SND_PCI_QUIRK!(0x144d, 0xc01e, "FSC V2060", AD1986A_FIXUP_LAPTOP),
    SND_PCI_QUIRK_MASK!(0x144d, 0xff00, 0xc000, "Samsung", AD1986A_FIXUP_SAMSUNG),
    SND_PCI_QUIRK!(0x144d, 0xc027, "Samsung Q1", AD1986A_FIXUP_ULTRA),
    SND_PCI_QUIRK!(0x1631, 0xc022, "PackardBell EasyNote MX65", AD1986A_FIXUP_EASYNOTE),
    SND_PCI_QUIRK!(0x17aa, 0x2066, "Lenovo N100", AD1986A_FIXUP_INV_JACK_DETECT),
    SND_PCI_QUIRK!(0x17aa, 0x1011, "Lenovo M55", AD1986A_FIXUP_3STACK),
    SND_PCI_QUIRK!(0x17aa, 0x1017, "Lenovo A60", AD1986A_FIXUP_3STACK),
    hda_quirk::default(),
];

static ad1986a_fixup_models: &[hda_model_fixup] = &[
    hda_model_fixup { id: AD1986A_FIXUP_3STACK, name: c"3stack".as_ptr() },
    hda_model_fixup { id: AD1986A_FIXUP_LAPTOP, name: c"laptop".as_ptr() },
    hda_model_fixup { id: AD1986A_FIXUP_LAPTOP_IMIC, name: c"laptop-imic".as_ptr() },
    hda_model_fixup { id: AD1986A_FIXUP_LAPTOP_IMIC, name: c"laptop-eapd".as_ptr() }, /* alias */
    hda_model_fixup { id: AD1986A_FIXUP_EAPD, name: c"eapd".as_ptr() },
    hda_model_fixup::default(),
];

/*
 */
unsafe fn ad1986a_probe(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int;
    let spec = (*codec).spec as *mut ad198x_spec;
    static preferred_pairs: [hda_nid_t; 11] = [
        0x1a, 0x03,
        0x1b, 0x03,
        0x1c, 0x04,
        0x1d, 0x05,
        0x1e, 0x03,
        0,
    ];

    /* AD1986A has the inverted EAPD implementation */
    (*codec).inv_eapd = 1;

    (*spec).gen.mixer_nid = 0x07;
    (*spec).gen.beep_nid = 0x19;
    set_beep_amp(spec, 0x18, 0, HDA_OUTPUT);

    /* AD1986A has a hardware problem that it can't share a stream
     * with multiple output pins.  The copy of front to surrounds
     * causes noisy or silent outputs at a certain timing, e.g.
     * changing the volume.
     * So, let's disable the shared stream.
     */
    (*spec).gen.multiout.no_share_stream = 1;
    /* give fixed DAC/pin pairs */
    (*spec).gen.preferred_dacs = preferred_pairs.as_ptr();

    /* AD1986A can't manage the dynamic pin on/off smoothly */
    (*spec).gen.auto_mute_via_amp = 1;

    snd_hda_pick_fixup(codec, ad1986a_fixup_models.as_ptr(), ad1986a_fixup_tbl.as_ptr(),
                       ad1986a_fixups.as_ptr());
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

    err = ad198x_parse_auto_config(codec, false);
    if err < 0 {
        return err;
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);

    0
}

/*
 * AD1983 specific
 */

/*
 * SPDIF mux control for AD1983 auto-parser
 */
unsafe extern "C" fn ad1983_auto_smux_enum_info(kcontrol: *mut snd_kcontrol,
                                                uinfo: *mut snd_ctl_elem_info) -> ::core::ffi::c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut ad198x_spec;
    static texts2: [*const ::core::ffi::c_char; 2] = [c"PCM".as_ptr(), c"ADC".as_ptr()];
    static texts3: [*const ::core::ffi::c_char; 3] = [c"PCM".as_ptr(), c"ADC1".as_ptr(), c"ADC2".as_ptr()];
    let num_conns = (*spec).num_smux_conns;

    if num_conns == 2 {
        snd_hda_enum_helper_info(kcontrol, uinfo, 2, texts2.as_ptr())
    } else if num_conns == 3 {
        snd_hda_enum_helper_info(kcontrol, uinfo, 3, texts3.as_ptr())
    } else {
        -EINVAL
    }
}

unsafe extern "C" fn ad1983_auto_smux_enum_get(kcontrol: *mut snd_kcontrol,
                                               ucontrol: *mut snd_ctl_elem_value) -> ::core::ffi::c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut ad198x_spec;

    (*ucontrol).value.enumerated.item[0] = (*spec).cur_smux;
    0
}

unsafe extern "C" fn ad1983_auto_smux_enum_put(kcontrol: *mut snd_kcontrol,
                                               ucontrol: *mut snd_ctl_elem_value) -> ::core::ffi::c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut ad198x_spec;
    let val = (*ucontrol).value.enumerated.item[0];
    let dig_out = (*spec).gen.multiout.dig_out_nid;
    let num_conns = (*spec).num_smux_conns;

    if val >= num_conns as ::core::ffi::c_uint {
        return -EINVAL;
    }
    if (*spec).cur_smux == val {
        return 0;
    }
    (*spec).cur_smux = val;
    snd_hda_codec_write_cache(codec, dig_out, 0,
                              AC_VERB_SET_CONNECT_SEL, val);
    1
}

static ad1983_auto_smux_mixer: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"IEC958 Playback Source".as_ptr(),
    info: Some(ad1983_auto_smux_enum_info),
    get: Some(ad1983_auto_smux_enum_get),
    put: Some(ad1983_auto_smux_enum_put),
    ..snd_kcontrol_new::default()
};

unsafe fn ad1983_add_spdif_mux_ctl(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let spec = (*codec).spec as *mut ad198x_spec;
    let dig_out = (*spec).gen.multiout.dig_out_nid;
    let num_conns: ::core::ffi::c_int;

    if dig_out == 0 {
        return 0;
    }
    num_conns = snd_hda_get_num_conns(codec, dig_out);
    if num_conns != 2 && num_conns != 3 {
        return 0;
    }
    (*spec).num_smux_conns = num_conns;
    if snd_hda_gen_add_kctl(&mut (*spec).gen, ::core::ptr::null(), &ad1983_auto_smux_mixer).is_null() {
        return -ENOMEM;
    }
    0
}

unsafe fn ad1983_probe(codec: *mut hda_codec) -> ::core::ffi::c_int {
    static conn_0c: [hda_nid_t; 1] = [0x08];
    static conn_0d: [hda_nid_t; 1] = [0x09];
    let spec = (*codec).spec as *mut ad198x_spec;
    let mut err: ::core::ffi::c_int;

    (*spec).gen.mixer_nid = 0x0e;
    (*spec).gen.beep_nid = 0x10;
    set_beep_amp(spec, 0x10, 0, HDA_OUTPUT);

    /* limit the loopback routes not to confuse the parser */
    snd_hda_override_conn_list(codec, 0x0c, ARRAY_SIZE(&conn_0c), conn_0c.as_ptr());
    snd_hda_override_conn_list(codec, 0x0d, ARRAY_SIZE(&conn_0d), conn_0d.as_ptr());

    err = ad198x_parse_auto_config(codec, false);
    if err < 0 {
        return err;
    }
    err = ad1983_add_spdif_mux_ctl(codec);
    if err < 0 {
        return err;
    }
    0
}

/*
 * AD1981 HD specific
 */

unsafe extern "C" fn ad1981_fixup_hp_eapd(codec: *mut hda_codec,
                                          _fix: *const hda_fixup,
                                          action: ::core::ffi::c_int) {
    let spec = (*codec).spec as *mut ad198x_spec;

    if action == HDA_FIXUP_ACT_PRE_PROBE {
        (*spec).gen.vmaster_mute.hook = Some(ad_vmaster_eapd_hook);
        (*spec).eapd_nid = 0x05;
    }
}

/* set the upper-limit for mixer amp to 0dB for avoiding the possible
 * damage by overloading
 */
unsafe extern "C" fn ad1981_fixup_amp_override(codec: *mut hda_codec,
                                               _fix: *const hda_fixup,
                                               action: ::core::ffi::c_int) {
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        snd_hda_override_amp_caps(codec, 0x11, HDA_INPUT,
                                  (0x17 << AC_AMPCAP_OFFSET_SHIFT) |
                                  (0x17 << AC_AMPCAP_NUM_STEPS_SHIFT) |
                                  (0x05 << AC_AMPCAP_STEP_SIZE_SHIFT) |
                                  (1 << AC_AMPCAP_MUTE_SHIFT));
    }
}

const AD1981_FIXUP_AMP_OVERRIDE: i32 = 0;
const AD1981_FIXUP_HP_EAPD: i32 = 1;

static ad1981_fixups: &[hda_fixup] = &[
    hda_fixup_func!(AD1981_FIXUP_AMP_OVERRIDE, ad1981_fixup_amp_override),
    hda_fixup_func_chained!(AD1981_FIXUP_HP_EAPD, ad1981_fixup_hp_eapd, AD1981_FIXUP_AMP_OVERRIDE),
];

static ad1981_fixup_tbl: &[hda_quirk] = &[
    SND_PCI_QUIRK_VENDOR!(0x1014, "Lenovo", AD1981_FIXUP_AMP_OVERRIDE),
    SND_PCI_QUIRK_VENDOR!(0x103c, "HP", AD1981_FIXUP_HP_EAPD),
    SND_PCI_QUIRK_VENDOR!(0x17aa, "Lenovo", AD1981_FIXUP_AMP_OVERRIDE),
    /* HP nx6320 (reversed SSID, H/W bug) */
    SND_PCI_QUIRK!(0x30b0, 0x103c, "HP nx6320", AD1981_FIXUP_HP_EAPD),
    hda_quirk::default(),
];

unsafe fn ad1981_probe(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let spec = (*codec).spec as *mut ad198x_spec;
    let mut err: ::core::ffi::c_int;

    (*spec).gen.mixer_nid = 0x0e;
    (*spec).gen.beep_nid = 0x10;
    set_beep_amp(spec, 0x0d, 0, HDA_OUTPUT);

    snd_hda_pick_fixup(codec, ::core::ptr::null(), ad1981_fixup_tbl.as_ptr(), ad1981_fixups.as_ptr());
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

    err = ad198x_parse_auto_config(codec, false);
    if err < 0 {
        return err;
    }
    err = ad1983_add_spdif_mux_ctl(codec);
    if err < 0 {
        return err;
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);

    0
}

/*
 * AD1988
 *
 * Output pins and routes
 *
 *        Pin               Mix     Sel     DAC (*)
 * port-A 0x11 (mute/hp) <- 0x22 <- 0x37 <- 03/04/06
 * port-B 0x14 (mute/hp) <- 0x2b <- 0x30 <- 03/04/06
 * port-C 0x15 (mute)    <- 0x2c <- 0x31 <- 05/0a
 * port-D 0x12 (mute/hp) <- 0x29         <- 04
 * port-E 0x17 (mute/hp) <- 0x26 <- 0x32 <- 05/0a
 * port-F 0x16 (mute)    <- 0x2a         <- 06
 * port-G 0x24 (mute)    <- 0x27         <- 05
 * port-H 0x25 (mute)    <- 0x28         <- 0a
 * mono   0x13 (mute/amp)<- 0x1e <- 0x36 <- 03/04/06
 *
 * DAC0 = 03h, DAC1 = 04h, DAC2 = 05h, DAC3 = 06h, DAC4 = 0ah
 * (*) DAC2/3/4 are swapped to DAC3/4/2 on AD198A rev.2 due to a h/w bug.
 *
 * Input pins and routes
 *
 *        pin     boost   mix input # / adc input #
 * port-A 0x11 -> 0x38 -> mix 2, ADC 0
 * port-B 0x14 -> 0x39 -> mix 0, ADC 1
 * port-C 0x15 -> 0x3a -> 33:0 - mix 1, ADC 2
 * port-D 0x12 -> 0x3d -> mix 3, ADC 8
 * port-E 0x17 -> 0x3c -> 34:0 - mix 4, ADC 4
 * port-F 0x16 -> 0x3b -> mix 5, ADC 3
 * port-G 0x24 -> N/A  -> 33:1 - mix 1, 34:1 - mix 4, ADC 6
 * port-H 0x25 -> N/A  -> 33:2 - mix 1, 34:2 - mix 4, ADC 7
 *
 *
 * DAC assignment
 *   6stack - front/surr/CLFE/side/opt DACs - 04/06/05/0a/03
 *   3stack - front/surr/CLFE/opt DACs - 04/05/0a/03
 *
 * Inputs of Analog Mix (0x20)
 *   0:Port-B (front mic)
 *   1:Port-C/G/H (line-in)
 *   2:Port-A
 *   3:Port-D (line-in/2)
 *   4:Port-E/G/H (mic-in)
 *   5:Port-F (mic2-in)
 *   6:CD
 *   7:Beep
 *
 * ADC selection
 *   0:Port-A
 *   1:Port-B (front mic-in)
 *   2:Port-C (line-in)
 *   3:Port-F (mic2-in)
 *   4:Port-E (mic-in)
 *   5:CD
 *   6:Port-G
 *   7:Port-H
 *   8:Port-D (line-in/2)
 *   9:Mix
 *
 * Proposed pin assignments by the datasheet
 *
 * 6-stack
 * Port-A front headphone
 *      B front mic-in
 *      C rear line-in
 *      D rear front-out
 *      E rear mic-in
 *      F rear surround
 *      G rear CLFE
 *      H rear side
 *
 * 3-stack
 * Port-A front headphone
 *      B front mic
 *      C rear line-in/surround
 *      D rear front-out
 *      E rear mic-in/CLFE
 *
 * laptop
 * Port-A headphone
 *      B mic-in
 *      C docking station
 *      D internal speaker (with EAPD)
 *      E/F quad mic array
 */

unsafe extern "C" fn ad1988_auto_smux_enum_info(kcontrol: *mut snd_kcontrol,
                                                uinfo: *mut snd_ctl_elem_info) -> ::core::ffi::c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut ad198x_spec;
    static texts: [*const ::core::ffi::c_char; 4] = [
        c"PCM".as_ptr(), c"ADC1".as_ptr(), c"ADC2".as_ptr(), c"ADC3".as_ptr(),
    ];
    let mut num_conns = (*spec).num_smux_conns;

    if num_conns > 4 {
        num_conns = 4;
    }
    snd_hda_enum_helper_info(kcontrol, uinfo, num_conns, texts.as_ptr())
}

unsafe extern "C" fn ad1988_auto_smux_enum_get(kcontrol: *mut snd_kcontrol,
                                               ucontrol: *mut snd_ctl_elem_value) -> ::core::ffi::c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut ad198x_spec;

    (*ucontrol).value.enumerated.item[0] = (*spec).cur_smux;
    0
}

unsafe extern "C" fn ad1988_auto_smux_enum_put(kcontrol: *mut snd_kcontrol,
                                               ucontrol: *mut snd_ctl_elem_value) -> ::core::ffi::c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut ad198x_spec;
    let val = (*ucontrol).value.enumerated.item[0];
    let mut path: *mut nid_path;
    let num_conns = (*spec).num_smux_conns;

    if val >= num_conns as ::core::ffi::c_uint {
        return -EINVAL;
    }
    if (*spec).cur_smux == val {
        return 0;
    }

    guard_mutex!(&mut (*codec).control_mutex);
    path = snd_hda_get_path_from_idx(codec, (*spec).smux_paths[(*spec).cur_smux as usize]);
    if !path.is_null() {
        snd_hda_activate_path(codec, path, false, true);
    }
    path = snd_hda_get_path_from_idx(codec, (*spec).smux_paths[val as usize]);
    if !path.is_null() {
        snd_hda_activate_path(codec, path, true, true);
    }
    (*spec).cur_smux = val;
    1
}

static ad1988_auto_smux_mixer: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"IEC958 Playback Source".as_ptr(),
    info: Some(ad1988_auto_smux_enum_info),
    get: Some(ad1988_auto_smux_enum_get),
    put: Some(ad1988_auto_smux_enum_put),
    ..snd_kcontrol_new::default()
};

unsafe fn ad_codec_init(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let spec = (*codec).spec as *mut ad198x_spec;
    let mut i: ::core::ffi::c_int;
    let err: ::core::ffi::c_int;

    err = snd_hda_gen_init(codec);
    if err < 0 {
        return err;
    }
    if (*spec).model != MODEL_AD1988 {
        return 0;
    }
    if (*spec).gen.autocfg.dig_outs == 0 {
        return 0;
    }

    i = 0;
    while i < 4 {
        let path: *mut nid_path;
        path = snd_hda_get_path_from_idx(codec, (*spec).smux_paths[i as usize]);
        if !path.is_null() {
            snd_hda_activate_path(codec, path, (*path).active != 0, false);
        }
        i += 1;
    }

    0
}

unsafe fn ad1988_add_spdif_mux_ctl(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let spec = (*codec).spec as *mut ad198x_spec;
    let mut i: ::core::ffi::c_int;
    let num_conns: ::core::ffi::c_int;
    /* we create four static faked paths, since AD codecs have odd
     * widget connections regarding the SPDIF out source
     */
    static fake_paths: [nid_path; 4] = [
        nid_path { depth: 3, path: [0x02, 0x1d, 0x1b, 0, 0, 0, 0, 0, 0, 0, 0, 0], idx: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], multi: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ..nid_path::default() },
        nid_path { depth: 4, path: [0x08, 0x0b, 0x1d, 0x1b, 0, 0, 0, 0, 0, 0, 0, 0], idx: [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], multi: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ..nid_path::default() },
        nid_path { depth: 4, path: [0x09, 0x0b, 0x1d, 0x1b, 0, 0, 0, 0, 0, 0, 0, 0], idx: [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], multi: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ..nid_path::default() },
        nid_path { depth: 4, path: [0x0f, 0x0b, 0x1d, 0x1b, 0, 0, 0, 0, 0, 0, 0, 0], idx: [0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], multi: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ..nid_path::default() },
    ];

    /* SPDIF source mux appears to be present only on AD1988A */
    if (*spec).gen.autocfg.dig_outs == 0 ||
        get_wcaps_type(get_wcaps(codec, 0x1d)) != AC_WID_AUD_MIX {
        return 0;
    }

    num_conns = snd_hda_get_num_conns(codec, 0x0b) + 1;
    if num_conns != 3 && num_conns != 4 {
        return 0;
    }
    (*spec).num_smux_conns = num_conns;

    i = 0;
    while i < num_conns {
        let path = snd_array_new(&mut (*spec).gen.paths) as *mut nid_path;
        if path.is_null() {
            return -ENOMEM;
        }
        *path = fake_paths[i as usize];
        if i == 0 {
            (*path).active = 1;
        }
        (*spec).smux_paths[i as usize] = snd_hda_get_path_idx(codec, path);
        i += 1;
    }

    if snd_hda_gen_add_kctl(&mut (*spec).gen, ::core::ptr::null(), &ad1988_auto_smux_mixer).is_null() {
        return -ENOMEM;
    }

    0
}

/*
 */

const AD1988_FIXUP_6STACK_DIG: i32 = 0;

static ad1988_fixups: &[hda_fixup] = &[
    hda_fixup_pins!(AD1988_FIXUP_6STACK_DIG, &[
        hda_pintbl { nid: 0x11, val: 0x02214130 }, /* front-hp */
        hda_pintbl { nid: 0x12, val: 0x01014010 }, /* line-out */
        hda_pintbl { nid: 0x14, val: 0x02a19122 }, /* front-mic */
        hda_pintbl { nid: 0x15, val: 0x01813021 }, /* line-in */
        hda_pintbl { nid: 0x16, val: 0x01011012 }, /* line-out */
        hda_pintbl { nid: 0x17, val: 0x01a19020 }, /* mic */
        hda_pintbl { nid: 0x1b, val: 0x0145f1f0 }, /* SPDIF */
        hda_pintbl { nid: 0x24, val: 0x01016011 }, /* line-out */
        hda_pintbl { nid: 0x25, val: 0x01012013 }, /* line-out */
        hda_pintbl::default(),
    ]),
];

static ad1988_fixup_models: &[hda_model_fixup] = &[
    hda_model_fixup { id: AD1988_FIXUP_6STACK_DIG, name: c"6stack-dig".as_ptr() },
    hda_model_fixup::default(),
];

unsafe fn ad1988_probe(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let spec = (*codec).spec as *mut ad198x_spec;
    let mut err: ::core::ffi::c_int;

    (*spec).gen.mixer_nid = 0x20;
    (*spec).gen.mixer_merge_nid = 0x21;
    (*spec).gen.beep_nid = 0x10;
    set_beep_amp(spec, 0x10, 0, HDA_OUTPUT);

    snd_hda_pick_fixup(codec, ad1988_fixup_models.as_ptr(), ::core::ptr::null(), ad1988_fixups.as_ptr());
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

    err = ad198x_parse_auto_config(codec, true);
    if err < 0 {
        return err;
    }
    err = ad1988_add_spdif_mux_ctl(codec);
    if err < 0 {
        return err;
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);

    0
}

/*
 * AD1884 / AD1984
 *
 * port-B - front line/mic-in
 * port-E - aux in/out
 * port-F - aux in/out
 * port-C - rear line/mic-in
 * port-D - rear line/hp-out
 * port-A - front line/hp-out
 *
 * AD1984 = AD1884 + two digital mic-ins
 *
 * AD1883 / AD1884A / AD1984A / AD1984B
 *
 * port-B (0x14) - front mic-in
 * port-E (0x1c) - rear mic-in
 * port-F (0x16) - CD / ext out
 * port-C (0x15) - rear line-in
 * port-D (0x12) - rear line-out
 * port-A (0x11) - front hp-out
 *
 * AD1984A = AD1884A + digital-mic
 * AD1883 = equivalent with AD1984A
 * AD1984B = AD1984A + extra SPDIF-out
 */

/* set the upper-limit for mixer amp to 0dB for avoiding the possible
 * damage by overloading
 */
unsafe extern "C" fn ad1884_fixup_amp_override(codec: *mut hda_codec,
                                               _fix: *const hda_fixup,
                                               action: ::core::ffi::c_int) {
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        snd_hda_override_amp_caps(codec, 0x20, HDA_INPUT,
                                  (0x17 << AC_AMPCAP_OFFSET_SHIFT) |
                                  (0x17 << AC_AMPCAP_NUM_STEPS_SHIFT) |
                                  (0x05 << AC_AMPCAP_STEP_SIZE_SHIFT) |
                                  (1 << AC_AMPCAP_MUTE_SHIFT));
    }
}

/* toggle GPIO1 according to the mute state */
unsafe extern "C" fn ad1884_vmaster_hp_gpio_hook(private_data: *mut ::core::ffi::c_void,
                                                 enabled: ::core::ffi::c_int) {
    let codec = private_data as *mut hda_codec;
    let spec = (*codec).spec as *mut ad198x_spec;

    if (*spec).eapd_nid != 0 {
        ad_vmaster_eapd_hook(private_data, enabled);
    }
    (*spec).gpio_data = if enabled != 0 { 0x00 } else { 0x02 };
    snd_hda_codec_write(codec, 0x01, 0,
                        AC_VERB_SET_GPIO_DATA, (*spec).gpio_data);
}

unsafe extern "C" fn ad1884_fixup_hp_eapd(codec: *mut hda_codec,
                                          _fix: *const hda_fixup,
                                          action: ::core::ffi::c_int) {
    let spec = (*codec).spec as *mut ad198x_spec;

    match action {
        HDA_FIXUP_ACT_PRE_PROBE => {
            (*spec).gen.vmaster_mute.hook = Some(ad1884_vmaster_hp_gpio_hook);
            (*spec).gen.own_eapd_ctl = 1;
            (*spec).gpio_data = 0x02;
        }
        HDA_FIXUP_ACT_PROBE => {
            if (*spec).gen.autocfg.line_out_type == AUTO_PIN_SPEAKER_OUT {
                (*spec).eapd_nid = (*spec).gen.autocfg.line_out_pins[0];
            } else {
                (*spec).eapd_nid = (*spec).gen.autocfg.speaker_pins[0];
            }
        }
        HDA_FIXUP_ACT_INIT => {
            snd_hda_codec_set_gpio(codec, 0x02, 0x02, (*spec).gpio_data, 0);
        }
        _ => {}
    }
}

unsafe extern "C" fn ad1884_fixup_thinkpad(codec: *mut hda_codec,
                                           _fix: *const hda_fixup,
                                           action: ::core::ffi::c_int) {
    let spec = (*codec).spec as *mut ad198x_spec;

    if action == HDA_FIXUP_ACT_PRE_PROBE {
        (*spec).gen.keep_eapd_on = 1;
        (*spec).gen.vmaster_mute.hook = Some(ad_vmaster_eapd_hook);
        (*spec).eapd_nid = 0x12;
        /* Analog PC Beeper - allow firmware/ACPI beeps */
        (*spec).beep_amp = HDA_COMPOSE_AMP_VAL(0x20, 3, 3, HDA_INPUT);
        (*spec).gen.beep_nid = 0; /* no digital beep */
    }
}

/* set magic COEFs for dmic */
static ad1884_dmic_init_verbs: &[hda_verb] = &[
    hda_verb { nid: 0x01, verb: AC_VERB_SET_COEF_INDEX, param: 0x13f7 },
    hda_verb { nid: 0x01, verb: AC_VERB_SET_PROC_COEF, param: 0x08 },
    hda_verb::default(),
];

const AD1884_FIXUP_AMP_OVERRIDE: i32 = 0;
const AD1884_FIXUP_HP_EAPD: i32 = 1;
const AD1884_FIXUP_DMIC_COEF: i32 = 2;
const AD1884_FIXUP_THINKPAD: i32 = 3;
const AD1884_FIXUP_HP_TOUCHSMART: i32 = 4;

static ad1884_fixups: &[hda_fixup] = &[
    hda_fixup_func!(AD1884_FIXUP_AMP_OVERRIDE, ad1884_fixup_amp_override),
    hda_fixup_func_chained!(AD1884_FIXUP_HP_EAPD, ad1884_fixup_hp_eapd, AD1884_FIXUP_AMP_OVERRIDE),
    hda_fixup_verbs!(AD1884_FIXUP_DMIC_COEF, ad1884_dmic_init_verbs.as_ptr()),
    hda_fixup_func_chained!(AD1884_FIXUP_THINKPAD, ad1884_fixup_thinkpad, AD1884_FIXUP_DMIC_COEF),
    hda_fixup_verbs_chained!(AD1884_FIXUP_HP_TOUCHSMART, ad1884_dmic_init_verbs.as_ptr(), AD1884_FIXUP_HP_EAPD),
];

static ad1884_fixup_tbl: &[hda_quirk] = &[
    SND_PCI_QUIRK!(0x103c, 0x2a82, "HP Touchsmart", AD1884_FIXUP_HP_TOUCHSMART),
    SND_PCI_QUIRK_VENDOR!(0x103c, "HP", AD1884_FIXUP_HP_EAPD),
    SND_PCI_QUIRK_VENDOR!(0x17aa, "Lenovo Thinkpad", AD1884_FIXUP_THINKPAD),
    hda_quirk::default(),
];

unsafe fn ad1884_probe(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let spec = (*codec).spec as *mut ad198x_spec;
    let mut err: ::core::ffi::c_int;

    (*spec).gen.mixer_nid = 0x20;
    (*spec).gen.mixer_merge_nid = 0x21;
    (*spec).gen.beep_nid = 0x10;
    set_beep_amp(spec, 0x10, 0, HDA_OUTPUT);

    snd_hda_pick_fixup(codec, ::core::ptr::null(), ad1884_fixup_tbl.as_ptr(), ad1884_fixups.as_ptr());
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

    err = ad198x_parse_auto_config(codec, true);
    if err < 0 {
        return err;
    }
    err = ad1983_add_spdif_mux_ctl(codec);
    if err < 0 {
        return err;
    }

    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);

    0
}

/*
 * AD1882 / AD1882A
 *
 * port-A - front hp-out
 * port-B - front mic-in
 * port-C - rear line-in, shared surr-out (3stack)
 * port-D - rear line-out
 * port-E - rear mic-in, shared clfe-out (3stack)
 * port-F - rear surr-out (6stack)
 * port-G - rear clfe-out (6stack)
 */

unsafe fn ad1882_probe(codec: *mut hda_codec) -> ::core::ffi::c_int {
    let spec = (*codec).spec as *mut ad198x_spec;
    let mut err: ::core::ffi::c_int;

    (*spec).gen.mixer_nid = 0x20;
    (*spec).gen.mixer_merge_nid = 0x21;
    (*spec).gen.beep_nid = 0x10;
    set_beep_amp(spec, 0x10, 0, HDA_OUTPUT);
    err = ad198x_parse_auto_config(codec, true);
    if err < 0 {
        return err;
    }
    err = ad1988_add_spdif_mux_ctl(codec);
    if err < 0 {
        return err;
    }
    0
}

/*
 * driver entries
 */
unsafe extern "C" fn ad_codec_probe(codec: *mut hda_codec,
                                    id: *const hda_device_id) -> ::core::ffi::c_int {
    let spec: *mut ad198x_spec;
    let mut err: ::core::ffi::c_int;

    err = alloc_ad_spec(codec);
    if err < 0 {
        return -ENOMEM;
    }
    spec = (*codec).spec as *mut ad198x_spec;
    (*spec).model = (*id).driver_data as ::core::ffi::c_int;

    match (*spec).model {
        MODEL_AD1882 => err = ad1882_probe(codec),
        MODEL_AD1884 => err = ad1884_probe(codec),
        MODEL_AD1981 => err = ad1981_probe(codec),
        MODEL_AD1983 => err = ad1983_probe(codec),
        MODEL_AD1986A => err = ad1986a_probe(codec),
        MODEL_AD1988 => err = ad1988_probe(codec),
        _ => err = -EINVAL,
    }

    if err < 0 {
        snd_hda_gen_remove(codec);
        return err;
    }

    0
}

static ad_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(ad_codec_probe),
    remove: Some(snd_hda_gen_remove),
    build_controls: Some(ad_codec_build_controls),
    build_pcms: Some(snd_hda_gen_build_pcms),
    init: Some(ad_codec_init),
    unsol_event: Some(snd_hda_jack_unsol_event),
    suspend: Some(ad_codec_suspend),
    check_power_status: Some(snd_hda_gen_check_power_status),
    stream_pm: Some(snd_hda_gen_stream_pm),
};

static snd_hda_id_analog: &[hda_device_id] = &[
    HDA_CODEC_ID_MODEL!(0x11d4184a, "AD1884A", MODEL_AD1884),
    HDA_CODEC_ID_MODEL!(0x11d41882, "AD1882", MODEL_AD1882),
    HDA_CODEC_ID_MODEL!(0x11d41883, "AD1883", MODEL_AD1884),
    HDA_CODEC_ID_MODEL!(0x11d41884, "AD1884", MODEL_AD1884),
    HDA_CODEC_ID_MODEL!(0x11d4194a, "AD1984A", MODEL_AD1884),
    HDA_CODEC_ID_MODEL!(0x11d4194b, "AD1984B", MODEL_AD1884),
    HDA_CODEC_ID_MODEL!(0x11d41981, "AD1981", MODEL_AD1981),
    HDA_CODEC_ID_MODEL!(0x11d41983, "AD1983", MODEL_AD1983),
    HDA_CODEC_ID_MODEL!(0x11d41984, "AD1984", MODEL_AD1884),
    HDA_CODEC_ID_MODEL!(0x11d41986, "AD1986A", MODEL_AD1986A),
    HDA_CODEC_ID_MODEL!(0x11d41988, "AD1988", MODEL_AD1988),
    HDA_CODEC_ID_MODEL!(0x11d4198b, "AD1988B", MODEL_AD1988),
    HDA_CODEC_ID_MODEL!(0x11d4882a, "AD1882A", MODEL_AD1882),
    HDA_CODEC_ID_MODEL!(0x11d4989a, "AD1989A", MODEL_AD1988),
    HDA_CODEC_ID_MODEL!(0x11d4989b, "AD1989B", MODEL_AD1988),
    hda_device_id::default(), /* terminator */
];
MODULE_DEVICE_TABLE!(hdaudio, snd_hda_id_analog);

MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("Analog Devices HD-audio codec");

static mut analog_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_analog.as_ptr(),
    ops: &ad_codec_ops,
};

module_hda_codec_driver!(analog_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
