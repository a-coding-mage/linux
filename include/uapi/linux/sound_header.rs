/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Minor numbers for the sound driver.
 *
 * The original header includes <linux/fs.h>; no declarations from that
 * header are used by this file.
 */

pub const SND_DEV_CTL: i32 = 0; // Control port /dev/mixer
pub const SND_DEV_SEQ: i32 = 1; // Sequencer output /dev/sequencer (FM synthesizer and MIDI output)
pub const SND_DEV_MIDIN: i32 = 2; // Raw midi access
pub const SND_DEV_DSP: i32 = 3; // Digitized voice /dev/dsp
pub const SND_DEV_AUDIO: i32 = 4; // Sparc compatible /dev/audio
pub const SND_DEV_DSP16: i32 = 5; // Like /dev/dsp but 16 bits/sample
// pub const SND_DEV_STATUS: i32 = 6; // /dev/sndstat (obsolete)
pub const SND_DEV_UNUSED: i32 = 6;
pub const SND_DEV_AWFM: i32 = 7; // Reserved
pub const SND_DEV_SEQ2: i32 = 8; // /dev/sequencer, level 2 interface
// pub const SND_DEV_SNDPROC: i32 = 9; // /dev/sndproc for programmable devices (not used)
// pub const SND_DEV_DMMIDI: i32 = 9;
pub const SND_DEV_SYNTH: i32 = 9; // Raw synth access /dev/synth (same as /dev/dmfm)
pub const SND_DEV_DMFM: i32 = 10; // Raw synth access /dev/dmfm
pub const SND_DEV_UNKNOWN11: i32 = 11;
pub const SND_DEV_ADSP: i32 = 12; // Like /dev/dsp (obsolete)
pub const SND_DEV_AMIDI: i32 = 13; // Like /dev/midi (obsolete)
pub const SND_DEV_ADMMIDI: i32 = 14; // Like /dev/dmmidi (onsolete)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
