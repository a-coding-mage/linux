// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  NRPN / SYSEX callbacks for Emu8k/Emu10k1
 *
 *  Copyright (c) 1999-2000 Takashi Iwai <tiwai@suse.de>
 */

// Requires: emux_voice.h, sound/asoundef.h

/*
 * conversion from NRPN/control parameters to Emu8000 raw parameters
 */

/* NRPN / CC -> Emu8000 parameter converter */
#[repr(C)]
struct nrpn_conv_table {
    control: i32,
    effect: i32,
    convert: fn(i32) -> i32,
}

/* effect sensitivity */

const FX_CUTOFF: usize = 0;
const FX_RESONANCE: usize = 1;
const FX_ATTACK: usize = 2;
const FX_RELEASE: usize = 3;
const FX_VIBRATE: usize = 4;
const FX_VIBDEPTH: usize = 5;
const FX_VIBDELAY: usize = 6;
const FX_NUMS: usize = 7;

/*
 * convert NRPN/control values
 */

fn send_converted_effect(
    table: *const nrpn_conv_table,
    num_tables: i32,
    port: *mut snd_emux_port,
    chan: *mut snd_midi_channel,
    type_: i32,
    val: i32,
    mode: i32,
) -> i32 {
    let mut i = 0;
    while i < num_tables {
        unsafe {
            if (*table.add(i as usize)).control == type_ {
                let cval = ((*table.add(i as usize)).convert)(val);
                snd_emux_send_effect(port, chan, (*table.add(i as usize)).effect, cval, mode);
                return 1;
            }
        }
        i += 1;
    }
    0
}

const DEF_FX_CUTOFF: i32 = 170;
const DEF_FX_RESONANCE: i32 = 6;
const DEF_FX_ATTACK: i32 = 50;
const DEF_FX_RELEASE: i32 = 50;
const DEF_FX_VIBRATE: i32 = 30;
const DEF_FX_VIBDEPTH: i32 = 4;
const DEF_FX_VIBDELAY: i32 = 1500;

/* effect sensitivities for GS NRPN:
 *  adjusted for chaos 8MB soundfonts
 */
static GS_SENSE: [i32; 7] = [
    DEF_FX_CUTOFF,
    DEF_FX_RESONANCE,
    DEF_FX_ATTACK,
    DEF_FX_RELEASE,
    DEF_FX_VIBRATE,
    DEF_FX_VIBDEPTH,
    DEF_FX_VIBDELAY,
];

/* effect sensitivities for XG controls:
 * adjusted for chaos 8MB soundfonts
 */
static XG_SENSE: [i32; 7] = [
    DEF_FX_CUTOFF,
    DEF_FX_RESONANCE,
    DEF_FX_ATTACK,
    DEF_FX_RELEASE,
    DEF_FX_VIBRATE,
    DEF_FX_VIBDEPTH,
    DEF_FX_VIBDELAY,
];


/*
 * AWE32 NRPN effects
 */

fn fx_delay(val: i32) -> i32 {
    snd_sf_calc_parm_delay(val) as u16 as i32
}

fn fx_attack(val: i32) -> i32 {
    snd_sf_calc_parm_attack(val) as u16 as i32
}

fn fx_hold(val: i32) -> i32 {
    snd_sf_calc_parm_hold(val) as u16 as i32
}

fn fx_decay(val: i32) -> i32 {
    snd_sf_calc_parm_decay(val) as u16 as i32
}

fn fx_the_value(val: i32) -> i32 {
    ((val & 0xff) as u16) as i32
}

fn fx_twice_value(val: i32) -> i32 {
    (((val * 2) & 0xff) as u16) as i32
}

fn fx_conv_pitch(val: i32) -> i32 {
    ((val * 4096 / 1200) as i16) as i32
}

fn fx_conv_Q(val: i32) -> i32 {
    (((val / 8) & 0xff) as u16) as i32
}


