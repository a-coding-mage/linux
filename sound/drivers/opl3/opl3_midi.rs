// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Uros Bizjak <uros@kss-loka.si>
 *
 *  Midi synth routines for OPL2/OPL3/OPL4 FM
 */

// C dependencies: "opl3_voice.h", <sound/asoundef.h>
// DEBUG_ALLOC and DEBUG_MIDI are undefined in the original source.

extern "C" {
    static mut use_internal_drums: bool;
    static mut jiffies: c_ulong;
    static snd_opl3_regmap: [[c_uchar; 4]; MAX_OPL2_VOICES as usize];

    fn snd_opl3_drum_switch(
        opl3: *mut snd_opl3,
        note: c_int,
        vel: c_int,
        on: c_int,
        chan: *mut snd_midi_channel,
    );
    fn snd_opl3_find_patch(
        opl3: *mut snd_opl3,
        prog: c_int,
        bank: c_int,
        create_patch: c_int,
    ) -> *mut fm_patch;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn snd_BUG_ON(cond: bool) -> bool;
}

use core::ffi::{c_char, c_int, c_uchar, c_ulong, c_uint, c_ushort, c_void};

/*
 * The next table looks magical, but it certainly is not. Its values have
 * been calculated as table[i]=8*log(i/64)/log(2) with an obvious exception
 * for i=0. This log-table converts a linear volume-scaling (0..127) to a
 * logarithmic scaling as present in the FM-synthesizer chips. so :    Volume
 * 64 =  0 db = relative volume  0 and:    Volume 32 = -6 db = relative
 * volume -8 it was implemented as a table because it is only 128 bytes and
 * it saves a lot of log() calculations. (Rob Hooft <hooft@chem.ruu.nl>)
 */

static opl3_volume_table: [c_char; 128] = [
    -63, -48, -40, -35, -32, -29, -27, -26,
    -24, -23, -21, -20, -19, -18, -18, -17,
    -16, -15, -15, -14, -13, -13, -12, -12,
    -11, -11, -10, -10, -10, -9, -9, -8,
    -8, -8, -7, -7, -7, -6, -6, -6,
    -5, -5, -5, -5, -4, -4, -4, -4,
    -3, -3, -3, -3, -2, -2, -2, -2,
    -2, -1, -1, -1, -1, 0, 0, 0,
    0, 0, 0, 1, 1, 1, 1, 1,
    1, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 4,
    4, 4, 4, 4, 4, 4, 4, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 6, 6, 6, 6, 6, 6, 6,
    6, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 8, 8, 8, 8, 8,
];

#[no_mangle]
pub unsafe extern "C" fn snd_opl3_calc_volume(
    volbyte: *mut c_uchar,
    vel: c_int,
    chan: *mut snd_midi_channel,
) {
    let mut newvol: c_int;
    let mut volume: c_int;

    volume = (vel * (*chan).gm_volume * (*chan).gm_expression) / (127 * 127);
    if volume > 127 {
        volume = 127;
    }

    let oldvol = OPL3_TOTAL_LEVEL_MASK - ((*volbyte as c_int) & OPL3_TOTAL_LEVEL_MASK);

    newvol = opl3_volume_table[volume as usize] as c_int + oldvol;
    if newvol > OPL3_TOTAL_LEVEL_MASK {
        newvol = OPL3_TOTAL_LEVEL_MASK;
    } else if newvol < 0 {
        newvol = 0;
    }

    let n = OPL3_TOTAL_LEVEL_MASK - (newvol & OPL3_TOTAL_LEVEL_MASK);

    *volbyte = ((*volbyte as c_int & OPL3_KSL_MASK) | (n & OPL3_TOTAL_LEVEL_MASK)) as c_uchar;
}

/*
 * Converts the note frequency to block and fnum values for the FM chip
 */
static opl3_note_table: [c_short; 16] = [
    305, 323,       /* for pitch bending, -2 semitones */
    343, 363, 385, 408, 432, 458, 485, 514, 544, 577, 611, 647,
    686, 726,      /* for pitch bending, +2 semitones */
];

