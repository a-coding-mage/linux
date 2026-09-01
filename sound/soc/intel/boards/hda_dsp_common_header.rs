// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright(c) 2019 Intel Corporation.
 */

/*
 * This file defines helper functions used by multiple
 * Intel HDA based machine drivers.
 */

// C dependencies:
// #include <sound/hda_codec.h>
// #include <sound/hda_i915.h>
// #include "../../codecs/hdac_hda.h"

#[repr(C)]
pub struct snd_soc_card {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

pub const EINVAL: i32 = 22;

// C conditional:
// #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC)
unsafe extern "C" {
    pub fn hda_dsp_hdmi_build_controls(
        card: *mut snd_soc_card,
        comp: *mut snd_soc_component,
    ) -> ::core::ffi::c_int;
}

// #else
#[inline]
pub unsafe fn hda_dsp_hdmi_build_controls__disabled(
    card: *mut snd_soc_card,
    comp: *mut snd_soc_component,
) -> ::core::ffi::c_int {
    let _ = card;
    let _ = comp;

    -EINVAL
}
// #endif

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
