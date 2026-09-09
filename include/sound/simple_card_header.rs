/* SPDX-License-Identifier: GPL-2.0
 *
 * ASoC simple sound card support
 *
 * Copyright (C) 2012 Renesas Solutions Corp.
 * Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
 */

// Dependencies corresponding to <sound/soc.h> and <sound/simple_card_utils.h>
// are supplied by other translation units.

#[repr(C)]
pub struct simple_util_info {
    pub name: *const core::ffi::c_char,
    pub card: *const core::ffi::c_char,
    pub codec: *const core::ffi::c_char,
    pub platform: *const core::ffi::c_char,

    pub daifmt: core::ffi::c_uint,
    pub cpu_dai: simple_util_dai,
    pub codec_dai: simple_util_dai,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
