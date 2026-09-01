// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Universal Interface for Intel High Definition Audio Codec
 *
 * HD audio codec driver for VIA VT17xx/VT18xx/VT20xx codec
 *
 *  (C) 2006-2009 VIA Technology, Inc.
 *  (C) 2006-2008 Takashi Iwai <tiwai@suse.de>
 */

/* * * * * * * * * * * * * * Release History * * * * * * * * * * * * * * * * */
/*									     */
/* 2006-03-03  Lydia Wang  Create the basic patch to support VT1708 codec    */
/* 2006-03-14  Lydia Wang  Modify hard code for some pin widget nid	     */
/* 2006-08-02  Lydia Wang  Add support to VT1709 codec			     */
/* 2006-09-08  Lydia Wang  Fix internal loopback recording source select bug */
/* 2007-09-12  Lydia Wang  Add EAPD enable during driver initialization	     */
/* 2007-09-17  Lydia Wang  Add VT1708B codec support			    */
/* 2007-11-14  Lydia Wang  Add VT1708A codec HP and CD pin connect config    */
/* 2008-02-03  Lydia Wang  Fix Rear channels and Back channels inverse issue */
/* 2008-03-06  Lydia Wang  Add VT1702 codec and VT1708S codec support	     */
/* 2008-04-09  Lydia Wang  Add mute front speaker when HP plugin	     */
/* 2008-04-09  Lydia Wang  Add Independent HP feature			     */
/* 2008-05-28  Lydia Wang  Add second S/PDIF Out support for VT1702	     */
/* 2008-09-15  Logan Li	   Add VT1708S Mic Boost workaround/backdoor	     */
/* 2009-02-16  Logan Li	   Add support for VT1718S			     */
/* 2009-03-13  Logan Li	   Add support for VT1716S			     */
/* 2009-04-14  Lydai Wang  Add support for VT1828S and VT2020		     */
/* 2009-07-08  Lydia Wang  Add support for VT2002P			     */
/* 2009-07-21  Lydia Wang  Add support for VT1812			     */
/* 2009-09-19  Lydia Wang  Add support for VT1818S			     */
/*									     */
/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

/* C dependencies removed from Rust executable code:
 * linux/init.h, linux/delay.h, linux/slab.h, linux/module.h,
 * sound/core.h, sound/asoundef.h, sound/hda_codec.h,
 * hda_local.h, hda_auto_parser.h, hda_jack.h, generic.h.
 */
use crate::*;

/* Pin Widget NID */
pub const VT1708_HP_PIN_NID: hda_nid_t = 0x20;
pub const VT1708_CD_PIN_NID: hda_nid_t = 0x24;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VIA_HDA_CODEC {
    UNKNOWN = -1,
    VT1708,
    VT1709,
    VT1709_10CH,
    VT1709_6CH,
    VT1708B,
    VT1708B_8CH,
    VT1708B_4CH,
    VT1708S,
    VT1708BCE,
    VT1702,
    VT1718S,
    VT1716S,
    VT2002P,
    VT1812,
    VT1802,
    VT1705CF,
    VT1808,
    VT3476,
    CODEC_TYPES,
}

#[inline]
unsafe fn VT2002P_COMPATIBLE(spec: *mut via_spec) -> bool {
    (*spec).codec_type == VIA_HDA_CODEC::VT2002P
        || (*spec).codec_type == VIA_HDA_CODEC::VT1812
        || (*spec).codec_type == VIA_HDA_CODEC::VT1802
}

#[repr(C)]
pub struct via_spec {
    pub gen: hda_gen_spec,

    /* HP mode source */
    pub dmic_enabled: c_uint,
    pub codec_type: VIA_HDA_CODEC,

    /* analog low-power control */
    pub alc_mode: bool,

    /* work to check hp jack state */
    pub hp_work_active: c_int,
    pub vt1708_jack_detect: c_int,
}

unsafe fn via_new_spec(codec: *mut hda_codec) -> *mut via_spec {
    let mut spec: *mut via_spec;

    spec = kzalloc_obj::<via_spec>();
    if spec.is_null() {
        return NULL;
    }

    (*codec).spec = spec as *mut c_void;
    snd_hda_gen_spec_init(&mut (*spec).gen);
    (*spec).codec_type = get_codec_type(codec);
    /* VT1708BCE & VT1708S are almost same */
    if (*spec).codec_type == VIA_HDA_CODEC::VT1708BCE {
        (*spec).codec_type = VIA_HDA_CODEC::VT1708S;
    }
    (*spec).gen.indep_hp = 1;
    (*spec).gen.keep_eapd_on = 1;
    (*spec).gen.dac_min_mute = 1;
    (*spec).gen.pcm_playback_hook = Some(via_playback_pcm_hook);
    (*spec).gen.add_stereo_mix_input = HDA_HINT_STEREO_MIX_AUTO;
    (*codec).power_save_node = 1;
    (*spec).gen.power_down_unused = 1;
    spec
}

