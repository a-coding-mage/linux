// SPDX-License-Identifier: GPL-2.0-or-later
//
// A structure to keep track of each hardware voice
//
// Copyright (C) 1999 Steve Ratcliffe
// Copyright (c) 1999-2000 Takashi Iwai <tiwai@suse.de>

// External types from dependent headers:
// - linux/wait.h
// - linux/sched.h
// - sound/core.h
// - sound/emux_synth.h

use std::ffi::c_void;

// Opaque types defined in external headers
#[repr(C)]
pub struct snd_emux {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_emux_port {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_port_callback {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_midi_channel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_midi_channel_set {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_emux_voice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

// Prototypes for emux_seq.c
extern "C" {
    pub fn snd_emux_init_seq(emu: *mut snd_emux, card: *mut snd_card, index: i32) -> i32;
    pub fn snd_emux_detach_seq(emu: *mut snd_emux);
    pub fn snd_emux_create_port(
        emu: *mut snd_emux,
        name: *mut i8,
        max_channels: i32,
        r#type: i32,
        callback: *mut snd_seq_port_callback,
    ) -> *mut snd_emux_port;
    pub fn snd_emux_reset_port(port: *mut snd_emux_port);
    pub fn snd_emux_event_input(
        ev: *mut snd_seq_event,
        direct: i32,
        private: *mut c_void,
        atomic: i32,
        hop: i32,
    ) -> i32;
    pub fn snd_emux_inc_count(emu: *mut snd_emux) -> i32;
    pub fn snd_emux_dec_count(emu: *mut snd_emux);
    pub fn snd_emux_init_virmidi(emu: *mut snd_emux, card: *mut snd_card) -> i32;
    pub fn snd_emux_delete_virmidi(emu: *mut snd_emux) -> i32;
}

// Prototypes for emux_synth.c
extern "C" {
    pub fn snd_emux_init_voices(emu: *mut snd_emux);

    pub fn snd_emux_note_on(
        p: *mut c_void,
        note: i32,
        vel: i32,
        chan: *mut snd_midi_channel,
    );
    pub fn snd_emux_note_off(
        p: *mut c_void,
        note: i32,
        vel: i32,
        chan: *mut snd_midi_channel,
    );
    pub fn snd_emux_key_press(
        p: *mut c_void,
        note: i32,
        vel: i32,
        chan: *mut snd_midi_channel,
    );
    pub fn snd_emux_terminate_note(
        p: *mut c_void,
        note: i32,
        chan: *mut snd_midi_channel,
    );
    pub fn snd_emux_control(p: *mut c_void, r#type: i32, chan: *mut snd_midi_channel);

    pub fn snd_emux_sounds_off_all(port: *mut snd_emux_port);
    pub fn snd_emux_update_channel(
        port: *mut snd_emux_port,
        chan: *mut snd_midi_channel,
        update: i32,
    );
    pub fn snd_emux_update_port(port: *mut snd_emux_port, update: i32);

    pub fn snd_emux_timer_callback(t: *mut timer_list);
}

// emux_effect.c
#[cfg(feature = "SNDRV_EMUX_USE_RAW_EFFECT")]
extern "C" {
    pub fn snd_emux_create_effect(p: *mut snd_emux_port);
    pub fn snd_emux_delete_effect(p: *mut snd_emux_port);
    pub fn snd_emux_clear_effect(p: *mut snd_emux_port);
    pub fn snd_emux_setup_effect(vp: *mut snd_emux_voice);
    pub fn snd_emux_send_effect_oss(
        port: *mut snd_emux_port,
        chan: *mut snd_midi_channel,
        r#type: i32,
        val: i32,
    );
    pub fn snd_emux_send_effect(
        port: *mut snd_emux_port,
        chan: *mut snd_midi_channel,
        r#type: i32,
        val: i32,
        mode: i32,
    );
}

// emux_nrpn.c
extern "C" {
    pub fn snd_emux_sysex(
        private_data: *mut c_void,
        buf: *mut u8,
        len: i32,
        parsed: i32,
        chset: *mut snd_midi_channel_set,
    );
    pub fn snd_emux_xg_control(
        port: *mut snd_emux_port,
        chan: *mut snd_midi_channel,
        param: i32,
    ) -> i32;
    pub fn snd_emux_nrpn(
        private_data: *mut c_void,
        chan: *mut snd_midi_channel,
        chset: *mut snd_midi_channel_set,
    );
}

// emux_oss.c
extern "C" {
    pub fn snd_emux_init_seq_oss(emu: *mut snd_emux);
    pub fn snd_emux_detach_seq_oss(emu: *mut snd_emux);
}

// emux_proc.c
#[cfg(feature = "CONFIG_SND_PROC_FS")]
extern "C" {
    pub fn snd_emux_proc_init(emu: *mut snd_emux, card: *mut snd_card, device: i32);
    pub fn snd_emux_proc_free(emu: *mut snd_emux);
}

#[cfg(not(feature = "CONFIG_SND_PROC_FS"))]
#[inline]
pub fn snd_emux_proc_init(_emu: *mut snd_emux, _card: *mut snd_card, _device: i32) {}

#[cfg(not(feature = "CONFIG_SND_PROC_FS"))]
#[inline]
pub fn snd_emux_proc_free(_emu: *mut snd_emux) {}

// emux_hwdep.c
extern "C" {
    pub fn snd_emux_init_hwdep(emu: *mut snd_emux) -> i32;
    pub fn snd_emux_delete_hwdep(emu: *mut snd_emux);
}

// STATE_IS_PLAYING: bitwise AND with SNDRV_EMUX_ST_ON flag (defined in sound/emux_synth.h)
#[macro_export]
macro_rules! STATE_IS_PLAYING {
    ($s:expr) => {
        (($s) & SNDRV_EMUX_ST_ON) != 0
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
