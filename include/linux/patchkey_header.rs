/* SPDX-License-Identifier: GPL-2.0 */
/*
 * <linux/patchkey.h> -- definition of _PATCHKEY macro
 *
 * Copyright (C) 2005 Stuart Brady
 *
 * This exists because awe_voice.h defined its own _PATCHKEY and it wasn't
 * clear whether removing this would break anything in userspace.
 *
 * Do not include this file directly.  Please use <sys/soundcard.h> instead.
 * For kernel code, use <linux/soundcard.h>
 */

// The C header includes <asm/byteorder.h> and <uapi/linux/patchkey.h>.

#[cfg(target_endian = "big")]
#[inline]
pub const fn _PATCHKEY(id: u32) -> u32 {
    0xfd00u32 | id
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn _PATCHKEY(id: u32) -> u32 {
    (id << 8) | 0x00fdu32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
