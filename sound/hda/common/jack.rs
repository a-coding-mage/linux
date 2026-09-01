// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Jack-detection handling for HD-audio
 *
 * Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
 */

// Translated from hda/common/jack.c. C header-provided types, constants,
// macros, and helpers are referenced here as external repository dependencies.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub type hda_nid_t = c_uint;
pub type u32 = c_uint;

#[repr(C)]
pub struct hda_codec {
    pub no_jack_detect: bool,
    pub no_trigger_sense: bool,
    pub inv_jack_detect: bool,
    pub eld_jack_detect: bool,
    pub jackpoll_interval: c_int,
    pub dp_mst: bool,
    pub jacktbl: snd_array,
    pub bus: *mut hda_bus,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_array {
    pub list: *mut hda_jack_tbl,
    pub used: c_int,
}

#[repr(C)]
pub struct hda_bus {
    pub shutdown: bool,
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_jack)>,
}

pub type hda_jack_callback_fn =
    Option<unsafe extern "C" fn(*mut hda_codec, *mut hda_jack_callback)>;

#[repr(C)]
pub struct hda_jack_callback {
    pub func: hda_jack_callback_fn,
    pub nid: hda_nid_t,
    pub dev_id: c_int,
    pub next: *mut hda_jack_callback,
    pub jack: *mut hda_jack_tbl,
    pub unsol_res: c_uint,
}

#[repr(C)]
pub struct hda_jack_tbl {
    pub nid: hda_nid_t,
    pub dev_id: c_int,
    pub jack_dirty: c_uint,
    pub tag: c_uint,
    pub jack_detect: c_uint,
    pub callback: *mut hda_jack_callback,
    pub pin_sense: u32,
    pub phantom_jack: c_uint,
    pub gating_jack: hda_nid_t,
    pub gated_jack: hda_nid_t,
    pub jack: *mut snd_jack,
    pub block_report: c_uint,
    pub button_state: c_int,
    pub type_: c_int,
    pub key_report_jack: hda_nid_t,
}

#[repr(C)]
pub struct hda_jack_keymap {
    pub type_: c_int,
    pub key: c_int,
}

#[repr(C)]
pub struct auto_pin_cfg {
    pub num_inputs: c_int,
    pub inputs: *const auto_pin_input,
    pub line_out_pins: *const hda_nid_t,
    pub line_outs: c_int,
    pub hp_pins: *const hda_nid_t,
    pub hp_outs: c_int,
    pub speaker_pins: *const hda_nid_t,
    pub speaker_outs: c_int,
    pub dig_out_pins: *const hda_nid_t,
    pub dig_outs: c_int,
    pub dig_in_pin: hda_nid_t,
    pub mono_out_pin: hda_nid_t,
}

#[repr(C)]
pub struct auto_pin_input {
    pub pin: hda_nid_t,
    pub is_headphone_mic: bool,
}