unsafe fn get_codec_type(codec: *mut hda_codec) -> VIA_HDA_CODEC {
    let vendor_id: u32 = (*codec).core.vendor_id;
    let ven_id: u16 = (vendor_id >> 16) as u16;
    let dev_id: u16 = (vendor_id & 0xffff) as u16;
    let codec_type: VIA_HDA_CODEC;

    /* get codec type */
    if ven_id != 0x1106 {
        codec_type = VIA_HDA_CODEC::UNKNOWN;
    } else if dev_id >= 0x1708 && dev_id <= 0x170b {
        codec_type = VIA_HDA_CODEC::VT1708;
    } else if dev_id >= 0xe710 && dev_id <= 0xe713 {
        codec_type = VIA_HDA_CODEC::VT1709_10CH;
    } else if dev_id >= 0xe714 && dev_id <= 0xe717 {
        codec_type = VIA_HDA_CODEC::VT1709_6CH;
    } else if dev_id >= 0xe720 && dev_id <= 0xe723 {
        codec_type = VIA_HDA_CODEC::VT1708B_8CH;
        if snd_hda_param_read(codec, 0x16, AC_PAR_CONNLIST_LEN) == 0x7 {
            codec_type = VIA_HDA_CODEC::VT1708BCE;
        }
    } else if dev_id >= 0xe724 && dev_id <= 0xe727 {
        codec_type = VIA_HDA_CODEC::VT1708B_4CH;
    } else if (dev_id & 0xfff) == 0x397 && (dev_id >> 12) < 8 {
        codec_type = VIA_HDA_CODEC::VT1708S;
    } else if (dev_id & 0xfff) == 0x398 && (dev_id >> 12) < 8 {
        codec_type = VIA_HDA_CODEC::VT1702;
    } else if (dev_id & 0xfff) == 0x428 && (dev_id >> 12) < 8 {
        codec_type = VIA_HDA_CODEC::VT1718S;
    } else if dev_id == 0x0433 || dev_id == 0xa721 {
        codec_type = VIA_HDA_CODEC::VT1716S;
    } else if dev_id == 0x0441 || dev_id == 0x4441 {
        codec_type = VIA_HDA_CODEC::VT1718S;
    } else if dev_id == 0x0438 || dev_id == 0x4438 {
        codec_type = VIA_HDA_CODEC::VT2002P;
    } else if dev_id == 0x0448 {
        codec_type = VIA_HDA_CODEC::VT1812;
    } else if dev_id == 0x0440 {
        codec_type = VIA_HDA_CODEC::VT1708S;
    } else if (dev_id & 0xfff) == 0x446 {
        codec_type = VIA_HDA_CODEC::VT1802;
    } else if dev_id == 0x4760 {
        codec_type = VIA_HDA_CODEC::VT1705CF;
    } else if dev_id == 0x4761 || dev_id == 0x4762 {
        codec_type = VIA_HDA_CODEC::VT1808;
    } else {
        codec_type = VIA_HDA_CODEC::UNKNOWN;
    }
    codec_type
}

#[inline]
unsafe fn hp_detect_with_aa(codec: *mut hda_codec) -> bool {
    snd_hda_get_bool_hint(codec, c_str!("analog_loopback_hp_detect")) == 1 && !is_aa_path_mute(codec)
}

unsafe fn vt1708_stop_hp_work(codec: *mut hda_codec) {
    let spec = (*codec).spec as *mut via_spec;
    if (*spec).codec_type != VIA_HDA_CODEC::VT1708 || (*spec).gen.autocfg.hp_outs == 0 {
        return;
    }
    if (*spec).hp_work_active != 0 {
        snd_hda_codec_write(codec, 0x1, 0, 0xf81, 1);
        (*codec).jackpoll_interval = 0;
        cancel_delayed_work_sync(&mut (*codec).jackpoll_work);
        (*spec).hp_work_active = false as c_int;
    }
}

unsafe fn vt1708_update_hp_work(codec: *mut hda_codec) {
    let spec = (*codec).spec as *mut via_spec;
    if (*spec).codec_type != VIA_HDA_CODEC::VT1708 || (*spec).gen.autocfg.hp_outs == 0 {
        return;
    }
    if (*spec).vt1708_jack_detect != 0 {
        if (*spec).hp_work_active == 0 {
            (*codec).jackpoll_interval = msecs_to_jiffies(100);
            snd_hda_codec_write(codec, 0x1, 0, 0xf81, 0);
            schedule_delayed_work(&mut (*codec).jackpoll_work, 0);
            (*spec).hp_work_active = true as c_int;
        }
    } else if !hp_detect_with_aa(codec) {
        vt1708_stop_hp_work(codec);
    }
}

unsafe fn via_pin_power_ctl_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_hda_enum_bool_helper_info(kcontrol, uinfo)
}

unsafe fn via_pin_power_ctl_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut via_spec;

    (*ucontrol).value.enumerated.item[0] = (*spec).gen.power_down_unused;
    0
}

unsafe fn via_pin_power_ctl_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut via_spec;
    let val: bool = (*ucontrol).value.enumerated.item[0] != 0;

    if val == ((*spec).gen.power_down_unused != 0) {
        return 0;
    }
    /* codec->power_save_node = val; */ /* widget PM seems yet broken */
    (*spec).gen.power_down_unused = val as c_uint;
    analog_low_current_mode(codec);
    1
}

static via_pin_power_ctl_enum: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c_str!("Dynamic Power-Control"),
    info: Some(via_pin_power_ctl_info),
    get: Some(via_pin_power_ctl_get),
    put: Some(via_pin_power_ctl_put),
    ..snd_kcontrol_new::zero()
};

/* CONFIG_SND_HDA_INPUT_BEEP: additional beep mixers; the actual parameters are overwritten at build */
#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
static via_beep_mixer: [snd_kcontrol_new; 2] = [
    HDA_CODEC_VOLUME_MONO(c_str!("Beep Playback Volume"), 0, 1, 0, HDA_OUTPUT),
    HDA_CODEC_MUTE_BEEP_MONO(c_str!("Beep Playback Switch"), 0, 1, 0, HDA_OUTPUT),
];

#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
unsafe fn set_beep_amp(spec: *mut via_spec, nid: hda_nid_t, idx: c_int, dir: c_int) -> c_int {
    let mut knew: *mut snd_kcontrol_new;
    let beep_amp: c_uint = HDA_COMPOSE_AMP_VAL(nid, 1, idx, dir);
    let mut i: c_int;

    (*spec).gen.beep_nid = nid;
    i = 0;
    while i < ARRAY_SIZE(&via_beep_mixer) as c_int {
        knew = snd_hda_gen_add_kctl(&mut (*spec).gen, NULL, &via_beep_mixer[i as usize]);
        if knew.is_null() {
            return -ENOMEM;
        }
        (*knew).private_value = beep_amp as _;
        i += 1;
    }
    0
}

#[cfg(CONFIG_SND_HDA_INPUT_BEEP)]
unsafe fn auto_parse_beep(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut nid: hda_nid_t = 0;

    for_each_hda_codec_node!(nid, codec, {
        if get_wcaps_type(get_wcaps(codec, nid)) == AC_WID_BEEP {
            return set_beep_amp(spec, nid, 0, HDA_OUTPUT);
        }
    });
    0
}

