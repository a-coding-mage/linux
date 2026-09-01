// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  synth callback routines for the emu8000 (AWE32/64)
 *
 *  Copyright (C) 1999 Steve Ratcliffe
 *  Copyright (C) 1999-2000 Takashi Iwai <tiwai@suse.de>
 */

/* Dependencies from emu8000_local.h, linux/export.h, and sound/asoundef.h
 * are expected to be supplied by the surrounding translated repository.
 */

use crate::*;

/*
 * prototypes
 */
unsafe fn get_voice(
    emu: *mut snd_emux,
    port: *mut snd_emux_port,
) -> *mut snd_emux_voice {
    let mut i: i32;
    let mut vp: *mut snd_emux_voice;
    let hw: *mut snd_emu8000;

    /* what we are looking for, in order of preference */
    const OFF: usize = 0;
    const RELEASED: usize = 1;
    const PLAYING: usize = 2;
    const END: usize = 3;

    /* Keeps track of what we are finding */
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct best {
        time: u32,
        voice: i32,
    }

    let mut best: [best; END] = [best {
        time: 0,
        voice: 0,
    }; END];
    let mut bp: *mut best;

    let _ = port;
    hw = (*emu).hw;

    i = 0;
    while i < END as i32 {
        best[i as usize].time = !0u32; /* XXX MAX_?INT really */
        best[i as usize].voice = -1;
        i += 1;
    }

    /*
     * Go through them all and get a best one to use.
     */
    i = 0;
    while i < (*emu).max_voices {
        let state: i32;
        let mut val: i32 = 0;

        vp = (*emu).voices.add(i as usize);
        state = (*vp).state;

        if state == SNDRV_EMUX_ST_OFF {
            bp = best.as_mut_ptr().add(OFF);
        } else if state == SNDRV_EMUX_ST_RELEASED || state == SNDRV_EMUX_ST_PENDING {
            bp = best.as_mut_ptr().add(RELEASED);
            val = ((EMU8000_CVCF_READ(hw, (*vp).ch) >> 16) & 0xffff) as i32;
            if val == 0 {
                bp = best.as_mut_ptr().add(OFF);
            }
        } else if (state & SNDRV_EMUX_ST_ON) != 0 {
            bp = best.as_mut_ptr().add(PLAYING);
        } else {
            i += 1;
            continue;
        }

        /* check if sample is finished playing (non-looping only) */
        if state != SNDRV_EMUX_ST_OFF
            && ((*vp).reg.sample_mode & SNDRV_SFNT_SAMPLE_SINGLESHOT) != 0
        {
            val = (EMU8000_CCCA_READ(hw, (*vp).ch) & 0xffffff) as i32;
            if val as u32 >= (*vp).reg.loopstart {
                bp = best.as_mut_ptr().add(OFF);
            }
        }

        if (*vp).time < (*bp).time {
            (*bp).time = (*vp).time;
            (*bp).voice = i;
        }
        i += 1;
    }

    i = 0;
    while i < END as i32 {
        if best[i as usize].voice >= 0 {
            vp = (*emu).voices.add(best[i as usize].voice as usize);
            (*vp).ch = best[i as usize].voice;
            return vp;
        }
        i += 1;
    }

    /* not found */
    core::ptr::null_mut()
}

