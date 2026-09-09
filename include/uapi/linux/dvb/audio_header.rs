// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// audio.h - DEPRECATED MPEG-TS audio decoder API
//
// NOTE: should not be used on future drivers
//
// Translated from the C header. The linux/types.h dependency is supplied by
// the surrounding bindings.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum audio_stream_source_t {
    AUDIO_SOURCE_DEMUX = 0,  // Select the demux as the main source
    AUDIO_SOURCE_MEMORY = 1, // Select internal memory as the main source
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum audio_play_state_t {
    AUDIO_STOPPED = 0, // Device is stopped
    AUDIO_PLAYING = 1, // Device is currently playing
    AUDIO_PAUSED = 2,  // Device is paused
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum audio_channel_select_t {
    AUDIO_STEREO = 0,
    AUDIO_MONO_LEFT = 1,
    AUDIO_MONO_RIGHT = 2,
    AUDIO_MONO = 3,
    AUDIO_STEREO_SWAPPED = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct audio_mixer_t {
    pub volume_left: u32,
    pub volume_right: u32,
    // what else do we need? bass, pass-through, ...
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct audio_status_t {
    pub AV_sync_state: i32, // sync audio and video?
    pub mute_state: i32,    // audio is muted
    pub play_state: audio_play_state_t, // current playback state
    pub stream_source: audio_stream_source_t, // current stream source
    pub channel_select: audio_channel_select_t, // currently selected channel
    pub bypass_mode: i32, // pass on audio data to
    pub mixer_state: audio_mixer_t, // current mixer state
} // separate decoder hardware

// For GET_CAPABILITIES and SET_FORMAT, the latter should only set one bit.
pub const AUDIO_CAP_DTS: u32 = 1;
pub const AUDIO_CAP_LPCM: u32 = 2;
pub const AUDIO_CAP_MP1: u32 = 4;
pub const AUDIO_CAP_MP2: u32 = 8;
pub const AUDIO_CAP_MP3: u32 = 16;
pub const AUDIO_CAP_AAC: u32 = 32;
pub const AUDIO_CAP_OGG: u32 = 64;
pub const AUDIO_CAP_SDDS: u32 = 128;
pub const AUDIO_CAP_AC3: u32 = 256;

// Linux ioctl encodings corresponding to _IO, _IOR, and _IOW in the source.
pub const AUDIO_STOP: u32 = 0x6f01;
pub const AUDIO_PLAY: u32 = 0x6f02;
pub const AUDIO_PAUSE: u32 = 0x6f03;
pub const AUDIO_CONTINUE: u32 = 0x6f04;
pub const AUDIO_SELECT_SOURCE: u32 = 0x6f05;
pub const AUDIO_SET_MUTE: u32 = 0x6f06;
pub const AUDIO_SET_AV_SYNC: u32 = 0x6f07;
pub const AUDIO_SET_BYPASS_MODE: u32 = 0x6f08;
pub const AUDIO_CHANNEL_SELECT: u32 = 0x6f09;
pub const AUDIO_GET_STATUS: u32 = 0x801c6f0a;
pub const AUDIO_GET_CAPABILITIES: u32 = 0x80046f0b;
pub const AUDIO_CLEAR_BUFFER: u32 = 0x6f0c;
pub const AUDIO_SET_ID: u32 = 0x6f0d;
pub const AUDIO_SET_MIXER: u32 = 0x40086f0e;
pub const AUDIO_SET_STREAMTYPE: u32 = 0x6f0f;
pub const AUDIO_BILINGUAL_CHANNEL_SELECT: u32 = 0x6f14;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
