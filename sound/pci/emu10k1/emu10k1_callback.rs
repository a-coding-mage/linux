// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  synth callback routines for Emu10k1
 *
 *  Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type u32 = u32;

#[repr(C)]
pub struct module;

#[repr(C)]
pub struct snd_emux_operators {
    pub owner: *mut module,
    pub get_voice: Option<unsafe extern "C" fn(*mut snd_emux, *mut snd_emux_port) -> *mut snd_emux_voice>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_emux_voice) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_emux_voice)>,
    pub release: Option<unsafe extern "C" fn(*mut snd_emux_voice)>,
    pub update: Option<unsafe extern "C" fn(*mut snd_emux_voice, i32)>,
    pub terminate: Option<unsafe extern "C" fn(*mut snd_emux_voice)>,
    pub free_voice: Option<unsafe extern "C" fn(*mut snd_emux_voice)>,
    pub sample_new: Option<unsafe extern "C" fn()>,
    pub sample_free: Option<unsafe extern "C" fn()>,
    pub get_pitch_shift: Option<unsafe extern "C" fn(*mut snd_emux) -> i32>,
}

#[repr(C)]
pub struct snd_emux {
    pub ops: snd_emux_operators,
    pub synth: *mut snd_emux,
    pub voices: *mut snd_emux_voice,
    pub max_voices: i32,
    pub num_voices: i32,
    pub hw: *mut snd_emu10k1,
}

#[repr(C)]
pub struct snd_emux_port;

#[repr(C)]
pub struct snd_emux_voice {
    pub time: u32,
    pub voice: i32,
    pub ch: i32,
    pub state: i32,
    pub emu: *mut snd_emux,
    pub hw: *mut snd_emu10k1,
    pub reg: snd_emux_voice_reg,
    pub block: *mut core::ffi::c_void,
    pub avol: u32,
    pub apitch: u32,
    pub apan: u32,
    pub aaux: u32,
    pub acutoff: u32,
    pub vtarget: u32,
    pub ftarget: u32,
    pub chan: *mut snd_midi_channel,
}

#[repr(C)]
pub struct snd_emux_voice_reg {
    pub parm: snd_emux_voice_parm,
    pub sample_mode: u32,
    pub loopstart: u32,
    pub loopend: u32,
    pub start: u32,
    pub end: u32,
}

#[repr(C)]
pub struct snd_emux_voice_parm {
    pub modrelease: u32,
    pub volrelease: u32,
    pub tremfrq: u32,
    pub filterQ: u32,
    pub reverb: u32,
    pub chorus: u32,
    pub moddelay: u32,
    pub modatkhld: u32,
    pub moddcysus: u32,
    pub voldelay: u32,
    pub volatkhld: u32,
    pub pefe: u32,
    pub lfo1delay: u32,
    pub lfo2delay: u32,
    pub voldcysus: u32,
    pub fmmod: u32,
    pub fm2frq2: u32,
}

#[repr(C)]
pub struct snd_midi_channel {
    pub control: [u8; 128],
    pub gm_modulation: i32,
    pub midi_pressure: i32,
}

#[repr(C)]
pub struct snd_emu10k1 {
    pub synth: *mut snd_emux,
    pub voices: *mut snd_emu10k1_voice,
    pub audigy: i32,
    pub silent_page: snd_emu10k1_silent_page,
    pub address_mode: u32,
    pub card_capabilities: *mut snd_emu10k1_card_capabilities,
    pub emu1010: snd_emu1010,
}

#[repr(C)]
pub struct snd_emu10k1_silent_page {
    pub addr: u32,
}

#[repr(C)]
pub struct snd_emu10k1_voice {
    pub number: i32,
    pub dirty: i32,
}

#[repr(C)]
pub struct snd_emu10k1_memblk {
    pub map_locked: i32,
    pub mapped_page: i32,
}

#[repr(C)]
pub struct snd_emu10k1_card_capabilities {
    pub emu_model: i32,
}

#[repr(C)]
pub struct snd_emu1010 {
    pub word_clock: i32,
}

