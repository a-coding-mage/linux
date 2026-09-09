/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Advanced Linux Sound Architecture - ALSA - Driver
 *  Copyright (c) 1994-2003 by Jaroslav Kysela <perex@perex.cz>,
 *                             Abramo Bagnara <abramo@alsa-project.org>
 */

// C header guard: __SOUND_ASOUND_H
// Dependencies supplied by the surrounding translation unit:
// <linux/ioctl.h>, <linux/time.h>, <asm/byteorder.h>, and
// <uapi/sound/asound.h>.

#[cfg(target_endian = "little")]
pub const SNDRV_LITTLE_ENDIAN: bool = true;

#[cfg(target_endian = "big")]
pub const SNDRV_BIG_ENDIAN: bool = true;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
