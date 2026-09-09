/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * OSS compatible macro definitions
 *
 * Copyright (C) 2000 Abramo Bagnara <abramo@alsa-project.org>
 */

/* Dependency provided by the surrounding soundcard interface. */

/* C source conditionally defines this macro only when it is not supplied
 * externally; Rust has no direct preprocessor equivalent here. */
pub const SAMPLE_TYPE_AWE32: i32 = 0x20;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