unsafe extern "C" {
    static AC_PINCAP_PRES_DETECT: u32;
    static AC_DEFCFG_MISC_NO_PRESENCE: c_uint;
    static AC_WCAP_UNSOL_CAP: c_uint;
    static AC_PINCAP_TRIG_REQ: u32;
    static AC_VERB_SET_PIN_SENSE: c_uint;
    static AC_VERB_GET_PIN_SENSE: c_uint;
    static AC_PINSENSE_PRESENCE: u32;
    static AC_PINSENSE_ELDV: u32;
    static AC_VERB_SET_UNSOLICITED_ENABLE: c_uint;
    static AC_USRSP_EN: c_uint;
    static AC_JACK_LINE_OUT: c_uint;
    static AC_JACK_SPEAKER: c_uint;
    static AC_JACK_HP_OUT: c_uint;
    static AC_JACK_SPDIF_OUT: c_uint;
    static AC_JACK_DIG_OTHER_OUT: c_uint;
    static AC_JACK_MIC_IN: c_uint;
    static SND_JACK_LINEOUT: c_int;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_AVOUT: c_int;
    static SND_JACK_MICROPHONE: c_int;
    static SND_JACK_LINEIN: c_int;
    static AC_JACK_PORT_NONE: c_uint;
    static AC_JACK_PORT_COMPLEX: c_uint;
    static SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize;
    static AC_UNSOL_RES_TAG: c_uint;
    static AC_UNSOL_RES_TAG_SHIFT: c_uint;
    static AC_UNSOL_RES_DE: c_uint;
    static AC_UNSOL_RES_DE_SHIFT: c_uint;
    static HDA_JACK_PHANTOM: c_int;
    static HDA_JACK_PRESENT: c_int;
    static HDA_JACK_NOT_PRESENT: c_int;
    static ENOMEM: c_int;
    static EINVAL: c_int;

    fn snd_hda_query_pin_caps(codec: *mut hda_codec, nid: hda_nid_t) -> u32;
    fn snd_hda_codec_get_pincfg(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn get_defcfg_misc(defcfg: c_uint) -> c_uint;
    fn get_wcaps(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn snd_hda_codec_read(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    ) -> u32;
    fn snd_array_new(array: *mut snd_array) -> *mut hda_jack_tbl;
    fn snd_array_free(array: *mut snd_array);
    fn snd_device_disconnect(card: *mut snd_card, device: *mut snd_jack) -> c_int;
    fn snd_device_free(card: *mut snd_card, device: *mut snd_jack) -> c_int;
    fn kfree(ptr: *mut c_void);
    fn ERR_PTR(err: c_int) -> *mut hda_jack_callback;
    fn PTR_ERR_OR_ZERO(ptr: *mut hda_jack_callback) -> c_int;
    fn snd_hda_codec_write_cache(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    ) -> c_int;
    fn snd_hda_jack_tbl_get(codec: *mut hda_codec, nid: hda_nid_t) -> *mut hda_jack_tbl;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, key: c_int) -> c_int;
    fn snd_jack_report(jack: *mut snd_jack, status: c_int);
    fn snd_jack_new(
        card: *mut snd_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut *mut snd_jack,
        initial_kctl: bool,
        phantom_jack: bool,
    ) -> c_int;
    fn get_defcfg_device(defcfg: c_uint) -> c_uint;
    fn get_defcfg_connect(defcfg: c_uint) -> c_uint;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn snd_hda_get_pin_label(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        cfg: *const auto_pin_cfg,
        name: *mut c_char,
        len: usize,
    );
    fn hda_append_suffix(name: *mut c_char, suffix: *const c_char, len: usize);
    fn snd_hda_jack_add_kctl(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        name: *const c_char,
        phantom_jack: bool,
        type_: c_int,
        keymap: *const hda_jack_keymap,
    ) -> c_int;
    fn auto_cfg_hp_outs(cfg: *const auto_pin_cfg) -> c_int;
    fn auto_cfg_hp_pins(cfg: *const auto_pin_cfg) -> *const hda_nid_t;
}

unsafe fn WARN_ON(condition: bool) -> bool {
    condition
}

unsafe fn kzalloc_obj_hda_jack_callback() -> *mut hda_jack_callback {
    extern "C" {
        fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    }
    calloc(1, core::mem::size_of::<hda_jack_callback>()) as *mut hda_jack_callback
}

unsafe fn get_jack_plug_state(sense: u32) -> bool {
    (sense & AC_PINSENSE_PRESENCE) != 0
}

/**
 * is_jack_detectable - Check whether the given pin is jack-detectable
 * @codec: the HDA codec
 * @nid: pin NID
 *
 * Check whether the given pin is capable to report the jack detection.
 * The jack detection might not work by various reasons, e.g. the jack
 * detection is prohibited in the codec level, the pin config has
 * AC_DEFCFG_MISC_NO_PRESENCE bit, no unsol support, etc.
 */
pub unsafe extern "C" fn is_jack_detectable(codec: *mut hda_codec, nid: hda_nid_t) -> bool {
    if (*codec).no_jack_detect {
        return false;
    }
    if (snd_hda_query_pin_caps(codec, nid) & AC_PINCAP_PRES_DETECT) == 0 {
        return false;
    }
    if (get_defcfg_misc(snd_hda_codec_get_pincfg(codec, nid)) & AC_DEFCFG_MISC_NO_PRESENCE) != 0 {
        return false;
    }
    if (get_wcaps(codec, nid) & AC_WCAP_UNSOL_CAP) == 0 && (*codec).jackpoll_interval == 0 {
        return false;
    }
    true
}

/* execute pin sense measurement */
unsafe fn read_pin_sense(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int) -> u32 {
    let pincap: u32;
    let mut val: u32;

    if !(*codec).no_trigger_sense {
        pincap = snd_hda_query_pin_caps(codec, nid);
        if (pincap & AC_PINCAP_TRIG_REQ) != 0 {
            /* need trigger? */
            snd_hda_codec_read(codec, nid, 0, AC_VERB_SET_PIN_SENSE, 0);
        }
    }
    val = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_PIN_SENSE, dev_id as c_uint);
    if (*codec).inv_jack_detect {
        val ^= AC_PINSENSE_PRESENCE;
    }
    if (*codec).eld_jack_detect {
        if (val & AC_PINSENSE_ELDV) != 0 {
            val |= AC_PINSENSE_PRESENCE;
        } else {
            val &= !AC_PINSENSE_PRESENCE;
        }
    }
    val
}

