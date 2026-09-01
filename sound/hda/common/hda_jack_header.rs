/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Jack-detection handling for HD-audio
 *
 * Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
 */

/* C dependencies: <linux/err.h>, <sound/jack.h> */

use core::ffi::{c_char, c_int, c_uint};

pub type hda_nid_t = u16;
pub type snd_jack_types = c_uint;

#[repr(C)]
pub struct hda_codec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct auto_pin_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

pub type hda_jack_callback_fn =
    Option<unsafe extern "C" fn(*mut hda_codec, *mut hda_jack_callback)>;

#[repr(C)]
pub struct hda_jack_callback {
    pub nid: hda_nid_t,
    pub dev_id: c_int,
    pub func: hda_jack_callback_fn,
    pub private_data: c_uint, /* arbitrary data */
    pub unsol_res: c_uint,    /* unsolicited event bits */
    pub jack: *mut hda_jack_tbl, /* associated jack entry */
    pub next: *mut hda_jack_callback,
}

#[repr(C)]
pub struct hda_jack_tbl {
    pub nid: hda_nid_t,
    pub dev_id: c_int,
    pub tag: u8, /* unsol event tag */
    pub callback: *mut hda_jack_callback,
    /* jack-detection stuff */
    pub pin_sense: c_uint, /* cached pin-sense value */
    /*
     * C bitfields packed into one unsigned int:
     * jack_detect:1 - capable of jack-detection?
     * jack_dirty:1 - needs to update?
     * phantom_jack:1 - a fixed, always present port?
     * block_report:1 - in a transitional state - do not report to userspace
     */
    pub bitfield_jack_detect_jack_dirty_phantom_jack_block_report: c_uint,
    pub gating_jack: hda_nid_t,     /* valid when gating jack plugged */
    pub gated_jack: hda_nid_t,      /* gated is dependent on this jack */
    pub key_report_jack: hda_nid_t, /* key reports to this jack */
    pub type_: c_int,
    pub button_state: c_int,
    pub jack: *mut snd_jack,
}

impl hda_jack_tbl {
    pub const JACK_DETECT_MASK: c_uint = 1 << 0;
    pub const JACK_DIRTY_MASK: c_uint = 1 << 1;
    pub const PHANTOM_JACK_MASK: c_uint = 1 << 2;
    pub const BLOCK_REPORT_MASK: c_uint = 1 << 3;
}

#[repr(C)]
pub struct hda_jack_keymap {
    pub type_: snd_jack_types,
    pub key: c_int,
}

unsafe extern "C" {
    pub fn snd_hda_jack_tbl_get_mst(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dev_id: c_int,
    ) -> *mut hda_jack_tbl;
}

/**
 * snd_hda_jack_tbl_get - query the jack-table entry for the given NID
 * @codec: the HDA codec
 * @nid: pin NID to refer to
 */
#[inline]
pub unsafe fn snd_hda_jack_tbl_get(
    codec: *mut hda_codec,
    nid: hda_nid_t,
) -> *mut hda_jack_tbl {
    unsafe { snd_hda_jack_tbl_get_mst(codec, nid, 0) }
}

unsafe extern "C" {
    pub fn snd_hda_jack_tbl_get_from_tag(
        codec: *mut hda_codec,
        tag: u8,
        dev_id: c_int,
    ) -> *mut hda_jack_tbl;

    pub fn snd_hda_jack_tbl_disconnect(codec: *mut hda_codec);
    pub fn snd_hda_jack_tbl_clear(codec: *mut hda_codec);

    pub fn snd_hda_jack_set_dirty_all(codec: *mut hda_codec);

    pub fn snd_hda_jack_detect_enable(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dev_id: c_int,
    ) -> c_int;

    pub fn snd_hda_jack_detect_enable_callback_mst(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dev_id: c_int,
        func: hda_jack_callback_fn,
    ) -> *mut hda_jack_callback;
}

/**
 * snd_hda_jack_detect_enable_callback - enable the jack-detection
 * @codec: the HDA codec
 * @nid: pin NID to enable
 * @cb: callback function to register
 *
 * In the case of error, the return value will be a pointer embedded with
 * errno.  Check and handle the return value appropriately with standard
 * macros such as @IS_ERR() and @PTR_ERR().
 */
