// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  GM/GS/XG midi module.
 *
 *  Copyright (C) 1999 Steve Ratcliffe
 *
 *  Based on awe_wave.c by Takashi Iwai
 */
/*
 * This module is used to keep track of the current midi state.
 * It can be used for drivers that are required to emulate midi when
 * the hardware doesn't.
 *
 * It was written for a AWE64 driver, but there should be no AWE specific
 * code in here.  If there is it should be reported as a bug.
 */

use core::ffi::{c_int, c_uchar, c_void};
use core::mem::size_of;
use core::ptr;

// Original C dependencies:
// <linux/init.h>, <linux/slab.h>, <linux/string.h>, <linux/module.h>,
// <sound/core.h>, <sound/seq_kernel.h>, <sound/seq_midi_emul.h>,
// <sound/initval.h>, <sound/asoundef.h>
// MODULE_AUTHOR("Takashi Iwai / Steve Ratcliffe");
// MODULE_DESCRIPTION("Advanced Linux Sound Architecture sequencer MIDI emulation.");
// MODULE_LICENSE("GPL");

pub const SNDRV_SEQ_EVENT_NOTE: c_int = 5;
pub const SNDRV_SEQ_EVENT_NOTEON: c_int = 6;
pub const SNDRV_SEQ_EVENT_NOTEOFF: c_int = 7;
pub const SNDRV_SEQ_EVENT_KEYPRESS: c_int = 8;
pub const SNDRV_SEQ_EVENT_CONTROLLER: c_int = 10;
pub const SNDRV_SEQ_EVENT_PGMCHANGE: c_int = 11;
pub const SNDRV_SEQ_EVENT_CHANPRESS: c_int = 12;
pub const SNDRV_SEQ_EVENT_PITCHBEND: c_int = 13;
pub const SNDRV_SEQ_EVENT_CONTROL14: c_int = 14;
pub const SNDRV_SEQ_EVENT_NONREGPARAM: c_int = 15;
pub const SNDRV_SEQ_EVENT_REGPARAM: c_int = 16;
pub const SNDRV_SEQ_EVENT_SONGPOS: c_int = 20;
pub const SNDRV_SEQ_EVENT_SONGSEL: c_int = 21;
pub const SNDRV_SEQ_EVENT_QFRAME: c_int = 22;
pub const SNDRV_SEQ_EVENT_TIMESIGN: c_int = 23;
pub const SNDRV_SEQ_EVENT_KEYSIGN: c_int = 24;
pub const SNDRV_SEQ_EVENT_START: c_int = 30;
pub const SNDRV_SEQ_EVENT_CONTINUE: c_int = 31;
pub const SNDRV_SEQ_EVENT_STOP: c_int = 32;
pub const SNDRV_SEQ_EVENT_CLOCK: c_int = 36;
pub const SNDRV_SEQ_EVENT_TEMPO: c_int = 33;
pub const SNDRV_SEQ_EVENT_SENSING: c_int = 40;
pub const SNDRV_SEQ_EVENT_CLIENT_START: c_int = 60;
pub const SNDRV_SEQ_EVENT_CLIENT_EXIT: c_int = 61;
pub const SNDRV_SEQ_EVENT_CLIENT_CHANGE: c_int = 62;
pub const SNDRV_SEQ_EVENT_PORT_START: c_int = 63;
pub const SNDRV_SEQ_EVENT_PORT_EXIT: c_int = 64;
pub const SNDRV_SEQ_EVENT_PORT_CHANGE: c_int = 65;
pub const SNDRV_SEQ_EVENT_SYSEX: c_int = 130;
pub const SNDRV_SEQ_EVENT_ECHO: c_int = 130 + 8;

pub const SNDRV_SEQ_EVENT_LENGTH_MASK: c_uchar = 0x0c;
pub const SNDRV_SEQ_EVENT_LENGTH_VARIABLE: c_uchar = 0x04;

pub const SNDRV_MIDI_NOTE_OFF: c_int = 0;
pub const SNDRV_MIDI_NOTE_ON: c_int = 1;
pub const SNDRV_MIDI_NOTE_RELEASED: c_int = 2;
pub const SNDRV_MIDI_NOTE_SOSTENUTO: c_int = 4;

pub const SNDRV_MIDI_MODE_NONE: c_int = 0;
pub const SNDRV_MIDI_MODE_GM: c_int = 1;
pub const SNDRV_MIDI_MODE_GS: c_int = 2;
pub const SNDRV_MIDI_MODE_XG: c_int = 3;

pub const SNDRV_MIDI_PARAM_TYPE_REGISTERED: c_int = 0;
pub const SNDRV_MIDI_PARAM_TYPE_NONREGISTERED: c_int = 1;

