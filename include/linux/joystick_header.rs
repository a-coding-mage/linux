/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (C) 1996-2000 Vojtech Pavlik
 *
 *  Sponsored by SuSE
 */
/*
 */

// Dependency supplied by <uapi/linux/joystick.h>.

#[cfg(target_pointer_width = "64")]
pub const JS_DATA_SAVE_TYPE: _ = JS_DATA_SAVE_TYPE_64;

#[cfg(target_pointer_width = "32")]
pub const JS_DATA_SAVE_TYPE: _ = JS_DATA_SAVE_TYPE_32;

// The C header emits an error for unsupported BITS_PER_LONG values.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