/* voice status */
const V_FREE: usize = 0;
const V_OFF: usize = 1;
const V_RELEASED: usize = 2;
const V_PLAYING: usize = 3;
const V_END: usize = 4;

/* Keeps track of what we are finding */
#[repr(C)]
struct best_voice {
    time: u32,
    voice: i32,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    static DCYSUSM: u32;
    static DCYSUSM_PHASE1_MASK: u32;
    static DCYSUSV: u32;
    static DCYSUSV_PHASE1_MASK: u32;
    static DCYSUSV_CHANNELENABLE_MASK: u32;
    static VTFT: u32;
    static VTFT_FILTERTARGET_MASK: u32;
    static CVCF: u32;
    static CVCF_CURRENTFILTER_MASK: u32;
    static PTRX: u32;
    static CPF: u32;
    static REGLIST_END: u32;
    static IFATN_ATTENUATION: u32;
    static IP: u32;
    static PTRX_FXSENDAMOUNT_A: u32;
    static PTRX_FXSENDAMOUNT_B: u32;
    static FMMOD: u32;
    static TREMFRQ: u32;
    static FM2FRQ2: u32;
    static CCCA_RESONANCE: u32;
    static CVCF_CURRENTVOL: u32;
    static CCCA_CURRADDR: u32;
    static SNDRV_EMUX_ST_OFF: i32;
    static SNDRV_EMUX_ST_RELEASED: i32;
    static SNDRV_EMUX_ST_PENDING: i32;
    static SNDRV_EMUX_ST_STANDBY: i32;
    static SNDRV_EMUX_ST_ON: i32;
    static SNDRV_SFNT_SAMPLE_SINGLESHOT: u32;
    static ENOMEM: i32;
    static EINVAL: i32;
    static EMU10K1_SYNTH: i32;
    static FXBUS_MIDI_LEFT: u32;
    static FXBUS_MIDI_RIGHT: u32;
    static FXBUS_MIDI_REVERB: u32;
    static FXBUS_MIDI_CHORUS: u32;
    static A_FXRT1: u32;
    static FXRT: u32;
    static MIDI_CTL_E1_REVERB_DEPTH: usize;
    static MIDI_CTL_E3_CHORUS_DEPTH: usize;
    static MAP_PTI_MASK1: u32;
    static MAP_PTI_MASK0: u32;
    static CCCA_INTERPROM_0: u32;
    static CCCA_8BITSELECT: u32;
    static ENVVAL: u32;
    static ATKHLDM: u32;
    static ATKHLDV: u32;
    static ENVVOL: u32;
    static IFATN: u32;
    static PEFE: u32;
    static LFOVAL1: u32;
    static LFOVAL2: u32;
    static PSST: u32;
    static DSL: u32;
    static Z1: u32;
    static Z2: u32;
    static MAPA: u32;
    static MAPB: u32;
    static CCCA: u32;
    static CCR: u32;
    static CCR_CACHEINVALIDSIZE: u32;
    static SNDRV_EMUX_UPDATE_VOLUME: i32;
    static SNDRV_EMUX_UPDATE_PITCH: i32;
    static SNDRV_EMUX_UPDATE_PAN: i32;
    static SNDRV_EMUX_UPDATE_FMMOD: i32;
    static SNDRV_EMUX_UPDATE_TREMFREQ: i32;
    static SNDRV_EMUX_UPDATE_FM2FRQ2: i32;
    static SNDRV_EMUX_UPDATE_Q: i32;