pub const SNDRV_MIDI_SYSEX_NOT_PARSED: c_int = 0;
pub const SNDRV_MIDI_SYSEX_GM_ON: c_int = 1;
pub const SNDRV_MIDI_SYSEX_GS_RESET: c_int = 2;
pub const SNDRV_MIDI_SYSEX_GS_DRUM_CHANNEL: c_int = 3;
pub const SNDRV_MIDI_SYSEX_GS_REVERB_MODE: c_int = 4;
pub const SNDRV_MIDI_SYSEX_GS_CHORUS_MODE: c_int = 5;
pub const SNDRV_MIDI_SYSEX_GS_MASTER_VOLUME: c_int = 6;
pub const SNDRV_MIDI_SYSEX_XG_ON: c_int = 7;

pub const MIDI_CTL_MSB_BANK: usize = 0x00;
pub const MIDI_CTL_MSB_DATA_ENTRY: usize = 0x06;
pub const MIDI_CTL_LSB_BANK: usize = 0x20;
pub const MIDI_CTL_LSB_DATA_ENTRY: usize = 0x26;
pub const MIDI_CTL_SUSTAIN: usize = 0x40;
pub const MIDI_CTL_PORTAMENTO: usize = 0x41;
pub const MIDI_CTL_SOSTENUTO: usize = 0x42;
pub const MIDI_CTL_SOFT_PEDAL: usize = 0x43;
pub const MIDI_CTL_LEGATO_FOOTSWITCH: usize = 0x44;
pub const MIDI_CTL_HOLD2: usize = 0x45;
pub const MIDI_CTL_SC1_SOUND_VARIATION: usize = 0x46;
pub const MIDI_CTL_SC2_TIMBRE: usize = 0x47;
pub const MIDI_CTL_SC3_RELEASE_TIME: usize = 0x48;
pub const MIDI_CTL_SC4_ATTACK_TIME: usize = 0x49;
pub const MIDI_CTL_SC5_BRIGHTNESS: usize = 0x4a;
pub const MIDI_CTL_E1_REVERB_DEPTH: usize = 0x5b;
pub const MIDI_CTL_E2_TREMOLO_DEPTH: usize = 0x5c;
pub const MIDI_CTL_E3_CHORUS_DEPTH: usize = 0x5d;
pub const MIDI_CTL_E4_DETUNE_DEPTH: usize = 0x5e;
pub const MIDI_CTL_E5_PHASER_DEPTH: usize = 0x5f;
pub const MIDI_CTL_NONREG_PARM_NUM_LSB: usize = 0x62;
pub const MIDI_CTL_NONREG_PARM_NUM_MSB: usize = 0x63;
pub const MIDI_CTL_REGIST_PARM_NUM_LSB: usize = 0x64;
pub const MIDI_CTL_REGIST_PARM_NUM_MSB: usize = 0x65;
pub const MIDI_CTL_ALL_SOUNDS_OFF: usize = 0x78;
pub const MIDI_CTL_RESET_CONTROLLERS: usize = 0x79;
pub const MIDI_CTL_ALL_NOTES_OFF: usize = 0x7b;
pub const MIDI_CTL_PITCHBEND: c_int = 0x80;
pub const MIDI_CTL_CHAN_PRESSURE: c_int = 0x81;

#[repr(C)]
pub struct snd_midi_channel {
    pub private: *mut c_void,
    pub number: c_int,
    pub note: [c_int; 128],
    pub control: [c_int; 128],
    pub midi_aftertouch: c_int,
    pub midi_pressure: c_int,
    pub midi_program: c_int,
    pub midi_pitchbend: c_int,
    pub gm_rpn_pitch_bend_range: c_int,
    pub gm_rpn_fine_tuning: c_int,
    pub gm_rpn_coarse_tuning: c_int,
    pub gm_volume: c_int,
    pub gm_expression: c_int,
    pub gm_pan: c_int,
    pub gm_hold: c_int,
    pub drum_channel: c_int,
    pub param_type: c_int,
}

#[repr(C)]
pub struct snd_midi_channel_set {
    pub channels: *mut snd_midi_channel,
    pub private_data: *mut c_void,
    pub max_channels: c_int,
    pub midi_mode: c_int,
    pub gs_master_volume: c_int,
    pub gs_reverb_mode: c_int,
    pub gs_chorus_mode: c_int,
}

