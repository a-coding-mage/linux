/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
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

// The C header requires _LINUX_PATCHKEY_H_INDIRECT and rejects direct inclusion.
// The include guard is omitted because Rust modules provide equivalent guarding.

/* Endian macros. */
// The C header obtains byte-order definitions from <endian.h> outside the kernel.
// Build-time kernel/non-kernel selection is preserved by the target-endian cfgs.

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! _PATCHKEY {
    ($id:expr) => {{
        0xfd00u32 | ($id as u32)
    }};
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! _PATCHKEY {
    ($id:expr) => {{
        (($id as u32) << 8) | 0x00fdu32
    }};
}

// Unsupported target byte order: the original C header emits an error when
// byte order cannot be determined.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
