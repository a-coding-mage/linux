// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) 2000 Uros Bizjak <uros@kss-loka.si>
 */

use core::ffi::{c_char, c_int, c_uchar, c_void};

// C dependency: #include <sound/opl3.h>

/* Prototypes for opl3_seq.c */
unsafe extern "C" {
    pub fn snd_opl3_synth_use_inc(opl3: *mut crate::snd_opl3) -> c_int;
    pub fn snd_opl3_synth_use_dec(opl3: *mut crate::snd_opl3);
    pub fn snd_opl3_synth_setup(opl3: *mut crate::snd_opl3) -> c_int;
    pub fn snd_opl3_synth_cleanup(opl3: *mut crate::snd_opl3);
}

/* Prototypes for opl3_midi.c */
unsafe extern "C" {
    pub fn snd_opl3_note_on(
        p: *mut c_void,
        note: c_int,
        vel: c_int,
        chan: *mut crate::snd_midi_channel,
    );
    pub fn snd_opl3_note_off(
        p: *mut c_void,
        note: c_int,
        vel: c_int,
        chan: *mut crate::snd_midi_channel,
    );
    pub fn snd_opl3_key_press(
        p: *mut c_void,
        note: c_int,
        vel: c_int,
        chan: *mut crate::snd_midi_channel,
    );
    pub fn snd_opl3_terminate_note(
        p: *mut c_void,
        note: c_int,
        chan: *mut crate::snd_midi_channel,
    );
    pub fn snd_opl3_control(
        p: *mut c_void,
        type_: c_int,
        chan: *mut crate::snd_midi_channel,
    );
    pub fn snd_opl3_nrpn(
        p: *mut c_void,
        chan: *mut crate::snd_midi_channel,
        chset: *mut crate::snd_midi_channel_set,
    );
    pub fn snd_opl3_sysex(
        p: *mut c_void,
        buf: *mut c_uchar,
        len: c_int,
        parsed: c_int,
        chset: *mut crate::snd_midi_channel_set,
    );

    pub fn snd_opl3_calc_volume(
        reg: *mut c_uchar,
        vel: c_int,
        chan: *mut crate::snd_midi_channel,
    );
    pub fn snd_opl3_timer_func(t: *mut crate::timer_list);
}

/* Prototypes for opl3_drums.c */
unsafe extern "C" {
    pub fn snd_opl3_load_drums(opl3: *mut crate::snd_opl3);
    pub fn snd_opl3_drum_switch(
        opl3: *mut crate::snd_opl3,
        note: c_int,
        vel: c_int,
        on_off: c_int,
        chan: *mut crate::snd_midi_channel,
    );
}

/* Prototypes for opl3_oss.c */
// C conditional: #if IS_ENABLED(CONFIG_SND_SEQUENCER_OSS)
unsafe extern "C" {
    pub fn snd_opl3_init_seq_oss(opl3: *mut crate::snd_opl3, name: *mut c_char);
    pub fn snd_opl3_free_seq_oss(opl3: *mut crate::snd_opl3);
}
// C #else provided NOP macros:
// #define snd_opl3_init_seq_oss(opl3, name) /* NOP */
// #define snd_opl3_free_seq_oss(opl3) /* NOP */

unsafe extern "C" {
    pub static mut snd_opl3_regmap: [[c_char; 4]; crate::MAX_OPL2_VOICES];
    pub static mut use_internal_drums: bool;
    pub static opl3_ops: crate::snd_midi_op;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