unsafe fn snd_opl3_calc_pitch(
    fnum: *mut c_uchar,
    blocknum: *mut c_uchar,
    note: c_int,
    chan: *mut snd_midi_channel,
) {
    let block = ((note / 12) & 0x07) - 1;
    let idx = (note % 12) + 2;
    let freq: c_int;

    if (*chan).midi_pitchbend != 0 {
        let mut pitchbend = (*chan).midi_pitchbend;
        let segment: c_int;

        if pitchbend < -0x2000 {
            pitchbend = -0x2000;
        }
        if pitchbend > 0x1FFF {
            pitchbend = 0x1FFF;
        }

        segment = pitchbend / 0x1000;
        freq = opl3_note_table[(idx + segment) as usize] as c_int
            + (((opl3_note_table[(idx + segment + 1) as usize] as c_int
                - opl3_note_table[(idx + segment) as usize] as c_int)
                * (pitchbend % 0x1000))
                / 0x1000);
    } else {
        freq = opl3_note_table[idx as usize] as c_int;
    }

    *fnum = freq as c_uchar;
    *blocknum = (((freq >> 8) & OPL3_FNUM_HIGH_MASK)
        | ((block << 2) & OPL3_BLOCKNUM_MASK)) as c_uchar;
}

/*
 * Get a FM voice (channel) to play a note on.
 */
unsafe fn opl3_get_voice(
    opl3: *mut snd_opl3,
    instr_4op: c_int,
    _chan: *mut snd_midi_channel,
) -> c_int {
    let mut chan_4op_1: c_int;     /* first voice for 4op instrument */
    let mut chan_4op_2: c_int;     /* second voice for 4op instrument */

    let mut vp: *mut snd_opl3_voice;
    let mut vp2: *mut snd_opl3_voice;
    let mut voice_time: c_uint;
    let mut i: c_int;

    /* This is our "allocation cost" table */
    const FREE: usize = 0;
    const CHEAP: usize = 1;
    const EXPENSIVE: usize = 2;
    const END: usize = 3;

    /* Keeps track of what we are finding */
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct best {
        time: c_uint,
        voice: c_int,
    }

    let mut best = [best { time: c_uint::MAX, voice: -1 }; END];

    i = 0;
    while i < END as c_int {
        best[i as usize].time = -1i32 as c_uint; /* XXX MAX_?INT really */
        best[i as usize].voice = -1;
        i += 1;
    }

    /* Look through all the channels for the most suitable. */
    i = 0;
    while i < (*opl3).max_voices {
        vp = (*opl3).voices.as_mut_ptr().add(i as usize);

        if (*vp).state == SNDRV_OPL3_ST_NOT_AVAIL {
            /* skip unavailable channels, allocated by
               drum voices or by bounded 4op voices) */
            i += 1;
            continue;
        }

        voice_time = (*vp).time;
        let mut bp = FREE;

        chan_4op_1 = ((i < 3) || (i > 8 && i < 12)) as c_int;
        chan_4op_2 = ((i > 2 && i < 6) || (i > 11 && i < 15)) as c_int;
        if instr_4op != 0 {
            /* allocate 4op voice */
            /* skip channels unavailable to 4op instrument */
            if chan_4op_1 == 0 {
                i += 1;
                continue;
            }

            if (*vp).state != 0 {
                /* kill one voice, CHEAP */
                bp += 1;
            }
            /* get state of bounded 2op channel
               to be allocated for 4op instrument */
            vp2 = (*opl3).voices.as_mut_ptr().add((i + 3) as usize);
            if (*vp2).state == SNDRV_OPL3_ST_ON_2OP {
                /* kill two voices, EXPENSIVE */
                bp += 1;
                voice_time = max(voice_time, (*vp2).time);
            }
        } else {
            /* allocate 2op voice */
            if chan_4op_1 != 0 || chan_4op_2 != 0 {
                /* use bounded channels for 2op, CHEAP */
                bp += 1;
            } else if (*vp).state != 0 {
                /* kill one voice on 2op channel, CHEAP */
                bp += 1;
            }
            /* raise kill cost to EXPENSIVE for all channels */
            if (*vp).state != 0 {
                bp += 1;
            }
        }
        if voice_time < best[bp].time {
            best[bp].time = voice_time;
            best[bp].voice = i;
        }
        i += 1;
    }

    i = 0;
    while i < END as c_int {
        if best[i as usize].voice >= 0 {
            return best[i as usize].voice;
        }
        i += 1;
    }
    /* not found */
    -1
}

