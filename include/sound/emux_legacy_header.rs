/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Copyright (c) 1999-2000 Takashi Iwai <tiwai@suse.de>
 *
 *  Definitions of OSS compatible headers for Emu8000 device informations
 */

/* Dependency provided by sound/seq_oss_legacy.h in the C source. */

/*
 * awe hardware controls
 */
pub const _EMUX_OSS_DEBUG_MODE: u32 = 0x00;
pub const _EMUX_OSS_REVERB_MODE: u32 = 0x01;
pub const _EMUX_OSS_CHORUS_MODE: u32 = 0x02;
pub const _EMUX_OSS_REMOVE_LAST_SAMPLES: u32 = 0x03;
pub const _EMUX_OSS_INITIALIZE_CHIP: u32 = 0x04;
pub const _EMUX_OSS_SEND_EFFECT: u32 = 0x05;
pub const _EMUX_OSS_TERMINATE_CHANNEL: u32 = 0x06;
pub const _EMUX_OSS_TERMINATE_ALL: u32 = 0x07;
pub const _EMUX_OSS_INITIAL_VOLUME: u32 = 0x08;
pub const _EMUX_OSS_INITIAL_ATTEN: u32 = _EMUX_OSS_INITIAL_VOLUME;
pub const _EMUX_OSS_RESET_CHANNEL: u32 = 0x09;
pub const _EMUX_OSS_CHANNEL_MODE: u32 = 0x0a;
pub const _EMUX_OSS_DRUM_CHANNELS: u32 = 0x0b;
pub const _EMUX_OSS_MISC_MODE: u32 = 0x0c;
pub const _EMUX_OSS_RELEASE_ALL: u32 = 0x0d;
pub const _EMUX_OSS_NOTEOFF_ALL: u32 = 0x0e;
pub const _EMUX_OSS_CHN_PRESSURE: u32 = 0x0f;
pub const _EMUX_OSS_EQUALIZER: u32 = 0x11;

pub const _EMUX_OSS_MODE_FLAG: u32 = 0x80;
pub const _EMUX_OSS_COOKED_FLAG: u32 = 0x40; /* not supported */
pub const _EMUX_OSS_MODE_VALUE_MASK: u32 = 0x3F;

/*
 * mode type definitions
 */
#[repr(i32)]
pub enum EmuxMode {
    EMUX_MD_EXCLUSIVE_OFF, /* obsolete */
    EMUX_MD_EXCLUSIVE_ON, /* obsolete */
    EMUX_MD_VERSION, /* read only */
    EMUX_MD_EXCLUSIVE_SOUND, /* 0/1: exclusive note on (default=1) */
    EMUX_MD_REALTIME_PAN, /* 0/1: do realtime pan change (default=1) */
    EMUX_MD_GUS_BANK, /* bank number for GUS patches (default=0) */
    EMUX_MD_KEEP_EFFECT, /* 0/1: keep effect values, (default=0) */
    EMUX_MD_ZERO_ATTEN, /* attenuation of max volume (default=32) */
    EMUX_MD_CHN_PRIOR, /* 0/1: set MIDI channel priority mode (default=1) */
    EMUX_MD_MOD_SENSE, /* integer: modwheel sensitivity (def=18) */
    EMUX_MD_DEF_PRESET, /* integer: default preset number (def=0) */
    EMUX_MD_DEF_BANK, /* integer: default bank number (def=0) */
    EMUX_MD_DEF_DRUM, /* integer: default drumset number (def=0) */
    EMUX_MD_TOGGLE_DRUM_BANK, /* 0/1: toggle drum flag with bank# (def=0) */
    EMUX_MD_NEW_VOLUME_CALC, /* 0/1: volume calculation mode (def=1) */
    EMUX_MD_CHORUS_MODE, /* integer: chorus mode (def=2) */
    EMUX_MD_REVERB_MODE, /* integer: chorus mode (def=4) */
    EMUX_MD_BASS_LEVEL, /* integer: bass level (def=5) */
    EMUX_MD_TREBLE_LEVEL, /* integer: treble level (def=9) */
    EMUX_MD_DEBUG_MODE, /* integer: debug level (def=0) */
    EMUX_MD_PAN_EXCHANGE, /* 0/1: exchange panning direction (def=0) */
    EMUX_MD_END,
}

/* effect parameters */
#[repr(i32)]
pub enum EmuxEffect {
    EMUX_FX_ENV1_DELAY,
    EMUX_FX_ENV1_ATTACK,
    EMUX_FX_ENV1_HOLD,
    EMUX_FX_ENV1_DECAY,
    EMUX_FX_ENV1_RELEASE,
    EMUX_FX_ENV1_SUSTAIN,
    EMUX_FX_ENV1_PITCH,
    EMUX_FX_ENV1_CUTOFF,
    EMUX_FX_ENV2_DELAY,
    EMUX_FX_ENV2_ATTACK,
    EMUX_FX_ENV2_HOLD,
    EMUX_FX_ENV2_DECAY,
    EMUX_FX_ENV2_RELEASE,
    EMUX_FX_ENV2_SUSTAIN,
    EMUX_FX_LFO1_DELAY,
    EMUX_FX_LFO1_FREQ,
    EMUX_FX_LFO1_VOLUME,
    EMUX_FX_LFO1_PITCH,
    EMUX_FX_LFO1_CUTOFF,
    EMUX_FX_LFO2_DELAY,
    EMUX_FX_LFO2_FREQ,
    EMUX_FX_LFO2_PITCH,
    EMUX_FX_INIT_PITCH,
    EMUX_FX_CHORUS,
    EMUX_FX_REVERB,
    EMUX_FX_CUTOFF,
    EMUX_FX_FILTERQ,
    EMUX_FX_SAMPLE_START,
    EMUX_FX_LOOP_START,
    EMUX_FX_LOOP_END,
    EMUX_FX_COARSE_SAMPLE_START,
    EMUX_FX_COARSE_LOOP_START,
    EMUX_FX_COARSE_LOOP_END,
    EMUX_FX_ATTEN,
    EMUX_FX_END,
}

/* number of effects */
pub const EMUX_NUM_EFFECTS: i32 = EmuxEffect::EMUX_FX_END as i32;

/* effect flag values */
pub const EMUX_FX_FLAG_OFF: i32 = 0;
pub const EMUX_FX_FLAG_SET: i32 = 1;
pub const EMUX_FX_FLAG_ADD: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