    fn snd_emu10k1_sample_new();
    fn snd_emu10k1_sample_free();
    fn snd_emu10k1_ptr_write_multiple(hw: *mut snd_emu10k1, ch: i32, ...);
    fn snd_emu10k1_ptr_write(hw: *mut snd_emu10k1, reg: u32, ch: i32, data: u32);
    fn snd_emu10k1_ptr_read(hw: *mut snd_emu10k1, reg: u32, ch: i32) -> u32;
    fn snd_emu10k1_voice_free(hw: *mut snd_emu10k1, voice: *mut snd_emu10k1_voice);
    fn snd_emu10k1_voice_alloc(
        hw: *mut snd_emu10k1,
        ty: i32,
        number: i32,
        stereo: i32,
        rvoice: *mut core::ffi::c_void,
        voice: *mut *mut snd_emu10k1_voice,
    ) -> i32;
    fn snd_emu10k1_memblk_map(hw: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> i32;
    fn snd_emu10k1_memblk_offset(blk: *mut snd_emu10k1_memblk) -> u32;
    fn snd_BUG_ON(cond: bool) -> bool;
    fn REG_VAL_PUT(reg: u32, val: u32) -> u32;
    fn IP_TO_CP(val: u32) -> u32;
}

/*
 * prototypes
 */

/*
 * Ensure a value is between two points
 * macro evaluates its args more than once, so changed to upper-case.
 */
fn LIMITVALUE<T: PartialOrd + Copy>(x: &mut T, a: T, b: T) {
    if *x < a {
        *x = a;
    } else if *x > b {
        *x = b;
    }
}

fn LIMITMAX<T: PartialOrd + Copy>(x: &mut T, a: T) {
    if *x > a {
        *x = a;
    }
}

/*
 * set up operators
 */
static emu10k1_ops: snd_emux_operators = snd_emux_operators {
    owner: unsafe { THIS_MODULE },
    get_voice: Some(get_voice),
    prepare: Some(start_voice),
    trigger: Some(trigger_voice),
    release: Some(release_voice),
    update: Some(update_voice),
    terminate: Some(terminate_voice),
    free_voice: Some(free_voice),
    sample_new: Some(snd_emu10k1_sample_new),
    sample_free: Some(snd_emu10k1_sample_free),
    get_pitch_shift: Some(get_pitch_shift),
};

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_ops_setup(emux: *mut snd_emux) {
    unsafe {
        (*emux).ops = emu10k1_ops;
    }
}

/*
 * get more voice for pcm
 *
 * terminate most inactive voice and give it as a pcm voice.
 *
 * voice_lock is already held.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_synth_get_voice(hw: *mut snd_emu10k1) -> i32 {
    unsafe {
        let emu: *mut snd_emux;
        let mut vp: *mut snd_emux_voice;
        let mut best = [best_voice { time: 0, voice: 0 }; V_END];
        let mut i: i32;

        emu = (*hw).synth;

        lookup_voices(emu, hw, best.as_mut_ptr(), 1); /* no OFF voices */
        i = 0;
        while i < V_END as i32 {
            if best[i as usize].voice >= 0 {
                let ch: i32;
                vp = (*emu).voices.add(best[i as usize].voice as usize);
                ch = (*vp).ch;
                if ch < 0 {
                    /*
                    dev_warn(emu->card->dev,
                           "synth_get_voice: ch < 0 (%d) ??", i);
                    */
                    i += 1;
                    continue;
                }
                (*(*vp).emu).num_voices -= 1;
                (*vp).ch = -1;
                (*vp).state = SNDRV_EMUX_ST_OFF;
                return ch;
            }
            i += 1;
        }

        /* not found */
        -ENOMEM
    }
}

/*
 * turn off the voice (not terminated)
 */
unsafe extern "C" fn release_voice(vp: *mut snd_emux_voice) {
    unsafe {
        let hw: *mut snd_emu10k1;

        hw = (*vp).hw;
        snd_emu10k1_ptr_write_multiple(
            hw,
            (*vp).ch,
            DCYSUSM,
            ((*vp).reg.parm.modrelease as u8 as u32) | DCYSUSM_PHASE1_MASK,
            DCYSUSV,
            ((*vp).reg.parm.volrelease as u8 as u32)
                | DCYSUSV_PHASE1_MASK
                | DCYSUSV_CHANNELENABLE_MASK,
            REGLIST_END,
        );
    }
}

/*
 * terminate the voice
 */