/* ------------------------------ */

/*
 * System timer interrupt function
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_timer_func(t: *mut timer_list) {
    let opl3: *mut snd_opl3 = timer_container_of_opl3_t_tlist(t);
    let mut again: c_int = 0;
    let mut i: c_int;

    spin_lock_irqsave(&mut (*opl3).voice_lock);
    i = 0;
    while i < (*opl3).max_voices {
        let vp = (*opl3).voices.as_mut_ptr().add(i as usize);
        if (*vp).state > 0 && (*vp).note_off_check != 0 {
            if (*vp).note_off == jiffies {
                snd_opl3_note_off_unsafe(opl3 as *mut c_void, (*vp).note, 0, (*vp).chan);
            } else {
                again += 1;
            }
        }
        i += 1;
    }
    spin_unlock_irqrestore(&mut (*opl3).voice_lock);

    spin_lock_irqsave(&mut (*opl3).sys_timer_lock);
    if again != 0 {
        mod_timer(&mut (*opl3).tlist, jiffies + 1);    /* invoke again */
    } else {
        (*opl3).sys_timer_status = 0;
    }
    spin_unlock_irqrestore(&mut (*opl3).sys_timer_lock);
}

/*
 * Start system timer
 */
unsafe fn snd_opl3_start_timer(opl3: *mut snd_opl3) {
    spin_lock_irqsave(&mut (*opl3).sys_timer_lock);
    if (*opl3).sys_timer_status == 0 {
        mod_timer(&mut (*opl3).tlist, jiffies + 1);
        (*opl3).sys_timer_status = 1;
    }
    spin_unlock_irqrestore(&mut (*opl3).sys_timer_lock);
}

/* ------------------------------ */

static snd_opl3_oss_map: [c_int; MAX_OPL3_VOICES as usize] = [
    0, 1, 2, 9, 10, 11, 6, 7, 8, 15, 16, 17, 3, 4, 5, 12, 13, 14,
];

