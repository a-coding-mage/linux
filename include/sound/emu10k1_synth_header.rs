/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Defines for the Emu10k1 WaveTable synth
 *
 *  Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
 */

// Dependencies supplied by <sound/emu10k1.h> and <sound/emux_synth.h>.

/* sequencer device id */
pub const SNDRV_SEQ_DEV_ID_EMU10K1_SYNTH: &str = "emu10k1-synth";

/* argument for snd_seq_device_new */
#[repr(C)]
pub struct snd_emu10k1_synth_arg {
    pub hwptr: *mut snd_emu10k1, /* chip */
    pub index: ::core::ffi::c_int, /* sequencer client index */
    pub seq_ports: ::core::ffi::c_int, /* number of sequencer ports to be created */
    pub max_voices: ::core::ffi::c_int, /* maximum number of voices for wavetable */
}

// Opaque type supplied by <sound/emu10k1.h>.
pub enum snd_emu10k1 {}

pub const EMU10K1_MAX_MEMSIZE: i32 = 32 * 1024 * 1024; /* 32MB */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