unsafe extern "C" fn terminate_voice(vp: *mut snd_emux_voice) {
    unsafe {
        let hw: *mut snd_emu10k1;

        if snd_BUG_ON(vp.is_null()) {
            return;
        }
        hw = (*vp).hw;
        snd_emu10k1_ptr_write_multiple(
            hw,
            (*vp).ch,
            DCYSUSV,
            0u32,
            VTFT,
            VTFT_FILTERTARGET_MASK,
            CVCF,
            CVCF_CURRENTFILTER_MASK,
            PTRX,
            0u32,
            CPF,
            0u32,
            REGLIST_END,
        );
        if !(*vp).block.is_null() {
            let emem: *mut snd_emu10k1_memblk;
            emem = (*vp).block as *mut snd_emu10k1_memblk;
            if (*emem).map_locked > 0 {
                (*emem).map_locked -= 1;
            }
        }
    }
}

/*
 * release the voice to system
 */
unsafe extern "C" fn free_voice(vp: *mut snd_emux_voice) {
    unsafe {
        let hw: *mut snd_emu10k1;

        hw = (*vp).hw;
        /* FIXME: emu10k1_synth is broken. */
        /* This can get called with hw == 0 */
        /* Problem apparent on plug, unplug then plug */
        /* on the Audigy 2 ZS Notebook. */
        if !hw.is_null() && (*vp).ch >= 0 {
            snd_emu10k1_voice_free(hw, (*hw).voices.add((*vp).ch as usize));
            (*(*vp).emu).num_voices -= 1;
            (*vp).ch = -1;
        }
    }
}

/*
 * update registers
 */
unsafe extern "C" fn update_voice(vp: *mut snd_emux_voice, update: i32) {
    unsafe {
        let hw: *mut snd_emu10k1;

        hw = (*vp).hw;
        if update & SNDRV_EMUX_UPDATE_VOLUME != 0 {
            snd_emu10k1_ptr_write(hw, IFATN_ATTENUATION, (*vp).ch, (*vp).avol);
        }
        if update & SNDRV_EMUX_UPDATE_PITCH != 0 {
            snd_emu10k1_ptr_write(hw, IP, (*vp).ch, (*vp).apitch);
        }
        if update & SNDRV_EMUX_UPDATE_PAN != 0 {
            snd_emu10k1_ptr_write(hw, PTRX_FXSENDAMOUNT_A, (*vp).ch, (*vp).apan);
            snd_emu10k1_ptr_write(hw, PTRX_FXSENDAMOUNT_B, (*vp).ch, (*vp).aaux);
        }
        if update & SNDRV_EMUX_UPDATE_FMMOD != 0 {
            snd_emu10k1_ptr_write(hw, FMMOD, (*vp).ch, make_fmmod(vp));
        }
        if update & SNDRV_EMUX_UPDATE_TREMFREQ != 0 {
            snd_emu10k1_ptr_write(hw, TREMFRQ, (*vp).ch, (*vp).reg.parm.tremfrq);
        }
        if update & SNDRV_EMUX_UPDATE_FM2FRQ2 != 0 {
            snd_emu10k1_ptr_write(hw, FM2FRQ2, (*vp).ch, make_fm2frq2(vp));
        }
        if update & SNDRV_EMUX_UPDATE_Q != 0 {
            snd_emu10k1_ptr_write(hw, CCCA_RESONANCE, (*vp).ch, (*vp).reg.parm.filterQ);
        }
    }
}

/*
 * look up voice table - get the best voice in order of preference
 */
