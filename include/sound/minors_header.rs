/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  MINOR numbers
 */

pub const SNDRV_OS_MINORS: i32 = 256;

pub const SNDRV_MINOR_DEVICES: i32 = 32;
macro_rules! SNDRV_MINOR_CARD { ($minor:expr) => { ($minor) >> 5 }; }
macro_rules! SNDRV_MINOR_DEVICE { ($minor:expr) => { ($minor) & 0x001f }; }
macro_rules! SNDRV_MINOR { ($card:expr, $dev:expr) => { (($card) << 5) | ($dev) }; }

/* these minors can still be used for autoloading devices (/dev/aload*) */
pub const SNDRV_MINOR_CONTROL: i32 = 0; /* 0 */
pub const SNDRV_MINOR_GLOBAL: i32 = 1; /* 1 */
pub const SNDRV_MINOR_SEQUENCER: i32 = 1; /* SNDRV_MINOR_GLOBAL + 0 * 32 */
pub const SNDRV_MINOR_TIMER: i32 = 33; /* SNDRV_MINOR_GLOBAL + 1 * 32 */

/* CONFIG_SND_DYNAMIC_MINORS selects the alternate declaration below. */
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_MINOR_COMPRESS: i32 = 2; /* 2 - 3 */
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_MINOR_HWDEP: i32 = 4; /* 4 - 7 */
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_MINOR_RAWMIDI: i32 = 8; /* 8 - 15 */
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_MINOR_PCM_PLAYBACK: i32 = 16; /* 16 - 23 */
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_MINOR_PCM_CAPTURE: i32 = 24; /* 24 - 31 */

/* same as first respective minor number to make minor allocation easier */
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_DEVICE_TYPE_CONTROL: i32 = SNDRV_MINOR_CONTROL;
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_DEVICE_TYPE_HWDEP: i32 = SNDRV_MINOR_HWDEP;
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_DEVICE_TYPE_RAWMIDI: i32 = SNDRV_MINOR_RAWMIDI;
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_DEVICE_TYPE_PCM_PLAYBACK: i32 = SNDRV_MINOR_PCM_PLAYBACK;
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_DEVICE_TYPE_PCM_CAPTURE: i32 = SNDRV_MINOR_PCM_CAPTURE;
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_DEVICE_TYPE_SEQUENCER: i32 = SNDRV_MINOR_SEQUENCER;
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_DEVICE_TYPE_TIMER: i32 = SNDRV_MINOR_TIMER;
#[cfg(not(feature = "CONFIG_SND_DYNAMIC_MINORS"))]
pub const SNDRV_DEVICE_TYPE_COMPRESS: i32 = SNDRV_MINOR_COMPRESS;

#[cfg(feature = "CONFIG_SND_DYNAMIC_MINORS")]
#[repr(i32)]
pub enum SndDeviceType {
    SNDRV_DEVICE_TYPE_CONTROL,
    SNDRV_DEVICE_TYPE_SEQUENCER,
    SNDRV_DEVICE_TYPE_TIMER,
    SNDRV_DEVICE_TYPE_HWDEP,
    SNDRV_DEVICE_TYPE_RAWMIDI,
    SNDRV_DEVICE_TYPE_PCM_PLAYBACK,
    SNDRV_DEVICE_TYPE_PCM_CAPTURE,
    SNDRV_DEVICE_TYPE_COMPRESS,
}

pub const SNDRV_MINOR_HWDEPS: i32 = 4;
pub const SNDRV_MINOR_RAWMIDIS: i32 = 8;
pub const SNDRV_MINOR_PCMS: i32 = 8;

/* CONFIG_SND_OSSEMUL enables the following OSS-compatible declarations. */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_DEVICES: i32 = 16;
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
macro_rules! SNDRV_MINOR_OSS_CARD { ($minor:expr) => { ($minor) >> 4 }; }
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
macro_rules! SNDRV_MINOR_OSS_DEVICE { ($minor:expr) => { ($minor) & 0x000f }; }
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
macro_rules! SNDRV_MINOR_OSS { ($card:expr, $dev:expr) => { (($card) << 4) | ($dev) }; }

#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_MIXER: i32 = 0; /* /dev/mixer - OSS 3.XX compatible */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_SEQUENCER: i32 = 1; /* /dev/sequencer - OSS 3.XX compatible */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_MIDI: i32 = 2; /* /dev/midi - native midi interface - OSS 3.XX compatible - UART */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_PCM: i32 = 3; /* alias */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_PCM_8: i32 = 3; /* /dev/dsp - 8bit PCM - OSS 3.XX compatible */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_AUDIO: i32 = 4; /* /dev/audio - SunSparc compatible */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_PCM_16: i32 = 5; /* /dev/dsp16 - 16bit PCM - OSS 3.XX compatible */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_SNDSTAT: i32 = 6; /* /dev/sndstat - for compatibility with OSS */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_RESERVED7: i32 = 7; /* reserved for future use */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_MUSIC: i32 = 8; /* /dev/music - OSS 3.XX compatible */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_DMMIDI: i32 = 9; /* /dev/dmmidi0 - this device can have another minor # with OSS */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_DMFM: i32 = 10; /* /dev/dmfm0 - this device can have another minor # with OSS */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_MIXER1: i32 = 11; /* alternate mixer */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_PCM1: i32 = 12; /* alternate PCM (GF-A-1) */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_MIDI1: i32 = 13; /* alternate midi - SYNTH */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_DMMIDI1: i32 = 14; /* alternate dmmidi - SYNTH */
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_MINOR_OSS_RESERVED15: i32 = 15; /* reserved for future use */

#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_OSS_DEVICE_TYPE_MIXER: i32 = 0;
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_OSS_DEVICE_TYPE_SEQUENCER: i32 = 1;
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_OSS_DEVICE_TYPE_PCM: i32 = 2;
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_OSS_DEVICE_TYPE_MIDI: i32 = 3;
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_OSS_DEVICE_TYPE_DMFM: i32 = 4;
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_OSS_DEVICE_TYPE_SNDSTAT: i32 = 5;
#[cfg(feature = "CONFIG_SND_OSSEMUL")]
pub const SNDRV_OSS_DEVICE_TYPE_MUSIC: i32 = 6;

#[cfg(feature = "CONFIG_SND_OSSEMUL")]
macro_rules! MODULE_ALIAS_SNDRV_MINOR {
    ($type:expr) => { MODULE_ALIAS!(concat!("sound-service-?-", stringify!($type))) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