#[cfg(not(CONFIG_SND_HDA_INPUT_BEEP))]
unsafe fn auto_parse_beep(_codec: *mut hda_codec) -> c_int {
    0
}

/* check AA path's mute status */
unsafe fn is_aa_path_mute(codec: *mut hda_codec) -> bool {
    let spec = (*codec).spec as *mut via_spec;
    let mut p: *const hda_amp_list;
    let mut ch: c_int;
    let mut v: c_int;

    p = (*spec).gen.loopback.amplist;
    if p.is_null() {
        return true;
    }
    while (*p).nid != 0 {
        ch = 0;
        while ch < 2 {
            v = snd_hda_codec_amp_read(codec, (*p).nid, ch, (*p).dir, (*p).idx);
            if (v & HDA_AMP_MUTE) == 0 && v > 0 {
                return false;
            }
            ch += 1;
        }
        p = p.add(1);
    }
    true
}

/* enter/exit analog low-current mode */
unsafe fn __analog_low_current_mode(codec: *mut hda_codec, force: bool) {
    let spec = (*codec).spec as *mut via_spec;
    let enable: bool;
    let verb: c_uint;
    let parm: c_uint;

    if (*codec).power_save_node == 0 {
        enable = false;
    } else {
        enable = is_aa_path_mute(codec) && (*spec).gen.active_streams == 0;
    }
    if enable == (*spec).alc_mode && !force {
        return;
    }
    (*spec).alc_mode = enable;

    /* decide low current mode's verb & parameter */
    match (*spec).codec_type {
        VIA_HDA_CODEC::VT1708B_8CH | VIA_HDA_CODEC::VT1708B_4CH => {
            verb = 0xf70;
            parm = if enable { 0x02 } else { 0x00 }; /* 0x02: 2/3x, 0x00: 1x */
        }
        VIA_HDA_CODEC::VT1708S | VIA_HDA_CODEC::VT1718S | VIA_HDA_CODEC::VT1716S => {
            verb = 0xf73;
            parm = if enable { 0x51 } else { 0xe1 }; /* 0x51: 4/28x, 0xe1: 1x */
        }
        VIA_HDA_CODEC::VT1702 => {
            verb = 0xf73;
            parm = if enable { 0x01 } else { 0x1d }; /* 0x01: 4/40x, 0x1d: 1x */
        }
        VIA_HDA_CODEC::VT2002P | VIA_HDA_CODEC::VT1812 | VIA_HDA_CODEC::VT1802 => {
            verb = 0xf93;
            parm = if enable { 0x00 } else { 0xe0 }; /* 0x00: 4/40x, 0xe0: 1x */
        }
        VIA_HDA_CODEC::VT1705CF | VIA_HDA_CODEC::VT1808 => {
            verb = 0xf82;
            parm = if enable { 0x00 } else { 0xe0 }; /* 0x00: 4/40x, 0xe0: 1x */
        }
        _ => return, /* other codecs are not supported */
    }
    /* send verb */
    snd_hda_codec_write(codec, (*codec).core.afg, 0, verb, parm);
}

unsafe fn analog_low_current_mode(codec: *mut hda_codec) {
    return __analog_low_current_mode(codec, false);
}

unsafe fn via_playback_pcm_hook(
    _hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    _substream: *mut snd_pcm_substream,
    _action: c_int,
) {
    analog_low_current_mode(codec);
    vt1708_update_hp_work(codec);
}

unsafe fn via_remove(codec: *mut hda_codec) {
    vt1708_stop_hp_work(codec);
    snd_hda_gen_remove(codec);
}

unsafe fn via_suspend(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    vt1708_stop_hp_work(codec);

    /* Fix pop noise on headphones */
    if (*spec).codec_type == VIA_HDA_CODEC::VT1802 {
        snd_hda_shutup_pins(codec);
    }

    0
}

unsafe fn via_resume(codec: *mut hda_codec) -> c_int {
    /* some delay here to make jack detection working (bko#98921) */
    msleep(10);
    snd_hda_codec_init(codec);
    snd_hda_regmap_sync(codec);
    0
}

unsafe fn via_check_power_status(codec: *mut hda_codec, nid: hda_nid_t) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    analog_low_current_mode(codec);
    vt1708_update_hp_work(codec);
    snd_hda_check_amp_list_power(codec, &mut (*spec).gen.loopback, nid)
}

/*
 */

static vt1708_init_verbs: [hda_verb; 2] = [
    hda_verb { nid: 0x1, verb: 0xf81, param: 0x1 },
    hda_verb::zero(),
];

unsafe fn vt1708_set_pinconfig_connect(codec: *mut hda_codec, nid: hda_nid_t) {
    let mut def_conf: c_uint;
    let mut seqassoc: c_uchar;

    def_conf = snd_hda_codec_get_pincfg(codec, nid);
    seqassoc = get_defcfg_association(def_conf) as c_uchar;
    seqassoc = (seqassoc << 4) | get_defcfg_sequence(def_conf) as c_uchar;
    if get_defcfg_connect(def_conf) == AC_JACK_PORT_NONE && (seqassoc == 0xf0 || seqassoc == 0xff) {
        def_conf = def_conf & !(AC_JACK_PORT_BOTH << 30);
        snd_hda_codec_set_pincfg(codec, nid, def_conf);
    }
}

unsafe fn vt1708_jack_detect_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut via_spec;

    if (*spec).codec_type != VIA_HDA_CODEC::VT1708 {
        return 0;
    }
    (*ucontrol).value.integer.value[0] = (*spec).vt1708_jack_detect as _;
    0
}

unsafe fn vt1708_jack_detect_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut via_spec;
    let val: c_int;

    if (*spec).codec_type != VIA_HDA_CODEC::VT1708 {
        return 0;
    }
    val = ((*ucontrol).value.integer.value[0] != 0) as c_int;
    if (*spec).vt1708_jack_detect == val {
        return 0;
    }
    (*spec).vt1708_jack_detect = val;
    vt1708_update_hp_work(codec);
    1
}