#[repr(C)]
pub struct snd_midi_op {
    pub note_on: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int, *mut snd_midi_channel)>,
    pub note_off: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int, *mut snd_midi_channel)>,
    pub key_press: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int, *mut snd_midi_channel)>,
    pub control: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut snd_midi_channel)>,
    pub nrpn: Option<unsafe extern "C" fn(*mut c_void, *mut snd_midi_channel, *mut snd_midi_channel_set)>,
    pub sysex: Option<unsafe extern "C" fn(*mut c_void, *mut c_uchar, c_int, c_int, *mut snd_midi_channel_set)>,
    pub note_terminate: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut snd_midi_channel)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_note {
    pub channel: c_uchar,
    pub note: c_uchar,
    pub velocity: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_ctrl {
    pub channel: c_uchar,
    pub param: c_uint,
    pub value: c_int,
}

pub type c_uint = u32;

#[repr(C)]
pub union snd_seq_event_data {
    pub note: snd_seq_ev_note,
    pub control: snd_seq_ev_ctrl,
}

#[repr(C)]
pub struct snd_seq_event {
    pub r#type: c_int,
    pub flags: c_uchar,
    pub data: snd_seq_event_data,
}

extern "C" {
    fn snd_seq_ev_is_channel_type(ev: *const snd_seq_event) -> c_int;
    fn snd_seq_expand_var_event(
        event: *const snd_seq_event,
        count: usize,
        buf: *mut c_uchar,
        in_kernel: c_int,
        size_aligned: c_int,
    ) -> c_int;
    fn kmalloc_flex_channel_set(n: c_int) -> *mut snd_midi_channel_set;
    fn kfree(ptr: *mut c_void);
}

unsafe fn memset<T>(ptr: *mut T, val: c_int, count: usize) {
    ptr::write_bytes(ptr.cast::<u8>(), val as u8, count);
}

unsafe fn memcmp(a: *const c_uchar, b: *const c_uchar, count: usize) -> c_int {
    for i in 0..count {
        let av = *a.add(i);
        let bv = *b.add(i);
        if av != bv {
            return av as c_int - bv as c_int;
        }
    }
    0
}

/*
 * Process an event in a driver independent way.  This means dealing
 * with RPN, NRPN, SysEx etc that are defined for common midi applications
 * such as GM, GS and XG.
 * There modes that this module will run in are:
 *   Generic MIDI - no interpretation at all, it will just save current values
 *                  of controllers etc.
 *   GM - You can use all gm_ prefixed elements of chan.  Controls, RPN, NRPN,
 *        SysEx will be interpreded as defined in General Midi.
 *   GS - You can use all gs_ prefixed elements of chan. Codes for GS will be
 *        interpreted.
 *   XG - You can use all xg_ prefixed elements of chan.  Codes for XG will
 *        be interpreted.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_midi_process_event(
    ops: *const snd_midi_op,
    ev: *mut snd_seq_event,
    chanset: *mut snd_midi_channel_set,
) {
    let mut dest_channel: c_int = 0;

    if ev.is_null() || chanset.is_null() {
        return;
    }
    if snd_seq_ev_is_channel_type(ev) != 0 {
        dest_channel = (*ev).data.note.channel as c_int;
        if dest_channel >= (*chanset).max_channels {
            return;
        }
    }

    let chan = (*chanset).channels.add(dest_channel as usize);
    let drv = (*chanset).private_data;

    /* EVENT_NOTE should be processed before queued */
    if (*ev).r#type == SNDRV_SEQ_EVENT_NOTE {
        return;
    }

    /* Make sure that we don't have a note on that should really be
     * a note off */
    if (*ev).r#type == SNDRV_SEQ_EVENT_NOTEON && (*ev).data.note.velocity == 0 {
        (*ev).r#type = SNDRV_SEQ_EVENT_NOTEOFF;
    }

    /* Make sure the note is within array range */
    if (*ev).r#type == SNDRV_SEQ_EVENT_NOTEON
        || (*ev).r#type == SNDRV_SEQ_EVENT_NOTEOFF
        || (*ev).r#type == SNDRV_SEQ_EVENT_KEYPRESS
    {
        if (*ev).data.note.note >= 128 {
            return;
        }
    }

    match (*ev).r#type {
        SNDRV_SEQ_EVENT_NOTEON => {
            let note = (*ev).data.note.note as usize;
            if ((*chan).note[note] & SNDRV_MIDI_NOTE_ON) != 0 {
                if let Some(note_off) = (*ops).note_off {
                    note_off(drv, note as c_int, 0, chan);
                }
            }
            (*chan).note[note] = SNDRV_MIDI_NOTE_ON;
            if let Some(note_on) = (*ops).note_on {
                note_on(drv, note as c_int, (*ev).data.note.velocity as c_int, chan);
            }
        }
        SNDRV_SEQ_EVENT_NOTEOFF => {
            let note = (*ev).data.note.note as usize;
            if ((*chan).note[note] & SNDRV_MIDI_NOTE_ON) == 0 {
                return;
            }
            if (*ops).note_off.is_some() {
                note_off(ops, drv, chan, note as c_int, (*ev).data.note.velocity as c_int);
            }
        }
        SNDRV_SEQ_EVENT_KEYPRESS => {
            if let Some(key_press) = (*ops).key_press {
                key_press(drv, (*ev).data.note.note as c_int, (*ev).data.note.velocity as c_int, chan);
            }
        }
        SNDRV_SEQ_EVENT_CONTROLLER => {
            do_control(ops, drv, chanset, chan, (*ev).data.control.param as c_int, (*ev).data.control.value);
        }
        SNDRV_SEQ_EVENT_PGMCHANGE => {
            (*chan).midi_program = (*ev).data.control.value;
        }
        SNDRV_SEQ_EVENT_PITCHBEND => {
            (*chan).midi_pitchbend = (*ev).data.control.value;
            if let Some(control) = (*ops).control {
                control(drv, MIDI_CTL_PITCHBEND, chan);
            }
        }
        SNDRV_SEQ_EVENT_CHANPRESS => {
            (*chan).midi_pressure = (*ev).data.control.value;
            if let Some(control) = (*ops).control {
                control(drv, MIDI_CTL_CHAN_PRESSURE, chan);
            }
        }
        SNDRV_SEQ_EVENT_CONTROL14 => {
            /* Best guess is that this is any of the 14 bit controller values */
            if (*ev).data.control.param < 32 {
                /* set low part first */
                (*chan).control[(*ev).data.control.param as usize + 32] = (*ev).data.control.value & 0x7f;
                do_control(
                    ops,
                    drv,
                    chanset,
                    chan,
                    (*ev).data.control.param as c_int,
                    ((*ev).data.control.value >> 7) & 0x7f,
                );
            } else {
                do_control(ops, drv, chanset, chan, (*ev).data.control.param as c_int, (*ev).data.control.value);
            }
        }
        SNDRV_SEQ_EVENT_NONREGPARAM => {
            /* Break it back into its controller values */
            (*chan).param_type = SNDRV_MIDI_PARAM_TYPE_NONREGISTERED;
            (*chan).control[MIDI_CTL_MSB_DATA_ENTRY] = ((*ev).data.control.value >> 7) & 0x7f;
            (*chan).control[MIDI_CTL_LSB_DATA_ENTRY] = (*ev).data.control.value & 0x7f;
            (*chan).control[MIDI_CTL_NONREG_PARM_NUM_MSB] = ((*ev).data.control.param >> 7) as c_int & 0x7f;
            (*chan).control[MIDI_CTL_NONREG_PARM_NUM_LSB] = (*ev).data.control.param as c_int & 0x7f;
            nrpn(ops, drv, chan, chanset);
        }
        SNDRV_SEQ_EVENT_REGPARAM => {
            /* Break it back into its controller values */
            (*chan).param_type = SNDRV_MIDI_PARAM_TYPE_REGISTERED;
            (*chan).control[MIDI_CTL_MSB_DATA_ENTRY] = ((*ev).data.control.value >> 7) & 0x7f;
            (*chan).control[MIDI_CTL_LSB_DATA_ENTRY] = (*ev).data.control.value & 0x7f;
            (*chan).control[MIDI_CTL_REGIST_PARM_NUM_MSB] = ((*ev).data.control.param >> 7) as c_int & 0x7f;
            (*chan).control[MIDI_CTL_REGIST_PARM_NUM_LSB] = (*ev).data.control.param as c_int & 0x7f;
            rpn(ops, drv, chan, chanset);
        }
        SNDRV_SEQ_EVENT_SYSEX => {
            if ((*ev).flags & SNDRV_SEQ_EVENT_LENGTH_MASK) == SNDRV_SEQ_EVENT_LENGTH_VARIABLE {
                let mut sysexbuf = [0u8; 64];
                let len = snd_seq_expand_var_event(ev, size_of::<[c_uchar; 64]>(), sysexbuf.as_mut_ptr(), 1, 0);
                if len > 0 {
                    sysex(ops, drv, sysexbuf.as_mut_ptr(), len, chanset);
                }
            }
        }
        SNDRV_SEQ_EVENT_SONGPOS
        | SNDRV_SEQ_EVENT_SONGSEL
        | SNDRV_SEQ_EVENT_CLOCK
        | SNDRV_SEQ_EVENT_START
        | SNDRV_SEQ_EVENT_CONTINUE
        | SNDRV_SEQ_EVENT_STOP
        | SNDRV_SEQ_EVENT_QFRAME
        | SNDRV_SEQ_EVENT_TEMPO
        | SNDRV_SEQ_EVENT_TIMESIGN
        | SNDRV_SEQ_EVENT_KEYSIGN
        | SNDRV_SEQ_EVENT_SENSING
        | SNDRV_SEQ_EVENT_CLIENT_START
        | SNDRV_SEQ_EVENT_CLIENT_EXIT
        | SNDRV_SEQ_EVENT_CLIENT_CHANGE
        | SNDRV_SEQ_EVENT_PORT_START
        | SNDRV_SEQ_EVENT_PORT_EXIT
        | SNDRV_SEQ_EVENT_PORT_CHANGE
        | SNDRV_SEQ_EVENT_ECHO
        | _ => {}
    }
}

