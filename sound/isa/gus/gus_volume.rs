// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// C includes translated as external dependencies:
// <linux/time.h>, <linux/export.h>, <sound/core.h>, <sound/gus.h>, "gus_tables.h"
// __GUS_TABLES_ALLOC__ was defined before including gus_tables.h.

// EXPORT_SYMBOL(snd_gf1_atten_table); /* for snd-gus-synth module */

pub unsafe fn snd_gf1_lvol_to_gvol_raw(mut vol: u32) -> u16 {
    let mut e: u16;
    let mut m: u16;
    let mut tmp: u16;

    if vol > 65535 {
        vol = 65535;
    }
    tmp = vol as u16;
    e = 7;
    if tmp < 128 {
        while e > 0 && tmp < ((1 as u16) << e) {
            e -= 1;
        }
    } else {
        while tmp > 255 {
            tmp >>= 1;
            e += 1;
        }
    }
    m = vol.wrapping_sub((1u32) << e) as u16;
    if m > 0 {
        if e > 8 {
            m >>= e - 8;
        } else if e < 8 {
            m <<= 8 - e;
        }
        m &= 255;
    }
    (e << 8) | m
}

// C source disabled this block with #if 0.
#[cfg(any())]
pub unsafe fn snd_gf1_gvol_to_lvol_raw(gf1_vol: u16) -> u32 {
    let rvol: u32;
    let e: u16;
    let m: u16;

    if gf1_vol == 0 {
        return 0;
    }
    e = gf1_vol >> 8;
    m = (gf1_vol as u8) as u16;
    rvol = (1 as u32) << e;
    if e > 8 {
        return rvol | ((m as u32) << (e - 8));
    }
    rvol | ((m as u32) >> (8 - e))
}

// C source disabled this block with #if 0.
#[cfg(any())]
pub unsafe fn snd_gf1_calc_ramp_rate(
    gus: *mut snd_gus_card,
    mut start: u16,
    mut end: u16,
    mut us: u32,
) -> u32 {
    static VOL_RATES: [u8; 19] = [
        23, 24, 26, 28, 29, 31, 32, 34,
        36, 37, 39, 40, 42, 44, 45, 47,
        49, 50, 52,
    ];
    let mut range: u16;
    let increment: u16;
    let mut value: u16;
    let mut i: u16;

    start >>= 4;
    end >>= 4;
    if start < end {
        us /= (end - start) as u32;
    } else {
        us /= (start - end) as u32;
    }
    range = 4;
    value = if (*gus).gf1.enh_mode != 0 {
        VOL_RATES[0] as u16
    } else {
        VOL_RATES[((*gus).gf1.active_voices - 14) as usize] as u16
    };
    i = 0;
    while i < 3 {
        if us < value as u32 {
            range = i;
            break;
        } else {
            value <<= 3;
        }
        i += 1;
    }
    if range == 4 {
        range = 3;
        increment = 1;
    } else {
        increment = ((value + (value >> 1)) as u32 / us) as u16;
    }
    ((range << 6) | (increment & 0x3f)) as u32
}

pub unsafe fn snd_gf1_translate_freq(gus: *mut snd_gus_card, mut freq16: u32) -> u16 {
    freq16 >>= 3;
    if freq16 < 50 {
        freq16 = 50;
    }
    if (freq16 & 0xf8000000) != 0 {
        freq16 = !0xf8000000u32;
        dev_err(
            (*(*gus).card).dev,
            c"%s: overflow - freq = 0x%x\n".as_ptr(),
            c"snd_gf1_translate_freq".as_ptr(),
            freq16,
        );
    }
    (((freq16 << 9) + ((*gus).gf1.playback_freq >> 1)) / (*gus).gf1.playback_freq) as u16
}