unsafe fn start_voice(vp: *mut snd_emux_voice) -> i32 {
    let mut temp: u32;
    let ch: i32;
    let mut addr: i32;
    let chan: *mut snd_midi_channel;
    let hw: *mut snd_emu8000;

    hw = (*vp).hw;
    ch = (*vp).ch;
    chan = (*vp).chan;

    /* channel to be silent and idle */
    EMU8000_DCYSUSV_WRITE(hw, ch, 0x0080);
    EMU8000_VTFT_WRITE(hw, ch, 0x0000FFFF);
    EMU8000_CVCF_WRITE(hw, ch, 0x0000FFFF);
    EMU8000_PTRX_WRITE(hw, ch, 0);
    EMU8000_CPF_WRITE(hw, ch, 0);

    /* set pitch offset */
    set_pitch(hw, vp);

    /* set envelope parameters */
    EMU8000_ENVVAL_WRITE(hw, ch, (*vp).reg.parm.moddelay);
    EMU8000_ATKHLD_WRITE(hw, ch, (*vp).reg.parm.modatkhld);
    EMU8000_DCYSUS_WRITE(hw, ch, (*vp).reg.parm.moddcysus);
    EMU8000_ENVVOL_WRITE(hw, ch, (*vp).reg.parm.voldelay);
    EMU8000_ATKHLDV_WRITE(hw, ch, (*vp).reg.parm.volatkhld);
    /* decay/sustain parameter for volume envelope is used
       for triggerg the voice */

    /* cutoff and volume */
    set_volume(hw, vp);

    /* modulation envelope heights */
    EMU8000_PEFE_WRITE(hw, ch, (*vp).reg.parm.pefe);

    /* lfo1/2 delay */
    EMU8000_LFO1VAL_WRITE(hw, ch, (*vp).reg.parm.lfo1delay);
    EMU8000_LFO2VAL_WRITE(hw, ch, (*vp).reg.parm.lfo2delay);

    /* lfo1 pitch & cutoff shift */
    set_fmmod(hw, vp);
    /* lfo1 volume & freq */
    set_tremfreq(hw, vp);
    /* lfo2 pitch & freq */
    set_fm2frq2(hw, vp);
    /* pan & loop start */
    set_pan(hw, vp);

    /* chorus & loop end (chorus 8bit, MSB) */
    addr = ((*vp).reg.loopend as i32).wrapping_sub(1);
    temp = (*vp).reg.parm.chorus as u32;
    temp = temp.wrapping_add(((*chan).control[MIDI_CTL_E3_CHORUS_DEPTH as usize] as i32 * 9 / 10) as u32);
    if temp > 255 {
        temp = 255;
    }
    temp = (temp << 24) | addr as u32;
    EMU8000_CSL_WRITE(hw, ch, temp);

    /* Q & current address (Q 4bit value, MSB) */
    addr = ((*vp).reg.start as i32).wrapping_sub(1);
    temp = (*vp).reg.parm.filterQ as u32;
    temp = (temp << 28) | addr as u32;
    EMU8000_CCCA_WRITE(hw, ch, temp);

    /* clear unknown registers */
    EMU8000_00A0_WRITE(hw, ch, 0);
    EMU8000_0080_WRITE(hw, ch, 0);

    /* reset volume */
    temp = (*vp).vtarget << 16;
    EMU8000_VTFT_WRITE(hw, ch, temp | (*vp).ftarget);
    EMU8000_CVCF_WRITE(hw, ch, temp | 0xff00);

    0
}

unsafe fn trigger_voice(vp: *mut snd_emux_voice) {
    let ch: i32 = (*vp).ch;
    let mut temp: u32;
    let hw: *mut snd_emu8000;

    hw = (*vp).hw;

    /* set reverb and pitch target */
    temp = (*vp).reg.parm.reverb as u32;
    temp = temp.wrapping_add(
        ((*(*vp).chan).control[MIDI_CTL_E1_REVERB_DEPTH as usize] as i32 * 9 / 10) as u32,
    );
    if temp > 255 {
        temp = 255;
    }
    temp = (temp << 8) | ((*vp).ptarget << 16) | (*vp).aaux;
    EMU8000_PTRX_WRITE(hw, ch, temp);
    EMU8000_CPF_WRITE(hw, ch, (*vp).ptarget << 16);
    EMU8000_DCYSUSV_WRITE(hw, ch, (*vp).reg.parm.voldcysus);
}

unsafe fn release_voice(vp: *mut snd_emux_voice) {
    let mut dcysusv: i32;
    let hw: *mut snd_emu8000;

    hw = (*vp).hw;
    dcysusv = 0x8000 | ((*vp).reg.parm.modrelease as u8 as i32);
    EMU8000_DCYSUS_WRITE(hw, (*vp).ch, dcysusv);
    dcysusv = 0x8000 | ((*vp).reg.parm.volrelease as u8 as i32);
    EMU8000_DCYSUSV_WRITE(hw, (*vp).ch, dcysusv);
}

unsafe fn update_voice(vp: *mut snd_emux_voice, update: i32) {
    let hw: *mut snd_emu8000;

    hw = (*vp).hw;
    if (update & SNDRV_EMUX_UPDATE_VOLUME) != 0 {
        set_volume(hw, vp);
    }
    if (update & SNDRV_EMUX_UPDATE_PITCH) != 0 {
        set_pitch(hw, vp);
    }
    if (update & SNDRV_EMUX_UPDATE_PAN) != 0
        && (*(*vp).port).ctrls[EMUX_MD_REALTIME_PAN as usize] != 0
    {
        set_pan(hw, vp);
    }
    if (update & SNDRV_EMUX_UPDATE_FMMOD) != 0 {
        set_fmmod(hw, vp);
    }
    if (update & SNDRV_EMUX_UPDATE_TREMFREQ) != 0 {
        set_tremfreq(hw, vp);
    }
    if (update & SNDRV_EMUX_UPDATE_FM2FRQ2) != 0 {
        set_fm2frq2(hw, vp);
    }
    if (update & SNDRV_EMUX_UPDATE_Q) != 0 {
        set_filterQ(hw, vp);
    }
}