static vt1708_jack_detect_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c_str!("Jack Detect"),
    count: 1,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(vt1708_jack_detect_get),
    put: Some(vt1708_jack_detect_put),
    ..snd_kcontrol_new::zero()
};

static via_main_out_badness_impl: badness_table = badness_table {
    no_primary_dac: 0x10000,
    no_dac: 0x4000,
    shared_primary: 0x10000,
    shared_surr: 0x20,
    shared_clfe: 0x20,
    shared_surr_main: 0x20,
    ..badness_table::zero()
};

static via_extra_out_badness_impl: badness_table = badness_table {
    no_primary_dac: 0x4000,
    no_dac: 0x4000,
    shared_primary: 0x12,
    shared_surr: 0x20,
    shared_clfe: 0x20,
    shared_surr_main: 0x10,
    ..badness_table::zero()
};

unsafe fn via_parse_auto_config(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut err: c_int;

    (*spec).gen.main_out_badness = &via_main_out_badness_impl;
    (*spec).gen.extra_out_badness = &via_extra_out_badness_impl;

    err = snd_hda_parse_pin_defcfg(codec, &mut (*spec).gen.autocfg, NULL, 0);
    if err < 0 {
        return err;
    }

    err = auto_parse_beep(codec);
    if err < 0 {
        return err;
    }

    err = snd_hda_gen_parse_auto_config(codec, &mut (*spec).gen.autocfg);
    if err < 0 {
        return err;
    }

    if snd_hda_gen_add_kctl(&mut (*spec).gen, NULL, &via_pin_power_ctl_enum).is_null() {
        return -ENOMEM;
    }

    /* disable widget PM at start for compatibility */
    (*codec).power_save_node = 0;
    (*spec).gen.power_down_unused = 0;
    0
}

unsafe fn via_init(codec: *mut hda_codec) -> c_int {
    /* init power states */
    __analog_low_current_mode(codec, true);

    snd_hda_gen_init(codec);

    vt1708_update_hp_work(codec);

    0
}

unsafe fn via_build_controls(codec: *mut hda_codec) -> c_int {
    /* In order not to create "Phantom Jack" controls,
       temporary enable jackpoll */
    let err: c_int;
    let old_interval: c_int = (*codec).jackpoll_interval;
    if old_interval != 0 {
        (*codec).jackpoll_interval = msecs_to_jiffies(100);
    }
    err = snd_hda_gen_build_controls(codec);
    if old_interval != 0 {
        (*codec).jackpoll_interval = old_interval;
    }
    err
}

unsafe fn via_build_pcms(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut i: c_int;
    let err: c_int;

    err = snd_hda_gen_build_pcms(codec);
    if err < 0 || (*codec).core.vendor_id != 0x11061708 {
        return err;
    }

    /* We got noisy outputs on the right channel on VT1708 when
     * 24bit samples are used.  Until any workaround is found,
     * disable the 24bit format, so far.
     */
    i = 0;
    while i < ARRAY_SIZE(&(*spec).gen.pcm_rec) as c_int {
        let info: *mut hda_pcm = (*spec).gen.pcm_rec[i as usize];
        if info.is_null() {
            i += 1;
            continue;
        }
        if (*info).stream[SNDRV_PCM_STREAM_PLAYBACK as usize].substreams == 0
            || (*info).pcm_type != HDA_PCM_TYPE_AUDIO
        {
            i += 1;
            continue;
        }
        (*info).stream[SNDRV_PCM_STREAM_PLAYBACK as usize].formats = SNDRV_PCM_FMTBIT_S16_LE;
        i += 1;
    }

    0
}

unsafe fn probe_vt1708(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut err: c_int;

    (*spec).gen.mixer_nid = 0x17;

    /* set jackpoll_interval while parsing the codec */
    (*codec).jackpoll_interval = msecs_to_jiffies(100);
    (*spec).vt1708_jack_detect = 1;

    /* don't support the input jack switching due to lack of unsol event */
    /* (it may work with polling, though, but it needs testing) */
    (*spec).gen.suppress_auto_mic = 1;
    /* Some machines show the broken speaker mute */
    (*spec).gen.auto_mute_via_amp = 1;

    /* Add HP and CD pin config connect bit re-config action */
    vt1708_set_pinconfig_connect(codec, VT1708_HP_PIN_NID);
    vt1708_set_pinconfig_connect(codec, VT1708_CD_PIN_NID);

    err = snd_hda_add_verbs(codec, vt1708_init_verbs.as_ptr());
    if err < 0 {
        return err;
    }

    /* automatic parse from the BIOS config */
    err = via_parse_auto_config(codec);
    if err < 0 {
        return err;
    }

    /* add jack detect on/off control */
    if snd_hda_gen_add_kctl(&mut (*spec).gen, NULL, &vt1708_jack_detect_ctl).is_null() {
        return -ENOMEM;
    }

    /* clear jackpoll_interval again; it's set dynamically */
    (*codec).jackpoll_interval = 0;

    0
}

unsafe fn probe_vt1709(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;

    (*spec).gen.mixer_nid = 0x18;

    via_parse_auto_config(codec)
}

unsafe fn probe_vt1708B(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;

    if get_codec_type(codec) == VIA_HDA_CODEC::VT1708BCE {
        return probe_vt1708S(codec);
    }

    (*spec).gen.mixer_nid = 0x16;

    /* automatic parse from the BIOS config */
    via_parse_auto_config(codec)
}

/* Support for VT1708S */
static vt1708S_init_verbs: [hda_verb; 3] = [
    /* Enable Mic Boost Volume backdoor */
    hda_verb { nid: 0x1, verb: 0xf98, param: 0x1 },
    /* don't bybass mixer */
    hda_verb { nid: 0x1, verb: 0xf88, param: 0xc0 },
    hda_verb::zero(),
];