/*
 * release note
 */
unsafe fn note_off(
    ops: *const snd_midi_op,
    drv: *mut c_void,
    chan: *mut snd_midi_channel,
    note: c_int,
    vel: c_int,
) {
    let note_idx = note as usize;
    if (*chan).gm_hold != 0 {
        /* Hold this note until pedal is turned off */
        (*chan).note[note_idx] |= SNDRV_MIDI_NOTE_RELEASED;
    } else if ((*chan).note[note_idx] & SNDRV_MIDI_NOTE_SOSTENUTO) != 0 {
        /* Mark this note as release; it will be turned off when sostenuto
         * is turned off */
        (*chan).note[note_idx] |= SNDRV_MIDI_NOTE_RELEASED;
    } else {
        (*chan).note[note_idx] = 0;
        if let Some(note_off) = (*ops).note_off {
            note_off(drv, note, vel, chan);
        }
    }
}

/*
 * Do all driver independent operations for this controller and pass
 * events that need to take place immediately to the driver.
 */
unsafe fn do_control(
    ops: *const snd_midi_op,
    drv: *mut c_void,
    chset: *mut snd_midi_channel_set,
    chan: *mut snd_midi_channel,
    control: c_int,
    mut value: c_int,
) {
    if control < 0 || control as usize >= (*chan).control.len() {
        return;
    }

    /* Switches */
    if (control >= 64 && control <= 69) || (control >= 80 && control <= 83) {
        /* These are all switches; either off or on so set to 0 or 127 */
        value = if value >= 64 { 127 } else { 0 };
    }
    (*chan).control[control as usize] = value;

    match control as usize {
        MIDI_CTL_SUSTAIN => {
            if value == 0 {
                /* Sustain has been released, turn off held notes */
                for i in 0..128 {
                    if ((*chan).note[i] & SNDRV_MIDI_NOTE_RELEASED) != 0 {
                        (*chan).note[i] = SNDRV_MIDI_NOTE_OFF;
                        if let Some(note_off) = (*ops).note_off {
                            note_off(drv, i as c_int, 0, chan);
                        }
                    }
                }
            }
        }
        MIDI_CTL_PORTAMENTO => {}
        MIDI_CTL_SOSTENUTO => {
            if value != 0 {
                /* Mark each note that is currently held down */
                for i in 0..128 {
                    if ((*chan).note[i] & SNDRV_MIDI_NOTE_ON) != 0 {
                        (*chan).note[i] |= SNDRV_MIDI_NOTE_SOSTENUTO;
                    }
                }
            } else {
                /* release all notes that were held */
                for i in 0..128 {
                    if ((*chan).note[i] & SNDRV_MIDI_NOTE_SOSTENUTO) != 0 {
                        (*chan).note[i] &= !SNDRV_MIDI_NOTE_SOSTENUTO;
                        if ((*chan).note[i] & SNDRV_MIDI_NOTE_RELEASED) != 0 {
                            (*chan).note[i] = SNDRV_MIDI_NOTE_OFF;
                            if let Some(note_off) = (*ops).note_off {
                                note_off(drv, i as c_int, 0, chan);
                            }
                        }
                    }
                }
            }
        }
        MIDI_CTL_MSB_DATA_ENTRY => {
            (*chan).control[MIDI_CTL_LSB_DATA_ENTRY] = 0;
            if (*chan).param_type == SNDRV_MIDI_PARAM_TYPE_REGISTERED {
                rpn(ops, drv, chan, chset);
            } else {
                nrpn(ops, drv, chan, chset);
            }
        }
        MIDI_CTL_LSB_DATA_ENTRY => {
            if (*chan).param_type == SNDRV_MIDI_PARAM_TYPE_REGISTERED {
                rpn(ops, drv, chan, chset);
            } else {
                nrpn(ops, drv, chan, chset);
            }
        }
        MIDI_CTL_REGIST_PARM_NUM_LSB | MIDI_CTL_REGIST_PARM_NUM_MSB => {
            (*chan).param_type = SNDRV_MIDI_PARAM_TYPE_REGISTERED;
        }
        MIDI_CTL_NONREG_PARM_NUM_LSB | MIDI_CTL_NONREG_PARM_NUM_MSB => {
            (*chan).param_type = SNDRV_MIDI_PARAM_TYPE_NONREGISTERED;
        }
        MIDI_CTL_ALL_SOUNDS_OFF => {
            all_sounds_off(ops, drv, chan);
        }
        MIDI_CTL_ALL_NOTES_OFF => {
            all_notes_off(ops, drv, chan);
        }
        MIDI_CTL_MSB_BANK => {
            if (*chset).midi_mode == SNDRV_MIDI_MODE_XG {
                if value == 127 {
                    (*chan).drum_channel = 1;
                } else {
                    (*chan).drum_channel = 0;
                }
            }
        }
        MIDI_CTL_LSB_BANK => {}
        MIDI_CTL_RESET_CONTROLLERS => {
            snd_midi_reset_controllers(chan);
        }
        MIDI_CTL_SOFT_PEDAL
        | MIDI_CTL_LEGATO_FOOTSWITCH
        | MIDI_CTL_HOLD2
        | MIDI_CTL_SC1_SOUND_VARIATION
        | MIDI_CTL_SC2_TIMBRE
        | MIDI_CTL_SC3_RELEASE_TIME
        | MIDI_CTL_SC4_ATTACK_TIME
        | MIDI_CTL_SC5_BRIGHTNESS
        | MIDI_CTL_E1_REVERB_DEPTH
        | MIDI_CTL_E2_TREMOLO_DEPTH
        | MIDI_CTL_E3_CHORUS_DEPTH
        | MIDI_CTL_E4_DETUNE_DEPTH
        | MIDI_CTL_E5_PHASER_DEPTH
        | _ => {
            if let Some(control_fn) = (*ops).control {
                control_fn(drv, control, chan);
            }
        }
    }
}

