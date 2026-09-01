// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Local defininitons for the emu8000 (AWE32/64)
 *
 *  Copyright (C) 1999 Steve Ratcliffe
 *  Copyright (C) 1999-2000 Takashi Iwai <tiwai@suse.de>
 */

// C header dependencies:
// <linux/wait.h>
// <linux/sched.h>
// <linux/slab.h>
// <sound/core.h>
// <sound/emu8000.h>
// <sound/emu8000_reg.h>

unsafe extern "C" {
    /* emu8000_patch.c */
    pub fn snd_emu8000_sample_new(
        rec: *mut snd_emux,
        sp: *mut snd_sf_sample,
        hdr: *mut snd_util_memhdr,
        data: *const core::ffi::c_void, /* __user */
        count: core::ffi::c_long,
    ) -> core::ffi::c_int;

    pub fn snd_emu8000_sample_free(
        rec: *mut snd_emux,
        sp: *mut snd_sf_sample,
        hdr: *mut snd_util_memhdr,
    ) -> core::ffi::c_int;

    pub fn snd_emu8000_sample_reset(rec: *mut snd_emux);

    /* emu8000_callback.c */
    pub fn snd_emu8000_ops_setup(emu: *mut snd_emu8000);

    /* emu8000_pcm.c */
    pub fn snd_emu8000_pcm_new(
        card: *mut snd_card,
        emu: *mut snd_emu8000,
        index: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
