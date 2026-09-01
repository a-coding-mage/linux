// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram miXart soundcards
 *
 * include file for mixer
 *
 * Copyright (c) 2003 by Digigram <alsa@digigram.com>
 */

// C header guard removed: __SOUND_MIXART_MIXER_H.
// External C struct dependencies: struct snd_mixart, struct mixart_mgr.

unsafe extern "C" {
    /* exported */
    pub fn mixart_update_playback_stream_level(
        chip: *mut snd_mixart,
        is_aes: ::std::os::raw::c_int,
        idx: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn mixart_update_capture_stream_level(
        chip: *mut snd_mixart,
        is_aes: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn snd_mixart_create_mixer(mgr: *mut mixart_mgr) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