unsafe fn override_mic_boost(codec: *mut hda_codec, pin: hda_nid_t, offset: c_int, num_steps: c_int, step_size: c_int) {
    snd_hda_override_wcaps(codec, pin, get_wcaps(codec, pin) | AC_WCAP_IN_AMP);
    snd_hda_override_amp_caps(
        codec,
        pin,
        HDA_INPUT,
        ((offset << AC_AMPCAP_OFFSET_SHIFT)
            | (num_steps << AC_AMPCAP_NUM_STEPS_SHIFT)
            | (step_size << AC_AMPCAP_STEP_SIZE_SHIFT)
            | (0 << AC_AMPCAP_MUTE_SHIFT)) as c_uint,
    );
}

unsafe fn probe_vt1708S(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut err: c_int;

    (*spec).gen.mixer_nid = 0x16;
    override_mic_boost(codec, 0x1a, 0, 3, 40);
    override_mic_boost(codec, 0x1e, 0, 3, 40);

    /* correct names for VT1708BCE */
    if get_codec_type(codec) == VIA_HDA_CODEC::VT1708BCE {
        snd_hda_codec_set_name(codec, c_str!("VT1708BCE"));
    }
    /* correct names for VT1705 */
    if (*codec).core.vendor_id == 0x11064397 {
        snd_hda_codec_set_name(codec, c_str!("VT1705"));
    }

    err = snd_hda_add_verbs(codec, vt1708S_init_verbs.as_ptr());
    if err < 0 {
        return err;
    }

    via_parse_auto_config(codec)
}

/* Support for VT1702 */

static vt1702_init_verbs: [hda_verb; 3] = [
    /* mixer enable */
    hda_verb { nid: 0x1, verb: 0xF88, param: 0x3 },
    /* GPIO 0~2 */
    hda_verb { nid: 0x1, verb: 0xF82, param: 0x3F },
    hda_verb::zero(),
];

unsafe fn probe_vt1702(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut err: c_int;

    (*spec).gen.mixer_nid = 0x1a;

    /* limit AA path volume to 0 dB */
    snd_hda_override_amp_caps(
        codec,
        0x1A,
        HDA_INPUT,
        ((0x17 << AC_AMPCAP_OFFSET_SHIFT)
            | (0x17 << AC_AMPCAP_NUM_STEPS_SHIFT)
            | (0x5 << AC_AMPCAP_STEP_SIZE_SHIFT)
            | (1 << AC_AMPCAP_MUTE_SHIFT)) as c_uint,
    );

    err = snd_hda_add_verbs(codec, vt1702_init_verbs.as_ptr());
    if err < 0 {
        return err;
    }

    /* automatic parse from the BIOS config */
    via_parse_auto_config(codec)
}

/* Support for VT1718S */

static vt1718S_init_verbs: [hda_verb; 3] = [
    /* Enable MW0 adjust Gain 5 */
    hda_verb { nid: 0x1, verb: 0xfb2, param: 0x10 },
    /* Enable Boost Volume backdoor */
    hda_verb { nid: 0x1, verb: 0xf88, param: 0x8 },

    hda_verb::zero(),
];

/* Add a connection to the primary DAC from AA-mixer for some codecs
 * This isn't listed from the raw info, but the chip has a secret connection.
 */
unsafe fn add_secret_dac_path(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut i: c_int;
    let mut nums: c_int;
    let mut conn: [hda_nid_t; 8] = [0; 8];
    let mut nid: hda_nid_t = 0;

    if (*spec).gen.mixer_nid == 0 {
        return 0;
    }
    nums = snd_hda_get_connections(codec, (*spec).gen.mixer_nid, conn.as_mut_ptr(), ARRAY_SIZE(&conn) - 1);
    if nums < 0 {
        return nums;
    }

    i = 0;
    while i < nums {
        if get_wcaps_type(get_wcaps(codec, conn[i as usize])) == AC_WID_AUD_OUT {
            return 0;
        }
        i += 1;
    }

    /* find the primary DAC and add to the connection list */
    for_each_hda_codec_node!(nid, codec, {
        let caps: c_uint = get_wcaps(codec, nid);
        if get_wcaps_type(caps) == AC_WID_AUD_OUT && (caps & AC_WCAP_DIGITAL) == 0 {
            conn[nums as usize] = nid;
            nums += 1;
            return snd_hda_override_conn_list(codec, (*spec).gen.mixer_nid, nums, conn.as_ptr());
        }
    });
    0
}

unsafe fn probe_vt1718S(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut err: c_int;

    (*spec).gen.mixer_nid = 0x21;
    override_mic_boost(codec, 0x2b, 0, 3, 40);
    override_mic_boost(codec, 0x29, 0, 3, 40);
    add_secret_dac_path(codec);

    err = snd_hda_add_verbs(codec, vt1718S_init_verbs.as_ptr());
    if err < 0 {
        return err;
    }

    /* automatic parse from the BIOS config */
    via_parse_auto_config(codec)
}

/* Support for VT1716S */

unsafe fn vt1716s_dmic_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe fn vt1716s_dmic_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let mut index: c_int = 0;

    index = snd_hda_codec_read(codec, 0x26, 0, AC_VERB_GET_CONNECT_SEL, 0);
    if index != -1 {
        *(*ucontrol).value.integer.value.as_mut_ptr() = index as _;
    }

    0
}

unsafe fn vt1716s_dmic_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol) as *mut hda_codec;
    let spec = (*codec).spec as *mut via_spec;
    let index: c_int = *(*ucontrol).value.integer.value.as_ptr() as c_int;

    snd_hda_codec_write(codec, 0x26, 0, AC_VERB_SET_CONNECT_SEL, index as c_uint);
    (*spec).dmic_enabled = index as c_uint;
    1
}

static vt1716s_dmic_mixer_vol: snd_kcontrol_new =
    HDA_CODEC_VOLUME(c_str!("Digital Mic Capture Volume"), 0x22, 0x0, HDA_INPUT);
static vt1716s_dmic_mixer_sw: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c_str!("Digital Mic Capture Switch"),
    subdevice: HDA_SUBDEV_NID_FLAG | 0x26,
    count: 1,
    info: Some(vt1716s_dmic_info),
    get: Some(vt1716s_dmic_get),
    put: Some(vt1716s_dmic_put),
    ..snd_kcontrol_new::zero()
};

