// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Uros Bizjak <uros@kss-loka.si>
 *
 *   OPL2/OPL3/OPL4 FM routines for internal percussion channels
 */

// C dependency: #include "opl3_voice.h"
use core::ffi::{c_char, c_int, c_uchar, c_ushort};
use crate::*;

static SND_OPL3_DRUM_TABLE: [c_char; 47] = [
    OPL3_BASSDRUM_ON as c_char,
    OPL3_BASSDRUM_ON as c_char,
    OPL3_HIHAT_ON as c_char, /* 35 - 37 */
    OPL3_SNAREDRUM_ON as c_char,
    OPL3_HIHAT_ON as c_char,
    OPL3_SNAREDRUM_ON as c_char, /* 38 - 40 */
    OPL3_BASSDRUM_ON as c_char,
    OPL3_HIHAT_ON as c_char,
    OPL3_BASSDRUM_ON as c_char, /* 41 - 43 */
    OPL3_HIHAT_ON as c_char,
    OPL3_TOMTOM_ON as c_char,
    OPL3_HIHAT_ON as c_char, /* 44 - 46 */
    OPL3_TOMTOM_ON as c_char,
    OPL3_TOMTOM_ON as c_char,
    OPL3_CYMBAL_ON as c_char, /* 47 - 49 */
    OPL3_TOMTOM_ON as c_char,
    OPL3_CYMBAL_ON as c_char,
    OPL3_CYMBAL_ON as c_char, /* 50 - 52 */
    OPL3_CYMBAL_ON as c_char,
    OPL3_CYMBAL_ON as c_char,
    OPL3_CYMBAL_ON as c_char, /* 53 - 55 */
    OPL3_HIHAT_ON as c_char,
    OPL3_CYMBAL_ON as c_char,
    OPL3_TOMTOM_ON as c_char, /* 56 - 58 */
    OPL3_CYMBAL_ON as c_char,
    OPL3_TOMTOM_ON as c_char,
    OPL3_TOMTOM_ON as c_char, /* 59 - 61 */
    OPL3_HIHAT_ON as c_char,
    OPL3_TOMTOM_ON as c_char,
    OPL3_TOMTOM_ON as c_char, /* 62 - 64 */
    OPL3_TOMTOM_ON as c_char,
    OPL3_TOMTOM_ON as c_char,
    OPL3_TOMTOM_ON as c_char, /* 65 - 67 */
    OPL3_TOMTOM_ON as c_char,
    OPL3_HIHAT_ON as c_char,
    OPL3_HIHAT_ON as c_char, /* 68 - 70 */
    OPL3_HIHAT_ON as c_char,
    OPL3_HIHAT_ON as c_char,
    OPL3_TOMTOM_ON as c_char, /* 71 - 73 */
    OPL3_TOMTOM_ON as c_char,
    OPL3_TOMTOM_ON as c_char,
    OPL3_TOMTOM_ON as c_char, /* 74 - 76 */
    OPL3_TOMTOM_ON as c_char,
    OPL3_TOMTOM_ON as c_char,
    OPL3_TOMTOM_ON as c_char, /* 77 - 79 */
    OPL3_CYMBAL_ON as c_char,
    OPL3_CYMBAL_ON as c_char, /* 80 - 81 */
];

#[repr(C)]
struct snd_opl3_drum_voice {
    voice: c_int,
    op: c_int,
    am_vib: c_uchar,
    ksl_level: c_uchar,
    attack_decay: c_uchar,
    sustain_release: c_uchar,
    feedback_connection: c_uchar,
    wave_select: c_uchar,
}

#[repr(C)]
struct snd_opl3_drum_note {
    voice: c_int,
    fnum: c_uchar,
    octave_f: c_uchar,
    feedback_connection: c_uchar,
}

static BASS_OP0: snd_opl3_drum_voice = snd_opl3_drum_voice {
    voice: 6,
    op: 0,
    am_vib: 0x00,
    ksl_level: 0x32,
    attack_decay: 0xf8,
    sustain_release: 0x66,
    feedback_connection: 0x30,
    wave_select: 0x00,
};
static BASS_OP1: snd_opl3_drum_voice = snd_opl3_drum_voice {
    voice: 6,
    op: 1,
    am_vib: 0x00,
    ksl_level: 0x03,
    attack_decay: 0xf6,
    sustain_release: 0x57,
    feedback_connection: 0x30,
    wave_select: 0x00,
};
static BASS_NOTE: snd_opl3_drum_note = snd_opl3_drum_note {
    voice: 6,
    fnum: 0x90,
    octave_f: 0x09,
    feedback_connection: 0,
};