/* spinlock held! */
unsafe fn lookup_voices(
    emu: *mut snd_emux,
    hw: *mut snd_emu10k1,
    best: *mut best_voice,
    active_only: i32,
) {
    unsafe {
        let mut vp: *mut snd_emux_voice;
        let mut bp: *mut best_voice;
        let mut i: i32;

        i = 0;
        while i < V_END as i32 {
            (*best.add(i as usize)).time = !0u32; /* XXX MAX_?INT really */
            (*best.add(i as usize)).voice = -1;
            i += 1;
        }

        /*
         * Go through them all and get a best one to use.
         * NOTE: could also look at volume and pick the quietest one.
         */
        i = 0;
        while i < (*emu).max_voices {
            let state: i32;
            let mut val: i32;

            vp = (*emu).voices.add(i as usize);
            state = (*vp).state;
            if state == SNDRV_EMUX_ST_OFF {
                if (*vp).ch < 0 {
                    if active_only != 0 {
                        i += 1;
                        continue;
                    }
                    bp = best.add(V_FREE);
                } else {
                    bp = best.add(V_OFF);
                }
            } else if state == SNDRV_EMUX_ST_RELEASED || state == SNDRV_EMUX_ST_PENDING {
                bp = best.add(V_RELEASED);
                /* Original C conditional: #if 1 */
                val = snd_emu10k1_ptr_read(hw, CVCF_CURRENTVOL, (*vp).ch) as i32;
                if val == 0 {
                    bp = best.add(V_OFF);
                }
            } else if state == SNDRV_EMUX_ST_STANDBY {
                i += 1;
                continue;
            } else if state & SNDRV_EMUX_ST_ON != 0 {
                bp = best.add(V_PLAYING);
            } else {
                i += 1;
                continue;
            }

            /* check if sample is finished playing (non-looping only) */
            if bp != best.add(V_OFF)
                && bp != best.add(V_FREE)
                && ((*vp).reg.sample_mode & SNDRV_SFNT_SAMPLE_SINGLESHOT) != 0
            {
                val = snd_emu10k1_ptr_read(hw, CCCA_CURRADDR, (*vp).ch)
                    .wrapping_sub(64)
                    .wrapping_add(3) as i32;
                if val as u32 >= (*vp).reg.loopstart {
                    bp = best.add(V_OFF);
                }
            }

            if (*vp).time < (*bp).time {
                (*bp).time = (*vp).time;
                (*bp).voice = i;
            }
            i += 1;
        }
    }
}

/*
 * get an empty voice
 *
 * emu->voice_lock is already held.
 */
unsafe extern "C" fn get_voice(
    emu: *mut snd_emux,
    _port: *mut snd_emux_port,
) -> *mut snd_emux_voice {
    unsafe {
        let hw: *mut snd_emu10k1;
        let mut vp: *mut snd_emux_voice;
        let mut best = [best_voice { time: 0, voice: 0 }; V_END];
        let mut i: i32;

        hw = (*emu).hw;

        lookup_voices(emu, hw, best.as_mut_ptr(), 0);
        i = 0;
        while i < V_END as i32 {
            if best[i as usize].voice >= 0 {
                vp = (*emu).voices.add(best[i as usize].voice as usize);
                if (*vp).ch < 0 {
                    /* allocate a voice */
                    let mut hwvoice: *mut snd_emu10k1_voice = core::ptr::null_mut();
                    if snd_emu10k1_voice_alloc(
                        hw,
                        EMU10K1_SYNTH,
                        1,
                        1,
                        core::ptr::null_mut(),
                        &mut hwvoice,
                    ) < 0
                    {
                        i += 1;
                        continue;
                    }
                    (*vp).ch = (*hwvoice).number;
                    (*emu).num_voices += 1;
                }
                return vp;
            }
            i += 1;
        }

        /* not found */
        core::ptr::null_mut()
    }
}

/*
 * prepare envelopes and LFOs
 */