/**
 * snd_hda_jack_tbl_get_mst - query the jack-table entry for the given NID
 * @codec: the HDA codec
 * @nid: pin NID to refer to
 * @dev_id: pin device entry id
 */
pub unsafe extern "C" fn snd_hda_jack_tbl_get_mst(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dev_id: c_int,
) -> *mut hda_jack_tbl {
    let mut jack = (*codec).jacktbl.list;
    let mut i: c_int;

    if nid == 0 || jack.is_null() {
        return ptr::null_mut();
    }
    i = 0;
    while i < (*codec).jacktbl.used {
        if (*jack).nid == nid && (*jack).dev_id == dev_id {
            return jack;
        }
        i += 1;
        jack = jack.add(1);
    }
    ptr::null_mut()
}

/**
 * snd_hda_jack_tbl_get_from_tag - query the jack-table entry for the given tag
 * @codec: the HDA codec
 * @tag: tag value to refer to
 * @dev_id: pin device entry id
 */
pub unsafe extern "C" fn snd_hda_jack_tbl_get_from_tag(
    codec: *mut hda_codec,
    tag: u8,
    dev_id: c_int,
) -> *mut hda_jack_tbl {
    let mut jack = (*codec).jacktbl.list;
    let mut i: c_int;

    if tag == 0 || jack.is_null() {
        return ptr::null_mut();
    }
    i = 0;
    while i < (*codec).jacktbl.used {
        if (*jack).tag == tag as c_uint && (*jack).dev_id == dev_id {
            return jack;
        }
        i += 1;
        jack = jack.add(1);
    }
    ptr::null_mut()
}

unsafe fn any_jack_tbl_get_from_nid(codec: *mut hda_codec, nid: hda_nid_t) -> *mut hda_jack_tbl {
    let mut jack = (*codec).jacktbl.list;
    let mut i: c_int;

    if nid == 0 || jack.is_null() {
        return ptr::null_mut();
    }
    i = 0;
    while i < (*codec).jacktbl.used {
        if (*jack).nid == nid {
            return jack;
        }
        i += 1;
        jack = jack.add(1);
    }
    ptr::null_mut()
}

/**
 * snd_hda_jack_tbl_new - create a jack-table entry for the given NID
 * @codec: the HDA codec
 * @nid: pin NID to assign
 * @dev_id: pin device entry id
 */
unsafe fn snd_hda_jack_tbl_new(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dev_id: c_int,
) -> *mut hda_jack_tbl {
    let mut jack = snd_hda_jack_tbl_get_mst(codec, nid, dev_id);
    let existing_nid_jack = any_jack_tbl_get_from_nid(codec, nid);

    WARN_ON(dev_id != 0 && !(*codec).dp_mst);

    if !jack.is_null() {
        return jack;
    }
    jack = snd_array_new(&mut (*codec).jacktbl);
    if jack.is_null() {
        return ptr::null_mut();
    }
    (*jack).nid = nid;
    (*jack).dev_id = dev_id;
    (*jack).jack_dirty = 1;
    if !existing_nid_jack.is_null() {
        (*jack).tag = (*existing_nid_jack).tag;

        /*
         * Copy jack_detect from existing_nid_jack to avoid
         * snd_hda_jack_detect_enable_callback_mst() making multiple
         * SET_UNSOLICITED_ENABLE calls on the same pin.
         */
        (*jack).jack_detect = (*existing_nid_jack).jack_detect;
    } else {
        (*jack).tag = (*codec).jacktbl.used as c_uint;
    }

    jack
}

pub unsafe extern "C" fn snd_hda_jack_tbl_disconnect(codec: *mut hda_codec) {
    let mut jack = (*codec).jacktbl.list;
    let mut i: c_int = 0;

    while i < (*codec).jacktbl.used {
        if !(*(*codec).bus).shutdown && !(*jack).jack.is_null() {
            snd_device_disconnect((*codec).card, (*jack).jack);
        }
        i += 1;
        jack = jack.add(1);
    }
}

