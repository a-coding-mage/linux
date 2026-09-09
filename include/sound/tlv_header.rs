/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Advanced Linux Sound Architecture - ALSA - Driver
 *  Copyright (c) 2006 by Jaroslav Kysela <perex@perex.cz>
 */

// Dependency supplied by the UAPI header: uapi/sound/tlv.h

/* For historical reasons, these macros are aliases to the ones in UAPI. */
pub const TLV_ITEM: _ = SNDRV_CTL_TLVD_ITEM;
pub const TLV_LENGTH: _ = SNDRV_CTL_TLVD_LENGTH;

pub const TLV_CONTAINER_ITEM: _ = SNDRV_CTL_TLVD_CONTAINER_ITEM;
pub const DECLARE_TLV_CONTAINER: _ = SNDRV_CTL_TLVD_DECLARE_CONTAINER;

pub const TLV_DB_SCALE_MASK: _ = SNDRV_CTL_TLVD_DB_SCALE_MASK;
pub const TLV_DB_SCALE_MUTE: _ = SNDRV_CTL_TLVD_DB_SCALE_MUTE;
pub const TLV_DB_SCALE_ITEM: _ = SNDRV_CTL_TLVD_DB_SCALE_ITEM;
pub const DECLARE_TLV_DB_SCALE: _ = SNDRV_CTL_TLVD_DECLARE_DB_SCALE;

pub const TLV_DB_MINMAX_ITEM: _ = SNDRV_CTL_TLVD_DB_MINMAX_ITEM;
pub const TLV_DB_MINMAX_MUTE_ITEM: _ = SNDRV_CTL_TLVD_DB_MINMAX_MUTE_ITEM;
pub const DECLARE_TLV_DB_MINMAX: _ = SNDRV_CTL_TLVD_DECLARE_DB_MINMAX;
pub const DECLARE_TLV_DB_MINMAX_MUTE: _ = SNDRV_CTL_TLVD_DECLARE_DB_MINMAX_MUTE;

pub const TLV_DB_LINEAR_ITEM: _ = SNDRV_CTL_TLVD_DB_LINEAR_ITEM;
pub const DECLARE_TLV_DB_LINEAR: _ = SNDRV_CTL_TLVD_DECLARE_DB_LINEAR;

pub const TLV_DB_RANGE_ITEM: _ = SNDRV_CTL_TLVD_DB_RANGE_ITEM;
pub const DECLARE_TLV_DB_RANGE: _ = SNDRV_CTL_TLVD_DECLARE_DB_RANGE;

pub const TLV_DB_GAIN_MUTE: _ = SNDRV_CTL_TLVD_DB_GAIN_MUTE;

/*
 * The below assumes that each item TLV is 4 words like DB_SCALE or LINEAR.
 * This is an old fasion and obsoleted by commit bf1d1c9b6179("ALSA: tlv: add
 * DECLARE_TLV_DB_RANGE()").
 */
#[macro_export]
macro_rules! TLV_DB_RANGE_HEAD {
    ($num:expr) => {
        SNDRV_CTL_TLVT_DB_RANGE,
        6 * ($num) * core::mem::size_of::<core::ffi::c_uint>()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