/* mono-out mixer elements */
static vt1716S_mono_out_mixer: snd_kcontrol_new =
    HDA_CODEC_MUTE(c_str!("Mono Playback Switch"), 0x2a, 0x0, HDA_OUTPUT);

static vt1716S_init_verbs: [hda_verb; 4] = [
    /* Enable Boost Volume backdoor */
    hda_verb { nid: 0x1, verb: 0xf8a, param: 0x80 },
    /* don't bybass mixer */
    hda_verb { nid: 0x1, verb: 0xf88, param: 0xc0 },
    /* Enable mono output */
    hda_verb { nid: 0x1, verb: 0xf90, param: 0x08 },
    hda_verb::zero(),
];

unsafe fn probe_vt1716S(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut err: c_int;

    (*spec).gen.mixer_nid = 0x16;
    override_mic_boost(codec, 0x1a, 0, 3, 40);
    override_mic_boost(codec, 0x1e, 0, 3, 40);

    err = snd_hda_add_verbs(codec, vt1716S_init_verbs.as_ptr());
    if err < 0 {
        return err;
    }

    /* automatic parse from the BIOS config */
    err = via_parse_auto_config(codec);
    if err < 0 {
        return err;
    }

    if snd_hda_gen_add_kctl(&mut (*spec).gen, NULL, &vt1716s_dmic_mixer_vol).is_null()
        || snd_hda_gen_add_kctl(&mut (*spec).gen, NULL, &vt1716s_dmic_mixer_sw).is_null()
        || snd_hda_gen_add_kctl(&mut (*spec).gen, NULL, &vt1716S_mono_out_mixer).is_null()
    {
        return -ENOMEM;
    }

    0
}

/* for vt2002P */

static vt2002P_init_verbs: [hda_verb; 6] = [
    /* Class-D speaker related verbs */
    hda_verb { nid: 0x1, verb: 0xfe0, param: 0x4 },
    hda_verb { nid: 0x1, verb: 0xfe9, param: 0x80 },
    hda_verb { nid: 0x1, verb: 0xfe2, param: 0x22 },
    /* Enable Boost Volume backdoor */
    hda_verb { nid: 0x1, verb: 0xfb9, param: 0x24 },
    /* Enable AOW0 to MW9 */
    hda_verb { nid: 0x1, verb: 0xfb8, param: 0x88 },
    hda_verb::zero(),
];

static vt1802_init_verbs: [hda_verb; 3] = [
    /* Enable Boost Volume backdoor */
    hda_verb { nid: 0x1, verb: 0xfb9, param: 0x24 },
    /* Enable AOW0 to MW9 */
    hda_verb { nid: 0x1, verb: 0xfb8, param: 0x88 },
    hda_verb::zero(),
];

/*
 * pin fix-up
 */
pub const VIA_FIXUP_INTMIC_BOOST: c_int = 0;
pub const VIA_FIXUP_ASUS_G75: c_int = 1;
pub const VIA_FIXUP_POWER_SAVE: c_int = 2;

unsafe fn via_fixup_intmic_boost(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        override_mic_boost(codec, 0x30, 0, 2, 40);
    }
}

unsafe fn via_fixup_power_save(codec: *mut hda_codec, _fix: *const hda_fixup, action: c_int) {
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        (*codec).power_save_node = 0;
    }
}

static via_fixups: [hda_fixup; 3] = [
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_union { func: Some(via_fixup_intmic_boost) },
        ..hda_fixup::zero()
    },
    hda_fixup {
        type_: HDA_FIXUP_PINS,
        v: hda_fixup_union {
            pins: [
                /* set 0x24 and 0x33 as speakers */
                hda_pintbl { nid: 0x24, val: 0x991301f0 },
                hda_pintbl { nid: 0x33, val: 0x991301f1 }, /* subwoofer */
                hda_pintbl::zero(),
            ]
            .as_ptr(),
        },
        ..hda_fixup::zero()
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_union { func: Some(via_fixup_power_save) },
        ..hda_fixup::zero()
    },
];

static vt2002p_fixups: [hda_quirk; 5] = [
    SND_PCI_QUIRK(0x1043, 0x13f7, c_str!("Asus B23E"), VIA_FIXUP_POWER_SAVE),
    SND_PCI_QUIRK(0x1043, 0x1487, c_str!("Asus G75"), VIA_FIXUP_ASUS_G75),
    SND_PCI_QUIRK(0x1043, 0x8532, c_str!("Asus X202E"), VIA_FIXUP_INTMIC_BOOST),
    SND_PCI_QUIRK_VENDOR(0x1558, c_str!("Clevo"), VIA_FIXUP_POWER_SAVE),
    hda_quirk::zero(),
];

/* NIDs 0x24 and 0x33 on VT1802 have connections to non-existing NID 0x3e
 * Replace this with mixer NID 0x1c
 */
unsafe fn fix_vt1802_connections(codec: *mut hda_codec) {
    static conn_24: [hda_nid_t; 2] = [0x14, 0x1c];
    static conn_33: [hda_nid_t; 1] = [0x1c];

    snd_hda_override_conn_list(codec, 0x24, ARRAY_SIZE(&conn_24) as c_int, conn_24.as_ptr());
    snd_hda_override_conn_list(codec, 0x33, ARRAY_SIZE(&conn_33) as c_int, conn_33.as_ptr());
}

/* Support for vt2002P */
unsafe fn probe_vt2002P(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut err: c_int;

    (*spec).gen.mixer_nid = 0x21;
    override_mic_boost(codec, 0x2b, 0, 3, 40);
    override_mic_boost(codec, 0x29, 0, 3, 40);
    if (*spec).codec_type == VIA_HDA_CODEC::VT1802 {
        fix_vt1802_connections(codec);
    }
    add_secret_dac_path(codec);

    snd_hda_pick_fixup(codec, NULL, vt2002p_fixups.as_ptr(), via_fixups.as_ptr());
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);

    if (*spec).codec_type == VIA_HDA_CODEC::VT1802 {
        err = snd_hda_add_verbs(codec, vt1802_init_verbs.as_ptr());
    } else {
        err = snd_hda_add_verbs(codec, vt2002P_init_verbs.as_ptr());
    }
    if err < 0 {
        return err;
    }

    /* automatic parse from the BIOS config */
    via_parse_auto_config(codec)
}