static AWE_EFFECTS: [nrpn_conv_table; 27] = [
    nrpn_conv_table { control: 0, effect: EMUX_FX_LFO1_DELAY, convert: fx_delay },
    nrpn_conv_table { control: 1, effect: EMUX_FX_LFO1_FREQ, convert: fx_twice_value },
    nrpn_conv_table { control: 2, effect: EMUX_FX_LFO2_DELAY, convert: fx_delay },
    nrpn_conv_table { control: 3, effect: EMUX_FX_LFO2_FREQ, convert: fx_twice_value },

    nrpn_conv_table { control: 4, effect: EMUX_FX_ENV1_DELAY, convert: fx_delay },
    nrpn_conv_table { control: 5, effect: EMUX_FX_ENV1_ATTACK, convert: fx_attack },
    nrpn_conv_table { control: 6, effect: EMUX_FX_ENV1_HOLD, convert: fx_hold },
    nrpn_conv_table { control: 7, effect: EMUX_FX_ENV1_DECAY, convert: fx_decay },
    nrpn_conv_table { control: 8, effect: EMUX_FX_ENV1_SUSTAIN, convert: fx_the_value },
    nrpn_conv_table { control: 9, effect: EMUX_FX_ENV1_RELEASE, convert: fx_decay },

    nrpn_conv_table { control: 10, effect: EMUX_FX_ENV2_DELAY, convert: fx_delay },
    nrpn_conv_table { control: 11, effect: EMUX_FX_ENV2_ATTACK, convert: fx_attack },
    nrpn_conv_table { control: 12, effect: EMUX_FX_ENV2_HOLD, convert: fx_hold },
    nrpn_conv_table { control: 13, effect: EMUX_FX_ENV2_DECAY, convert: fx_decay },
    nrpn_conv_table { control: 14, effect: EMUX_FX_ENV2_SUSTAIN, convert: fx_the_value },
    nrpn_conv_table { control: 15, effect: EMUX_FX_ENV2_RELEASE, convert: fx_decay },

    nrpn_conv_table { control: 16, effect: EMUX_FX_INIT_PITCH, convert: fx_conv_pitch },
    nrpn_conv_table { control: 17, effect: EMUX_FX_LFO1_PITCH, convert: fx_the_value },
    nrpn_conv_table { control: 18, effect: EMUX_FX_LFO2_PITCH, convert: fx_the_value },
    nrpn_conv_table { control: 19, effect: EMUX_FX_ENV1_PITCH, convert: fx_the_value },
    nrpn_conv_table { control: 20, effect: EMUX_FX_LFO1_VOLUME, convert: fx_twice_value },
    nrpn_conv_table { control: 21, effect: EMUX_FX_CUTOFF, convert: fx_twice_value },
    nrpn_conv_table { control: 22, effect: EMUX_FX_FILTERQ, convert: fx_conv_Q },
    nrpn_conv_table { control: 23, effect: EMUX_FX_LFO1_CUTOFF, convert: fx_twice_value },
    nrpn_conv_table { control: 24, effect: EMUX_FX_ENV1_CUTOFF, convert: fx_the_value },
    nrpn_conv_table { control: 25, effect: EMUX_FX_CHORUS, convert: fx_the_value },
    nrpn_conv_table { control: 26, effect: EMUX_FX_REVERB, convert: fx_the_value },
];


/*
 * GS(SC88) NRPN effects; still experimental
 */

/* cutoff: quarter semitone step, max=255 */
fn gs_cutoff(val: i32) -> i32 {
    (val - 64) * GS_SENSE[FX_CUTOFF] / 50
}

/* resonance: 0 to 15(max) */
fn gs_filterQ(val: i32) -> i32 {
    (val - 64) * GS_SENSE[FX_RESONANCE] / 50
}

/* attack: */
fn gs_attack(val: i32) -> i32 {
    -((val - 64) * GS_SENSE[FX_ATTACK] / 50)
}

/* decay: */
fn gs_decay(val: i32) -> i32 {
    -((val - 64) * GS_SENSE[FX_RELEASE] / 50)
}

/* release: */
fn gs_release(val: i32) -> i32 {
    -((val - 64) * GS_SENSE[FX_RELEASE] / 50)
}

/* vibrato freq: 0.042Hz step, max=255 */
fn gs_vib_rate(val: i32) -> i32 {
    (val - 64) * GS_SENSE[FX_VIBRATE] / 50
}

/* vibrato depth: max=127, 1 octave */
fn gs_vib_depth(val: i32) -> i32 {
    (val - 64) * GS_SENSE[FX_VIBDEPTH] / 50
}

/* vibrato delay: -0.725msec step */
fn gs_vib_delay(val: i32) -> i32 {
    -((val - 64) * GS_SENSE[FX_VIBDELAY] / 50)
}