/*
 * Start a note.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_note_on(
    p: *mut c_void,
    mut note: c_int,
    vel: c_int,
    chan: *mut snd_midi_channel,
) {
    let opl3: *mut snd_opl3;
    let instr_4op: c_int;

    let voice: c_int;
    let mut vp: *mut snd_opl3_voice;
    let mut vp2: *mut snd_opl3_voice;
    let connect_mask: c_ushort;
    let mut connection: c_uchar;
    let mut vol_op: [c_uchar; 4] = [0; 4];

    let mut extra_prg: c_int = 0;

    let reg_side: c_ushort;
    let mut op_offset: c_uchar;
    let voice_offset: c_uchar;
    let mut opl3_reg: c_ushort;
    let mut reg_val: c_uchar;
    let mut prg: c_uchar;
    let mut bank: c_uchar;

    let key = note;
    let mut fnum: c_uchar = 0;
    let mut blocknum: c_uchar = 0;
    let mut i: c_int;

    let mut patch: *mut fm_patch;
    let fm: *mut fm_instrument;

    opl3 = p as *mut snd_opl3;

    /* in SYNTH mode, application takes care of voices */
    /* in SEQ mode, drum voice numbers are notes on drum channel */
    if (*opl3).synth_mode == SNDRV_OPL3_MODE_SEQ {
        if (*chan).drum_channel != 0 {
            /* percussion instruments are located in bank 128 */
            bank = 128;
            prg = note as c_uchar;
        } else {
            bank = (*chan).gm_bank_select as c_uchar;
            prg = (*chan).midi_program as c_uchar;
        }
    } else {
        /* Prepare for OSS mode */
        if (*chan).number >= MAX_OPL3_VOICES {
            return;
        }

        /* OSS instruments are located in bank 127 */
        bank = 127;
        prg = (*chan).midi_program as c_uchar;
    }

    spin_lock_irqsave(&mut (*opl3).voice_lock);

    if use_internal_drums {
        snd_opl3_drum_switch(opl3, note, vel, 1, chan);
        spin_unlock_irqrestore(&mut (*opl3).voice_lock);
        return;
    }

    loop {
        patch = snd_opl3_find_patch(opl3, prg as c_int, bank as c_int, 0);
        if patch.is_null() {
            spin_unlock_irqrestore(&mut (*opl3).voice_lock);
            return;
        }

        fm = &mut (*patch).inst;
        match (*patch).type_ {
            FM_PATCH_OPL2 => {
                instr_4op = 0;
            }
            FM_PATCH_OPL3 => {
                if (*opl3).hardware >= OPL3_HW_OPL3 {
                    instr_4op = 1;
                } else {
                    spin_unlock_irqrestore(&mut (*opl3).voice_lock);
                    return;
                }
            }
            _ => {
                spin_unlock_irqrestore(&mut (*opl3).voice_lock);
                return;
            }
        }
        /* in SYNTH mode, application takes care of voices */
        /* in SEQ mode, allocate voice on free OPL3 channel */
        if (*opl3).synth_mode == SNDRV_OPL3_MODE_SEQ {
            voice = opl3_get_voice(opl3, instr_4op, chan);
        } else {
            /* remap OSS voice */
            voice = snd_opl3_oss_map[(*chan).number as usize];
        }

        if voice < 0 {
            spin_unlock_irqrestore(&mut (*opl3).voice_lock);
            return;
        }

        if voice < MAX_OPL2_VOICES {
            /* Left register block for voices 0 .. 8 */
            reg_side = OPL3_LEFT;
            voice_offset = voice as c_uchar;
            connect_mask = ((OPL3_LEFT_4OP_0 << voice_offset) & 0x07) as c_ushort;
        } else {
            /* Right register block for voices 9 .. 17 */
            reg_side = OPL3_RIGHT;
            voice_offset = (voice - MAX_OPL2_VOICES) as c_uchar;
            connect_mask = ((OPL3_RIGHT_4OP_0 << voice_offset) & 0x38) as c_ushort;
        }

        /* kill voice on channel */
        vp = (*opl3).voices.as_mut_ptr().add(voice as usize);
        if (*vp).state > 0 {
            opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK + voice_offset as c_ushort);
            reg_val = (*vp).keyon_reg & !OPL3_KEYON_BIT;
            ((*opl3).command)(opl3, opl3_reg, reg_val);
        }
        if instr_4op != 0 {
            vp2 = (*opl3).voices.as_mut_ptr().add((voice + 3) as usize);
            if (*vp2).state > 0 {
                opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK + voice_offset as c_ushort + 3);
                reg_val = (*vp).keyon_reg & !OPL3_KEYON_BIT;
                ((*opl3).command)(opl3, opl3_reg, reg_val);
            }
        }

        /* set connection register */
        if instr_4op != 0 {
            if (((*opl3).connection_reg as c_ushort ^ connect_mask) & connect_mask) != 0 {
                (*opl3).connection_reg |= connect_mask as c_uchar;
                /* set connection bit */
                opl3_reg = OPL3_RIGHT | OPL3_REG_CONNECTION_SELECT;
                ((*opl3).command)(opl3, opl3_reg, (*opl3).connection_reg);
            }
        } else if (((*opl3).connection_reg as c_ushort ^ !connect_mask) & connect_mask) != 0 {
            (*opl3).connection_reg &= !(connect_mask as c_uchar);
            /* clear connection bit */
            opl3_reg = OPL3_RIGHT | OPL3_REG_CONNECTION_SELECT;
            ((*opl3).command)(opl3, opl3_reg, (*opl3).connection_reg);
        }

        /*
         * calculate volume depending on connection
         * between FM operators (see include/opl3.h)
         */
        i = 0;
        while i < if instr_4op != 0 { 4 } else { 2 } {
            vol_op[i as usize] = (*fm).op[i as usize].ksl_level;
            i += 1;
        }

        connection = (*fm).feedback_connection[0] & 0x01;
        if instr_4op != 0 {
            connection <<= 1;
            connection |= (*fm).feedback_connection[1] & 0x01;

            snd_opl3_calc_volume(&mut vol_op[3], vel, chan);
            match connection {
                0x03 => {
                    snd_opl3_calc_volume(&mut vol_op[2], vel, chan);
                    snd_opl3_calc_volume(&mut vol_op[0], vel, chan);
                }
                0x02 => {
                    snd_opl3_calc_volume(&mut vol_op[0], vel, chan);
                }
                0x01 => {
                    snd_opl3_calc_volume(&mut vol_op[1], vel, chan);
                }
                _ => {}
            }
        } else {
            snd_opl3_calc_volume(&mut vol_op[1], vel, chan);
            if connection != 0 {
                snd_opl3_calc_volume(&mut vol_op[0], vel, chan);
            }
        }

        /* Program the FM voice characteristics */
        i = 0;
        while i < if instr_4op != 0 { 4 } else { 2 } {
            op_offset = snd_opl3_regmap[voice_offset as usize][i as usize];

            /* Set OPL3 AM_VIB register of requested voice/operator */
            reg_val = (*fm).op[i as usize].am_vib;
            opl3_reg = reg_side | (OPL3_REG_AM_VIB + op_offset as c_ushort);
            ((*opl3).command)(opl3, opl3_reg, reg_val);

            /* Set OPL3 KSL_LEVEL register of requested voice/operator */
            reg_val = vol_op[i as usize];
            opl3_reg = reg_side | (OPL3_REG_KSL_LEVEL + op_offset as c_ushort);
            ((*opl3).command)(opl3, opl3_reg, reg_val);

            /* Set OPL3 ATTACK_DECAY register of requested voice/operator */
            reg_val = (*fm).op[i as usize].attack_decay;
            opl3_reg = reg_side | (OPL3_REG_ATTACK_DECAY + op_offset as c_ushort);
            ((*opl3).command)(opl3, opl3_reg, reg_val);

            /* Set OPL3 SUSTAIN_RELEASE register of requested voice/operator */
            reg_val = (*fm).op[i as usize].sustain_release;
            opl3_reg = reg_side | (OPL3_REG_SUSTAIN_RELEASE + op_offset as c_ushort);
            ((*opl3).command)(opl3, opl3_reg, reg_val);

            /* Select waveform */
            reg_val = (*fm).op[i as usize].wave_select;
            opl3_reg = reg_side | (OPL3_REG_WAVE_SELECT + op_offset as c_ushort);
            ((*opl3).command)(opl3, opl3_reg, reg_val);
            i += 1;
        }

        /* Set operator feedback and 2op inter-operator connection */
        reg_val = (*fm).feedback_connection[0];
        /* Set output voice connection */
        reg_val |= OPL3_STEREO_BITS;
        if (*chan).gm_pan < 43 {
            reg_val &= !OPL3_VOICE_TO_RIGHT;
        }
        if (*chan).gm_pan > 85 {
            reg_val &= !OPL3_VOICE_TO_LEFT;
        }
        opl3_reg = reg_side | (OPL3_REG_FEEDBACK_CONNECTION + voice_offset as c_ushort);
        ((*opl3).command)(opl3, opl3_reg, reg_val);

        if instr_4op != 0 {
            /* Set 4op inter-operator connection */
            reg_val = (*fm).feedback_connection[1] & OPL3_CONNECTION_BIT;
            /* Set output voice connection */
            reg_val |= OPL3_STEREO_BITS;
            if (*chan).gm_pan < 43 {
                reg_val &= !OPL3_VOICE_TO_RIGHT;
            }
            if (*chan).gm_pan > 85 {
                reg_val &= !OPL3_VOICE_TO_LEFT;
            }
            opl3_reg = reg_side | (OPL3_REG_FEEDBACK_CONNECTION + voice_offset as c_ushort + 3);
            ((*opl3).command)(opl3, opl3_reg, reg_val);
        }

        /*
         * Special treatment of percussion notes for fm:
         * Requested pitch is really program, and pitch for
         * device is whatever was specified in the patch library.
         */
        if (*fm).fix_key != 0 {
            note = (*fm).fix_key as c_int;
        }
        /*
         * use transpose if defined in patch library
         */
        if (*fm).trnsps != 0 {
            note += (*fm).trnsps as c_int - 64;
        }

        snd_opl3_calc_pitch(&mut fnum, &mut blocknum, note, chan);

        /* Set OPL3 FNUM_LOW register of requested voice */
        opl3_reg = reg_side | (OPL3_REG_FNUM_LOW + voice_offset as c_ushort);
        ((*opl3).command)(opl3, opl3_reg, fnum);

        (*opl3).voices[voice as usize].keyon_reg = blocknum;

        /* Set output sound flag */
        blocknum |= OPL3_KEYON_BIT;

        /* Set OPL3 KEYON_BLOCK register of requested voice */
        opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK + voice_offset as c_ushort);
        ((*opl3).command)(opl3, opl3_reg, blocknum);

        /* kill note after fixed duration (in centiseconds) */
        if (*fm).fix_dur != 0 {
            (*opl3).voices[voice as usize].note_off =
                jiffies + ((*fm).fix_dur as c_ulong * HZ as c_ulong) / 100;
            snd_opl3_start_timer(opl3);
            (*opl3).voices[voice as usize].note_off_check = 1;
        } else {
            (*opl3).voices[voice as usize].note_off_check = 0;
        }

        /* get extra pgm, but avoid possible loops */
        extra_prg = if extra_prg != 0 { 0 } else { (*fm).modes as c_int };

        /* do the bookkeeping */
        (*vp).time = (*opl3).use_time;
        (*opl3).use_time = (*opl3).use_time.wrapping_add(1);
        (*vp).note = key;
        (*vp).chan = chan;

        if instr_4op != 0 {
            (*vp).state = SNDRV_OPL3_ST_ON_4OP;

            vp2 = (*opl3).voices.as_mut_ptr().add((voice + 3) as usize);
            (*vp2).time = (*opl3).use_time;
            (*opl3).use_time = (*opl3).use_time.wrapping_add(1);
            (*vp2).note = key;
            (*vp2).chan = chan;
            (*vp2).state = SNDRV_OPL3_ST_NOT_AVAIL;
        } else {
            if (*vp).state == SNDRV_OPL3_ST_ON_4OP {
                /* 4op killed by 2op, release bounded voice */
                vp2 = (*opl3).voices.as_mut_ptr().add((voice + 3) as usize);
                (*vp2).time = (*opl3).use_time;
                (*opl3).use_time = (*opl3).use_time.wrapping_add(1);
                (*vp2).state = SNDRV_OPL3_ST_OFF;
            }
            (*vp).state = SNDRV_OPL3_ST_ON_2OP;
        }

        /* allocate extra program if specified in patch library */
        if extra_prg != 0 {
            if extra_prg > 128 {
                bank = 128;
                /* percussions start at 35 */
                prg = (extra_prg - 128 + 35 - 1) as c_uchar;
            } else {
                bank = 0;
                prg = (extra_prg - 1) as c_uchar;
            }
            continue;
        }

        spin_unlock_irqrestore(&mut (*opl3).voice_lock);
        break;
    }
}