pub unsafe extern "C" fn snd_hda_jack_tbl_clear(codec: *mut hda_codec) {
    let mut jack = (*codec).jacktbl.list;
    let mut i: c_int = 0;

    while i < (*codec).jacktbl.used {
        let mut cb: *mut hda_jack_callback;
        let mut next: *mut hda_jack_callback;

        /* free jack instances manually when clearing/reconfiguring */
        if !(*(*codec).bus).shutdown && !(*jack).jack.is_null() {
            snd_device_free((*codec).card, (*jack).jack);
        }

        cb = (*jack).callback;
        while !cb.is_null() {
            next = (*cb).next;
            kfree(cb as *mut c_void);
            cb = next;
        }
        i += 1;
        jack = jack.add(1);
    }
    snd_array_free(&mut (*codec).jacktbl);
}

/* update the cached value and notification flag if needed */
unsafe fn jack_detect_update(codec: *mut hda_codec, jack: *mut hda_jack_tbl) {
    if (*jack).jack_dirty == 0 {
        return;
    }

    if (*jack).phantom_jack != 0 {
        (*jack).pin_sense = AC_PINSENSE_PRESENCE;
    } else {
        (*jack).pin_sense = read_pin_sense(codec, (*jack).nid, (*jack).dev_id);
    }

    /* A gating jack indicates the jack is invalid if gating is unplugged */
    if (*jack).gating_jack != 0
        && !snd_hda_jack_detect_mst(codec, (*jack).gating_jack, (*jack).dev_id)
    {
        (*jack).pin_sense &= !AC_PINSENSE_PRESENCE;
    }

    (*jack).jack_dirty = 0;

    /* If a jack is gated by this one update it. */
    if (*jack).gated_jack != 0 {
        let gated = snd_hda_jack_tbl_get_mst(codec, (*jack).gated_jack, (*jack).dev_id);
        if !gated.is_null() {
            (*gated).jack_dirty = 1;
            jack_detect_update(codec, gated);
        }
    }
}

/**
 * snd_hda_jack_set_dirty_all - Mark all the cached as dirty
 * @codec: the HDA codec
 *
 * This function sets the dirty flag to all entries of jack table.
 * It's called from the resume path in hda_codec.c.
 */
pub unsafe extern "C" fn snd_hda_jack_set_dirty_all(codec: *mut hda_codec) {
    let mut jack = (*codec).jacktbl.list;
    let mut i: c_int = 0;

    while i < (*codec).jacktbl.used {
        if (*jack).nid != 0 {
            (*jack).jack_dirty = 1;
        }
        i += 1;
        jack = jack.add(1);
    }
}

/**
 * snd_hda_jack_pin_sense - execute pin sense measurement
 * @codec: the CODEC to sense
 * @nid: the pin NID to sense
 * @dev_id: pin device entry id
 *
 * Execute necessary pin sense measurement and return its Presence Detect,
 * Impedance, ELD Valid etc. status bits.
 */
pub unsafe extern "C" fn snd_hda_jack_pin_sense(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dev_id: c_int,
) -> u32 {
    let jack = snd_hda_jack_tbl_get_mst(codec, nid, dev_id);
    if !jack.is_null() {
        jack_detect_update(codec, jack);
        return (*jack).pin_sense;
    }
    read_pin_sense(codec, nid, dev_id)
}

/**
 * snd_hda_jack_detect_state_mst - query pin Presence Detect status
 * @codec: the CODEC to sense
 * @nid: the pin NID to sense
 * @dev_id: pin device entry id
 *
 * Query and return the pin's Presence Detect status, as either
 * HDA_JACK_NOT_PRESENT, HDA_JACK_PRESENT or HDA_JACK_PHANTOM.
 */
pub unsafe extern "C" fn snd_hda_jack_detect_state_mst(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dev_id: c_int,
) -> c_int {
    let jack = snd_hda_jack_tbl_get_mst(codec, nid, dev_id);
    if !jack.is_null() && (*jack).phantom_jack != 0 {
        HDA_JACK_PHANTOM
    } else if (snd_hda_jack_pin_sense(codec, nid, dev_id) & AC_PINSENSE_PRESENCE) != 0 {
        HDA_JACK_PRESENT
    } else {
        HDA_JACK_NOT_PRESENT
    }
}

unsafe fn find_callback_from_list(
    jack: *mut hda_jack_tbl,
    func: hda_jack_callback_fn,
) -> *mut hda_jack_callback {
    let mut cb: *mut hda_jack_callback;

    if func.is_none() {
        return ptr::null_mut();
    }

    cb = (*jack).callback;
    while !cb.is_null() {
        if (*cb).func == func {
            return cb;
        }
        cb = (*cb).next;
    }

    ptr::null_mut()
}