static HIHAT: snd_opl3_drum_voice = snd_opl3_drum_voice {
    voice: 7,
    op: 0,
    am_vib: 0x00,
    ksl_level: 0x03,
    attack_decay: 0xf0,
    sustain_release: 0x06,
    feedback_connection: 0x20,
    wave_select: 0x00,
};

static SNARE: snd_opl3_drum_voice = snd_opl3_drum_voice {
    voice: 7,
    op: 1,
    am_vib: 0x00,
    ksl_level: 0x03,
    attack_decay: 0xf0,
    sustain_release: 0x07,
    feedback_connection: 0x20,
    wave_select: 0x02,
};
static SNARE_NOTE: snd_opl3_drum_note = snd_opl3_drum_note {
    voice: 7,
    fnum: 0xf4,
    octave_f: 0x0d,
    feedback_connection: 0,
};

static TOMTOM: snd_opl3_drum_voice = snd_opl3_drum_voice {
    voice: 8,
    op: 0,
    am_vib: 0x02,
    ksl_level: 0x03,
    attack_decay: 0xf0,
    sustain_release: 0x06,
    feedback_connection: 0x10,
    wave_select: 0x00,
};
static TOMTOM_NOTE: snd_opl3_drum_note = snd_opl3_drum_note {
    voice: 8,
    fnum: 0xf4,
    octave_f: 0x09,
    feedback_connection: 0,
};

static CYMBAL: snd_opl3_drum_voice = snd_opl3_drum_voice {
    voice: 8,
    op: 1,
    am_vib: 0x04,
    ksl_level: 0x03,
    attack_decay: 0xf0,
    sustain_release: 0x06,
    feedback_connection: 0x10,
    wave_select: 0x00,
};

/*
 * set drum voice characteristics
 */
unsafe fn snd_opl3_drum_voice_set(opl3: *mut snd_opl3, data: *const snd_opl3_drum_voice) {
    let op_offset: c_uchar =
        snd_opl3_regmap[(*data).voice as usize][(*data).op as usize] as c_uchar;
    let voice_offset: c_uchar = (*data).voice as c_uchar;
    let mut opl3_reg: c_ushort;

    /* Set OPL3 AM_VIB register */
    opl3_reg = (OPL3_LEFT | (OPL3_REG_AM_VIB + op_offset as c_int)) as c_ushort;
    ((*opl3).command)(opl3, opl3_reg, (*data).am_vib);

    /* Set OPL3 KSL_LEVEL register */
    opl3_reg = (OPL3_LEFT | (OPL3_REG_KSL_LEVEL + op_offset as c_int)) as c_ushort;
    ((*opl3).command)(opl3, opl3_reg, (*data).ksl_level);

    /* Set OPL3 ATTACK_DECAY register */
    opl3_reg = (OPL3_LEFT | (OPL3_REG_ATTACK_DECAY + op_offset as c_int)) as c_ushort;
    ((*opl3).command)(opl3, opl3_reg, (*data).attack_decay);

    /* Set OPL3 SUSTAIN_RELEASE register */
    opl3_reg = (OPL3_LEFT | (OPL3_REG_SUSTAIN_RELEASE + op_offset as c_int)) as c_ushort;
    ((*opl3).command)(opl3, opl3_reg, (*data).sustain_release);

    /* Set OPL3 FEEDBACK_CONNECTION register */
    opl3_reg = (OPL3_LEFT | (OPL3_REG_FEEDBACK_CONNECTION + voice_offset as c_int)) as c_ushort;
    ((*opl3).command)(opl3, opl3_reg, (*data).feedback_connection);

    /* Select waveform */
    opl3_reg = (OPL3_LEFT | (OPL3_REG_WAVE_SELECT + op_offset as c_int)) as c_ushort;
    ((*opl3).command)(opl3, opl3_reg, (*data).wave_select);
}

/*
 * Set drum voice pitch
 */
unsafe fn snd_opl3_drum_note_set(opl3: *mut snd_opl3, data: *const snd_opl3_drum_note) {
    let voice_offset: c_uchar = (*data).voice as c_uchar;
    let mut opl3_reg: c_ushort;

    /* Set OPL3 FNUM_LOW register */
    opl3_reg = (OPL3_LEFT | (OPL3_REG_FNUM_LOW + voice_offset as c_int)) as c_ushort;
    ((*opl3).command)(opl3, opl3_reg, (*data).fnum);

    /* Set OPL3 KEYON_BLOCK register */
    opl3_reg = (OPL3_LEFT | (OPL3_REG_KEYON_BLOCK + voice_offset as c_int)) as c_ushort;
    ((*opl3).command)(opl3, opl3_reg, (*data).octave_f);
}