unsafe fn snd_opl3_kill_voice(opl3: *mut snd_opl3, voice: c_int) {
    let reg_side: c_ushort;
    let voice_offset: c_uchar;
    let opl3_reg: c_ushort;

    let vp: *mut snd_opl3_voice;
    let vp2: *mut snd_opl3_voice;

    if snd_BUG_ON(voice >= MAX_OPL3_VOICES) {
        return;
    }

    vp = (*opl3).voices.as_mut_ptr().add(voice as usize);
    if voice < MAX_OPL2_VOICES {
        /* Left register block for voices 0 .. 8 */
        reg_side = OPL3_LEFT;
        voice_offset = voice as c_uchar;
    } else {
        /* Right register block for voices 9 .. 17 */
        reg_side = OPL3_RIGHT;
        voice_offset = (voice - MAX_OPL2_VOICES) as c_uchar;
    }

    /* kill voice */
    opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK + voice_offset as c_ushort);
    /* clear Key ON bit */
    ((*opl3).command)(opl3, opl3_reg, (*vp).keyon_reg);

    /* do the bookkeeping */
    (*vp).time = (*opl3).use_time;
    (*opl3).use_time = (*opl3).use_time.wrapping_add(1);

    if (*vp).state == SNDRV_OPL3_ST_ON_4OP {
        vp2 = (*opl3).voices.as_mut_ptr().add((voice + 3) as usize);

        (*vp2).time = (*opl3).use_time;
        (*opl3).use_time = (*opl3).use_time.wrapping_add(1);
        (*vp2).state = SNDRV_OPL3_ST_OFF;
    }
    (*vp).state = SNDRV_OPL3_ST_OFF;
}