/**
 * snd_hda_jack_detect_enable_callback_mst - enable the jack-detection
 * @codec: the HDA codec
 * @nid: pin NID to enable
 * @func: callback function to register
 * @dev_id: pin device entry id
 *
 * In the case of error, the return value will be a pointer embedded with
 * errno.  Check and handle the return value appropriately with standard
 * macros such as @IS_ERR() and @PTR_ERR().
 */
pub unsafe extern "C" fn snd_hda_jack_detect_enable_callback_mst(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dev_id: c_int,
    func: hda_jack_callback_fn,
) -> *mut hda_jack_callback {
    let jack: *mut hda_jack_tbl;
    let mut callback: *mut hda_jack_callback = ptr::null_mut();
    let err: c_int;

    jack = snd_hda_jack_tbl_new(codec, nid, dev_id);
    if jack.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    callback = find_callback_from_list(jack, func);

    if func.is_some() && callback.is_null() {
        callback = kzalloc_obj_hda_jack_callback();
        if callback.is_null() {
            return ERR_PTR(-ENOMEM);
        }
        (*callback).func = func;
        (*callback).nid = (*jack).nid;
        (*callback).dev_id = (*jack).dev_id;
        (*callback).next = (*jack).callback;
        (*jack).callback = callback;
    }

    if (*jack).jack_detect != 0 {
        return callback; /* already registered */
    }
    (*jack).jack_detect = 1;
    if (*codec).jackpoll_interval > 0 {
        return callback; /* No unsol if we're polling instead */
    }
    err = snd_hda_codec_write_cache(
        codec,
        nid,
        0,
        AC_VERB_SET_UNSOLICITED_ENABLE,
        AC_USRSP_EN | (*jack).tag,
    );
    if err < 0 {
        return ERR_PTR(err);
    }
    callback
}

/**
 * snd_hda_jack_detect_enable - Enable the jack detection on the given pin
 * @codec: the HDA codec
 * @nid: pin NID to enable jack detection
 * @dev_id: pin device entry id
 *
 * Enable the jack detection with the default callback.  Returns zero if
 * successful or a negative error code.
 */
pub unsafe extern "C" fn snd_hda_jack_detect_enable(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dev_id: c_int,
) -> c_int {
    PTR_ERR_OR_ZERO(snd_hda_jack_detect_enable_callback_mst(
        codec,
        nid,
        dev_id,
        None,
    ))
}

/**
 * snd_hda_jack_set_gating_jack - Set gating jack.
 * @codec: the HDA codec
 * @gated_nid: gated pin NID
 * @gating_nid: gating pin NID
 *
 * Indicates the gated jack is only valid when the gating jack is plugged.
 */
pub unsafe extern "C" fn snd_hda_jack_set_gating_jack(
    codec: *mut hda_codec,
    gated_nid: hda_nid_t,
    gating_nid: hda_nid_t,
) -> c_int {
    let gated = snd_hda_jack_tbl_new(codec, gated_nid, 0);
    let gating = snd_hda_jack_tbl_new(codec, gating_nid, 0);

    WARN_ON((*codec).dp_mst);

    if gated.is_null() || gating.is_null() {
        return -EINVAL;
    }

    (*gated).gating_jack = gating_nid;
    (*gating).gated_jack = gated_nid;

    0
}

/**
 * snd_hda_jack_bind_keymap - bind keys generated from one NID to another jack.
 * @codec: the HDA codec
 * @key_nid: key event is generated by this pin NID
 * @keymap: map of key type and key code
 * @jack_nid: key reports to the jack of this pin NID
 *
 * This function is used in the case of key is generated from one NID while is
 * reported to the jack of another NID.
 */
pub unsafe extern "C" fn snd_hda_jack_bind_keymap(
    codec: *mut hda_codec,
    key_nid: hda_nid_t,
    keymap: *const hda_jack_keymap,
    jack_nid: hda_nid_t,
) -> c_int {
    let key_gen = snd_hda_jack_tbl_get(codec, key_nid);
    let report_to = snd_hda_jack_tbl_get(codec, jack_nid);

    WARN_ON((*codec).dp_mst);

    if key_gen.is_null() || report_to.is_null() || (*report_to).jack.is_null() {
        return -EINVAL;
    }

    (*key_gen).key_report_jack = jack_nid;

    if !keymap.is_null() {
        let mut map = keymap;
        while (*map).type_ != 0 {
            snd_jack_set_key((*report_to).jack, (*map).type_, (*map).key);
            map = map.add(1);
        }
    }

    0
}

