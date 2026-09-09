/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from sound/seq_midi_emul.h. */

use core::ffi::c_void;

/* Dependency supplied by sound/seq_kernel.h. */
#[repr(C)]
pub struct snd_seq_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_midi_channel {
    pub private: *mut c_void,
    pub number: i32,
    pub client: i32,
    pub port: i32,
    pub midi_mode: u8,
    pub drum_channel: u32,
    pub param_type: u32,
    pub midi_aftertouch: u8,
    pub midi_pressure: u8,
    pub midi_program: u8,
    pub midi_pitchbend: i16,
    pub control: [u8; 128],
    pub note: [u8; 128],
    pub gm_rpn_pitch_bend_range: i16,
    pub gm_rpn_fine_tuning: i16,
    pub gm_rpn_coarse_tuning: i16,
}

#[repr(C)]
pub struct snd_midi_channel_set {
    pub private_data: *mut c_void,
    pub client: i32,
    pub port: i32,
    pub midi_mode: u8,
    pub gs_master_volume: u8,
    pub gs_chorus_mode: u8,
    pub gs_reverb_mode: u8,
    pub max_channels: i32,
    pub channels: [snd_midi_channel; 0],
}

pub type NoteOn = unsafe extern "C" fn(*mut c_void, i32, i32, *mut snd_midi_channel);
pub type NoteOff = unsafe extern "C" fn(*mut c_void, i32, i32, *mut snd_midi_channel);
pub type KeyPress = unsafe extern "C" fn(*mut c_void, i32, i32, *mut snd_midi_channel);
pub type NoteTerminate = unsafe extern "C" fn(*mut c_void, i32, *mut snd_midi_channel);
pub type Control = unsafe extern "C" fn(*mut c_void, i32, *mut snd_midi_channel);
pub type Nrpn = unsafe extern "C" fn(*mut c_void, *mut snd_midi_channel, *mut snd_midi_channel_set);
pub type Sysex = unsafe extern "C" fn(*mut c_void, *mut u8, i32, i32, *mut snd_midi_channel_set);

#[repr(C)]
pub struct snd_midi_op {
    pub note_on: Option<NoteOn>,
    pub note_off: Option<NoteOff>,
    pub key_press: Option<KeyPress>,
    pub note_terminate: Option<NoteTerminate>,
    pub control: Option<Control>,
    pub nrpn: Option<Nrpn>,
    pub sysex: Option<Sysex>,
}

pub const MIDI_CTL_PITCHBEND: usize = 0x80;
pub const MIDI_CTL_AFTERTOUCH: usize = 0x81;
pub const MIDI_CTL_CHAN_PRESSURE: usize = 0x82;

pub const SNDRV_MIDI_MODE_NONE: i32 = 0;
pub const SNDRV_MIDI_MODE_GM: i32 = 1;
pub const SNDRV_MIDI_MODE_GS: i32 = 2;
pub const SNDRV_MIDI_MODE_XG: i32 = 3;
pub const SNDRV_MIDI_MODE_MT32: i32 = 4;

pub const SNDRV_MIDI_NOTE_OFF: u8 = 0x00;
pub const SNDRV_MIDI_NOTE_ON: u8 = 0x01;
pub const SNDRV_MIDI_NOTE_RELEASED: u8 = 0x02;
pub const SNDRV_MIDI_NOTE_SOSTENUTO: u8 = 0x04;
pub const SNDRV_MIDI_PARAM_TYPE_REGISTERED: u8 = 0;
pub const SNDRV_MIDI_PARAM_TYPE_NONREGISTERED: u8 = 1;

pub const SNDRV_MIDI_SYSEX_NOT_PARSED: i32 = 0;
pub const SNDRV_MIDI_SYSEX_GM_ON: i32 = 1;
pub const SNDRV_MIDI_SYSEX_GS_ON: i32 = 2;
pub const SNDRV_MIDI_SYSEX_GS_RESET: i32 = 3;
pub const SNDRV_MIDI_SYSEX_GS_CHORUS_MODE: i32 = 4;
pub const SNDRV_MIDI_SYSEX_GS_REVERB_MODE: i32 = 5;
pub const SNDRV_MIDI_SYSEX_GS_MASTER_VOLUME: i32 = 6;
pub const SNDRV_MIDI_SYSEX_GS_PROGRAM: i32 = 7;
pub const SNDRV_MIDI_SYSEX_GS_DRUM_CHANNEL: i32 = 8;
pub const SNDRV_MIDI_SYSEX_XG_ON: i32 = 9;

pub unsafe extern "C" {
    pub fn snd_midi_process_event(ops: *const snd_midi_op, ev: *mut snd_seq_event, chanset: *mut snd_midi_channel_set);
    pub fn snd_midi_channel_set_clear(chset: *mut snd_midi_channel_set);
    pub fn snd_midi_channel_alloc_set(n: i32) -> *mut snd_midi_channel_set;
    pub fn snd_midi_channel_free_set(chset: *mut snd_midi_channel_set);
}

#[inline] pub unsafe fn snDRV_gm_bank_select(cp: *const snd_midi_channel) -> u16 { ((*cp).control[0] as u16) << 7 | (*cp).control[32] as u16 }
#[inline] pub unsafe fn snDRV_gm_modulation_wheel(cp: *const snd_midi_channel) -> u16 { ((*cp).control[1] as u16) << 7 | (*cp).control[33] as u16 }
#[inline] pub unsafe fn snDRV_gm_breath(cp: *const snd_midi_channel) -> u16 { ((*cp).control[2] as u16) << 7 | (*cp).control[34] as u16 }
#[inline] pub unsafe fn snDRV_gm_foot_pedal(cp: *const snd_midi_channel) -> u16 { ((*cp).control[4] as u16) << 7 | (*cp).control[36] as u16 }
#[inline] pub unsafe fn snDRV_gm_portamento_time(cp: *const snd_midi_channel) -> u16 { ((*cp).control[5] as u16) << 7 | (*cp).control[37] as u16 }
#[inline] pub unsafe fn snDRV_gm_data_entry(cp: *const snd_midi_channel) -> u16 { ((*cp).control[6] as u16) << 7 | (*cp).control[38] as u16 }
#[inline] pub unsafe fn snDRV_gm_volume(cp: *const snd_midi_channel) -> u16 { ((*cp).control[7] as u16) << 7 | (*cp).control[39] as u16 }
#[inline] pub unsafe fn snDRV_gm_balance(cp: *const snd_midi_channel) -> u16 { ((*cp).control[8] as u16) << 7 | (*cp).control[40] as u16 }
#[inline] pub unsafe fn snDRV_gm_pan(cp: *const snd_midi_channel) -> u16 { ((*cp).control[10] as u16) << 7 | (*cp).control[42] as u16 }
#[inline] pub unsafe fn snDRV_gm_expression(cp: *const snd_midi_channel) -> u16 { ((*cp).control[11] as u16) << 7 | (*cp).control[43] as u16 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