/*
 * Release a note in response to a midi note off.
 */
unsafe fn snd_opl3_note_off_unsafe(
    p: *mut c_void,
    note: c_int,
    vel: c_int,
    chan: *mut snd_midi_channel,
) {
    let opl3: *mut snd_opl3;

    let mut voice: c_int;
    let mut vp: *mut snd_opl3_voice;

    opl3 = p as *mut snd_opl3;

    if (*opl3).synth_mode == SNDRV_OPL3_MODE_SEQ {
        if (*chan).drum_channel != 0 && use_internal_drums {
            snd_opl3_drum_switch(opl3, note, vel, 0, chan);
            return;
        }
        /* this loop will hopefully kill all extra voices, because
           they are grouped by the same channel and note values */
        voice = 0;
        while voice < (*opl3).max_voices {
            vp = (*opl3).voices.as_mut_ptr().add(voice as usize);
            if (*vp).state > 0 && (*vp).chan == chan && (*vp).note == note {
                snd_opl3_kill_voice(opl3, voice);
            }
            voice += 1;
        }
    } else {
        /* remap OSS voices */
        if (*chan).number < MAX_OPL3_VOICES {
            voice = snd_opl3_oss_map[(*chan).number as usize];
            snd_opl3_kill_voice(opl3, voice);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_opl3_note_off(
    p: *mut c_void,
    note: c_int,
    vel: c_int,
    chan: *mut snd_midi_channel,
) {
    let opl3: *mut snd_opl3 = p as *mut snd_opl3;

    spin_lock_irqsave(&mut (*opl3).voice_lock);
    snd_opl3_note_off_unsafe(p, note, vel, chan);
    spin_unlock_irqrestore(&mut (*opl3).voice_lock);
}

/*
 * key pressure change
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_key_press(
    _p: *mut c_void,
    _note: c_int,
    _vel: c_int,
    _chan: *mut snd_midi_channel,
) {
}

/*
 * terminate note
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_terminate_note(
    _p: *mut c_void,
    _note: c_int,
    _chan: *mut snd_midi_channel,
) {
}

unsafe fn snd_opl3_update_pitch(opl3: *mut snd_opl3, voice: c_int) {
    let reg_side: c_ushort;
    let voice_offset: c_uchar;
    let mut opl3_reg: c_ushort;

    let mut fnum: c_uchar = 0;
    let mut blocknum: c_uchar = 0;

    let vp: *mut snd_opl3_voice;

    if snd_BUG_ON(voice >= MAX_OPL3_VOICES) {
        return;
    }

    vp = (*opl3).voices.as_mut_ptr().add(voice as usize);
    if (*vp).chan.is_null() {
        return; /* not allocated? */
    }

    if voice < MAX_OPL2_VOICES {
        /* Left register block for voices 0 .. 8 */
        reg_side = OPL3_LEFT;
        voice_offset = voice as c_uchar;
    } else {
        /* Right register block for voices 9 .. 17 */
        reg_side = OPL3_RIGHT;
        voice_offset = (voice - MAX_OPL2_VOICES) as c_uchar;
    }

    snd_opl3_calc_pitch(&mut fnum, &mut blocknum, (*vp).note, (*vp).chan);

    /* Set OPL3 FNUM_LOW register of requested voice */
    opl3_reg = reg_side | (OPL3_REG_FNUM_LOW + voice_offset as c_ushort);
    ((*opl3).command)(opl3, opl3_reg, fnum);

    (*vp).keyon_reg = blocknum;

    /* Set output sound flag */
    blocknum |= OPL3_KEYON_BIT;

    /* Set OPL3 KEYON_BLOCK register of requested voice */
    opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK + voice_offset as c_ushort);
    ((*opl3).command)(opl3, opl3_reg, blocknum);

    (*vp).time = (*opl3).use_time;
    (*opl3).use_time = (*opl3).use_time.wrapping_add(1);
}