/*
 * initialize the MIDI status
 */
#[no_mangle]
pub unsafe extern "C" fn snd_midi_channel_set_clear(chset: *mut snd_midi_channel_set) {
    (*chset).midi_mode = SNDRV_MIDI_MODE_GM;
    (*chset).gs_master_volume = 127;

    for i in 0..(*chset).max_channels {
        let chan = (*chset).channels.add(i as usize);
        memset((*chan).note.as_mut_ptr(), 0, size_of::<[c_int; 128]>());

        (*chan).midi_aftertouch = 0;
        (*chan).midi_pressure = 0;
        (*chan).midi_program = 0;
        (*chan).midi_pitchbend = 0;
        snd_midi_reset_controllers(chan);
        (*chan).gm_rpn_pitch_bend_range = 256; /* 2 semitones */
        (*chan).gm_rpn_fine_tuning = 0;
        (*chan).gm_rpn_coarse_tuning = 0;

        if i == 9 {
            (*chan).drum_channel = 1;
        } else {
            (*chan).drum_channel = 0;
        }
    }
}

/*
 * Process a rpn message.
 */
unsafe fn rpn(
    _ops: *const snd_midi_op,
    _drv: *mut c_void,
    chan: *mut snd_midi_channel,
    chset: *mut snd_midi_channel_set,
) {
    if (*chset).midi_mode != SNDRV_MIDI_MODE_NONE {
        let r#type = ((*chan).control[MIDI_CTL_REGIST_PARM_NUM_MSB] << 8)
            | (*chan).control[MIDI_CTL_REGIST_PARM_NUM_LSB];
        let val = ((*chan).control[MIDI_CTL_MSB_DATA_ENTRY] << 7)
            | (*chan).control[MIDI_CTL_LSB_DATA_ENTRY];

        match r#type {
            0x0000 => {
                /* Pitch bend sensitivity */
                /* MSB only / 1 semitone per 128 */
                (*chan).gm_rpn_pitch_bend_range = val;
            }
            0x0001 => {
                /* fine tuning: */
                /* MSB/LSB, 8192=center, 100/8192 cent step */
                (*chan).gm_rpn_fine_tuning = val - 8192;
            }
            0x0002 => {
                /* coarse tuning */
                /* MSB only / 8192=center, 1 semitone per 128 */
                (*chan).gm_rpn_coarse_tuning = val - 8192;
            }
            0x7f7f => {
                /* "lock-in" RPN */
                /* ignored */
            }
            _ => {}
        }
    }
    /* should call nrpn or rpn callback here.. */
}