/**
 * snd_hda_jack_set_button_state - report button event to the hda_jack_tbl button_state.
 * @codec: the HDA codec
 * @jack_nid: the button event reports to the jack_tbl of this NID
 * @button_state: the button event captured by codec
 *
 * Codec driver calls this function to report the button event.
 */
pub unsafe extern "C" fn snd_hda_jack_set_button_state(
    codec: *mut hda_codec,
    jack_nid: hda_nid_t,
    button_state: c_int,
) {
    let jack = snd_hda_jack_tbl_get(codec, jack_nid);

    if jack.is_null() {
        return;
    }

    if (*jack).key_report_jack != 0 {
        let report_to = snd_hda_jack_tbl_get(codec, (*jack).key_report_jack);

        if !report_to.is_null() {
            (*report_to).button_state = button_state;
            return;
        }
    }

    (*jack).button_state = button_state;
}

/**
 * snd_hda_jack_report_sync - sync the states of all jacks and report if changed
 * @codec: the HDA codec
 */
pub unsafe extern "C" fn snd_hda_jack_report_sync(codec: *mut hda_codec) {
    let mut jack: *mut hda_jack_tbl;
    let mut i: c_int;
    let mut state: c_int;

    /* update all jacks at first */
    jack = (*codec).jacktbl.list;
    i = 0;
    while i < (*codec).jacktbl.used {
        if (*jack).nid != 0 {
            jack_detect_update(codec, jack);
        }
        i += 1;
        jack = jack.add(1);
    }

    /*
     * report the updated jacks; it's done after updating all jacks
     * to make sure that all gating jacks properly have been set
     */
    jack = (*codec).jacktbl.list;
    i = 0;
    while i < (*codec).jacktbl.used {
        if (*jack).nid != 0 {
            if (*jack).jack.is_null() || (*jack).block_report != 0 {
                i += 1;
                jack = jack.add(1);
                continue;
            }
            state = (*jack).button_state;
            if get_jack_plug_state((*jack).pin_sense) {
                state |= (*jack).type_;
            }
            snd_jack_report((*jack).jack, state);
            if (*jack).button_state != 0 {
                snd_jack_report((*jack).jack, state & !(*jack).button_state);
                (*jack).button_state = 0; /* button released */
            }
        }
        i += 1;
        jack = jack.add(1);
    }
}

/* guess the jack type from the pin-config */
unsafe fn get_input_jack_type(codec: *mut hda_codec, nid: hda_nid_t) -> c_int {
    let def_conf = snd_hda_codec_get_pincfg(codec, nid);
    match get_defcfg_device(def_conf) {
        x if x == AC_JACK_LINE_OUT => SND_JACK_LINEOUT,
        x if x == AC_JACK_SPEAKER => SND_JACK_LINEOUT,
        x if x == AC_JACK_HP_OUT => SND_JACK_HEADPHONE,
        x if x == AC_JACK_SPDIF_OUT => SND_JACK_AVOUT,
        x if x == AC_JACK_DIG_OTHER_OUT => SND_JACK_AVOUT,
        x if x == AC_JACK_MIC_IN => SND_JACK_MICROPHONE,
        _ => SND_JACK_LINEIN,
    }
}

unsafe extern "C" fn hda_free_jack_priv(jack: *mut snd_jack) {
    let jacks = (*jack).private_data as *mut hda_jack_tbl;
    (*jacks).nid = 0;
    (*jacks).jack = ptr::null_mut();
}

/**
 * snd_hda_jack_add_kctl_mst - Add a kctl for the given pin
 * @codec: the HDA codec
 * @nid: pin NID to assign
 * @dev_id : pin device entry id
 * @name: string name for the jack
 * @phantom_jack: flag to deal as a phantom jack
 * @type: jack type bits to be reported, 0 for guessing from pincfg
 * @keymap: optional jack / key mapping
 *
 * This assigns a jack-detection kctl to the given pin.  The kcontrol
 * will have the given name and index.
 */
