/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * BIOS auto-parser helper functions for HD-audio
 *
 * Copyright (c) 2012 Takashi Iwai <tiwai@suse.de>
 */

/* Depends on hda_local.h for hda_codec, hda_nid_t, and HDA_MAX_OUTS. */

/*
 * Helper for automatic pin configuration
 */

pub const AUTO_PIN_MIC: ::std::os::raw::c_int = 0;
pub const AUTO_PIN_LINE_IN: ::std::os::raw::c_int = 1;
pub const AUTO_PIN_CD: ::std::os::raw::c_int = 2;
pub const AUTO_PIN_AUX: ::std::os::raw::c_int = 3;
pub const AUTO_PIN_LAST: ::std::os::raw::c_int = 4;

pub const AUTO_PIN_LINE_OUT: ::std::os::raw::c_int = 0;
pub const AUTO_PIN_SPEAKER_OUT: ::std::os::raw::c_int = 1;
pub const AUTO_PIN_HP_OUT: ::std::os::raw::c_int = 2;

pub const AUTO_CFG_MAX_OUTS: usize = HDA_MAX_OUTS;
pub const AUTO_CFG_MAX_INS: usize = 18;

#[repr(C)]
pub struct auto_pin_cfg_item {
    pub pin: hda_nid_t,
    pub type_: ::std::os::raw::c_int,
    /*
     * C bit-fields packed into one unsigned int:
     * is_headset_mic:1;
     * is_headphone_mic:1; Mic-only in headphone jack
     * has_boost_on_pin:1;
     */
    pub bitfield_is_headset_mic_is_headphone_mic_has_boost_on_pin: ::std::os::raw::c_uint,
    pub order: ::std::os::raw::c_int,
}

impl auto_pin_cfg_item {
    pub const IS_HEADSET_MIC_MASK: ::std::os::raw::c_uint = 1 << 0;
    pub const IS_HEADPHONE_MIC_MASK: ::std::os::raw::c_uint = 1 << 1;
    pub const HAS_BOOST_ON_PIN_MASK: ::std::os::raw::c_uint = 1 << 2;

    pub unsafe fn is_headset_mic(&self) -> ::std::os::raw::c_uint {
        self.bitfield_is_headset_mic_is_headphone_mic_has_boost_on_pin & Self::IS_HEADSET_MIC_MASK
    }

    pub unsafe fn set_is_headset_mic(&mut self, value: ::std::os::raw::c_uint) {
        self.bitfield_is_headset_mic_is_headphone_mic_has_boost_on_pin =
            (self.bitfield_is_headset_mic_is_headphone_mic_has_boost_on_pin
                & !Self::IS_HEADSET_MIC_MASK)
                | ((value & 1) << 0);
    }

    pub unsafe fn is_headphone_mic(&self) -> ::std::os::raw::c_uint {
        (self.bitfield_is_headset_mic_is_headphone_mic_has_boost_on_pin
            & Self::IS_HEADPHONE_MIC_MASK)
            >> 1
    }

    pub unsafe fn set_is_headphone_mic(&mut self, value: ::std::os::raw::c_uint) {
        self.bitfield_is_headset_mic_is_headphone_mic_has_boost_on_pin =
            (self.bitfield_is_headset_mic_is_headphone_mic_has_boost_on_pin
                & !Self::IS_HEADPHONE_MIC_MASK)
                | ((value & 1) << 1);
    }

    pub unsafe fn has_boost_on_pin(&self) -> ::std::os::raw::c_uint {
        (self.bitfield_is_headset_mic_is_headphone_mic_has_boost_on_pin
            & Self::HAS_BOOST_ON_PIN_MASK)
            >> 2
    }

    pub unsafe fn set_has_boost_on_pin(&mut self, value: ::std::os::raw::c_uint) {
        self.bitfield_is_headset_mic_is_headphone_mic_has_boost_on_pin =
            (self.bitfield_is_headset_mic_is_headphone_mic_has_boost_on_pin
                & !Self::HAS_BOOST_ON_PIN_MASK)
                | ((value & 1) << 2);
    }
}