/*
 * Process an nrpn message.
 */
unsafe fn nrpn(
    ops: *const snd_midi_op,
    drv: *mut c_void,
    chan: *mut snd_midi_channel,
    chset: *mut snd_midi_channel_set,
) {
    /* parse XG NRPNs here if possible */
    if let Some(nrpn) = (*ops).nrpn {
        nrpn(drv, chan, chset);
    }
}

/*
 * convert channel parameter in GS sysex
 */
fn get_channel(cmd: c_uchar) -> c_int {
    let mut p = (cmd & 0x0f) as c_int;
    if p == 0 {
        p = 9;
    } else if p < 10 {
        p -= 1;
    }
    p
}

/*
 * Process a sysex message.
 */
unsafe fn sysex(
    ops: *const snd_midi_op,
    private: *mut c_void,
    mut buf: *mut c_uchar,
    mut len: c_int,
    chset: *mut snd_midi_channel_set,
) {
    /* GM on */
    static GM_ON_MACRO: [c_uchar; 4] = [0x7e, 0x7f, 0x09, 0x01];
    /* XG on */
    static XG_ON_MACRO: [c_uchar; 7] = [0x43, 0x10, 0x4c, 0x00, 0x00, 0x7e, 0x00];
    /* GS prefix
     * drum channel: XX=0x1?(channel), YY=0x15, ZZ=on/off
     * reverb mode: XX=0x01, YY=0x30, ZZ=0-7
     * chorus mode: XX=0x01, YY=0x38, ZZ=0-7
     * master vol:  XX=0x00, YY=0x04, ZZ=0-127
     */
    static GS_PFX_MACRO: [c_uchar; 5] = [0x41, 0x10, 0x42, 0x12, 0x40];

    let mut parsed = SNDRV_MIDI_SYSEX_NOT_PARSED;

    if len <= 0 || *buf != 0xf0 {
        return;
    }
    /* skip first byte */
    buf = buf.add(1);
    len -= 1;

    /* GM on */
    if len >= size_of::<[c_uchar; 4]>() as c_int
        && memcmp(buf, GM_ON_MACRO.as_ptr(), size_of::<[c_uchar; 4]>()) == 0
    {
        if (*chset).midi_mode != SNDRV_MIDI_MODE_GS && (*chset).midi_mode != SNDRV_MIDI_MODE_XG {
            (*chset).midi_mode = SNDRV_MIDI_MODE_GM;
            reset_all_channels(chset);
            parsed = SNDRV_MIDI_SYSEX_GM_ON;
        }
    } else if len >= 8 && memcmp(buf, GS_PFX_MACRO.as_ptr(), size_of::<[c_uchar; 5]>()) == 0 {
        /* GS macros */
        if (*chset).midi_mode != SNDRV_MIDI_MODE_GS && (*chset).midi_mode != SNDRV_MIDI_MODE_XG {
            (*chset).midi_mode = SNDRV_MIDI_MODE_GS;
        }

        if *buf.add(5) == 0x00 && *buf.add(6) == 0x7f && *buf.add(7) == 0x00 {
            /* GS reset */
            parsed = SNDRV_MIDI_SYSEX_GS_RESET;
            reset_all_channels(chset);
        } else if (*buf.add(5) & 0xf0) == 0x10 && *buf.add(6) == 0x15 {
            /* drum pattern */
            let p = get_channel(*buf.add(5));
            if p < (*chset).max_channels {
                parsed = SNDRV_MIDI_SYSEX_GS_DRUM_CHANNEL;
                if *buf.add(7) != 0 {
                    (*(*chset).channels.add(p as usize)).drum_channel = 1;
                } else {
                    (*(*chset).channels.add(p as usize)).drum_channel = 0;
                }
            }
        } else if (*buf.add(5) & 0xf0) == 0x10 && *buf.add(6) == 0x21 {
            /* program */
            let p = get_channel(*buf.add(5));
            if p < (*chset).max_channels && (*(*chset).channels.add(p as usize)).drum_channel == 0 {
                parsed = SNDRV_MIDI_SYSEX_GS_DRUM_CHANNEL;
                (*(*chset).channels.add(p as usize)).midi_program = *buf.add(7) as c_int;
            }
        } else if *buf.add(5) == 0x01 && *buf.add(6) == 0x30 {
            /* reverb mode */
            parsed = SNDRV_MIDI_SYSEX_GS_REVERB_MODE;
            (*chset).gs_reverb_mode = *buf.add(7) as c_int;
        } else if *buf.add(5) == 0x01 && *buf.add(6) == 0x38 {
            /* chorus mode */
            parsed = SNDRV_MIDI_SYSEX_GS_CHORUS_MODE;
            (*chset).gs_chorus_mode = *buf.add(7) as c_int;
        } else if *buf.add(5) == 0x00 && *buf.add(6) == 0x04 {
            /* master volume */
            parsed = SNDRV_MIDI_SYSEX_GS_MASTER_VOLUME;
            (*chset).gs_master_volume = *buf.add(7) as c_int;
        }
    } else if len >= size_of::<[c_uchar; 7]>() as c_int
        && memcmp(buf, XG_ON_MACRO.as_ptr(), size_of::<[c_uchar; 7]>()) == 0
    {
        /* XG on */
        (*chset).midi_mode = SNDRV_MIDI_MODE_XG;
        parsed = SNDRV_MIDI_SYSEX_XG_ON;
        /* reset CC#0 for drums */
        for i in 0..(*chset).max_channels {
            if (*(*chset).channels.add(i as usize)).drum_channel != 0 {
                (*(*chset).channels.add(i as usize)).control[MIDI_CTL_MSB_BANK] = 127;
            } else {
                (*(*chset).channels.add(i as usize)).control[MIDI_CTL_MSB_BANK] = 0;
            }
        }
    }

    if let Some(sysex) = (*ops).sysex {
        sysex(private, buf.sub(1), len + 1, parsed, chset);
    }
}