// C source disabled this block with #if 0.
#[cfg(any())]
pub unsafe fn snd_gf1_compute_vibrato(cents: i16, fc_register: u16) -> i16 {
    static VIBRATO_TABLE: [i16; 20] = [
        0, 0, 32, 592, 61, 1175, 93, 1808,
        124, 2433, 152, 3007, 182, 3632, 213, 4290,
        241, 4834, 255, 5200,
    ];

    let mut depth: libc::c_long;
    let mut vi1: *const i16;
    let mut vi2: *const i16;
    let pcents: i16;
    let v1: i16;

    pcents = if cents < 0 { -cents } else { cents };
    vi1 = VIBRATO_TABLE.as_ptr();
    vi2 = vi1.add(2);
    while pcents > *vi2 {
        vi1 = vi2;
        vi2 = vi2.add(2);
    }
    v1 = *vi1.add(1);
    /* The FC table above is a list of pairs. The first number in the pair     */
    /* is the cents index from 0-255 cents, and the second number in the       */
    /* pair is the FC adjustment needed to change the pitch by the indexed     */
    /* number of cents. The table was created for an FC of 32768.              */
    /* The following expression does a linear interpolation against the        */
    /* approximated log curve in the table above, and then scales the number   */
    /* by the FC before the LFO. This calculation also adjusts the output      */
    /* value to produce the appropriate depth for the hardware. The depth      */
    /* is 2 * desired FC + 1.                                                  */
    depth = (((((*vi2.add(1) - *vi1) as i32) * ((pcents - *vi1) as i32)
        / ((*vi2 - *vi1) as i32))
        + v1 as i32) as libc::c_long
        * fc_register as libc::c_long)
        >> 14;
    if depth != 0 {
        depth += 1;
    }
    if depth > 255 {
        depth = 255;
    }
    if cents < 0 {
        -(depth as i16)
    } else {
        depth as i16
    }
}

// C source disabled this block with #if 0.
#[cfg(any())]
pub unsafe fn snd_gf1_compute_pitchbend(pitchbend: u16, sens: u16) -> u16 {
    static LOG_TABLE: [libc::c_long; 12] = [
        1024, 1085, 1149, 1218, 1290, 1367, 1448, 1534, 1625, 1722, 1825, 1933,
    ];
    let wheel: i32;
    let mut sensitivity: i32;
    let mantissa: u32;
    let f1: u32;
    let f2: u32;
    let semitones: u16;
    let f1_index: u16;
    let f2_index: u16;
    let f1_power: u16;
    let f2_power: u16;
    let mut bend_down: i8 = 0;
    let mut bend: i32;

    if sens == 0 {
        return 1024;
    }
    wheel = pitchbend as i32 - 8192;
    sensitivity = (sens as i32 * wheel) / 128;
    if sensitivity < 0 {
        bend_down = 1;
        sensitivity = -sensitivity;
    }
    semitones = (sensitivity >> 13) as u32 as u16;
    mantissa = (sensitivity % 8192) as u32;
    f1_index = semitones % 12;
    f2_index = (semitones + 1) % 12;
    f1_power = semitones / 12;
    f2_power = (semitones + 1) / 12;
    f1 = (LOG_TABLE[f1_index as usize] as u32) << f1_power;
    f2 = (LOG_TABLE[f2_index as usize] as u32) << f2_power;
    bend = (((f2 - f1) * mantissa) >> 13).wrapping_add(f1) as i32;
    if bend_down != 0 {
        bend = 1048576i64.wrapping_div(bend as i64) as i32;
    }
    bend as u16
}

// C source disabled this block with #if 0.
#[cfg(any())]
pub unsafe fn snd_gf1_compute_freq(freq: u32, rate: u32, mix_rate: u16) -> u16 {
    let mut freq = freq;
    let mut fc: u32;
    let mut scale: i32 = 0;

    while freq >= 4194304 {
        scale += 1;
        freq >>= 1;
    }
    fc = (freq << 10) / rate;
    if fc > 97391 {
        fc = 97391;
        pr_err(c"patch: (1) fc frequency overflow - %u\n".as_ptr(), fc);
    }
    fc = (fc * 44100) / mix_rate as u32;
    while scale != 0 {
        scale -= 1;
        fc <<= 1;
    }
    if fc > 65535 {
        fc = 65535;
        pr_err(c"patch: (2) fc frequency overflow - %u\n".as_ptr(), fc);
    }
    fc as u16
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
