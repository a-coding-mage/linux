/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * DTS binding definitions used for the Chromium OS Embedded Controller.
 *
 * Copyright (c) 2022 The Chromium OS Authors. All rights reserved.
 */

/* C header guard: _DT_BINDINGS_MFD_CROS_EC_H */

/* Typed channel for keyboard backlight. */
pub const CROS_EC_PWM_DT_KB_LIGHT: i32 = 0;
/* Typed channel for display backlight. */
pub const CROS_EC_PWM_DT_DISPLAY_LIGHT: i32 = 1;
/* Number of typed channels. */
pub const CROS_EC_PWM_DT_COUNT: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