/*
 * all sound off
 */
unsafe fn all_sounds_off(ops: *const snd_midi_op, drv: *mut c_void, chan: *mut snd_midi_channel) {
    if (*ops).note_terminate.is_none() {
        return;
    }
    for n in 0..128 {
        if (*chan).note[n] != 0 {
            if let Some(note_terminate) = (*ops).note_terminate {
                note_terminate(drv, n as c_int, chan);
            }
            (*chan).note[n] = 0;
        }
    }
}

/*
 * all notes off
 */
unsafe fn all_notes_off(ops: *const snd_midi_op, drv: *mut c_void, chan: *mut snd_midi_channel) {
    if (*ops).note_off.is_none() {
        return;
    }
    for n in 0..128 {
        if (*chan).note[n] == SNDRV_MIDI_NOTE_ON {
            note_off(ops, drv, chan, n as c_int, 0);
        }
    }
}

/*
 * Initialise a single midi channel control block.
 */
unsafe fn snd_midi_channel_init(p: *mut snd_midi_channel, n: c_int) {
    if p.is_null() {
        return;
    }

    memset(p, 0, size_of::<snd_midi_channel>());
    (*p).private = ptr::null_mut();
    (*p).number = n;

    snd_midi_reset_controllers(p);
    (*p).gm_rpn_pitch_bend_range = 256; /* 2 semitones */
    (*p).gm_rpn_fine_tuning = 0;
    (*p).gm_rpn_coarse_tuning = 0;

    if n == 9 {
        (*p).drum_channel = 1; /* Default ch 10 as drums */
    }
}