unsafe extern "C" fn start_voice(vp: *mut snd_emux_voice) -> i32 {
    unsafe {
        let mut temp: u32;
        let ch: i32;
        let w_16: bool;
        let psst: u32;
        let dsl: u32;
        let map: u32;
        let mut ccca: u32;
        let vtarget: u32;
        let mut addr: u32;
        let mapped_offset: u32;
        let chan: *mut snd_midi_channel;
        let hw: *mut snd_emu10k1;
        let emem: *mut snd_emu10k1_memblk;

        hw = (*vp).hw;
        ch = (*vp).ch;
        if snd_BUG_ON(ch < 0) {
            return -EINVAL;
        }
        chan = (*vp).chan;
        w_16 = !((*vp).reg.sample_mode & SNDRV_SFNT_SAMPLE_8BITS != 0);

        emem = (*vp).block as *mut snd_emu10k1_memblk;
        if emem.is_null() {
            return -EINVAL;
        }
        (*emem).map_locked += 1;
        if snd_emu10k1_memblk_map(hw, emem) < 0 {
            /* dev_err(hw->card->devK, "emu: cannot map!\n"); */
            return -ENOMEM;
        }
        mapped_offset = snd_emu10k1_memblk_offset(emem) >> (w_16 as u32);
        (*vp).reg.start = (*vp).reg.start.wrapping_add(mapped_offset);
        (*vp).reg.end = (*vp).reg.end.wrapping_add(mapped_offset);
        (*vp).reg.loopstart = (*vp).reg.loopstart.wrapping_add(mapped_offset);
        (*vp).reg.loopend = (*vp).reg.loopend.wrapping_add(mapped_offset);

        /* set channel routing */
        /* A = left(0), B = right(1), C = reverb(c), D = chorus(d) */
        if (*hw).audigy != 0 {
            temp = FXBUS_MIDI_LEFT
                | (FXBUS_MIDI_RIGHT << 8)
                | (FXBUS_MIDI_REVERB << 16)
                | (FXBUS_MIDI_CHORUS << 24);
            snd_emu10k1_ptr_write(hw, A_FXRT1, ch, temp);
        } else {
            temp = (FXBUS_MIDI_LEFT << 16)
                | (FXBUS_MIDI_RIGHT << 20)
                | (FXBUS_MIDI_REVERB << 24)
                | (FXBUS_MIDI_CHORUS << 28);
            snd_emu10k1_ptr_write(hw, FXRT, ch, temp);
        }

        temp = (*vp).reg.parm.reverb;
        temp = temp.wrapping_add((*(*vp).chan).control[MIDI_CTL_E1_REVERB_DEPTH] as i32 as u32 * 9 / 10);
        LIMITMAX(&mut temp, 255);
        addr = (*vp).reg.loopstart;
        psst = (temp << 24) | addr;

        addr = (*vp).reg.loopend;
        temp = (*vp).reg.parm.chorus;
        temp = temp.wrapping_add((*chan).control[MIDI_CTL_E3_CHORUS_DEPTH] as i32 as u32 * 9 / 10);
        LIMITMAX(&mut temp, 255);
        dsl = (temp << 24) | addr;

        map = ((*hw).silent_page.addr << (*hw).address_mode)
            | if (*hw).address_mode != 0 { MAP_PTI_MASK1 } else { MAP_PTI_MASK0 };

        addr = (*vp).reg.start.wrapping_add(64).wrapping_sub(3);
        temp = (*vp).reg.parm.filterQ;
        ccca = (temp << 28) | addr;
        if (*vp).apitch < 0xe400 {
            ccca |= CCCA_INTERPROM_0;
        } else {
            let shift: u32 = ((*vp).apitch - 0xe000) >> 10;
            ccca |= shift << 25;
        }
        if !w_16 {
            ccca |= CCCA_8BITSELECT;
        }

        vtarget = (*vp).vtarget << 16;

        snd_emu10k1_ptr_write_multiple(
            hw,
            ch,
            /* channel to be silent and idle */
            DCYSUSV,
            0u32,
            VTFT,
            VTFT_FILTERTARGET_MASK,
            CVCF,
            CVCF_CURRENTFILTER_MASK,
            PTRX,
            0u32,
            CPF,
            0u32,
            /* set pitch offset */
            IP,
            (*vp).apitch,
            /* set envelope parameters */
            ENVVAL,
            (*vp).reg.parm.moddelay,
            ATKHLDM,
            (*vp).reg.parm.modatkhld,
            DCYSUSM,
            (*vp).reg.parm.moddcysus,
            ENVVOL,
            (*vp).reg.parm.voldelay,
            ATKHLDV,
            (*vp).reg.parm.volatkhld,
            /* decay/sustain parameter for volume envelope is used
               for triggerg the voice */
            /* cutoff and volume */
            IFATN,
            ((*vp).acutoff << 8) | ((*vp).avol as u8 as u32),
            /* modulation envelope heights */
            PEFE,
            (*vp).reg.parm.pefe,
            /* lfo1/2 delay */
            LFOVAL1,
            (*vp).reg.parm.lfo1delay,
            LFOVAL2,
            (*vp).reg.parm.lfo2delay,
            /* lfo1 pitch & cutoff shift */
            FMMOD,
            make_fmmod(vp),
            /* lfo1 volume & freq */
            TREMFRQ,
            (*vp).reg.parm.tremfrq,
            /* lfo2 pitch & freq */
            FM2FRQ2,
            make_fm2frq2(vp),
            /* reverb and loop start (reverb 8bit, MSB) */
            PSST,
            psst,
            /* chorus & loop end (chorus 8bit, MSB) */
            DSL,
            dsl,
            /* clear filter delay memory */
            Z1,
            0u32,
            Z2,
            0u32,
            /* invalidate maps */
            MAPA,
            map,
            MAPB,
            map,
            /* Q & current address (Q 4bit value, MSB) */
            CCCA,
            ccca,
            /* cache */
            CCR,
            REG_VAL_PUT(CCR_CACHEINVALIDSIZE, 64),
            /* reset volume */
            VTFT,
            vtarget | (*vp).ftarget,
            CVCF,
            vtarget | CVCF_CURRENTFILTER_MASK,
            REGLIST_END,
        );

        (*(*hw).voices.add(ch as usize)).dirty = 1;
        0
    }
}