static GS_EFFECTS: [nrpn_conv_table; 8] = [
    nrpn_conv_table { control: 32, effect: EMUX_FX_CUTOFF, convert: gs_cutoff },
    nrpn_conv_table { control: 33, effect: EMUX_FX_FILTERQ, convert: gs_filterQ },
    nrpn_conv_table { control: 99, effect: EMUX_FX_ENV2_ATTACK, convert: gs_attack },
    nrpn_conv_table { control: 100, effect: EMUX_FX_ENV2_DECAY, convert: gs_decay },
    nrpn_conv_table { control: 102, effect: EMUX_FX_ENV2_RELEASE, convert: gs_release },
    nrpn_conv_table { control: 8, effect: EMUX_FX_LFO1_FREQ, convert: gs_vib_rate },
    nrpn_conv_table { control: 9, effect: EMUX_FX_LFO1_VOLUME, convert: gs_vib_depth },
    nrpn_conv_table { control: 10, effect: EMUX_FX_LFO1_DELAY, convert: gs_vib_delay },
];


/*
 * NRPN events
 */
pub fn snd_emux_nrpn(
    p: *mut std::ffi::c_void,
    chan: *mut snd_midi_channel,
    chset: *mut snd_midi_channel_set,
) {
    let port = p as *mut snd_emux_port;

    if snd_BUG_ON(port.is_null() || chan.is_null()) {
        return;
    }

    if unsafe {
        (*chan).control[MIDI_CTL_NONREG_PARM_NUM_MSB as usize] == 127
            && (*chan).control[MIDI_CTL_NONREG_PARM_NUM_LSB as usize] <= 26
    } {
        unsafe {
            let val = (((*chan).control[MIDI_CTL_MSB_DATA_ENTRY as usize] << 7)
                | (*chan).control[MIDI_CTL_LSB_DATA_ENTRY as usize])
                - 8192;
            send_converted_effect(
                AWE_EFFECTS.as_ptr(),
                AWE_EFFECTS.len() as i32,
                port,
                chan,
                (*chan).control[MIDI_CTL_NONREG_PARM_NUM_LSB as usize],
                val,
                EMUX_FX_FLAG_SET,
            );
        }
        return;
    }

    if unsafe {
        (*port).chset.midi_mode == SNDRV_MIDI_MODE_GS
            && (*chan).control[MIDI_CTL_NONREG_PARM_NUM_MSB as usize] == 1
    } {
        unsafe {
            let val = (*chan).control[MIDI_CTL_MSB_DATA_ENTRY as usize];
            send_converted_effect(
                GS_EFFECTS.as_ptr(),
                GS_EFFECTS.len() as i32,
                port,
                chan,
                (*chan).control[MIDI_CTL_NONREG_PARM_NUM_LSB as usize],
                val,
                EMUX_FX_FLAG_ADD,
            );
        }
        return;
    }
}


/*
 * XG control effects; still experimental
 */

/* cutoff: quarter semitone step, max=255 */
fn xg_cutoff(val: i32) -> i32 {
    (val - 64) * XG_SENSE[FX_CUTOFF] / 64
}

/* resonance: 0(open) to 15(most nasal) */
fn xg_filterQ(val: i32) -> i32 {
    (val - 64) * XG_SENSE[FX_RESONANCE] / 64
}

/* attack: */
fn xg_attack(val: i32) -> i32 {
    -((val - 64) * XG_SENSE[FX_ATTACK] / 64)
}

/* release: */
fn xg_release(val: i32) -> i32 {
    -((val - 64) * XG_SENSE[FX_RELEASE] / 64)
}

static XG_EFFECTS: [nrpn_conv_table; 4] = [
    nrpn_conv_table { control: 71, effect: EMUX_FX_CUTOFF, convert: xg_cutoff },
    nrpn_conv_table { control: 74, effect: EMUX_FX_FILTERQ, convert: xg_filterQ },
    nrpn_conv_table { control: 72, effect: EMUX_FX_ENV2_RELEASE, convert: xg_release },
    nrpn_conv_table { control: 73, effect: EMUX_FX_ENV2_ATTACK, convert: xg_attack },
];

pub fn snd_emux_xg_control(
    port: *mut snd_emux_port,
    chan: *mut snd_midi_channel,
    param: i32,
) -> i32 {
    if param >= unsafe { (*chan).control.len() } as i32 {
        return -22; // -EINVAL
    }

    unsafe {
        send_converted_effect(
            XG_EFFECTS.as_ptr(),
            XG_EFFECTS.len() as i32,
            port,
            chan,
            param,
            (*chan).control[param as usize],
            EMUX_FX_FLAG_ADD,
        )
    }
}