pub unsafe extern "C" fn snd_hda_jack_add_kctl_mst(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dev_id: c_int,
    name: *const c_char,
    phantom_jack: bool,
    mut type_: c_int,
    keymap: *const hda_jack_keymap,
) -> c_int {
    let jack: *mut hda_jack_tbl;
    let mut err: c_int;
    let state: c_int;
    let mut buttons: c_int;

    jack = snd_hda_jack_tbl_new(codec, nid, dev_id);
    if jack.is_null() {
        return 0;
    }
    if !(*jack).jack.is_null() {
        return 0; /* already created */
    }

    if type_ == 0 {
        type_ = get_input_jack_type(codec, nid);
    }

    buttons = 0;
    if !keymap.is_null() {
        let mut map = keymap;
        while (*map).type_ != 0 {
            buttons |= (*map).type_;
            map = map.add(1);
        }
    }

    err = snd_jack_new(
        (*codec).card,
        name,
        type_ | buttons,
        &mut (*jack).jack,
        true,
        phantom_jack,
    );
    if err < 0 {
        return err;
    }

    (*jack).phantom_jack = (phantom_jack as c_uint) != 0 as c_uint;
    (*jack).type_ = type_;
    (*jack).button_state = 0;
    (*(*jack).jack).private_data = jack as *mut c_void;
    (*(*jack).jack).private_free = Some(hda_free_jack_priv);
    if !keymap.is_null() {
        let mut map = keymap;
        while (*map).type_ != 0 {
            snd_jack_set_key((*jack).jack, (*map).type_, (*map).key);
            map = map.add(1);
        }
    }

    state = snd_hda_jack_detect_mst(codec, nid, dev_id) as c_int;
    snd_jack_report((*jack).jack, if state != 0 { (*jack).type_ } else { 0 });

    0
}

unsafe fn add_jack_kctl(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    cfg: *const auto_pin_cfg,
    base_name: *const c_char,
) -> c_int {
    let def_conf: c_uint;
    let conn: c_uint;
    let mut name = [0 as c_char; 128];
    let err: c_int;
    let phantom_jack: bool;

    WARN_ON((*codec).dp_mst);

    if nid == 0 {
        return 0;
    }
    def_conf = snd_hda_codec_get_pincfg(codec, nid);
    conn = get_defcfg_connect(def_conf);
    if conn == AC_JACK_PORT_NONE {
        return 0;
    }
    phantom_jack = conn != AC_JACK_PORT_COMPLEX || !is_jack_detectable(codec, nid);

    if !base_name.is_null() {
        strscpy(name.as_mut_ptr(), base_name, name.len());
    } else {
        snd_hda_get_pin_label(codec, nid, cfg, name.as_mut_ptr(), name.len());
    }
    if phantom_jack {
        /* Example final name: "Internal Mic Phantom Jack" */
        hda_append_suffix(name.as_mut_ptr(), b" Phantom\0".as_ptr() as *const c_char, name.len());
    }
    err = snd_hda_jack_add_kctl(codec, nid, name.as_ptr(), phantom_jack, 0, ptr::null());
    if err < 0 {
        return err;
    }

    if !phantom_jack {
        return snd_hda_jack_detect_enable(codec, nid, 0);
    }
    0
}

/**
 * snd_hda_jack_add_kctls - Add kctls for all pins included in the given pincfg
 * @codec: the HDA codec
 * @cfg: pin config table to parse
 */
pub unsafe extern "C" fn snd_hda_jack_add_kctls(
    codec: *mut hda_codec,
    cfg: *const auto_pin_cfg,
) -> c_int {
    let mut p: *const hda_nid_t;
    let mut i: c_int;
    let mut err: c_int;

    i = 0;
    while i < (*cfg).num_inputs {
        /*
         * If we have headphone mics; make sure they get the right name
         * before grabbed by output pins
         */
        if (*(*cfg).inputs.add(i as usize)).is_headphone_mic {
            if auto_cfg_hp_outs(cfg) == 1 {
                err = add_jack_kctl(
                    codec,
                    *auto_cfg_hp_pins(cfg),
                    cfg,
                    b"Headphone Mic\0".as_ptr() as *const c_char,
                );
            } else {
                err = add_jack_kctl(
                    codec,
                    (*(*cfg).inputs.add(i as usize)).pin,
                    cfg,
                    b"Headphone Mic\0".as_ptr() as *const c_char,
                );
            }
        } else {
            err = add_jack_kctl(codec, (*(*cfg).inputs.add(i as usize)).pin, cfg, ptr::null());
        }
        if err < 0 {
            return err;
        }
        i += 1;
    }

    i = 0;
    p = (*cfg).line_out_pins;
    while i < (*cfg).line_outs {
        err = add_jack_kctl(codec, *p, cfg, ptr::null());
        if err < 0 {
            return err;
        }
        i += 1;
        p = p.add(1);
    }
    i = 0;
    p = (*cfg).hp_pins;
    while i < (*cfg).hp_outs {
        if *p == *(*cfg).line_out_pins {
            /* might be duplicated */
            break;
        }
        err = add_jack_kctl(codec, *p, cfg, ptr::null());
        if err < 0 {
            return err;
        }
        i += 1;
        p = p.add(1);
    }
    i = 0;
    p = (*cfg).speaker_pins;
    while i < (*cfg).speaker_outs {
        if *p == *(*cfg).line_out_pins {
            /* might be duplicated */
            break;
        }
        err = add_jack_kctl(codec, *p, cfg, ptr::null());
        if err < 0 {
            return err;
        }
        i += 1;
        p = p.add(1);
    }
    i = 0;
    p = (*cfg).dig_out_pins;
    while i < (*cfg).dig_outs {
        err = add_jack_kctl(codec, *p, cfg, ptr::null());
        if err < 0 {
            return err;
        }
        i += 1;
        p = p.add(1);
    }
    err = add_jack_kctl(codec, (*cfg).dig_in_pin, cfg, ptr::null());
    if err < 0 {
        return err;
    }
    err = add_jack_kctl(codec, (*cfg).mono_out_pin, cfg, ptr::null());
    if err < 0 {
        return err;
    }
    0
}