/* for vt1812 */

static vt1812_init_verbs: [hda_verb; 3] = [
    /* Enable Boost Volume backdoor */
    hda_verb { nid: 0x1, verb: 0xfb9, param: 0x24 },
    /* Enable AOW0 to MW9 */
    hda_verb { nid: 0x1, verb: 0xfb8, param: 0xa8 },
    hda_verb::zero(),
];

unsafe fn probe_vt1812(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut err: c_int;

    (*spec).gen.mixer_nid = 0x21;
    override_mic_boost(codec, 0x2b, 0, 3, 40);
    override_mic_boost(codec, 0x29, 0, 3, 40);
    add_secret_dac_path(codec);

    err = snd_hda_add_verbs(codec, vt1812_init_verbs.as_ptr());
    if err < 0 {
        return err;
    }

    /* automatic parse from the BIOS config */
    via_parse_auto_config(codec)
}

/* Support for vt3476 */

static vt3476_init_verbs: [hda_verb; 4] = [
    /* Enable DMic 8/16/32K */
    hda_verb { nid: 0x1, verb: 0xF7B, param: 0x30 },
    /* Enable Boost Volume backdoor */
    hda_verb { nid: 0x1, verb: 0xFB9, param: 0x20 },
    /* Enable AOW-MW9 path */
    hda_verb { nid: 0x1, verb: 0xFB8, param: 0x10 },
    hda_verb::zero(),
];

unsafe fn probe_vt3476(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec as *mut via_spec;
    let mut err: c_int;

    (*spec).gen.mixer_nid = 0x3f;
    add_secret_dac_path(codec);

    err = snd_hda_add_verbs(codec, vt3476_init_verbs.as_ptr());
    if err < 0 {
        return err;
    }

    /* automatic parse from the BIOS config */
    via_parse_auto_config(codec)
}

/*
 * common driver probe
 */
unsafe fn via_probe(codec: *mut hda_codec, id: *const hda_device_id) -> c_int {
    let spec: *mut via_spec;
    let mut err: c_int;

    /* create a codec specific record */
    spec = via_new_spec(codec);
    if spec.is_null() {
        return -ENOMEM;
    }

    match (*id).driver_data as c_int {
        x if x == VIA_HDA_CODEC::VT1708 as c_int => err = probe_vt1708(codec),
        x if x == VIA_HDA_CODEC::VT1709 as c_int => err = probe_vt1709(codec),
        x if x == VIA_HDA_CODEC::VT1708B as c_int => err = probe_vt1708B(codec),
        x if x == VIA_HDA_CODEC::VT1708S as c_int => err = probe_vt1708S(codec),
        x if x == VIA_HDA_CODEC::VT1702 as c_int => err = probe_vt1702(codec),
        x if x == VIA_HDA_CODEC::VT1718S as c_int => err = probe_vt1718S(codec),
        x if x == VIA_HDA_CODEC::VT1716S as c_int => err = probe_vt1716S(codec),
        x if x == VIA_HDA_CODEC::VT2002P as c_int => err = probe_vt2002P(codec),
        x if x == VIA_HDA_CODEC::VT1812 as c_int => err = probe_vt1812(codec),
        x if x == VIA_HDA_CODEC::VT3476 as c_int => err = probe_vt3476(codec),
        _ => err = -EINVAL,
    }

    if err < 0 {
        via_remove(codec);
        return err;
    }

    0
}

static via_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(via_probe),
    remove: Some(via_remove),
    build_controls: Some(via_build_controls),
    build_pcms: Some(via_build_pcms),
    init: Some(via_init),
    unsol_event: Some(snd_hda_jack_unsol_event),
    suspend: Some(via_suspend),
    resume: Some(via_resume),
    check_power_status: Some(via_check_power_status),
    stream_pm: Some(snd_hda_gen_stream_pm),
    ..hda_codec_ops::zero()
};

/*
 * driver entries
 */
