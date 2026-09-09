/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * HWDEP Interface for HD-audio codec
 *
 * Copyright (c) 2007 Takashi Iwai <tiwai@suse.de>
 */

// HDA_HWDEP_VERSION ((1 << 16) | (0 << 8) | (0 << 0)) /* 1.0.0 */
pub const HDA_HWDEP_VERSION: u32 = (1u32 << 16) | (0u32 << 8) | (0u32 << 0);

/* verb */
pub const HDA_REG_NID_SHIFT: u32 = 24;
pub const HDA_REG_VERB_SHIFT: u32 = 8;
pub const HDA_REG_VAL_SHIFT: u32 = 0;

#[inline]
pub const fn HDA_VERB(nid: u32, verb: u32, param: u32) -> u32 {
    (nid << 24) | (verb << 8) | param
}

#[repr(C)]
pub struct hda_verb_ioctl {
    pub verb: u32, /* HDA_VERB() */
    pub res: u32,  /* response */
}

/*
 * ioctls
 */
// These ioctl request codes depend on the platform's _IOR/_IOWR definitions.
pub const HDA_IOCTL_PVERSION: _ = _IOR('H', 0x10, core::ffi::c_int);
pub const HDA_IOCTL_VERB_WRITE: _ = _IOWR('H', 0x11, hda_verb_ioctl);
pub const HDA_IOCTL_GET_WCAP: _ = _IOWR('H', 0x12, hda_verb_ioctl);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