unsafe fn reset_voice(emu: *mut snd_emux, ch: i32) {
    let hw: *mut snd_emu8000;

    hw = (*emu).hw;
    EMU8000_DCYSUSV_WRITE(hw, ch, 0x807F);
    snd_emu8000_tweak_voice(hw, ch);
}

unsafe fn terminate_voice(vp: *mut snd_emux_voice) {
    let hw: *mut snd_emu8000;

    hw = (*vp).hw;
    EMU8000_DCYSUSV_WRITE(hw, (*vp).ch, 0x807F);
}

unsafe fn sysex(
    emu: *mut snd_emux,
    buf: *mut core::ffi::c_char,
    len: i32,
    parsed: i32,
    chset: *mut snd_midi_channel_set,
) {
    let hw: *mut snd_emu8000;

    let _ = buf;
    let _ = len;
    hw = (*emu).hw;

    match parsed {
        SNDRV_MIDI_SYSEX_GS_CHORUS_MODE => {
            (*hw).chorus_mode = (*chset).gs_chorus_mode;
            snd_emu8000_update_chorus_mode(hw);
        }

        SNDRV_MIDI_SYSEX_GS_REVERB_MODE => {
            (*hw).reverb_mode = (*chset).gs_reverb_mode;
            snd_emu8000_update_reverb_mode(hw);
        }

        _ => {}
    }
}

/* Present in C only when IS_ENABLED(CONFIG_SND_SEQUENCER_OSS). */
#[cfg(CONFIG_SND_SEQUENCER_OSS)]
unsafe fn oss_ioctl(emu: *mut snd_emux, cmd: i32, p1: i32, p2: i32) -> i32 {
    let hw: *mut snd_emu8000;

    hw = (*emu).hw;

    match cmd {
        _EMUX_OSS_REVERB_MODE => {
            (*hw).reverb_mode = p1;
            snd_emu8000_update_reverb_mode(hw);
        }

        _EMUX_OSS_CHORUS_MODE => {
            (*hw).chorus_mode = p1;
            snd_emu8000_update_chorus_mode(hw);
        }

        _EMUX_OSS_INITIALIZE_CHIP => {
            /* snd_emu8000_init(hw); */ /*ignored*/
        }

        _EMUX_OSS_EQUALIZER => {
            (*hw).bass_level = p1;
            (*hw).treble_level = p2;
            snd_emu8000_update_equalizer(hw);
        }

        _ => {}
    }
    0
}

unsafe fn load_fx(
    emu: *mut snd_emux,
    type_: i32,
    mode: i32,
    buf: *const core::ffi::c_void,
    mut len: i64,
) -> i32 {
    let hw: *mut snd_emu8000;
    let mut buf = buf as *const u8;
    hw = (*emu).hw;

    /* skip header */
    buf = buf.add(16);
    len -= 16;

    match type_ {
        SNDRV_EMU8000_LOAD_CHORUS_FX => {
            return snd_emu8000_load_chorus_fx(hw, mode, buf as *const core::ffi::c_void, len);
        }
        SNDRV_EMU8000_LOAD_REVERB_FX => {
            return snd_emu8000_load_reverb_fx(hw, mode, buf as *const core::ffi::c_void, len);
        }
        _ => {}
    }
    -EINVAL
}

unsafe fn set_pitch(hw: *mut snd_emu8000, vp: *mut snd_emux_voice) {
    EMU8000_IP_WRITE(hw, (*vp).ch, (*vp).apitch);
}

unsafe fn set_volume(hw: *mut snd_emu8000, vp: *mut snd_emux_voice) {
    let mut ifatn: i32;

    ifatn = (*vp).acutoff as u8 as i32;
    ifatn <<= 8;
    ifatn |= (*vp).avol as u8 as i32;
    EMU8000_IFATN_WRITE(hw, (*vp).ch, ifatn);
}

unsafe fn set_pan(hw: *mut snd_emu8000, vp: *mut snd_emux_voice) {
    let temp: u32;

    temp = ((*vp).apan as u32) << 24 | ((*vp).reg.loopstart as u32).wrapping_sub(1);
    EMU8000_PSST_WRITE(hw, (*vp).ch, temp);
}

const MOD_SENSE: i32 = 18;