/*
 * reset all midi channels
 */
unsafe fn reset_all_channels(chset: *mut snd_midi_channel_set) {
    for ch in 0..(*chset).max_channels {
        let chan = (*chset).channels.add(ch as usize);
        snd_midi_reset_controllers(chan);
        (*chan).gm_rpn_pitch_bend_range = 256; /* 2 semitones */
        (*chan).gm_rpn_fine_tuning = 0;
        (*chan).gm_rpn_coarse_tuning = 0;

        if ch == 9 {
            (*chan).drum_channel = 1;
        } else {
            (*chan).drum_channel = 0;
        }
    }
}

/*
 * Allocate and initialise a midi channel set.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_midi_channel_alloc_set(n: c_int) -> *mut snd_midi_channel_set {
    let chset = kmalloc_flex_channel_set(n);
    if chset.is_null() {
        return ptr::null_mut();
    }

    (*chset).max_channels = n;
    (*chset).private_data = ptr::null_mut();

    for i in 0..n {
        snd_midi_channel_init((*chset).channels.add(i as usize), i);
    }

    chset
}

/*
 * Reset the midi controllers on a particular channel to default values.
 */
unsafe fn snd_midi_reset_controllers(chan: *mut snd_midi_channel) {
    memset((*chan).control.as_mut_ptr(), 0, size_of::<[c_int; 128]>());
    (*chan).gm_volume = 127;
    (*chan).gm_expression = 127;
    (*chan).gm_pan = 64;
}

/*
 * Free a midi channel set.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_midi_channel_free_set(chset: *mut snd_midi_channel_set) {
    if chset.is_null() {
        return;
    }
    kfree(chset.cast::<c_void>());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