#[inline]
pub unsafe fn snd_hda_jack_detect_enable_callback(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    cb: hda_jack_callback_fn,
) -> *mut hda_jack_callback {
    unsafe { snd_hda_jack_detect_enable_callback_mst(codec, nid, 0, cb) }
}

unsafe extern "C" {
    pub fn snd_hda_jack_set_gating_jack(
        codec: *mut hda_codec,
        gated_nid: hda_nid_t,
        gating_nid: hda_nid_t,
    ) -> c_int;

    pub fn snd_hda_jack_bind_keymap(
        codec: *mut hda_codec,
        key_nid: hda_nid_t,
        keymap: *const hda_jack_keymap,
        jack_nid: hda_nid_t,
    ) -> c_int;

    pub fn snd_hda_jack_set_button_state(
        codec: *mut hda_codec,
        jack_nid: hda_nid_t,
        button_state: c_int,
    );

    pub fn snd_hda_jack_pin_sense(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dev_id: c_int,
    ) -> u32;
}

/* the jack state returned from snd_hda_jack_detect_state() */
pub const HDA_JACK_NOT_PRESENT: c_int = 0;
pub const HDA_JACK_PRESENT: c_int = 1;
pub const HDA_JACK_PHANTOM: c_int = 2;

unsafe extern "C" {
    pub fn snd_hda_jack_detect_state_mst(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dev_id: c_int,
    ) -> c_int;
}

/**
 * snd_hda_jack_detect_state - query pin Presence Detect status
 * @codec: the CODEC to sense
 * @nid: the pin NID to sense
 *
 * Query and return the pin's Presence Detect status, as either
 * HDA_JACK_NOT_PRESENT, HDA_JACK_PRESENT or HDA_JACK_PHANTOM.
 */
#[inline]
pub unsafe fn snd_hda_jack_detect_state(codec: *mut hda_codec, nid: hda_nid_t) -> c_int {
    unsafe { snd_hda_jack_detect_state_mst(codec, nid, 0) }
}

/**
 * snd_hda_jack_detect_mst - Detect the jack
 * @codec: the HDA codec
 * @nid: pin NID to check jack detection
 * @dev_id: pin device entry id
 */
#[inline]
pub unsafe fn snd_hda_jack_detect_mst(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    dev_id: c_int,
) -> bool {
    unsafe { snd_hda_jack_detect_state_mst(codec, nid, dev_id) != HDA_JACK_NOT_PRESENT }
}

/**
 * snd_hda_jack_detect - Detect the jack
 * @codec: the HDA codec
 * @nid: pin NID to check jack detection
 */
#[inline]
pub unsafe fn snd_hda_jack_detect(codec: *mut hda_codec, nid: hda_nid_t) -> bool {
    unsafe { snd_hda_jack_detect_mst(codec, nid, 0) }
}

unsafe extern "C" {
    pub fn is_jack_detectable(codec: *mut hda_codec, nid: hda_nid_t) -> bool;

    pub fn snd_hda_jack_add_kctl_mst(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dev_id: c_int,
        name: *const c_char,
        phantom_jack: bool,
        type_: c_int,
        keymap: *const hda_jack_keymap,
    ) -> c_int;
}

/**
 * snd_hda_jack_add_kctl - Add a kctl for the given pin
 * @codec: the HDA codec
 * @nid: pin NID to assign
 * @name: string name for the jack
 * @phantom_jack: flag to deal as a phantom jack
 * @type: jack type bits to be reported, 0 for guessing from pincfg
 * @keymap: optional jack / key mapping
 *
 * This assigns a jack-detection kctl to the given pin.  The kcontrol
 * will have the given name and index.
 */
#[inline]
pub unsafe fn snd_hda_jack_add_kctl(
    codec: *mut hda_codec,
    nid: hda_nid_t,
    name: *const c_char,
    phantom_jack: bool,
    type_: c_int,
    keymap: *const hda_jack_keymap,
) -> c_int {
    unsafe { snd_hda_jack_add_kctl_mst(codec, nid, 0, name, phantom_jack, type_, keymap) }
}

unsafe extern "C" {
    pub fn snd_hda_jack_add_kctls(
        codec: *mut hda_codec,
        cfg: *const auto_pin_cfg,
    ) -> c_int;

    pub fn snd_hda_jack_report_sync(codec: *mut hda_codec);

    pub fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: c_uint);

    pub fn snd_hda_jack_poll_all(codec: *mut hda_codec);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
