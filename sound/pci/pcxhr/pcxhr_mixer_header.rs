// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram pcxhr compatible soundcards
 *
 * include file for mixer
 *
 * Copyright (c) 2004 by Digigram <alsa@digigram.com>
 */

/* Header guard __SOUND_PCXHR_MIXER_H omitted in Rust. */

#[repr(C)]
pub struct pcxhr_mgr {
    _unused: [u8; 0],
}

/* exported */
unsafe extern "C" {
    pub fn pcxhr_create_mixer(mgr: *mut pcxhr_mgr) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