/*
 * Update voice pitch controller
 */
unsafe fn snd_opl3_pitch_ctrl(opl3: *mut snd_opl3, chan: *mut snd_midi_channel) {
    let mut voice: c_int;
    let mut vp: *mut snd_opl3_voice;

    spin_lock_irqsave(&mut (*opl3).voice_lock);

    if (*opl3).synth_mode == SNDRV_OPL3_MODE_SEQ {
        voice = 0;
        while voice < (*opl3).max_voices {
            vp = (*opl3).voices.as_mut_ptr().add(voice as usize);
            if (*vp).state > 0 && (*vp).chan == chan {
                snd_opl3_update_pitch(opl3, voice);
            }
            voice += 1;
        }
    } else {
        /* remap OSS voices */
        if (*chan).number < MAX_OPL3_VOICES {
            voice = snd_opl3_oss_map[(*chan).number as usize];
            snd_opl3_update_pitch(opl3, voice);
        }
    }

    spin_unlock_irqrestore(&mut (*opl3).voice_lock);
}

/*
 * Deal with a controller type event.  This includes all types of
 * control events, not just the midi controllers
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_control(
    p: *mut c_void,
    type_: c_int,
    chan: *mut snd_midi_channel,
) {
    let opl3: *mut snd_opl3;

    opl3 = p as *mut snd_opl3;

    match type_ {
        MIDI_CTL_MSB_MODWHEEL => {
            if (*chan).control[MIDI_CTL_MSB_MODWHEEL as usize] > 63 {
                (*opl3).drum_reg |= OPL3_VIBRATO_DEPTH;
            } else {
                (*opl3).drum_reg &= !OPL3_VIBRATO_DEPTH;
            }
            ((*opl3).command)(opl3, OPL3_LEFT | OPL3_REG_PERCUSSION, (*opl3).drum_reg);
        }
        MIDI_CTL_E2_TREMOLO_DEPTH => {
            if (*chan).control[MIDI_CTL_E2_TREMOLO_DEPTH as usize] > 63 {
                (*opl3).drum_reg |= OPL3_TREMOLO_DEPTH;
            } else {
                (*opl3).drum_reg &= !OPL3_TREMOLO_DEPTH;
            }
            ((*opl3).command)(opl3, OPL3_LEFT | OPL3_REG_PERCUSSION, (*opl3).drum_reg);
        }
        MIDI_CTL_PITCHBEND => {
            snd_opl3_pitch_ctrl(opl3, chan);
        }
        _ => {}
    }
}

/*
 * NRPN events
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_nrpn(
    _p: *mut c_void,
    _chan: *mut snd_midi_channel,
    _chset: *mut snd_midi_channel_set,
) {
}

/*
 * receive sysex
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_sysex(
    _p: *mut c_void,
    _buf: *mut c_uchar,
    _len: c_int,
    _parsed: c_int,
    _chset: *mut snd_midi_channel_set,
) {
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