static snd_hda_id_via: [hda_device_id; 57] = [
    HDA_CODEC_ID_MODEL(0x11061708, c_str!("VT1708"), VIA_HDA_CODEC::VT1708 as _),
    HDA_CODEC_ID_MODEL(0x11061709, c_str!("VT1708"), VIA_HDA_CODEC::VT1708 as _),
    HDA_CODEC_ID_MODEL(0x1106170a, c_str!("VT1708"), VIA_HDA_CODEC::VT1708 as _),
    HDA_CODEC_ID_MODEL(0x1106170b, c_str!("VT1708"), VIA_HDA_CODEC::VT1708 as _),
    HDA_CODEC_ID_MODEL(0x1106e710, c_str!("VT1709 10-Ch"), VIA_HDA_CODEC::VT1709 as _),
    HDA_CODEC_ID_MODEL(0x1106e711, c_str!("VT1709 10-Ch"), VIA_HDA_CODEC::VT1709 as _),
    HDA_CODEC_ID_MODEL(0x1106e712, c_str!("VT1709 10-Ch"), VIA_HDA_CODEC::VT1709 as _),
    HDA_CODEC_ID_MODEL(0x1106e713, c_str!("VT1709 10-Ch"), VIA_HDA_CODEC::VT1709 as _),
    HDA_CODEC_ID_MODEL(0x1106e714, c_str!("VT1709 6-Ch"), VIA_HDA_CODEC::VT1709 as _),
    HDA_CODEC_ID_MODEL(0x1106e715, c_str!("VT1709 6-Ch"), VIA_HDA_CODEC::VT1709 as _),
    HDA_CODEC_ID_MODEL(0x1106e716, c_str!("VT1709 6-Ch"), VIA_HDA_CODEC::VT1709 as _),
    HDA_CODEC_ID_MODEL(0x1106e717, c_str!("VT1709 6-Ch"), VIA_HDA_CODEC::VT1709 as _),
    HDA_CODEC_ID_MODEL(0x1106e720, c_str!("VT1708B 8-Ch"), VIA_HDA_CODEC::VT1708B as _),
    HDA_CODEC_ID_MODEL(0x1106e721, c_str!("VT1708B 8-Ch"), VIA_HDA_CODEC::VT1708B as _),
    HDA_CODEC_ID_MODEL(0x1106e722, c_str!("VT1708B 8-Ch"), VIA_HDA_CODEC::VT1708B as _),
    HDA_CODEC_ID_MODEL(0x1106e723, c_str!("VT1708B 8-Ch"), VIA_HDA_CODEC::VT1708B as _),
    HDA_CODEC_ID_MODEL(0x1106e724, c_str!("VT1708B 4-Ch"), VIA_HDA_CODEC::VT1708B as _),
    HDA_CODEC_ID_MODEL(0x1106e725, c_str!("VT1708B 4-Ch"), VIA_HDA_CODEC::VT1708B as _),
    HDA_CODEC_ID_MODEL(0x1106e726, c_str!("VT1708B 4-Ch"), VIA_HDA_CODEC::VT1708B as _),
    HDA_CODEC_ID_MODEL(0x1106e727, c_str!("VT1708B 4-Ch"), VIA_HDA_CODEC::VT1708B as _),
    HDA_CODEC_ID_MODEL(0x11060397, c_str!("VT1708S"), VIA_HDA_CODEC::VT1708S as _),
    HDA_CODEC_ID_MODEL(0x11061397, c_str!("VT1708S"), VIA_HDA_CODEC::VT1708S as _),
    HDA_CODEC_ID_MODEL(0x11062397, c_str!("VT1708S"), VIA_HDA_CODEC::VT1708S as _),
    HDA_CODEC_ID_MODEL(0x11063397, c_str!("VT1708S"), VIA_HDA_CODEC::VT1708S as _),
    HDA_CODEC_ID_MODEL(0x11064397, c_str!("VT1705"), VIA_HDA_CODEC::VT1708S as _),
    HDA_CODEC_ID_MODEL(0x11065397, c_str!("VT1708S"), VIA_HDA_CODEC::VT1708S as _),
    HDA_CODEC_ID_MODEL(0x11066397, c_str!("VT1708S"), VIA_HDA_CODEC::VT1708S as _),
    HDA_CODEC_ID_MODEL(0x11067397, c_str!("VT1708S"), VIA_HDA_CODEC::VT1708S as _),
    HDA_CODEC_ID_MODEL(0x11060398, c_str!("VT1702"), VIA_HDA_CODEC::VT1702 as _),
    HDA_CODEC_ID_MODEL(0x11061398, c_str!("VT1702"), VIA_HDA_CODEC::VT1702 as _),
    HDA_CODEC_ID_MODEL(0x11062398, c_str!("VT1702"), VIA_HDA_CODEC::VT1702 as _),
    HDA_CODEC_ID_MODEL(0x11063398, c_str!("VT1702"), VIA_HDA_CODEC::VT1702 as _),
    HDA_CODEC_ID_MODEL(0x11064398, c_str!("VT1702"), VIA_HDA_CODEC::VT1702 as _),
    HDA_CODEC_ID_MODEL(0x11065398, c_str!("VT1702"), VIA_HDA_CODEC::VT1702 as _),
    HDA_CODEC_ID_MODEL(0x11066398, c_str!("VT1702"), VIA_HDA_CODEC::VT1702 as _),
    HDA_CODEC_ID_MODEL(0x11067398, c_str!("VT1702"), VIA_HDA_CODEC::VT1702 as _),
    HDA_CODEC_ID_MODEL(0x11060428, c_str!("VT1718S"), VIA_HDA_CODEC::VT1718S as _),
    HDA_CODEC_ID_MODEL(0x11064428, c_str!("VT1718S"), VIA_HDA_CODEC::VT1718S as _),
    HDA_CODEC_ID_MODEL(0x11060441, c_str!("VT2020"), VIA_HDA_CODEC::VT1718S as _),
    HDA_CODEC_ID_MODEL(0x11064441, c_str!("VT1828S"), VIA_HDA_CODEC::VT1718S as _),
    HDA_CODEC_ID_MODEL(0x11060433, c_str!("VT1716S"), VIA_HDA_CODEC::VT1716S as _),
    HDA_CODEC_ID_MODEL(0x1106a721, c_str!("VT1716S"), VIA_HDA_CODEC::VT1716S as _),
    HDA_CODEC_ID_MODEL(0x11060438, c_str!("VT2002P"), VIA_HDA_CODEC::VT2002P as _),
    HDA_CODEC_ID_MODEL(0x11064438, c_str!("VT2002P"), VIA_HDA_CODEC::VT2002P as _),
    HDA_CODEC_ID_MODEL(0x11060448, c_str!("VT1812"), VIA_HDA_CODEC::VT1812 as _),
    HDA_CODEC_ID_MODEL(0x11060440, c_str!("VT1818S"), VIA_HDA_CODEC::VT1708S as _),
    HDA_CODEC_ID_MODEL(0x11060446, c_str!("VT1802"), VIA_HDA_CODEC::VT2002P as _),
    HDA_CODEC_ID_MODEL(0x11068446, c_str!("VT1802"), VIA_HDA_CODEC::VT2002P as _),
    HDA_CODEC_ID_MODEL(0x11064760, c_str!("VT1705CF"), VIA_HDA_CODEC::VT3476 as _),
    HDA_CODEC_ID_MODEL(0x11064761, c_str!("VT1708SCE"), VIA_HDA_CODEC::VT3476 as _),
    HDA_CODEC_ID_MODEL(0x11064762, c_str!("VT1808"), VIA_HDA_CODEC::VT3476 as _),
    hda_device_id::zero(), /* terminator */
];
MODULE_DEVICE_TABLE!(hdaudio, snd_hda_id_via);

static mut via_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_via.as_ptr(),
    ops: &via_codec_ops,
    ..hda_codec_driver::zero()
};

MODULE_LICENSE!("GPL");
MODULE_DESCRIPTION!("VIA HD-audio codec");

module_hda_codec_driver!(via_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