/*
 * Start envelope
 */
unsafe extern "C" fn trigger_voice(vp: *mut snd_emux_voice) {
    unsafe {
        let ptarget: u32;
        let hw: *mut snd_emu10k1;
        let emem: *mut snd_emu10k1_memblk;

        hw = (*vp).hw;

        emem = (*vp).block as *mut snd_emu10k1_memblk;
        if emem.is_null() || (*emem).mapped_page < 0 {
            return; /* not mapped */
        }

        /* Original C conditional: #if 0 used ptarget = (unsigned int)vp->ptarget << 16; */
        ptarget = IP_TO_CP((*vp).apitch);
        snd_emu10k1_ptr_write_multiple(
            hw,
            (*vp).ch,
            /* set pitch target and pan (volume) */
            PTRX,
            ptarget | ((*vp).apan << 8) | (*vp).aaux,
            /* current pitch and fractional address */
            CPF,
            ptarget,
            /* enable envelope engine */
            DCYSUSV,
            (*vp).reg.parm.voldcysus | DCYSUSV_CHANNELENABLE_MASK,
            REGLIST_END,
        );
    }
}

const MOD_SENSE: i32 = 18;

/* calculate lfo1 modulation height and cutoff register */
unsafe fn make_fmmod(vp: *mut snd_emux_voice) -> u32 {
    unsafe {
        let mut pitch: i16;
        let cutoff: u8;
        let modulation: i32;

        pitch = (((*vp).reg.parm.fmmod >> 8) as i8) as i16;
        cutoff = ((*vp).reg.parm.fmmod & 0xff) as u8;
        modulation = (*(*vp).chan).gm_modulation + (*(*vp).chan).midi_pressure;
        pitch += ((MOD_SENSE * modulation) / 1200) as i16;
        LIMITVALUE(&mut pitch, -128, 127);
        ((pitch as u8 as u32) << 8) | cutoff as u32
    }
}

/* calculate set lfo2 pitch & frequency register */
unsafe fn make_fm2frq2(vp: *mut snd_emux_voice) -> u32 {
    unsafe {
        let mut pitch: i16;
        let freq: u8;
        let modulation: i32;

        pitch = (((*vp).reg.parm.fm2frq2 >> 8) as i8) as i16;
        freq = ((*vp).reg.parm.fm2frq2 & 0xff) as u8;
        modulation = (*(*vp).chan).gm_modulation + (*(*vp).chan).midi_pressure;
        pitch += ((MOD_SENSE * modulation) / 1200) as i16;
        LIMITVALUE(&mut pitch, -128, 127);
        ((pitch as u8 as u32) << 8) | freq as u32
    }
}

unsafe extern "C" fn get_pitch_shift(emu: *mut snd_emux) -> i32 {
    unsafe {
        let hw: *mut snd_emu10k1 = (*emu).hw;

        if (*(*hw).card_capabilities).emu_model != 0 && (*hw).emu1010.word_clock == 44100 {
            0
        } else {
            -501
        }
    }
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
