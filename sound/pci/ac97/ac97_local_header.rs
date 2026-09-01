/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Universal interface for Audio Codec '97
 *
 *  For more details look to AC '97 component specification revision 2.2
 *  by Intel Corporation (http://developer.intel.com).
 */

use crate::{snd_ac97, snd_ac97_bus};

unsafe extern "C" {
    pub fn snd_ac97_get_name(
        ac97: *mut snd_ac97,
        id: ::core::ffi::c_uint,
        name: *mut ::core::ffi::c_char,
        maxlen: usize,
        modem: ::core::ffi::c_int,
    );
    pub fn snd_ac97_update_bits_nolock(
        ac97: *mut snd_ac97,
        reg: ::core::ffi::c_ushort,
        mask: ::core::ffi::c_ushort,
        value: ::core::ffi::c_ushort,
    ) -> ::core::ffi::c_int;
}

/* ac97_proc.c */
/* C conditional: #ifdef CONFIG_SND_PROC_FS */
#[cfg(CONFIG_SND_PROC_FS)]
unsafe extern "C" {
    pub fn snd_ac97_bus_proc_init(ac97: *mut snd_ac97_bus);
    pub fn snd_ac97_bus_proc_done(ac97: *mut snd_ac97_bus);
    pub fn snd_ac97_proc_init(ac97: *mut snd_ac97);
    pub fn snd_ac97_proc_done(ac97: *mut snd_ac97);
}

/* C conditional: #else, no-op macros */
#[cfg(not(CONFIG_SND_PROC_FS))]
pub unsafe fn snd_ac97_bus_proc_init(_ac97_bus_t: *mut snd_ac97_bus) {}

#[cfg(not(CONFIG_SND_PROC_FS))]
pub unsafe fn snd_ac97_bus_proc_done(_ac97_bus_t: *mut snd_ac97_bus) {}

#[cfg(not(CONFIG_SND_PROC_FS))]
pub unsafe fn snd_ac97_proc_init(_ac97_t: *mut snd_ac97) {}

#[cfg(not(CONFIG_SND_PROC_FS))]
pub unsafe fn snd_ac97_proc_done(_ac97_t: *mut snd_ac97) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