unsafe extern "C" {
    pub fn hda_get_autocfg_input_label(
        codec: *mut hda_codec,
        cfg: *const auto_pin_cfg,
        input: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;

    pub fn snd_hda_get_pin_label(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        cfg: *const auto_pin_cfg,
        label: *mut ::std::os::raw::c_char,
        maxlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

pub const INPUT_PIN_ATTR_UNUSED: ::std::os::raw::c_int = 0; /* pin not connected */
pub const INPUT_PIN_ATTR_INT: ::std::os::raw::c_int = 1; /* internal mic/line-in */
pub const INPUT_PIN_ATTR_DOCK: ::std::os::raw::c_int = 2; /* docking mic/line-in */
pub const INPUT_PIN_ATTR_NORMAL: ::std::os::raw::c_int = 3; /* mic/line-in jack */
pub const INPUT_PIN_ATTR_REAR: ::std::os::raw::c_int = 4; /* mic/line-in jack in rear */
pub const INPUT_PIN_ATTR_FRONT: ::std::os::raw::c_int = 5; /* mic/line-in jack in front */
pub const INPUT_PIN_ATTR_LAST: ::std::os::raw::c_int = INPUT_PIN_ATTR_FRONT;

unsafe extern "C" {
    pub fn snd_hda_get_input_pin_attr(
        def_conf: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}

#[repr(C)]
pub struct auto_pin_cfg {
    pub line_outs: ::std::os::raw::c_int,
    /* sorted in the order of Front/Surr/CLFE/Side */
    pub line_out_pins: [hda_nid_t; AUTO_CFG_MAX_OUTS],
    pub speaker_outs: ::std::os::raw::c_int,
    pub speaker_pins: [hda_nid_t; AUTO_CFG_MAX_OUTS],
    pub hp_outs: ::std::os::raw::c_int,
    pub line_out_type: ::std::os::raw::c_int, /* AUTO_PIN_XXX_OUT */
    pub hp_pins: [hda_nid_t; AUTO_CFG_MAX_OUTS],
    pub num_inputs: ::std::os::raw::c_int,
    pub inputs: [auto_pin_cfg_item; AUTO_CFG_MAX_INS],
    pub dig_outs: ::std::os::raw::c_int,
    pub dig_out_pins: [hda_nid_t; 2],
    pub dig_in_pin: hda_nid_t,
    pub mono_out_pin: hda_nid_t,
    pub dig_out_type: [::std::os::raw::c_int; 2], /* HDA_PCM_TYPE_XXX */
    pub dig_in_type: ::std::os::raw::c_int,       /* HDA_PCM_TYPE_XXX */
}

/* bit-flags for snd_hda_parse_pin_def_config() behavior */
pub const HDA_PINCFG_NO_HP_FIXUP: ::std::os::raw::c_uint = 1 << 0; /* no HP-split */
pub const HDA_PINCFG_NO_LO_FIXUP: ::std::os::raw::c_uint = 1 << 1; /* don't take other outs as LO */
pub const HDA_PINCFG_HEADSET_MIC: ::std::os::raw::c_uint = 1 << 2; /* Try to find headset mic; mark seq number as 0xc to trigger */
pub const HDA_PINCFG_HEADPHONE_MIC: ::std::os::raw::c_uint = 1 << 3; /* Try to find headphone mic; mark seq number as 0xd to trigger */

unsafe extern "C" {
    pub fn snd_hda_parse_pin_defcfg(
        codec: *mut hda_codec,
        cfg: *mut auto_pin_cfg,
        ignore_nids: *const hda_nid_t,
        cond_flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}

/* older function */
pub unsafe fn snd_hda_parse_pin_def_config(
    codec: *mut hda_codec,
    cfg: *mut auto_pin_cfg,
    ignore: *const hda_nid_t,
) -> ::std::os::raw::c_int {
    unsafe { snd_hda_parse_pin_defcfg(codec, cfg, ignore, 0) }
}

pub unsafe fn auto_cfg_hp_outs(cfg: *const auto_pin_cfg) -> ::std::os::raw::c_int {
    unsafe {
        if (*cfg).line_out_type == AUTO_PIN_HP_OUT {
            (*cfg).line_outs
        } else {
            (*cfg).hp_outs
        }
    }
}

pub unsafe fn auto_cfg_hp_pins(cfg: *const auto_pin_cfg) -> *const hda_nid_t {
    unsafe {
        if (*cfg).line_out_type == AUTO_PIN_HP_OUT {
            (*cfg).line_out_pins.as_ptr()
        } else {
            (*cfg).hp_pins.as_ptr()
        }
    }
}

pub unsafe fn auto_cfg_speaker_outs(cfg: *const auto_pin_cfg) -> ::std::os::raw::c_int {
    unsafe {
        if (*cfg).line_out_type == AUTO_PIN_SPEAKER_OUT {
            (*cfg).line_outs
        } else {
            (*cfg).speaker_outs
        }
    }
}

pub unsafe fn auto_cfg_speaker_pins(cfg: *const auto_pin_cfg) -> *const hda_nid_t {
    unsafe {
        if (*cfg).line_out_type == AUTO_PIN_SPEAKER_OUT {
            (*cfg).line_out_pins.as_ptr()
        } else {
            (*cfg).speaker_pins.as_ptr()
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