unsafe fn set_fmmod(hw: *mut snd_emu8000, vp: *mut snd_emux_voice) {
    let fmmod: u16;
    let mut pitch: i16;
    let cutoff: u8;
    let modulation: i32;

    pitch = (((*vp).reg.parm.fmmod >> 8) as u8 as i8) as i16;
    cutoff = ((*vp).reg.parm.fmmod & 0xff) as u8;
    modulation = (*(*vp).chan).gm_modulation + (*(*vp).chan).midi_pressure;
    pitch = pitch.wrapping_add(((MOD_SENSE * modulation) / 1200) as i16);
    if pitch < -128 {
        pitch = -128;
    } else if pitch > 127 {
        pitch = 127;
    }
    fmmod = (((pitch as u8 as u16) << 8) | cutoff as u16) as u16;
    EMU8000_FMMOD_WRITE(hw, (*vp).ch, fmmod);
}

/* set tremolo (lfo1) volume & frequency */
unsafe fn set_tremfreq(hw: *mut snd_emu8000, vp: *mut snd_emux_voice) {
    EMU8000_TREMFRQ_WRITE(hw, (*vp).ch, (*vp).reg.parm.tremfrq);
}

/* set lfo2 pitch & frequency */
unsafe fn set_fm2frq2(hw: *mut snd_emu8000, vp: *mut snd_emux_voice) {
    let fm2frq2: u16;
    let mut pitch: i16;
    let freq: u8;
    let modulation: i32;

    pitch = (((*vp).reg.parm.fm2frq2 >> 8) as u8 as i8) as i16;
    freq = ((*vp).reg.parm.fm2frq2 & 0xff) as u8;
    modulation = (*(*vp).chan).gm_modulation + (*(*vp).chan).midi_pressure;
    pitch = pitch.wrapping_add(((MOD_SENSE * modulation) / 1200) as i16);
    if pitch < -128 {
        pitch = -128;
    } else if pitch > 127 {
        pitch = 127;
    }
    fm2frq2 = (((pitch as u8 as u16) << 8) | freq as u16) as u16;
    EMU8000_FM2FRQ2_WRITE(hw, (*vp).ch, fm2frq2);
}

/* set filterQ */
unsafe fn set_filterQ(hw: *mut snd_emu8000, vp: *mut snd_emux_voice) {
    let mut addr: u32;
    addr = EMU8000_CCCA_READ(hw, (*vp).ch) & 0xffffff;
    addr |= ((*vp).reg.parm.filterQ as u32) << 28;
    EMU8000_CCCA_WRITE(hw, (*vp).ch, addr);
}

/*
 * set the envelope & LFO parameters to the default values
 */
unsafe fn snd_emu8000_tweak_voice(emu: *mut snd_emu8000, i: i32) {
    /* set all mod/vol envelope shape to minimum */
    EMU8000_ENVVOL_WRITE(emu, i, 0x8000);
    EMU8000_ENVVAL_WRITE(emu, i, 0x8000);
    EMU8000_DCYSUS_WRITE(emu, i, 0x7F7F);
    EMU8000_ATKHLDV_WRITE(emu, i, 0x7F7F);
    EMU8000_ATKHLD_WRITE(emu, i, 0x7F7F);
    EMU8000_PEFE_WRITE(emu, i, 0); /* mod envelope height to zero */
    EMU8000_LFO1VAL_WRITE(emu, i, 0x8000); /* no delay for LFO1 */
    EMU8000_LFO2VAL_WRITE(emu, i, 0x8000);
    EMU8000_IP_WRITE(emu, i, 0xE000); /* no pitch shift */
    EMU8000_IFATN_WRITE(emu, i, 0xFF00); /* volume to minimum */
    EMU8000_FMMOD_WRITE(emu, i, 0);
    EMU8000_TREMFRQ_WRITE(emu, i, 0);
    EMU8000_FM2FRQ2_WRITE(emu, i, 0);
}

/*
 * additional patch keys
 */

const SNDRV_EMU8000_LOAD_CHORUS_FX: i32 = 0x10; /* optarg=mode */
const SNDRV_EMU8000_LOAD_REVERB_FX: i32 = 0x11; /* optarg=mode */

/*
 * set up operators
 */
static emu8000_ops: snd_emux_operators = snd_emux_operators {
    owner: THIS_MODULE,
    get_voice: Some(get_voice),
    prepare: Some(start_voice),
    trigger: Some(trigger_voice),
    release: Some(release_voice),
    update: Some(update_voice),
    terminate: Some(terminate_voice),
    reset: Some(reset_voice),
    sample_new: Some(snd_emu8000_sample_new),
    sample_free: Some(snd_emu8000_sample_free),
    sample_reset: Some(snd_emu8000_sample_reset),
    load_fx: Some(load_fx),
    sysex: Some(sysex),
    #[cfg(CONFIG_SND_SEQUENCER_OSS)]
    oss_ioctl: Some(oss_ioctl),
};

#[no_mangle]
pub unsafe extern "C" fn snd_emu8000_ops_setup(hw: *mut snd_emu8000) {
    (*(*hw).emu).ops = emu8000_ops;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
