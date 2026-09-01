// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * BIOS auto-parser helper functions for HD-audio
 *
 * Copyright (c) 2012 Takashi Iwai <tiwai@suse.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{copy, copy_nonoverlapping, null, null_mut, write_bytes};

type bool_ = bool;
type hda_nid_t = u16;
type u16_ = u16;
type u32_ = u32;

/* External declarations and constants are supplied by the translated headers:
 * linux/slab.h, linux/export.h, linux/sort.h, sound/core.h,
 * sound/hda_codec.h, hda_local.h, and hda_auto_parser.h.
 */
const AUTO_CFG_MAX_INS: usize = 18;
const AUTO_CFG_MAX_OUTS: usize = 5;
const ENOMEM: c_int = 12;
const HDA_INPUT: c_uint = 0;
const AC_WID_PIN: c_uint = 4;
const AC_WCAP_STEREO: c_uint = 1;
const AC_PINCAP_OUT: c_uint = 1 << 4;
const AC_PINCAP_IN: c_uint = 1 << 5;
const AC_JACK_LINE_OUT: c_uint = 0;
const AC_JACK_SPEAKER: c_uint = 1;
const AC_JACK_HP_OUT: c_uint = 2;
const AC_JACK_CD: c_uint = 3;
const AC_JACK_SPDIF_OUT: c_uint = 4;
const AC_JACK_DIG_OTHER_OUT: c_uint = 5;
const AC_JACK_MIC_IN: c_uint = 6;
const AC_JACK_LINE_IN: c_uint = 7;
const AC_JACK_AUX: c_uint = 8;
const AC_JACK_SPDIF_IN: c_uint = 9;
const AC_JACK_DIG_OTHER_IN: c_uint = 10;
const AC_JACK_PORT_NONE: c_uint = 0;
const AC_JACK_PORT_FIXED: c_uint = 1;
const AC_JACK_PORT_BOTH: c_uint = 2;
const AC_JACK_LOC_HDMI: c_uint = 0x18;
const AC_JACK_LOC_INTERNAL: c_uint = 0x10;
const AC_JACK_LOC_SEPARATE: c_uint = 0x20;
const AC_JACK_LOC_REAR: c_uint = 0x07;
const AC_JACK_LOC_FRONT: c_uint = 0x0a;
const HDA_PCM_TYPE_HDMI: c_int = 3;
const HDA_PCM_TYPE_SPDIF: c_int = 1;
const AUTO_PIN_MIC: c_int = 0;
const AUTO_PIN_LINE_IN: c_int = 1;
const AUTO_PIN_CD: c_int = 2;
const AUTO_PIN_AUX: c_int = 3;
const AUTO_PIN_HP_OUT: c_int = 1;
const AUTO_PIN_SPEAKER_OUT: c_int = 2;
const HDA_PINCFG_HEADSET_MIC: c_uint = 1 << 0;
const HDA_PINCFG_HEADPHONE_MIC: c_uint = 1 << 1;
const HDA_PINCFG_NO_HP_FIXUP: c_uint = 1 << 2;
const HDA_PINCFG_NO_LO_FIXUP: c_uint = 1 << 3;
const INPUT_PIN_ATTR_UNUSED: c_int = 0;
const INPUT_PIN_ATTR_INT: c_int = 1;
const INPUT_PIN_ATTR_DOCK: c_int = 2;
const INPUT_PIN_ATTR_NORMAL: c_int = 3;
const INPUT_PIN_ATTR_REAR: c_int = 4;
const INPUT_PIN_ATTR_FRONT: c_int = 5;
const HDA_FIXUP_ID_NOT_SET: c_int = -1;
const HDA_FIXUP_ID_NO_FIXUP: c_int = -2;
const HDA_FIXUP_PINS: c_int = 0;
const HDA_FIXUP_VERBS: c_int = 1;
const HDA_FIXUP_FUNC: c_int = 2;
const HDA_FIXUP_PINCTLS: c_int = 3;
const HDA_FIXUP_ACT_PRE_PROBE: c_int = 0;
const HDA_FIXUP_ACT_PROBE: c_int = 1;
const AC_DEFCFG_SEQUENCE: u32_ = 0x0000000f;
const AC_DEFCFG_DEF_ASSOC: u32_ = 0x000000f0;
const IGNORE_SEQ_ASSOC: u32_ = !(AC_DEFCFG_SEQUENCE | AC_DEFCFG_DEF_ASSOC);

#[repr(C)]
pub struct hda_codec_core {
    pub chip_name: *const c_char,
    pub subsystem_id: c_uint,
    pub vendor_id: c_uint,
}

#[repr(C)]
pub struct pci_dev {
    pub subsystem_vendor: u16_,
    pub subsystem_device: u16_,
}

#[repr(C)]
pub struct hda_bus {
    pub pci: *mut pci_dev,
}