/*
 * Set drum voice volume and position
 */
unsafe fn snd_opl3_drum_vol_set(
    opl3: *mut snd_opl3,
    data: *const snd_opl3_drum_voice,
    vel: c_int,
    chan: *mut snd_midi_channel,
) {
    let op_offset: c_uchar =
        snd_opl3_regmap[(*data).voice as usize][(*data).op as usize] as c_uchar;
    let voice_offset: c_uchar = (*data).voice as c_uchar;
    let mut reg_val: c_uchar;
    let mut opl3_reg: c_ushort;

    /* Set OPL3 KSL_LEVEL register */
    reg_val = (*data).ksl_level;
    snd_opl3_calc_volume(&mut reg_val, vel, chan);
    opl3_reg = (OPL3_LEFT | (OPL3_REG_KSL_LEVEL + op_offset as c_int)) as c_ushort;
    ((*opl3).command)(opl3, opl3_reg, reg_val);

    /* Set OPL3 FEEDBACK_CONNECTION register */
    /* Set output voice connection */
    reg_val = ((*data).feedback_connection as c_int | OPL3_STEREO_BITS) as c_uchar;
    if (*chan).gm_pan < 43 {
        reg_val = (reg_val as c_int & !OPL3_VOICE_TO_RIGHT) as c_uchar;
    }
    if (*chan).gm_pan > 85 {
        reg_val = (reg_val as c_int & !OPL3_VOICE_TO_LEFT) as c_uchar;
    }
    opl3_reg = (OPL3_LEFT | (OPL3_REG_FEEDBACK_CONNECTION + voice_offset as c_int)) as c_ushort;
    ((*opl3).command)(opl3, opl3_reg, reg_val);
}

/*
 * Loads drum voices at init time
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_load_drums(opl3: *mut snd_opl3) {
    snd_opl3_drum_voice_set(opl3, &BASS_OP0);
    snd_opl3_drum_voice_set(opl3, &BASS_OP1);
    snd_opl3_drum_note_set(opl3, &BASS_NOTE);

    snd_opl3_drum_voice_set(opl3, &HIHAT);

    snd_opl3_drum_voice_set(opl3, &SNARE);
    snd_opl3_drum_note_set(opl3, &SNARE_NOTE);

    snd_opl3_drum_voice_set(opl3, &TOMTOM);
    snd_opl3_drum_note_set(opl3, &TOMTOM_NOTE);

    snd_opl3_drum_voice_set(opl3, &CYMBAL);
}

/*
 * Switch drum voice on or off
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_drum_switch(
    opl3: *mut snd_opl3,
    note: c_int,
    vel: c_int,
    on_off: c_int,
    chan: *mut snd_midi_channel,
) {
    let drum_mask: c_uchar;
    let drum_voice: *const snd_opl3_drum_voice;

    if ((*opl3).drum_reg as c_int & OPL3_PERCUSSION_ENABLE) == 0 {
        return;
    }

    if note < 35 || note > 81 {
        return;
    }
    drum_mask = SND_OPL3_DRUM_TABLE[(note - 35) as usize] as c_uchar;

    if on_off != 0 {
        match drum_mask as c_int {
            OPL3_BASSDRUM_ON => {
                drum_voice = &BASS_OP1;
            }
            OPL3_HIHAT_ON => {
                drum_voice = &HIHAT;
            }
            OPL3_SNAREDRUM_ON => {
                drum_voice = &SNARE;
            }
            OPL3_TOMTOM_ON => {
                drum_voice = &TOMTOM;
            }
            OPL3_CYMBAL_ON => {
                drum_voice = &CYMBAL;
            }
            _ => {
                drum_voice = &TOMTOM;
            }
        }

        snd_opl3_drum_vol_set(opl3, drum_voice, vel, chan);
        (*opl3).drum_reg = ((*opl3).drum_reg as c_int | drum_mask as c_int) as c_uchar;
    } else {
        (*opl3).drum_reg = ((*opl3).drum_reg as c_int & !(drum_mask as c_int)) as c_uchar;
    }
    ((*opl3).command)(
        opl3,
        (OPL3_LEFT | OPL3_REG_PERCUSSION) as c_ushort,
        (*opl3).drum_reg,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
