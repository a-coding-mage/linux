/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Local defininitons for Emu10k1 wavetable
 *
 *  Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
 */

// C header dependencies:
// #include <linux/time.h>
// #include <sound/core.h>
// #include <sound/emu10k1_synth.h>

use core::ffi::{c_int, c_long, c_void};

unsafe extern "C" {
    /* emu10k1_patch.c */
    pub fn snd_emu10k1_sample_new(
        private_data: *mut snd_emux,
        sp: *mut snd_sf_sample,
        hdr: *mut snd_util_memhdr,
        _data: *const c_void,
        count: c_long,
    ) -> c_int;
    pub fn snd_emu10k1_sample_free(
        private_data: *mut snd_emux,
        sp: *mut snd_sf_sample,
        hdr: *mut snd_util_memhdr,
    ) -> c_int;
    pub fn snd_emu10k1_memhdr_init(emu: *mut snd_emux) -> c_int;

    /* emu10k1_callback.c */
    pub fn snd_emu10k1_ops_setup(emu: *mut snd_emux);
    pub fn snd_emu10k1_synth_get_voice(hw: *mut snd_emu10k1) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