/*
 * receive sysex
 */
pub fn snd_emux_sysex(
    p: *mut std::ffi::c_void,
    buf: *mut u8,
    len: i32,
    parsed: i32,
    chset: *mut snd_midi_channel_set,
) {
    let port = p as *mut snd_emux_port;

    if snd_BUG_ON(port.is_null() || chset.is_null()) {
        return;
    }

    let emu = unsafe { (*port).emu };

    match parsed {
        SNDRV_MIDI_SYSEX_GS_MASTER_VOLUME => {
            snd_emux_update_port(port, SNDRV_EMUX_UPDATE_VOLUME);
        }
        _ => {
            if !emu.is_null() {
                unsafe {
                    if let Some(sysex_fn) = (*emu).ops.sysex {
                        sysex_fn(emu, buf, len, parsed, chset);
                    }
                }
            }
        }
    }
}

// External declarations
#[repr(C)]
pub struct snd_emux_port;

#[repr(C)]
pub struct snd_midi_channel;

#[repr(C)]
pub struct snd_midi_channel_set;

#[repr(C)]
pub struct snd_emux;

extern "C" {
    fn snd_emux_send_effect(
        port: *mut snd_emux_port,
        chan: *mut snd_midi_channel,
        effect: i32,
        val: i32,
        mode: i32,
    );
    fn snd_sf_calc_parm_delay(val: i32) -> i32;
    fn snd_sf_calc_parm_attack(val: i32) -> i32;
    fn snd_sf_calc_parm_hold(val: i32) -> i32;
    fn snd_sf_calc_parm_decay(val: i32) -> i32;
    fn snd_BUG_ON(cond: bool) -> bool;
    fn snd_emux_update_port(port: *mut snd_emux_port, update_type: i32);
}

// External constants
const MIDI_CTL_NONREG_PARM_NUM_MSB: i32 = 0;
const MIDI_CTL_NONREG_PARM_NUM_LSB: i32 = 0;
const MIDI_CTL_MSB_DATA_ENTRY: i32 = 0;
const MIDI_CTL_LSB_DATA_ENTRY: i32 = 0;
const EMUX_FX_LFO1_DELAY: i32 = 0;
const EMUX_FX_LFO1_FREQ: i32 = 0;
const EMUX_FX_LFO2_DELAY: i32 = 0;
const EMUX_FX_LFO2_FREQ: i32 = 0;
const EMUX_FX_ENV1_DELAY: i32 = 0;
const EMUX_FX_ENV1_ATTACK: i32 = 0;
const EMUX_FX_ENV1_HOLD: i32 = 0;
const EMUX_FX_ENV1_DECAY: i32 = 0;
const EMUX_FX_ENV1_SUSTAIN: i32 = 0;
const EMUX_FX_ENV1_RELEASE: i32 = 0;
const EMUX_FX_ENV2_DELAY: i32 = 0;
const EMUX_FX_ENV2_ATTACK: i32 = 0;
const EMUX_FX_ENV2_HOLD: i32 = 0;
const EMUX_FX_ENV2_DECAY: i32 = 0;
const EMUX_FX_ENV2_SUSTAIN: i32 = 0;
const EMUX_FX_ENV2_RELEASE: i32 = 0;
const EMUX_FX_INIT_PITCH: i32 = 0;
const EMUX_FX_LFO1_PITCH: i32 = 0;
const EMUX_FX_LFO2_PITCH: i32 = 0;
const EMUX_FX_ENV1_PITCH: i32 = 0;
const EMUX_FX_LFO1_VOLUME: i32 = 0;
const EMUX_FX_CUTOFF: i32 = 0;
const EMUX_FX_FILTERQ: i32 = 0;
const EMUX_FX_LFO1_CUTOFF: i32 = 0;
const EMUX_FX_ENV1_CUTOFF: i32 = 0;
const EMUX_FX_CHORUS: i32 = 0;
const EMUX_FX_REVERB: i32 = 0;
const EMUX_FX_FLAG_SET: i32 = 0;
const EMUX_FX_FLAG_ADD: i32 = 0;
const SNDRV_MIDI_MODE_GS: i32 = 0;
const SNDRV_MIDI_SYSEX_GS_MASTER_VOLUME: i32 = 0;
const SNDRV_EMUX_UPDATE_VOLUME: i32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
