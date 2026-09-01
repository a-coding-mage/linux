/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   ALSA sequencer /proc info
 *   Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

/* Dependencies from the original header:
 *   <sound/info.h>
 *   <sound/seq_kernel.h>
 */

unsafe extern "C" {
    pub fn snd_seq_info_clients_read(
        entry: *mut snd_info_entry,
        buffer: *mut snd_info_buffer,
    );
    pub fn snd_seq_info_timer_read(
        entry: *mut snd_info_entry,
        buffer: *mut snd_info_buffer,
    );
    pub fn snd_seq_info_queues_read(
        entry: *mut snd_info_entry,
        buffer: *mut snd_info_buffer,
    );
}

/* C conditional: #ifdef CONFIG_SND_PROC_FS */
#[cfg(CONFIG_SND_PROC_FS)]
unsafe extern "C" {
    pub fn snd_seq_info_init() -> core::ffi::c_int;
    pub fn snd_seq_info_done();
}

/* C conditional fallback: #else */
#[cfg(not(CONFIG_SND_PROC_FS))]
#[inline]
pub unsafe fn snd_seq_info_init() -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_SND_PROC_FS))]
#[inline]
pub unsafe fn snd_seq_info_done() {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