unsafe fn call_jack_callback(codec: *mut hda_codec, res: c_uint, jack: *mut hda_jack_tbl) {
    let mut cb: *mut hda_jack_callback;

    cb = (*jack).callback;
    while !cb.is_null() {
        (*cb).jack = jack;
        (*cb).unsol_res = res;
        if let Some(func) = (*cb).func {
            func(codec, cb);
        }
        cb = (*cb).next;
    }
    if (*jack).gated_jack != 0 {
        let gated = snd_hda_jack_tbl_get_mst(codec, (*jack).gated_jack, (*jack).dev_id);
        if !gated.is_null() {
            cb = (*gated).callback;
            while !cb.is_null() {
                (*cb).jack = gated;
                (*cb).unsol_res = res;
                if let Some(func) = (*cb).func {
                    func(codec, cb);
                }
                cb = (*cb).next;
            }
        }
    }
}

/**
 * snd_hda_jack_unsol_event - Handle an unsolicited event
 * @codec: the HDA codec
 * @res: the unsolicited event data
 */
pub unsafe extern "C" fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: c_uint) {
    let event: *mut hda_jack_tbl;
    let tag = ((res & AC_UNSOL_RES_TAG) >> AC_UNSOL_RES_TAG_SHIFT) as c_int;

    if (*codec).dp_mst {
        let dev_entry = ((res & AC_UNSOL_RES_DE) >> AC_UNSOL_RES_DE_SHIFT) as c_int;

        event = snd_hda_jack_tbl_get_from_tag(codec, tag as u8, dev_entry);
    } else {
        event = snd_hda_jack_tbl_get_from_tag(codec, tag as u8, 0);
    }
    if event.is_null() {
        return;
    }

    if (*event).key_report_jack != 0 {
        let report_to =
            snd_hda_jack_tbl_get_mst(codec, (*event).key_report_jack, (*event).dev_id);
        if !report_to.is_null() {
            (*report_to).jack_dirty = 1;
        }
    } else {
        (*event).jack_dirty = 1;
    }

    call_jack_callback(codec, res, event);
    snd_hda_jack_report_sync(codec);
}

/**
 * snd_hda_jack_poll_all - Poll all jacks
 * @codec: the HDA codec
 *
 * Poll all detectable jacks with dirty flag, update the status, call
 * callbacks and call snd_hda_jack_report_sync() if any changes are found.
 */
pub unsafe extern "C" fn snd_hda_jack_poll_all(codec: *mut hda_codec) {
    let mut jack = (*codec).jacktbl.list;
    let mut i: c_int = 0;
    let mut changes: c_int = 0;

    while i < (*codec).jacktbl.used {
        let old_sense: bool;
        if (*jack).nid == 0 || (*jack).jack_dirty == 0 || (*jack).phantom_jack != 0 {
            i += 1;
            jack = jack.add(1);
            continue;
        }
        old_sense = get_jack_plug_state((*jack).pin_sense);
        jack_detect_update(codec, jack);
        if old_sense == get_jack_plug_state((*jack).pin_sense) {
            i += 1;
            jack = jack.add(1);
            continue;
        }
        changes = 1;
        call_jack_callback(codec, 0, jack);
        i += 1;
        jack = jack.add(1);
    }
    if changes != 0 {
        snd_hda_jack_report_sync(codec);
    }
}

unsafe fn snd_hda_jack_detect_mst(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dev_id: c_int,
) -> bool {
    snd_hda_jack_detect_state_mst(codec, nid, dev_id) != HDA_JACK_NOT_PRESENT
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