#[repr(C)]
pub struct snd_array {
    pub list: *mut c_void,
    pub used: c_int,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hda_codec_core,
    pub verbs: snd_array,
    pub init_pins: snd_array,
    pub force_pin_prefix: c_int,
    pub fixup_name: *const c_char,
    pub fixup_list: *const hda_fixup,
    pub fixup_id: c_int,
    pub modelname: *const c_char,
    pub bus: *mut hda_bus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct auto_pin_cfg_item {
    pub pin: hda_nid_t,
    pub type_: c_int,
    pub has_boost_on_pin: c_int,
    pub is_headset_mic: c_int,
    pub is_headphone_mic: c_int,
    pub order: c_int,
}

#[repr(C)]
pub struct auto_pin_cfg {
    pub line_outs: c_int,
    pub line_out_pins: [hda_nid_t; AUTO_CFG_MAX_OUTS],
    pub speaker_outs: c_int,
    pub speaker_pins: [hda_nid_t; AUTO_CFG_MAX_OUTS],
    pub hp_outs: c_int,
    pub hp_pins: [hda_nid_t; AUTO_CFG_MAX_OUTS],
    pub mono_out_pin: hda_nid_t,
    pub dig_outs: c_int,
    pub dig_out_pins: [hda_nid_t; 2],
    pub dig_out_type: [c_int; 2],
    pub dig_in_pin: hda_nid_t,
    pub dig_in_type: c_int,
    pub num_inputs: c_int,
    pub inputs: [auto_pin_cfg_item; AUTO_CFG_MAX_INS],
    pub line_out_type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct auto_out_pin {
    pin: hda_nid_t,
    seq: i16,
}

#[repr(C)]
pub struct hda_verb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_pintbl {
    pub nid: hda_nid_t,
    pub val: u32_,
}

#[repr(C)]
pub struct hda_pincfg {
    pub nid: hda_nid_t,
    pub cfg: u32_,
}

#[repr(C)]
pub union hda_fixup_val {
    pub pins: *const hda_pintbl,
    pub verbs: *const hda_verb,
    pub func: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_fixup, c_int)>,
}

#[repr(C)]
pub struct hda_fixup {
    pub type_: c_int,
    pub chained: c_int,
    pub chained_before: c_int,
    pub chain_id: c_int,
    pub v: hda_fixup_val,
}

#[repr(C)]
pub struct snd_hda_pin_quirk {
    pub codec: c_uint,
    pub subvendor: c_uint,
    pub value: c_int,
    pub name: *const c_char,
    pub pins: *const hda_pintbl,
}

#[repr(C)]
pub struct hda_quirk {
    pub subvendor: u16_,
    pub subdevice: u16_,
    pub subdevice_mask: u16_,
    pub value: c_int,
    pub name: *const c_char,
    pub match_codec_ssid: c_int,
}

#[repr(C)]
pub struct hda_model_fixup {
    pub id: c_int,
    pub name: *const c_char,
}

unsafe extern "C" {
    fn snd_hda_query_pin_caps(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn snd_hda_get_int_hint(codec: *mut hda_codec, key: *const c_char, valp: *mut c_int) -> c_int;
    fn snd_hda_codec_get_pincfg(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn snd_hda_codec_set_pincfg(codec: *mut hda_codec, nid: hda_nid_t, val: u32_);
    fn snd_hda_set_pin_ctl_cache(codec: *mut hda_codec, nid: hda_nid_t, val: u32_);
    fn nid_has_volume(codec: *mut hda_codec, nid: hda_nid_t, dir: c_uint) -> c_int;
    fn get_wcaps(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn get_wcaps_type(wcaps: c_uint) -> c_uint;
    fn get_defcfg_sequence(def_conf: c_uint) -> c_int;
    fn get_defcfg_association(def_conf: c_uint) -> c_int;
    fn get_defcfg_connect(def_conf: c_uint) -> c_uint;
    fn get_defcfg_location(def_conf: c_uint) -> c_uint;
    fn get_defcfg_device(def_conf: c_uint) -> c_uint;
    /* Rust-side expansion dependency for for_each_hda_codec_node(nid, codec). */
    fn for_each_hda_codec_node_next(codec: *mut hda_codec, nid: *mut hda_nid_t) -> bool;
    fn snd_array_new(array: *mut snd_array) -> *mut *const hda_verb;
    fn snd_hda_sequence_write(codec: *mut hda_codec, list: *const hda_verb);
    fn codec_info(codec: *mut hda_codec, fmt: *const c_char, ...);
    fn codec_dbg(codec: *mut hda_codec, fmt: *const c_char, ...);
    fn codec_err(codec: *mut hda_codec, fmt: *const c_char, ...);
    fn sort(
        base: *mut c_void,
        num: usize,
        size: usize,
        cmp: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
        swap_func: *mut c_void,
    );
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
}

unsafe fn snd_array_elem<T>(array: *mut snd_array, i: c_int) -> *mut T {
    ((*array).list as *mut T).add(i as usize)
}

/* Helper for automatic pin configuration */

unsafe fn is_in_nid_list(nid: hda_nid_t, mut list: *const hda_nid_t) -> c_int {
    while *list != 0 {
        if *list == nid {
            return 1;
        }
        list = list.add(1);
    }
    0
}

unsafe extern "C" fn compare_seq(ap: *const c_void, bp: *const c_void) -> c_int {
    let a = ap as *const auto_out_pin;
    let b = bp as *const auto_out_pin;
    ((*a).seq as c_int).wrapping_sub((*b).seq as c_int)
}

/*
 * Sort an associated group of pins according to their sequence numbers.
 * then store it to a pin array.
 */
unsafe fn sort_pins_by_sequence(pins: *mut hda_nid_t, list: *mut auto_out_pin, num_pins: c_int) {
    sort(
        list as *mut c_void,
        num_pins as usize,
        size_of::<auto_out_pin>(),
        Some(compare_seq),
        null_mut(),
    );
    for i in 0..num_pins {
        *pins.add(i as usize) = (*list.add(i as usize)).pin;
    }
}

/* add the found input-pin to the cfg->inputs[] table */
unsafe fn add_auto_cfg_input_pin(
    codec: *mut hda_codec,
    cfg: *mut auto_pin_cfg,
    nid: hda_nid_t,
    type_: c_int,
) {
    if (*cfg).num_inputs < AUTO_CFG_MAX_INS as c_int {
        let item = &mut (*cfg).inputs[(*cfg).num_inputs as usize];
        item.pin = nid;
        item.type_ = type_;
        item.has_boost_on_pin = nid_has_volume(codec, nid, HDA_INPUT);
        (*cfg).num_inputs += 1;
    }
}

unsafe extern "C" fn compare_input_type(ap: *const c_void, bp: *const c_void) -> c_int {
    let a = ap as *const auto_pin_cfg_item;
    let b = bp as *const auto_pin_cfg_item;
    if (*a).type_ != (*b).type_ {
        return (*a).type_.wrapping_sub((*b).type_);
    }

    /* If has both hs_mic and hp_mic, pick the hs_mic ahead of hp_mic. */
    if (*a).is_headset_mic != 0 && (*b).is_headphone_mic != 0 {
        return -1; /* don't swap */
    } else if (*a).is_headphone_mic != 0 && (*b).is_headset_mic != 0 {
        return 1; /* swap */
    }

    /* In case one has boost and the other one has not,
       pick the one with boost first. */
    if (*a).has_boost_on_pin != (*b).has_boost_on_pin {
        return (*b).has_boost_on_pin.wrapping_sub((*a).has_boost_on_pin);
    }

    /* Keep the original order */
    (*a).order.wrapping_sub((*b).order)
}

/* Reorder the surround channels
 * ALSA sequence is front/surr/clfe/side
 * HDA sequence is:
 *    4-ch: front/surr  =>  OK as it is
 *    6-ch: front/clfe/surr
 *    8-ch: front/clfe/rear/side|fc
 */
unsafe fn reorder_outputs(nums: c_uint, pins: *mut hda_nid_t) {
    match nums {
        3 | 4 => {
            let tmp = *pins.add(1);
            *pins.add(1) = *pins.add(2);
            *pins.add(2) = tmp;
        }
        _ => {}
    }
}

/* check whether the given pin has a proper pin I/O capability bit */
unsafe fn check_pincap_validity(codec: *mut hda_codec, pin: hda_nid_t, dev: c_uint) -> bool_ {
    let pincap = snd_hda_query_pin_caps(codec, pin);

    /* some old hardware don't return the proper pincaps */
    if pincap == 0 {
        return true;
    }

    match dev {
        AC_JACK_LINE_OUT | AC_JACK_SPEAKER | AC_JACK_HP_OUT | AC_JACK_SPDIF_OUT
        | AC_JACK_DIG_OTHER_OUT => (pincap & AC_PINCAP_OUT) != 0,
        _ => (pincap & AC_PINCAP_IN) != 0,
    }
}

unsafe fn can_be_headset_mic(
    codec: *mut hda_codec,
    item: *mut auto_pin_cfg_item,
    seq_number: c_int,
) -> bool_ {
    let attr: c_int;
    let def_conf: c_uint;
    if (*item).type_ != AUTO_PIN_MIC {
        return false;
    }

    if (*item).is_headset_mic != 0 || (*item).is_headphone_mic != 0 {
        return false; /* Already assigned */
    }

    def_conf = snd_hda_codec_get_pincfg(codec, (*item).pin);
    attr = snd_hda_get_input_pin_attr(def_conf);
    if attr <= INPUT_PIN_ATTR_DOCK {
        return false;
    }

    if seq_number >= 0 {
        let seq = get_defcfg_sequence(def_conf);
        if seq != seq_number {
            return false;
        }
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_parse_pin_defcfg(
    codec: *mut hda_codec,
    cfg: *mut auto_pin_cfg,
    ignore_nids: *const hda_nid_t,
    mut cond_flags: c_uint,
) -> c_int {
    let mut nid: hda_nid_t;
    let mut seq: i16;
    let mut assoc_line_out: i16;
    let mut line_out: [auto_out_pin; AUTO_CFG_MAX_OUTS] = zeroed();
    let mut speaker_out: [auto_out_pin; AUTO_CFG_MAX_OUTS] = zeroed();
    let mut hp_out: [auto_out_pin; AUTO_CFG_MAX_OUTS] = zeroed();
    let mut i: c_int = 0;

    if snd_hda_get_int_hint(codec, c"parser_flags".as_ptr(), &mut i) == 0 {
        cond_flags = i as c_uint;
    }

    write_bytes(cfg as *mut u8, 0, size_of::<auto_pin_cfg>());
    assoc_line_out = 0;

    nid = 0;
    /* for_each_hda_codec_node(nid, codec) */
    while for_each_hda_codec_node_next(codec, &mut nid) {
        let wid_caps = get_wcaps(codec, nid);
        let wid_type = get_wcaps_type(wid_caps);
        let def_conf: c_uint;
        let mut assoc: i16;
        let loc: i16;
        let conn: i16;
        let mut dev: i16;

        /* read all default configuration for pin complex */
        if wid_type != AC_WID_PIN {
            nid = nid.wrapping_add(1);
            continue;
        }
        /* ignore the given nids (e.g. pc-beep returns error) */
        if !ignore_nids.is_null() && is_in_nid_list(nid, ignore_nids) != 0 {
            nid = nid.wrapping_add(1);
            continue;
        }

        def_conf = snd_hda_codec_get_pincfg(codec, nid);
        conn = get_defcfg_connect(def_conf) as i16;
        if conn as c_uint == AC_JACK_PORT_NONE {
            nid = nid.wrapping_add(1);
            continue;
        }
        loc = get_defcfg_location(def_conf) as i16;
        dev = get_defcfg_device(def_conf) as i16;

        /* workaround for buggy BIOS setups */
        if dev as c_uint == AC_JACK_LINE_OUT {
            if conn as c_uint == AC_JACK_PORT_FIXED || conn as c_uint == AC_JACK_PORT_BOTH {
                dev = AC_JACK_SPEAKER as i16;
            }
        }

        if !check_pincap_validity(codec, nid, dev as c_uint) {
            nid = nid.wrapping_add(1);
            continue;
        }

        match dev as c_uint {
            AC_JACK_LINE_OUT => {
                seq = get_defcfg_sequence(def_conf) as i16;
                assoc = get_defcfg_association(def_conf) as i16;

                if (wid_caps & AC_WCAP_STEREO) == 0 {
                    if (*cfg).mono_out_pin == 0 {
                        (*cfg).mono_out_pin = nid;
                    }
                }
                if assoc == 0 {
                    nid = nid.wrapping_add(1);
                    continue;
                }
                if assoc_line_out == 0 {
                    assoc_line_out = assoc;
                } else if assoc_line_out != assoc {
                    codec_info(codec, c"ignore pin 0x%x with mismatching assoc# 0x%x vs 0x%x\n".as_ptr(), nid as c_uint, assoc as c_uint, assoc_line_out as c_uint);
                    nid = nid.wrapping_add(1);
                    continue;
                }
                if (*cfg).line_outs >= AUTO_CFG_MAX_OUTS as c_int {
                    codec_info(codec, c"ignore pin 0x%x, too many assigned pins\n".as_ptr(), nid as c_uint);
                    nid = nid.wrapping_add(1);
                    continue;
                }
                line_out[(*cfg).line_outs as usize].pin = nid;
                line_out[(*cfg).line_outs as usize].seq = seq;
                (*cfg).line_outs += 1;
            }
            AC_JACK_SPEAKER => {
                seq = get_defcfg_sequence(def_conf) as i16;
                assoc = get_defcfg_association(def_conf) as i16;
                if (*cfg).speaker_outs >= AUTO_CFG_MAX_OUTS as c_int {
                    codec_info(codec, c"ignore pin 0x%x, too many assigned pins\n".as_ptr(), nid as c_uint);
                    nid = nid.wrapping_add(1);
                    continue;
                }
                speaker_out[(*cfg).speaker_outs as usize].pin = nid;
                speaker_out[(*cfg).speaker_outs as usize].seq = ((assoc << 4) | seq) as i16;
                (*cfg).speaker_outs += 1;
            }
            AC_JACK_HP_OUT => {
                seq = get_defcfg_sequence(def_conf) as i16;
                assoc = get_defcfg_association(def_conf) as i16;
                if (*cfg).hp_outs >= AUTO_CFG_MAX_OUTS as c_int {
                    codec_info(codec, c"ignore pin 0x%x, too many assigned pins\n".as_ptr(), nid as c_uint);
                    nid = nid.wrapping_add(1);
                    continue;
                }
                hp_out[(*cfg).hp_outs as usize].pin = nid;
                hp_out[(*cfg).hp_outs as usize].seq = ((assoc << 4) | seq) as i16;
                (*cfg).hp_outs += 1;
            }
            AC_JACK_MIC_IN => add_auto_cfg_input_pin(codec, cfg, nid, AUTO_PIN_MIC),
            AC_JACK_LINE_IN => add_auto_cfg_input_pin(codec, cfg, nid, AUTO_PIN_LINE_IN),
            AC_JACK_CD => add_auto_cfg_input_pin(codec, cfg, nid, AUTO_PIN_CD),
            AC_JACK_AUX => add_auto_cfg_input_pin(codec, cfg, nid, AUTO_PIN_AUX),
            AC_JACK_SPDIF_OUT | AC_JACK_DIG_OTHER_OUT => {
                if (*cfg).dig_outs >= (*cfg).dig_out_pins.len() as c_int {
                    codec_info(codec, c"ignore pin 0x%x, too many assigned pins\n".as_ptr(), nid as c_uint);
                    nid = nid.wrapping_add(1);
                    continue;
                }
                (*cfg).dig_out_pins[(*cfg).dig_outs as usize] = nid;
                (*cfg).dig_out_type[(*cfg).dig_outs as usize] = if loc as c_uint == AC_JACK_LOC_HDMI {
                    HDA_PCM_TYPE_HDMI
                } else {
                    HDA_PCM_TYPE_SPDIF
                };
                (*cfg).dig_outs += 1;
            }
            AC_JACK_SPDIF_IN | AC_JACK_DIG_OTHER_IN => {
                (*cfg).dig_in_pin = nid;
                if loc as c_uint == AC_JACK_LOC_HDMI {
                    (*cfg).dig_in_type = HDA_PCM_TYPE_HDMI;
                } else {
                    (*cfg).dig_in_type = HDA_PCM_TYPE_SPDIF;
                }
            }
            _ => {}
        }
        nid = nid.wrapping_add(1);
    }

    /* Find a pin that could be a headset or headphone mic */
    if (cond_flags & HDA_PINCFG_HEADSET_MIC) != 0 || (cond_flags & HDA_PINCFG_HEADPHONE_MIC) != 0 {
        let mut hsmic = (cond_flags & HDA_PINCFG_HEADSET_MIC) != 0;
        let mut hpmic = (cond_flags & HDA_PINCFG_HEADPHONE_MIC) != 0;
        i = 0;
        while (hsmic || hpmic) && i < (*cfg).num_inputs {
            if hsmic && can_be_headset_mic(codec, &mut (*cfg).inputs[i as usize], 0xc) {
                (*cfg).inputs[i as usize].is_headset_mic = 1;
                hsmic = false;
            } else if hpmic && can_be_headset_mic(codec, &mut (*cfg).inputs[i as usize], 0xd) {
                (*cfg).inputs[i as usize].is_headphone_mic = 1;
                hpmic = false;
            }
            i += 1;
        }

        /* If we didn't find our sequence number mark, fall back to any sequence number */
        i = 0;
        while (hsmic || hpmic) && i < (*cfg).num_inputs {
            if !can_be_headset_mic(codec, &mut (*cfg).inputs[i as usize], -1) {
                i += 1;
                continue;
            }
            if hsmic {
                (*cfg).inputs[i as usize].is_headset_mic = 1;
                hsmic = false;
            } else if hpmic {
                (*cfg).inputs[i as usize].is_headphone_mic = 1;
                hpmic = false;
            }
            i += 1;
        }

        if hsmic {
            codec_dbg(codec, c"Told to look for a headset mic, but didn't find any.\n".as_ptr());
        }
        if hpmic {
            codec_dbg(codec, c"Told to look for a headphone mic, but didn't find any.\n".as_ptr());
        }
    }

    if (*cfg).line_outs == 0
        && (*cfg).hp_outs > 1
        && (cond_flags & HDA_PINCFG_NO_HP_FIXUP) == 0
    {
        i = 0;
        while i < (*cfg).hp_outs {
            /* The real HPs should have the sequence 0x0f */
            if (hp_out[i as usize].seq & 0x0f) == 0x0f {
                i += 1;
                continue;
            }
            /* Move it to the line-out table */
            line_out[(*cfg).line_outs as usize] = hp_out[i as usize];
            (*cfg).line_outs += 1;
            (*cfg).hp_outs -= 1;
            copy(
                hp_out.as_ptr().add(i as usize + 1),
                hp_out.as_mut_ptr().add(i as usize),
                ((*cfg).hp_outs - i) as usize,
            );
        }
        write_bytes(
            hp_out.as_mut_ptr().add((*cfg).hp_outs as usize),
            0,
            AUTO_CFG_MAX_OUTS - (*cfg).hp_outs as usize,
        );
        if (*cfg).hp_outs == 0 {
            (*cfg).line_out_type = AUTO_PIN_HP_OUT;
        }
    }

    /* sort by sequence */
    sort_pins_by_sequence((*cfg).line_out_pins.as_mut_ptr(), line_out.as_mut_ptr(), (*cfg).line_outs);
    sort_pins_by_sequence((*cfg).speaker_pins.as_mut_ptr(), speaker_out.as_mut_ptr(), (*cfg).speaker_outs);
    sort_pins_by_sequence((*cfg).hp_pins.as_mut_ptr(), hp_out.as_mut_ptr(), (*cfg).hp_outs);

    if (*cfg).line_outs == 0 && (cond_flags & HDA_PINCFG_NO_LO_FIXUP) == 0 {
        if (*cfg).speaker_outs != 0 {
            (*cfg).line_outs = (*cfg).speaker_outs;
            copy_nonoverlapping((*cfg).speaker_pins.as_ptr(), (*cfg).line_out_pins.as_mut_ptr(), AUTO_CFG_MAX_OUTS);
            (*cfg).speaker_outs = 0;
            write_bytes((*cfg).speaker_pins.as_mut_ptr(), 0, AUTO_CFG_MAX_OUTS);
            (*cfg).line_out_type = AUTO_PIN_SPEAKER_OUT;
        } else if (*cfg).hp_outs != 0 {
            (*cfg).line_outs = (*cfg).hp_outs;
            copy_nonoverlapping((*cfg).hp_pins.as_ptr(), (*cfg).line_out_pins.as_mut_ptr(), AUTO_CFG_MAX_OUTS);
            (*cfg).hp_outs = 0;
            write_bytes((*cfg).hp_pins.as_mut_ptr(), 0, AUTO_CFG_MAX_OUTS);
            (*cfg).line_out_type = AUTO_PIN_HP_OUT;
        }
    }

    reorder_outputs((*cfg).line_outs as c_uint, (*cfg).line_out_pins.as_mut_ptr());
    reorder_outputs((*cfg).hp_outs as c_uint, (*cfg).hp_pins.as_mut_ptr());
    reorder_outputs((*cfg).speaker_outs as c_uint, (*cfg).speaker_pins.as_mut_ptr());

    /* sort inputs in the order of AUTO_PIN_* type */
    i = 0;
    while i < (*cfg).num_inputs {
        (*cfg).inputs[i as usize].order = i;
        i += 1;
    }
    sort(
        (*cfg).inputs.as_mut_ptr() as *mut c_void,
        (*cfg).num_inputs as usize,
        size_of::<auto_pin_cfg_item>(),
        Some(compare_input_type),
        null_mut(),
    );

    codec_info(codec, c"autoconfig for %s: line_outs=%d (0x%x/0x%x/0x%x/0x%x/0x%x) type:%s\n".as_ptr(), (*codec).core.chip_name, (*cfg).line_outs, (*cfg).line_out_pins[0] as c_uint, (*cfg).line_out_pins[1] as c_uint, (*cfg).line_out_pins[2] as c_uint, (*cfg).line_out_pins[3] as c_uint, (*cfg).line_out_pins[4] as c_uint, if (*cfg).line_out_type == AUTO_PIN_HP_OUT { c"hp".as_ptr() } else if (*cfg).line_out_type == AUTO_PIN_SPEAKER_OUT { c"speaker".as_ptr() } else { c"line".as_ptr() });
    codec_info(codec, c"   speaker_outs=%d (0x%x/0x%x/0x%x/0x%x/0x%x)\n".as_ptr(), (*cfg).speaker_outs, (*cfg).speaker_pins[0] as c_uint, (*cfg).speaker_pins[1] as c_uint, (*cfg).speaker_pins[2] as c_uint, (*cfg).speaker_pins[3] as c_uint, (*cfg).speaker_pins[4] as c_uint);
    codec_info(codec, c"   hp_outs=%d (0x%x/0x%x/0x%x/0x%x/0x%x)\n".as_ptr(), (*cfg).hp_outs, (*cfg).hp_pins[0] as c_uint, (*cfg).hp_pins[1] as c_uint, (*cfg).hp_pins[2] as c_uint, (*cfg).hp_pins[3] as c_uint, (*cfg).hp_pins[4] as c_uint);
    codec_info(codec, c"   mono: mono_out=0x%x\n".as_ptr(), (*cfg).mono_out_pin as c_uint);
    if (*cfg).dig_outs != 0 {
        codec_info(codec, c"   dig-out=0x%x/0x%x\n".as_ptr(), (*cfg).dig_out_pins[0] as c_uint, (*cfg).dig_out_pins[1] as c_uint);
    }
    codec_info(codec, c"   inputs:\n".as_ptr());
    i = 0;
    while i < (*cfg).num_inputs {
        codec_info(codec, c"     %s=0x%x\n".as_ptr(), hda_get_autocfg_input_label(codec, cfg, i), (*cfg).inputs[i as usize].pin as c_uint);
        i += 1;
    }
    if (*cfg).dig_in_pin != 0 {
        codec_info(codec, c"   dig-in=0x%x\n".as_ptr(), (*cfg).dig_in_pin as c_uint);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_get_input_pin_attr(def_conf: c_uint) -> c_int {
    let loc = get_defcfg_location(def_conf);
    let conn = get_defcfg_connect(def_conf);
    if conn == AC_JACK_PORT_NONE {
        return INPUT_PIN_ATTR_UNUSED;
    }
    /* Windows may claim the internal mic to be BOTH, too */
    if conn == AC_JACK_PORT_FIXED || conn == AC_JACK_PORT_BOTH {
        return INPUT_PIN_ATTR_INT;
    }
    if (loc & 0x30) == AC_JACK_LOC_INTERNAL {
        return INPUT_PIN_ATTR_INT;
    }
    if (loc & 0x30) == AC_JACK_LOC_SEPARATE {
        return INPUT_PIN_ATTR_DOCK;
    }
    if loc == AC_JACK_LOC_REAR {
        return INPUT_PIN_ATTR_REAR;
    }
    if loc == AC_JACK_LOC_FRONT {
        return INPUT_PIN_ATTR_FRONT;
    }
    INPUT_PIN_ATTR_NORMAL
}

unsafe fn hda_get_input_pin_label(
    codec: *mut hda_codec,
    item: *const auto_pin_cfg_item,
    pin: hda_nid_t,
    check_location: bool_,
) -> *const c_char {
    static mic_names: [*const c_char; 5] = [
        c"Internal Mic".as_ptr(),
        c"Dock Mic".as_ptr(),
        c"Mic".as_ptr(),
        c"Rear Mic".as_ptr(),
        c"Front Mic".as_ptr(),
    ];
    let def_conf = snd_hda_codec_get_pincfg(codec, pin);

    match get_defcfg_device(def_conf) {
        AC_JACK_MIC_IN => {
            if !item.is_null() && (*item).is_headset_mic != 0 {
                return c"Headset Mic".as_ptr();
            }
            if !item.is_null() && (*item).is_headphone_mic != 0 {
                return c"Headphone Mic".as_ptr();
            }
            if !check_location {
                return c"Mic".as_ptr();
            }
            let attr = snd_hda_get_input_pin_attr(def_conf);
            if attr == 0 {
                return c"None".as_ptr();
            }
            mic_names[(attr - 1) as usize]
        }
        AC_JACK_LINE_IN => {
            if !check_location {
                return c"Line".as_ptr();
            }
            let attr = snd_hda_get_input_pin_attr(def_conf);
            if attr == 0 {
                return c"None".as_ptr();
            }
            if attr == INPUT_PIN_ATTR_DOCK {
                return c"Dock Line".as_ptr();
            }
            c"Line".as_ptr()
        }
        AC_JACK_AUX => c"Aux".as_ptr(),
        AC_JACK_CD => c"CD".as_ptr(),
        AC_JACK_SPDIF_IN => c"SPDIF In".as_ptr(),
        AC_JACK_DIG_OTHER_IN => c"Digital In".as_ptr(),
        AC_JACK_HP_OUT => c"Headphone Mic".as_ptr(),
        _ => c"Misc".as_ptr(),
    }
}

unsafe fn check_mic_location_need(codec: *mut hda_codec, cfg: *const auto_pin_cfg, input: c_int) -> c_int {
    let mut attr: c_int;
    let mut i: c_int;

    let mut defc = snd_hda_codec_get_pincfg(codec, (*cfg).inputs[input as usize].pin);
    attr = snd_hda_get_input_pin_attr(defc);
    /* for internal or docking mics, we need locations */
    if attr <= INPUT_PIN_ATTR_NORMAL {
        return 1;
    }

    attr = 0;
    i = 0;
    while i < (*cfg).num_inputs {
        defc = snd_hda_codec_get_pincfg(codec, (*cfg).inputs[i as usize].pin);
        let attr2 = snd_hda_get_input_pin_attr(defc);
        if attr2 >= INPUT_PIN_ATTR_NORMAL {
            if attr != 0 && attr != attr2 {
                return 1; /* different locations found */
            }
            attr = attr2;
        }
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_get_autocfg_input_label(
    codec: *mut hda_codec,
    cfg: *const auto_pin_cfg,
    input: c_int,
) -> *const c_char {
    let type_ = (*cfg).inputs[input as usize].type_;
    let mut has_multiple_pins = 0;

    if (input > 0 && (*cfg).inputs[(input - 1) as usize].type_ == type_)
        || (input < (*cfg).num_inputs - 1 && (*cfg).inputs[(input + 1) as usize].type_ == type_)
    {
        has_multiple_pins = 1;
    }
    if has_multiple_pins != 0 && type_ == AUTO_PIN_MIC {
        has_multiple_pins &= check_mic_location_need(codec, cfg, input);
    }
    has_multiple_pins |= (*codec).force_pin_prefix;
    hda_get_input_pin_label(
        codec,
        &(*cfg).inputs[input as usize],
        (*cfg).inputs[input as usize].pin,
        has_multiple_pins != 0,
    )
}

unsafe fn find_idx_in_nid_list(nid: hda_nid_t, list: *const hda_nid_t, nums: c_int) -> c_int {
    for i in 0..nums {
        if *list.add(i as usize) == nid {
            return i;
        }
    }
    -1
}

unsafe fn check_output_sfx(nid: hda_nid_t, pins: *const hda_nid_t, num_pins: c_int) -> *const c_char {
    static channel_sfx: [*const c_char; 4] = [
        c" Front".as_ptr(),
        c" Surround".as_ptr(),
        c" CLFE".as_ptr(),
        c" Side".as_ptr(),
    ];
    let i = find_idx_in_nid_list(nid, pins, num_pins);
    if i < 0 {
        return null();
    }
    if num_pins == 1 {
        return c"".as_ptr();
    }
    if num_pins as usize > channel_sfx.len() {
        return c"".as_ptr();
    }
    channel_sfx[i as usize]
}

unsafe fn check_output_pfx(codec: *mut hda_codec, nid: hda_nid_t) -> *const c_char {
    let def_conf = snd_hda_codec_get_pincfg(codec, nid);
    let attr = snd_hda_get_input_pin_attr(def_conf);

    /* check the location */
    match attr {
        INPUT_PIN_ATTR_DOCK => c"Dock ".as_ptr(),
        INPUT_PIN_ATTR_FRONT => c"Front ".as_ptr(),
        _ => c"".as_ptr(),
    }
}

unsafe fn fill_audio_out_name(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    cfg: *const auto_pin_cfg,
    mut name: *const c_char,
    label: *mut c_char,
    maxlen: c_int,
) -> c_int {
    let def_conf = snd_hda_codec_get_pincfg(codec, nid);
    let attr = snd_hda_get_input_pin_attr(def_conf);
    let pfx: *const c_char;
    let mut sfx = c"".as_ptr();

    /* handle as a speaker if it's a fixed line-out */
    if strcmp(name, c"Line Out".as_ptr()) == 0 && attr == INPUT_PIN_ATTR_INT {
        name = c"Speaker".as_ptr();
    }
    pfx = check_output_pfx(codec, nid);

    if !cfg.is_null() {
        /* try to give a unique suffix if needed */
        sfx = check_output_sfx(nid, (*cfg).line_out_pins.as_ptr(), (*cfg).line_outs);
        if sfx.is_null() {
            sfx = check_output_sfx(nid, (*cfg).speaker_pins.as_ptr(), (*cfg).speaker_outs);
        }
        if sfx.is_null() {
            sfx = c"".as_ptr();
        }
    }
    snprintf(label, maxlen as usize, c"%s%s%s".as_ptr(), pfx, name, sfx);
    1
}

fn is_hdmi_cfg(conf: c_uint) -> bool_ {
    unsafe { get_defcfg_location(conf) == AC_JACK_LOC_HDMI }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_get_pin_label(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    cfg: *const auto_pin_cfg,
    label: *mut c_char,
    maxlen: c_int,
) -> c_int {
    let def_conf = snd_hda_codec_get_pincfg(codec, nid);
    let mut name: *const c_char = null();
    let mut i: c_int;
    let hdmi: bool_;

    if get_defcfg_connect(def_conf) == AC_JACK_PORT_NONE {
        return 0;
    }

    match get_defcfg_device(def_conf) {
        AC_JACK_LINE_OUT => return fill_audio_out_name(codec, nid, cfg, c"Line Out".as_ptr(), label, maxlen),
        AC_JACK_SPEAKER => return fill_audio_out_name(codec, nid, cfg, c"Speaker".as_ptr(), label, maxlen),
        AC_JACK_HP_OUT => return fill_audio_out_name(codec, nid, cfg, c"Headphone".as_ptr(), label, maxlen),
        AC_JACK_SPDIF_OUT | AC_JACK_DIG_OTHER_OUT => {
            hdmi = is_hdmi_cfg(def_conf);
            name = if hdmi { c"HDMI".as_ptr() } else { c"SPDIF".as_ptr() };
        }
        _ => {
            if !cfg.is_null() {
                i = 0;
                while i < (*cfg).num_inputs {
                    if (*cfg).inputs[i as usize].pin != nid {
                        i += 1;
                        continue;
                    }
                    name = hda_get_autocfg_input_label(codec, cfg, i);
                    if !name.is_null() {
                        break;
                    }
                    i += 1;
                }
            }
            if name.is_null() {
                name = hda_get_input_pin_label(codec, null(), nid, true);
            }
        }
    }
    if name.is_null() {
        return 0;
    }
    strscpy(label, name, maxlen as usize);
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_add_verbs(codec: *mut hda_codec, list: *const hda_verb) -> c_int {
    let v = snd_array_new(&mut (*codec).verbs);
    if v.is_null() {
        return -ENOMEM;
    }
    *v = list;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_apply_verbs(codec: *mut hda_codec) {
    let mut i = 0;
    while i < (*codec).verbs.used {
        let v = snd_array_elem::<*const hda_verb>(&mut (*codec).verbs, i);
        snd_hda_sequence_write(codec, *v);
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_apply_pincfgs(codec: *mut hda_codec, mut cfg: *const hda_pintbl) {
    while (*cfg).nid != 0 {
        snd_hda_codec_set_pincfg(codec, (*cfg).nid, (*cfg).val);
        cfg = cfg.add(1);
    }
}

unsafe fn set_pin_targets(codec: *mut hda_codec, mut cfg: *const hda_pintbl) {
    while (*cfg).nid != 0 {
        snd_hda_set_pin_ctl_cache(codec, (*cfg).nid, (*cfg).val);
        cfg = cfg.add(1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __snd_hda_apply_fixup(
    codec: *mut hda_codec,
    mut id: c_int,
    action: c_int,
    mut depth: c_int,
) {
    let modelname = (*codec).fixup_name;

    while id >= 0 {
        let fix = (*codec).fixup_list.add(id as usize);

        depth += 1;
        if depth > 10 {
            break;
        }
        if (*fix).chained_before != 0 {
            __snd_hda_apply_fixup(codec, (*fix).chain_id, action, depth + 1);
        }

        match (*fix).type_ {
            HDA_FIXUP_PINS => {
                if action != HDA_FIXUP_ACT_PRE_PROBE || (*fix).v.pins.is_null() {
                    /* break */
                } else {
                    codec_dbg(codec, c"%s: Apply pincfg for %s\n".as_ptr(), (*codec).core.chip_name, modelname);
                    snd_hda_apply_pincfgs(codec, (*fix).v.pins);
                }
            }
            HDA_FIXUP_VERBS => {
                if action != HDA_FIXUP_ACT_PROBE || (*fix).v.verbs.is_null() {
                    /* break */
                } else {
                    codec_dbg(codec, c"%s: Apply fix-verbs for %s\n".as_ptr(), (*codec).core.chip_name, modelname);
                    snd_hda_add_verbs(codec, (*fix).v.verbs);
                }
            }
            HDA_FIXUP_FUNC => {
                if let Some(func) = (*fix).v.func {
                    codec_dbg(codec, c"%s: Apply fix-func for %s\n".as_ptr(), (*codec).core.chip_name, modelname);
                    func(codec, fix, action);
                }
            }
            HDA_FIXUP_PINCTLS => {
                if action != HDA_FIXUP_ACT_PROBE || (*fix).v.pins.is_null() {
                    /* break */
                } else {
                    codec_dbg(codec, c"%s: Apply pinctl for %s\n".as_ptr(), (*codec).core.chip_name, modelname);
                    set_pin_targets(codec, (*fix).v.pins);
                }
            }
            _ => {
                codec_err(codec, c"%s: Invalid fixup type %d\n".as_ptr(), (*codec).core.chip_name, (*fix).type_);
            }
        }
        if (*fix).chained == 0 || (*fix).chained_before != 0 {
            break;
        }
        id = (*fix).chain_id;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_apply_fixup(codec: *mut hda_codec, action: c_int) {
    if !(*codec).fixup_list.is_null() {
        __snd_hda_apply_fixup(codec, (*codec).fixup_id, action, 0);
    }
}

unsafe fn pin_config_match(
    codec: *mut hda_codec,
    pins: *const hda_pintbl,
    match_all_pins: bool_,
) -> bool_ {
    let mut i = 0;

    while i < (*codec).init_pins.used {
        let pin = snd_array_elem::<hda_pincfg>(&mut (*codec).init_pins, i);
        let nid = (*pin).nid;
        let cfg = (*pin).cfg;
        let mut t_pins: *const hda_pintbl;
        let mut found: c_int;

        t_pins = pins;
        found = 0;
        while (*t_pins).nid != 0 {
            if (*t_pins).nid == nid {
                found = 1;
                if ((*t_pins).val & IGNORE_SEQ_ASSOC) == (cfg & IGNORE_SEQ_ASSOC) {
                    break;
                } else if (cfg & 0xf0000000) == 0x40000000
                    && ((*t_pins).val & 0xf0000000) == 0x40000000
                {
                    break;
                } else {
                    return false;
                }
            }
            t_pins = t_pins.add(1);
        }
        if match_all_pins && found == 0 && (cfg & 0xf0000000) != 0x40000000 {
            return false;
        }
        i += 1;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_pick_pin_fixup(
    codec: *mut hda_codec,
    pin_quirk: *const snd_hda_pin_quirk,
    fixlist: *const hda_fixup,
    match_all_pins: bool_,
) {
    let mut pq: *const snd_hda_pin_quirk;
    let mut name: *const c_char = null();

    if (*codec).fixup_id != HDA_FIXUP_ID_NOT_SET {
        return;
    }

    pq = pin_quirk;
    while (*pq).subvendor != 0 {
        if ((*codec).core.subsystem_id & 0xffff0000) != ((*pq).subvendor << 16) {
            pq = pq.add(1);
            continue;
        }
        if (*codec).core.vendor_id != (*pq).codec {
            pq = pq.add(1);
            continue;
        }
        if pin_config_match(codec, (*pq).pins, match_all_pins) {
            (*codec).fixup_id = (*pq).value;
            /* CONFIG_SND_DEBUG_VERBOSE: codec->fixup_name = pq->name; name = pq->name; */
            codec_info(codec, c"%s: picked fixup %s (pin match)\n".as_ptr(), (*codec).core.chip_name, if !name.is_null() { name } else { c"".as_ptr() });
            (*codec).fixup_list = fixlist;
            return;
        }
        pq = pq.add(1);
    }
}

/* check whether the given quirk entry matches with vendor/device pair */
unsafe fn hda_quirk_match(vendor: u16_, device: u16_, q: *const hda_quirk) -> bool_ {
    if (*q).subvendor != vendor {
        return false;
    }
    (*q).subdevice == 0 || (device & (*q).subdevice_mask) == (*q).subdevice
}

/* look through the quirk list and return the matching entry */
unsafe fn hda_quirk_lookup_id(
    vendor: u16_,
    device: u16_,
    list: *const hda_quirk,
) -> *const hda_quirk {
    let mut q: *const hda_quirk;

    q = list;
    while (*q).subvendor != 0 || (*q).subdevice != 0 {
        if hda_quirk_match(vendor, device, q) {
            return q;
        }
        q = q.add(1);
    }
    null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_pick_fixup(
    codec: *mut hda_codec,
    mut models: *const hda_model_fixup,
    quirk: *const hda_quirk,
    mut fixlist: *const hda_fixup,
) {
    let mut q: *const hda_quirk;
    let mut id = HDA_FIXUP_ID_NOT_SET;
    let mut name: *const c_char = null();
    let mut type_: *const c_char = null();
    let mut vendor: c_uint = 0;
    let mut device: c_uint = 0;
    let mut pci_vendor: u16_ = 0;
    let mut pci_device: u16_ = 0;
    let codec_vendor: u16_;
    let codec_device: u16_;

    if (*codec).fixup_id != HDA_FIXUP_ID_NOT_SET {
        return;
    }

    /* when model=nofixup is given, don't pick up any fixups */
    if !(*codec).modelname.is_null() && strcmp((*codec).modelname, c"nofixup".as_ptr()) == 0 {
        id = HDA_FIXUP_ID_NO_FIXUP;
        fixlist = null();
        codec_info(codec, c"%s: picked no fixup (nofixup specified)\n".as_ptr(), (*codec).core.chip_name);
        (*codec).fixup_id = id;
        (*codec).fixup_list = fixlist;
        (*codec).fixup_name = name;
        return;
    }

    /* match with the model name string */
    if !(*codec).modelname.is_null() && !models.is_null() {
        while !(*models).name.is_null() {
            if strcmp((*codec).modelname, (*models).name) == 0 {
                id = (*models).id;
                name = (*models).name;
                codec_info(codec, c"%s: picked fixup %s (model specified)\n".as_ptr(), (*codec).core.chip_name, name);
                (*codec).fixup_id = id;
                (*codec).fixup_list = fixlist;
                (*codec).fixup_name = name;
                return;
            }
            models = models.add(1);
        }
    }

    if quirk.is_null() {
        return;
    }

    if !(*codec).bus.is_null() && !(*(*codec).bus).pci.is_null() {
        pci_vendor = (*(*(*codec).bus).pci).subsystem_vendor;
        pci_device = (*(*(*codec).bus).pci).subsystem_device;
    }

    codec_vendor = ((*codec).core.subsystem_id >> 16) as u16_;
    codec_device = ((*codec).core.subsystem_id & 0xffff) as u16_;

    /* match with the SSID alias given by the model string "XXXX:YYYY" */
    if !(*codec).modelname.is_null()
        && sscanf((*codec).modelname, c"%04x:%04x".as_ptr(), &mut vendor, &mut device) == 2
    {
        q = hda_quirk_lookup_id(vendor as u16_, device as u16_, quirk);
        if !q.is_null() {
            type_ = c"alias SSID".as_ptr();
            id = (*q).value;
            codec_info(codec, c"%s: picked fixup %s for %s %04x:%04x\n".as_ptr(), (*codec).core.chip_name, if !name.is_null() { name } else { c"".as_ptr() }, type_, (*q).subvendor as c_uint, (*q).subdevice as c_uint);
            (*codec).fixup_id = id;
            (*codec).fixup_list = fixlist;
            (*codec).fixup_name = name;
            return;
        }
    }

    /* match primarily with the PCI SSID */
    q = quirk;
    while (*q).subvendor != 0 || (*q).subdevice != 0 {
        /* if the entry is specific to codec SSID, check with it */
        if pci_vendor == 0 || pci_device == 0 || (*q).match_codec_ssid != 0 {
            if hda_quirk_match(codec_vendor, codec_device, q) {
                type_ = c"codec SSID".as_ptr();
                id = (*q).value;
                codec_info(codec, c"%s: picked fixup %s for %s %04x:%04x\n".as_ptr(), (*codec).core.chip_name, if !name.is_null() { name } else { c"".as_ptr() }, type_, (*q).subvendor as c_uint, (*q).subdevice as c_uint);
                (*codec).fixup_id = id;
                (*codec).fixup_list = fixlist;
                (*codec).fixup_name = name;
                return;
            }
        } else if hda_quirk_match(pci_vendor, pci_device, q) {
            type_ = c"PCI SSID".as_ptr();
            id = (*q).value;
            codec_info(codec, c"%s: picked fixup %s for %s %04x:%04x\n".as_ptr(), (*codec).core.chip_name, if !name.is_null() { name } else { c"".as_ptr() }, type_, (*q).subvendor as c_uint, (*q).subdevice as c_uint);
            (*codec).fixup_id = id;
            (*codec).fixup_list = fixlist;
            (*codec).fixup_name = name;
            return;
        }
        q = q.add(1);
    }

    /* match with the codec SSID */
    q = hda_quirk_lookup_id(codec_vendor, codec_device, quirk);
    if !q.is_null() {
        type_ = c"codec SSID".as_ptr();
        id = (*q).value;
        /* CONFIG_SND_DEBUG_VERBOSE: name = q->name; */
        codec_info(codec, c"%s: picked fixup %s for %s %04x:%04x\n".as_ptr(), (*codec).core.chip_name, if !name.is_null() { name } else { c"".as_ptr() }, type_, (*q).subvendor as c_uint, (*q).subdevice as c_uint);
        (*codec).fixup_id = id;
        (*codec).fixup_list = fixlist;
        (*codec).fixup_name = name;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
