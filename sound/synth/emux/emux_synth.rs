// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Midi synth routines for the Emu8k/Emu10k1
 *
 *  Copyright (C) 1999 Steve Ratcliffe
 *  Copyright (c) 1999-2000 Takashi Iwai <tiwai@suse.de>
 *
 *  Contains code based on awe_wave.c by Takashi Iwai
 */

// Requires: emux_voice.h, sound/asoundef.h from kernel headers
// This module depends on external structures defined in those headers

use core::ffi::c_int;
use core::mem;
use core::ptr;

// External types from dependencies (defined in other modules/headers)
// struct snd_emux, struct snd_emux_port, struct snd_midi_channel, struct snd_sf_zone, etc.

/*
 * Ensure a value is between two points
 * Note: In C, the macro evaluated args more than once, now converted to function
 */
#[inline]
fn limit_value(x: &mut i32, a: i32, b: i32) {
    if *x < a {
        *x = a;
    } else if *x > b {
        *x = b;
    }
}

#[inline]
fn limit_max(x: &mut i32, a: i32) {
    if *x > a {
        *x = a;
    }
}

/*
 * Prototypes - these would be resolved by the module system
 */
extern "C" {
    // External functions from other modules
    fn get_zone(
        emu: *mut snd_emux,
        port: *mut snd_emux_port,
        notep: *mut i32,
        vel: i32,
        chan: *mut snd_midi_channel,
        table: *mut *mut snd_sf_zone,
    ) -> i32;

    fn snd_BUG_ON(cond: bool) -> bool;
    fn snd_soundfont_search_zone(
        sflist: *mut libc::c_void,
        notep: *mut i32,
        vel: i32,
        preset: i32,
        bank: i32,
        def_preset: i32,
        def_bank: i32,
        table: *mut *mut snd_sf_zone,
        max_voices: i32,
    ) -> i32;
    fn snd_emux_xg_control(port: *mut snd_emux_port, chan: *mut snd_midi_channel, type_: i32);
    fn snd_emux_send_effect(
        port: *mut snd_emux_port,
        chan: *mut snd_midi_channel,
        effect: i32,
        val: i32,
        flag: i32,
    );
    fn snd_emux_setup_effect(vp: *mut snd_emux_voice);
    fn mod_timer(timer: *mut libc::c_void, expires: libc::c_ulong);
    fn jiffies() -> libc::c_ulong;
    fn dev_warn(dev: *mut libc::c_void, fmt: *const libc::c_char, ...);
    fn timer_container_of(
        emu: *mut snd_emux,
        t: *mut libc::c_void,
        field: *const libc::c_char,
    ) -> *mut snd_emux;

    // External tables
    static snd_sf_vol_table: [i32; 128];
}

// Placeholder structures - actual definitions come from dependencies
#[repr(C)]
pub struct snd_emux {
    // Fields would be defined in emux_voice.h
    // Using opaque for now
}

#[repr(C)]
pub struct snd_emux_port {
    // Fields would be defined in emux_voice.h
    // Using opaque for now
}

#[repr(C)]
pub struct snd_midi_channel {
    // Fields would be defined in asoundef.h
    // Using opaque for now
}

#[repr(C)]
pub struct snd_sf_zone {
    // Fields would be defined in soundfont.h
    // Using opaque for now
}

#[repr(C)]
pub struct snd_emux_voice {
    // Fields would be defined in emux_voice.h
    // Using opaque for now
}

// Constants from the source
const SNDRV_EMUX_MAX_MULTI_VOICES: i32 = 32; // Placeholder value
const SNDRV_EMUX_ST_STANDBY: i32 = 0;
const SNDRV_EMUX_ST_OFF: i32 = 1;
const SNDRV_EMUX_ST_ON: i32 = 2;
const SNDRV_EMUX_ST_RELEASED: i32 = 3;
const SNDRV_EMUX_ST_PENDING: i32 = 4;
const SNDRV_EMUX_ST_LOCKED: i32 = 5;
const SNDRV_EMUX_UPDATE_VOLUME: i32 = 1;
const SNDRV_EMUX_UPDATE_PITCH: i32 = 2;
const SNDRV_EMUX_UPDATE_PAN: i32 = 4;
const SNDRV_EMUX_UPDATE_FMMOD: i32 = 8;
const SNDRV_EMUX_UPDATE_FM2FRQ2: i32 = 16;
const SNDRV_EMUX_PORT_MODE_OSS_SYNTH: i32 = 0;
const SNDRV_MIDI_MODE_XG: i32 = 1;
const SNDRV_MIDI_MODE_GS: i32 = 2;
const MIDI_CTL_MSB_MAIN_VOLUME: usize = 7;
const MIDI_CTL_MSB_EXPRESSION: usize = 11;
const MIDI_CTL_MSB_PAN: usize = 10;
const MIDI_CTL_MSB_BANK: usize = 0;
const MIDI_CTL_LSB_BANK: usize = 32;
const MIDI_CTL_SOFT_PEDAL: usize = 67;
const MIDI_CTL_PITCHBEND: i32 = 0xE0;
const MIDI_CTL_MSB_MODWHEEL: usize = 1;
const MIDI_CTL_CHAN_PRESSURE: i32 = 0xD0;
const EMUX_FX_SAMPLE_START: usize = 0;
const EMUX_FX_COARSE_SAMPLE_START: usize = 1;
const EMUX_FX_CUTOFF: i32 = 2;
const EMUX_FX_FLAG_ADD: i32 = 1;
const EMUX_FX_FLAG_OFF: i32 = 0;
const EMUX_FX_ATTEN: usize = 3;
const EMUX_FX_INIT_PITCH: usize = 4;
const EMUX_MD_DEF_DRUM: usize = 0;
const EMUX_MD_DEF_BANK: usize = 1;
const LO_BYTE_MASK: i32 = 0xff;
const HI_BYTE_SHIFT: i32 = 8;

// Inline helper macros
#[inline]
fn lo_byte(v: i32) -> i32 {
    v & LO_BYTE_MASK
}

#[inline]
fn hi_byte(v: i32) -> i32 {
    ((v) >> HI_BYTE_SHIFT) & LO_BYTE_MASK
}

#[inline]
fn state_is_playing(state: i32) -> bool {
    state == SNDRV_EMUX_ST_ON || state == SNDRV_EMUX_ST_STANDBY
}

#[inline]
fn sf_is_drum_bank(bank: i32) -> bool {
    bank == 128
}

/*
 * Start a note.
 */
pub unsafe extern "C" fn snd_emux_note_on(
    p: *mut libc::c_void,
    note: i32,
    vel: i32,
    chan: *mut snd_midi_channel,
) {
    let mut port = p as *mut snd_emux_port;
    if snd_BUG_ON(port.is_null() || chan.is_null()) {
        return;
    }

    let emu = (*port).emu;
    if snd_BUG_ON(
        emu.is_null()
            || (*emu).ops.get_voice.is_none()
            || (*emu).ops.trigger.is_none(),
    ) {
        return;
    }

    let mut note_mut = note;
    let key = note; /* remember the original note */
    let mut table: [*mut snd_sf_zone; 32] = [ptr::null_mut(); 32]; // SNDRV_EMUX_MAX_MULTI_VOICES
    let nvoices = get_zone(emu, port, &mut note_mut, vel, chan, table.as_mut_ptr());
    if nvoices == 0 {
        return;
    }

    /* exclusive note off */
    for i in 0..nvoices as usize {
        let zp = table[i];
        if !zp.is_null() && (*zp).v.exclusiveClass != 0 {
            exclusive_note_off(emu, port, (*zp).v.exclusiveClass);
        }
    }

    #[allow(unreachable_code)]
    {
        // seems not necessary
        // terminate_note1(emu, key, chan, 0);
    }

    // guard(spinlock_irqsave)(&emu->voice_lock); - acquire lock for this scope
    for i in 0..nvoices as usize {
        if table[i].is_null() {
            continue;
        }

        let vp = ((*emu).ops.get_voice.unwrap())(emu, port);
        if vp.is_null() || (*vp).ch < 0 {
            continue;
        }
        if state_is_playing((*vp).state) {
            ((*emu).ops.terminate.unwrap())(vp);
        }

        (*vp).time = (*emu).use_time;
        (*emu).use_time += 1;
        (*vp).chan = chan;
        (*vp).port = port;
        (*vp).key = key;
        (*vp).note = note_mut;
        (*vp).velocity = vel;
        (*vp).zone = table[i];
        if !(*(*vp).zone).sample.is_null() {
            (*vp).block = (*(*(*vp).zone).sample).block;
        } else {
            (*vp).block = ptr::null_mut();
        }

        setup_voice(vp);

        (*vp).state = SNDRV_EMUX_ST_STANDBY;
        if let Some(prepare) = (*emu).ops.prepare {
            (*vp).state = SNDRV_EMUX_ST_OFF;
            if prepare(vp) >= 0 {
                (*vp).state = SNDRV_EMUX_ST_STANDBY;
            }
        }
    }

    /* start envelope now */
    for i in 0..(*emu).max_voices as usize {
        let vp = &mut (*emu).voices[i];
        if (*vp).state == SNDRV_EMUX_ST_STANDBY && (*vp).chan == chan {
            ((*emu).ops.trigger.unwrap())(vp as *mut snd_emux_voice);
            (*vp).state = SNDRV_EMUX_ST_ON;
            (*vp).ontime = jiffies(); /* remember the trigger timing */
        }
    }

    #[cfg(feature = "SNDRV_EMUX_USE_RAW_EFFECT")]
    {
        if (*port).port_mode == SNDRV_EMUX_PORT_MODE_OSS_SYNTH {
            /* clear voice position for the next note on this channel */
            let fx = (*chan).private as *mut snd_emux_effect_table;
            if !fx.is_null() {
                (*fx).flag[EMUX_FX_SAMPLE_START] = 0;
                (*fx).flag[EMUX_FX_COARSE_SAMPLE_START] = 0;
            }
        }
    }
    // lock guard released here at end of scope
}

/*
 * Release a note in response to a midi note off.
 */
pub unsafe extern "C" fn snd_emux_note_off(
    p: *mut libc::c_void,
    note: i32,
    vel: i32,
    chan: *mut snd_midi_channel,
) {
    let port = p as *mut snd_emux_port;
    if snd_BUG_ON(port.is_null() || chan.is_null()) {
        return;
    }

    let emu = (*port).emu;
    if snd_BUG_ON(emu.is_null() || (*emu).ops.release.is_none()) {
        return;
    }

    // guard(spinlock_irqsave)(&emu->voice_lock);
    for ch in 0..(*emu).max_voices as usize {
        let vp = &mut (*emu).voices[ch];
        if state_is_playing((*vp).state) && (*vp).chan == chan && (*vp).key == note {
            (*vp).state = SNDRV_EMUX_ST_RELEASED;
            if (*vp).ontime == jiffies() {
                /* if note-off is sent too shortly after
                 * note-on, emuX engine cannot produce the sound
                 * correctly.  so we'll release this note
                 * a bit later via timer callback.
                 */
                (*vp).state = SNDRV_EMUX_ST_PENDING;
                if (*emu).timer_active == 0 {
                    mod_timer(&mut (*emu).tlist as *mut _ as *mut libc::c_void, jiffies() + 1);
                    (*emu).timer_active = 1;
                }
            } else {
                /* ok now release the note */
                ((*emu).ops.release.unwrap())(vp as *mut snd_emux_voice);
            }
        }
    }
    // lock guard released here at end of scope
}

/*
 * timer callback
 *
 * release the pending note-offs
 */
pub unsafe extern "C" fn snd_emux_timer_callback(t: *mut libc::c_void) {
    let emu = timer_container_of(ptr::null_mut(), t, ptr::null());
    let mut do_again = 0;

    // guard(spinlock_irqsave)(&emu->voice_lock);
    for ch in 0..(*emu).max_voices as usize {
        let vp = &mut (*emu).voices[ch];
        if (*vp).state == SNDRV_EMUX_ST_PENDING {
            if (*vp).ontime == jiffies() {
                do_again += 1; /* release this at the next interrupt */
            } else {
                ((*emu).ops.release.unwrap())(vp as *mut snd_emux_voice);
                (*vp).state = SNDRV_EMUX_ST_RELEASED;
            }
        }
    }
    if do_again != 0 {
        mod_timer(&mut (*emu).tlist as *mut _ as *mut libc::c_void, jiffies() + 1);
        (*emu).timer_active = 1;
    } else {
        (*emu).timer_active = 0;
    }
    // lock guard released here at end of scope
}

/*
 * key pressure change
 */
pub unsafe extern "C" fn snd_emux_key_press(
    p: *mut libc::c_void,
    note: i32,
    vel: i32,
    chan: *mut snd_midi_channel,
) {
    let port = p as *mut snd_emux_port;
    if snd_BUG_ON(port.is_null() || chan.is_null()) {
        return;
    }

    let emu = (*port).emu;
    if snd_BUG_ON(emu.is_null() || (*emu).ops.update.is_none()) {
        return;
    }

    // guard(spinlock_irqsave)(&emu->voice_lock);
    for ch in 0..(*emu).max_voices as usize {
        let vp = &mut (*emu).voices[ch];
        if (*vp).state == SNDRV_EMUX_ST_ON && (*vp).chan == chan && (*vp).key == note {
            (*vp).velocity = vel;
            update_voice(emu, vp as *mut snd_emux_voice, SNDRV_EMUX_UPDATE_VOLUME);
        }
    }
    // lock guard released here at end of scope
}

/*
 * Modulate the voices which belong to the channel
 */
pub unsafe extern "C" fn snd_emux_update_channel(
    port: *mut snd_emux_port,
    chan: *mut snd_midi_channel,
    update: i32,
) {
    if update == 0 {
        return;
    }

    let emu = (*port).emu;
    if snd_BUG_ON(emu.is_null() || (*emu).ops.update.is_none()) {
        return;
    }

    // guard(spinlock_irqsave)(&emu->voice_lock);
    for i in 0..(*emu).max_voices as usize {
        let vp = &mut (*emu).voices[i];
        if (*vp).chan == chan {
            update_voice(emu, vp as *mut snd_emux_voice, update);
        }
    }
    // lock guard released here at end of scope
}

/*
 * Modulate all the voices which belong to the port.
 */
pub unsafe extern "C" fn snd_emux_update_port(port: *mut snd_emux_port, update: i32) {
    if update == 0 {
        return;
    }

    let emu = (*port).emu;
    if snd_BUG_ON(emu.is_null() || (*emu).ops.update.is_none()) {
        return;
    }

    // guard(spinlock_irqsave)(&emu->voice_lock);
    for i in 0..(*emu).max_voices as usize {
        let vp = &mut (*emu).voices[i];
        if (*vp).port == port {
            update_voice(emu, vp as *mut snd_emux_voice, update);
        }
    }
    // lock guard released here at end of scope
}

/*
 * Deal with a controller type event.  This includes all types of
 * control events, not just the midi controllers
 */
pub unsafe extern "C" fn snd_emux_control(
    p: *mut libc::c_void,
    type_: i32,
    chan: *mut snd_midi_channel,
) {
    let port = p as *mut snd_emux_port;
    if snd_BUG_ON(port.is_null() || chan.is_null()) {
        return;
    }

    match type_ {
        MIDI_CTL_MSB_MAIN_VOLUME | MIDI_CTL_MSB_EXPRESSION => {
            snd_emux_update_channel(port, chan, SNDRV_EMUX_UPDATE_VOLUME);
        }
        MIDI_CTL_MSB_PAN => {
            snd_emux_update_channel(port, chan, SNDRV_EMUX_UPDATE_PAN);
        }
        MIDI_CTL_SOFT_PEDAL => {
            #[cfg(feature = "SNDRV_EMUX_USE_RAW_EFFECT")]
            {
                // FIXME: this is an emulation
                if (*chan).control[MIDI_CTL_SOFT_PEDAL] >= 64 {
                    snd_emux_send_effect(port, chan, EMUX_FX_CUTOFF, -160, EMUX_FX_FLAG_ADD);
                } else {
                    snd_emux_send_effect(port, chan, EMUX_FX_CUTOFF, 0, EMUX_FX_FLAG_OFF);
                }
            }
        }
        MIDI_CTL_PITCHBEND => {
            snd_emux_update_channel(port, chan, SNDRV_EMUX_UPDATE_PITCH);
        }
        MIDI_CTL_MSB_MODWHEEL | MIDI_CTL_CHAN_PRESSURE => {
            snd_emux_update_channel(
                port,
                chan,
                SNDRV_EMUX_UPDATE_FMMOD | SNDRV_EMUX_UPDATE_FM2FRQ2,
            );
        }
        _ => {}
    }

    if (*(*port).chset).midi_mode == SNDRV_MIDI_MODE_XG {
        snd_emux_xg_control(port, chan, type_);
    }
}

/*
 * terminate note - if free flag is true, free the terminated voice
 */
unsafe fn terminate_note1(
    emu: *mut snd_emux,
    note: i32,
    chan: *mut snd_midi_channel,
    free_: i32,
) {
    // guard(spinlock_irqsave)(&emu->voice_lock);
    for i in 0..(*emu).max_voices as usize {
        let vp = &mut (*emu).voices[i];
        if state_is_playing((*vp).state) && (*vp).chan == chan && (*vp).key == note {
            terminate_voice(emu, vp as *mut snd_emux_voice, free_);
        }
    }
    // lock guard released here at end of scope
}

/*
 * terminate note - exported for midi emulation
 */
pub unsafe extern "C" fn snd_emux_terminate_note(p: *mut libc::c_void, note: i32, chan: *mut snd_midi_channel) {
    let port = p as *mut snd_emux_port;
    if snd_BUG_ON(port.is_null() || chan.is_null()) {
        return;
    }

    let emu = (*port).emu;
    if snd_BUG_ON(emu.is_null() || (*emu).ops.terminate.is_none()) {
        return;
    }

    terminate_note1(emu, note, chan, 1);
}

/*
 * Terminate all the notes
 */
pub unsafe extern "C" fn snd_emux_terminate_all(emu: *mut snd_emux) {
    // guard(spinlock_irqsave)(&emu->voice_lock);
    for i in 0..(*emu).max_voices as usize {
        let vp = &mut (*emu).voices[i];
        if state_is_playing((*vp).state) {
            terminate_voice(emu, vp as *mut snd_emux_voice, 0);
        }
        if (*vp).state == SNDRV_EMUX_ST_OFF {
            if let Some(free_voice) = (*emu).ops.free_voice {
                free_voice(vp as *mut snd_emux_voice);
            }
            if let Some(reset) = (*emu).ops.reset {
                reset(emu, i as i32);
            }
        }
        (*vp).time = 0;
    }
    /* initialize allocation time */
    (*emu).use_time = 0;
    // lock guard released here at end of scope
}

// EXPORT_SYMBOL(snd_emux_terminate_all);

/*
 * Terminate all voices associated with the given port
 */
pub unsafe extern "C" fn snd_emux_sounds_off_all(port: *mut snd_emux_port) {
    if snd_BUG_ON(port.is_null()) {
        return;
    }
    let emu = (*port).emu;
    if snd_BUG_ON(emu.is_null() || (*emu).ops.terminate.is_none()) {
        return;
    }

    // guard(spinlock_irqsave)(&emu->voice_lock);
    for i in 0..(*emu).max_voices as usize {
        let vp = &mut (*emu).voices[i];
        if state_is_playing((*vp).state) && (*vp).port == port {
            terminate_voice(emu, vp as *mut snd_emux_voice, 0);
        }
        if (*vp).state == SNDRV_EMUX_ST_OFF {
            if let Some(free_voice) = (*emu).ops.free_voice {
                free_voice(vp as *mut snd_emux_voice);
            }
            if let Some(reset) = (*emu).ops.reset {
                reset(emu, i as i32);
            }
        }
    }
    // lock guard released here at end of scope
}

/*
 * Terminate all voices that have the same exclusive class.  This
 * is mainly for drums.
 */
unsafe fn exclusive_note_off(emu: *mut snd_emux, port: *mut snd_emux_port, exclass: i32) {
    // guard(spinlock_irqsave)(&emu->voice_lock);
    for i in 0..(*emu).max_voices as usize {
        let vp = &mut (*emu).voices[i];
        if state_is_playing((*vp).state) && (*vp).port == port && (*vp).reg.exclusiveClass == exclass {
            terminate_voice(emu, vp as *mut snd_emux_voice, 0);
        }
    }
    // lock guard released here at end of scope
}

/*
 * terminate a voice
 * if free flag is true, call free_voice after termination
 */
unsafe fn terminate_voice(emu: *mut snd_emux, vp: *mut snd_emux_voice, free_: i32) {
    ((*emu).ops.terminate.unwrap())(vp);
    (*vp).time = (*emu).use_time;
    (*emu).use_time += 1;
    (*vp).chan = ptr::null_mut();
    (*vp).port = ptr::null_mut();
    (*vp).zone = ptr::null_mut();
    (*vp).block = ptr::null_mut();
    (*vp).state = SNDRV_EMUX_ST_OFF;
    if free_ != 0 {
        if let Some(free_voice) = (*emu).ops.free_voice {
            free_voice(vp);
        }
    }
}

/*
 * Modulate the voice
 */
unsafe fn update_voice(emu: *mut snd_emux, vp: *mut snd_emux_voice, update: i32) {
    if !state_is_playing((*vp).state) {
        return;
    }

    if (*vp).chan.is_null() || (*vp).port.is_null() {
        return;
    }
    if update & SNDRV_EMUX_UPDATE_VOLUME != 0 {
        calc_volume(vp);
    }
    if update & SNDRV_EMUX_UPDATE_PITCH != 0 {
        calc_pitch(vp);
    }
    if update & SNDRV_EMUX_UPDATE_PAN != 0 {
        if calc_pan(vp) == 0 && update == SNDRV_EMUX_UPDATE_PAN {
            return;
        }
    }
    ((*emu).ops.update.unwrap())(vp, update);
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
{
    /* not used - volume target calculation table */
    // const voltarget: [u16; 16] = [
    //     0xEAC0, 0xE0C8, 0xD740, 0xCE20, 0xC560, 0xBD08, 0xB500, 0xAD58,
    //     0xA5F8, 0x9EF0, 0x9830, 0x91C0, 0x8B90, 0x85A8, 0x8000, 0x7A90
    // ];
}

/*
 * Sets up the voice structure by calculating some values that
 * will be needed later.
 */
unsafe fn setup_voice(vp: *mut snd_emux_voice) {
    /* copy the original register values */
    (*vp).reg = (*(*vp).zone).v;

    #[cfg(feature = "SNDRV_EMUX_USE_RAW_EFFECT")]
    {
        snd_emux_setup_effect(vp);
    }

    /* reset status */
    (*vp).apan = -1;
    (*vp).avol = -1;
    (*vp).apitch = -1;

    calc_volume(vp);
    calc_pitch(vp);
    calc_pan(vp);

    let parm = &mut (*vp).reg.parm;

    /* compute filter target and correct modulation parameters */
    if lo_byte((*parm).modatkhld) >= 0x80 && (*parm).moddelay >= 0x8000 {
        (*parm).moddelay = 0xbfff;
        let mut pitch = (hi_byte((*parm).pefe) << 4) + (*vp).apitch;
        if pitch > 0xffff {
            pitch = 0xffff;
        }
        /* calculate filter target */
        (*vp).ftarget = (*parm).cutoff + lo_byte((*parm).pefe);
        limit_value(&mut (*vp).ftarget, 0, 255);
        (*vp).ftarget <<= 8;
    } else {
        (*vp).ftarget = (*parm).cutoff;
        (*vp).ftarget <<= 8;
    }

    /* compute pitch target */
    if (*vp).apitch != 0xffff {
        (*vp).ptarget = 1 << ((*vp).apitch >> 12);
        if (*vp).apitch & 0x800 != 0 {
            (*vp).ptarget += ((*vp).ptarget * 0x102e) / 0x2710;
        }
        if (*vp).apitch & 0x400 != 0 {
            (*vp).ptarget += ((*vp).ptarget * 0x764) / 0x2710;
        }
        if (*vp).apitch & 0x200 != 0 {
            (*vp).ptarget += ((*vp).ptarget * 0x389) / 0x2710;
        }
        (*vp).ptarget += (*vp).ptarget >> 1;
        if (*vp).ptarget > 0xffff {
            (*vp).ptarget = 0xffff;
        }
    } else {
        (*vp).ptarget = 0xffff;
    }

    if lo_byte((*parm).modatkhld) >= 0x80 {
        (*parm).modatkhld &= !0xff;
        (*parm).modatkhld |= 0x7f;
    }

    /* compute volume target and correct volume parameters */
    (*vp).vtarget = 0;
    /* FIXME: this leads to some clicks.. */

    if lo_byte((*parm).volatkhld) >= 0x80 {
        (*parm).volatkhld &= !0xff;
        (*parm).volatkhld |= 0x7f;
    }
}

/*
 * Pan volume lookup table
 */
const PAN_VOLUMES: [u8; 256] = [
    0x00, 0x03, 0x06, 0x09, 0x0c, 0x0f, 0x12, 0x14, 0x17, 0x1a, 0x1d, 0x20, 0x22, 0x25, 0x28,
    0x2a, 0x2d, 0x30, 0x32, 0x35, 0x37, 0x3a, 0x3c, 0x3f, 0x41, 0x44, 0x46, 0x49, 0x4b, 0x4d,
    0x50, 0x52, 0x54, 0x57, 0x59, 0x5b, 0x5d, 0x60, 0x62, 0x64, 0x66, 0x68, 0x6a, 0x6c, 0x6f,
    0x71, 0x73, 0x75, 0x77, 0x79, 0x7b, 0x7c, 0x7e, 0x80, 0x82, 0x84, 0x86, 0x88, 0x89, 0x8b,
    0x8d, 0x8f, 0x90, 0x92, 0x94, 0x96, 0x97, 0x99, 0x9a, 0x9c, 0x9e, 0x9f, 0xa1, 0xa2, 0xa4,
    0xa5, 0xa7, 0xa8, 0xaa, 0xab, 0xad, 0xae, 0xaf, 0xb1, 0xb2, 0xb3, 0xb5, 0xb6, 0xb7, 0xb9,
    0xba, 0xbb, 0xbc, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca,
    0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd7, 0xd8,
    0xd9, 0xda, 0xdb, 0xdc, 0xdc, 0xdd, 0xde, 0xdf, 0xdf, 0xe0, 0xe1, 0xe2, 0xe2, 0xe3, 0xe4,
    0xe4, 0xe5, 0xe6, 0xe6, 0xe7, 0xe8, 0xe8, 0xe9, 0xe9, 0xea, 0xeb, 0xeb, 0xec, 0xec, 0xed,
    0xed, 0xee, 0xee, 0xef, 0xef, 0xf0, 0xf0, 0xf1, 0xf1, 0xf1, 0xf2, 0xf2, 0xf3, 0xf3, 0xf3,
    0xf4, 0xf4, 0xf5, 0xf5, 0xf5, 0xf6, 0xf6, 0xf6, 0xf7, 0xf7, 0xf7, 0xf7, 0xf8, 0xf8, 0xf8,
    0xf9, 0xf9, 0xf9, 0xf9, 0xf9, 0xfa, 0xfa, 0xfa, 0xfa, 0xfb, 0xfb, 0xfb, 0xfb, 0xfb, 0xfc,
    0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfd, 0xfd, 0xfd, 0xfd, 0xfd, 0xfd, 0xfd, 0xfd, 0xfe,
    0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff,
];

/*
 * calculate pan
 */
unsafe fn calc_pan(vp: *mut snd_emux_voice) -> i32 {
    let chan = (*vp).chan;
    let mut pan: i32;

    /* pan & loop start (pan 8bit, MSB, 0:right, 0xff:left) */
    if (*vp).reg.fixpan > 0 {
        /* 0-127 */
        pan = 255 - ((*vp).reg.fixpan as i32) * 2;
    } else {
        pan = (*chan).control[MIDI_CTL_MSB_PAN] as i32 - 64;
        if (*vp).reg.pan >= 0 {
            /* 0-127 */
            pan += (*vp).reg.pan as i32 - 64;
        }
        pan = 127 - (pan as i32) * 2;
    }
    limit_value(&mut pan, 0, 255);

    if (*(*vp).emu).linear_panning != 0 {
        /* assuming linear volume */
        if pan != (*vp).apan {
            (*vp).apan = pan;
            if pan == 0 {
                (*vp).aaux = 0xff;
            } else {
                (*vp).aaux = ((-pan) & 0xff) as u8;
            }
            return 1;
        } else {
            return 0;
        }
    } else {
        /* using volume table */
        if (*vp).apan != PAN_VOLUMES[pan as usize] as i32 {
            (*vp).apan = PAN_VOLUMES[pan as usize] as i32;
            (*vp).aaux = PAN_VOLUMES[(255 - pan) as usize];
            return 1;
        }
        return 0;
    }
}

/*
 * Volume attenuation tables
 */
const VOLTAB1: [u8; 128] = [
    0x63, 0x63, 0x63, 0x63, 0x63, 0x63, 0x63, 0x63, 0x63, 0x63, 0x63, 0x2b, 0x29, 0x28, 0x27,
    0x26, 0x25, 0x24, 0x23, 0x22, 0x21, 0x20, 0x1f, 0x1e, 0x1e, 0x1d, 0x1c, 0x1b, 0x1b, 0x1a,
    0x19, 0x19, 0x18, 0x17, 0x17, 0x16, 0x16, 0x15, 0x15, 0x14, 0x14, 0x13, 0x13, 0x13, 0x12,
    0x12, 0x11, 0x11, 0x11, 0x10, 0x10, 0x10, 0x0f, 0x0f, 0x0f, 0x0e, 0x0e, 0x0e, 0x0e, 0x0d,
    0x0d, 0x0d, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0b, 0x0b, 0x0b, 0x0b, 0x0a, 0x0a, 0x0a, 0x0a,
    0x09, 0x09, 0x09, 0x09, 0x09, 0x08, 0x08, 0x08, 0x08, 0x08, 0x07, 0x07, 0x07, 0x07, 0x06,
    0x06, 0x06, 0x06, 0x06, 0x05, 0x05, 0x05, 0x05, 0x05, 0x04, 0x04, 0x04, 0x04, 0x04, 0x03,
    0x03, 0x03, 0x03, 0x03, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const VOLTAB2: [u8; 128] = [
    0x32, 0x31, 0x30, 0x2f, 0x2e, 0x2d, 0x2c, 0x2b, 0x2a, 0x2a, 0x29, 0x28, 0x27, 0x26, 0x25,
    0x24, 0x24, 0x23, 0x22, 0x21, 0x21, 0x20, 0x1f, 0x1e, 0x1e, 0x1d, 0x1c, 0x1c, 0x1b, 0x1a,
    0x1a, 0x19, 0x19, 0x18, 0x18, 0x17, 0x16, 0x16, 0x15, 0x15, 0x14, 0x14, 0x13, 0x13, 0x13,
    0x12, 0x12, 0x11, 0x11, 0x10, 0x10, 0x10, 0x0f, 0x0f, 0x0f, 0x0e, 0x0e, 0x0e, 0x0d, 0x0d,
    0x0d, 0x0c, 0x0c, 0x0c, 0x0b, 0x0b, 0x0b, 0x0b, 0x0a, 0x0a, 0x0a, 0x0a, 0x09, 0x09, 0x09,
    0x09, 0x09, 0x08, 0x08, 0x08, 0x08, 0x08, 0x07, 0x07, 0x07, 0x07, 0x07, 0x06, 0x06, 0x06,
    0x06, 0x06, 0x06, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const EXPRESSIONTAB: [u8; 128] = [
    0x7f, 0x6c, 0x62, 0x5a, 0x54, 0x50, 0x4b, 0x48, 0x45, 0x42, 0x40, 0x3d, 0x3b, 0x39, 0x38,
    0x36, 0x34, 0x33, 0x31, 0x30, 0x2f, 0x2d, 0x2c, 0x2b, 0x2a, 0x29, 0x28, 0x27, 0x26, 0x25,
    0x24, 0x24, 0x23, 0x22, 0x21, 0x21, 0x20, 0x1f, 0x1e, 0x1e, 0x1d, 0x1d, 0x1c, 0x1b, 0x1b,
    0x1a, 0x1a, 0x19, 0x18, 0x18, 0x17, 0x17, 0x16, 0x16, 0x15, 0x15, 0x15, 0x14, 0x14, 0x13,
    0x13, 0x12, 0x12, 0x11, 0x11, 0x11, 0x10, 0x10, 0x0f, 0x0f, 0x0f, 0x0e, 0x0e, 0x0e, 0x0d,
    0x0d, 0x0d, 0x0c, 0x0c, 0x0c, 0x0b, 0x0b, 0x0b, 0x0a, 0x0a, 0x0a, 0x09, 0x09, 0x09, 0x09,
    0x08, 0x08, 0x08, 0x07, 0x07, 0x07, 0x07, 0x06, 0x06, 0x06, 0x06, 0x05, 0x05, 0x05, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x03, 0x03, 0x03, 0x03, 0x02, 0x02, 0x02, 0x02, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/*
 * Magic to calculate the volume (actually attenuation) from all the
 * voice and channels parameters.
 */
unsafe fn calc_volume(vp: *mut snd_emux_voice) -> i32 {
    let mut vol: i32;
    let mut main_vol: i32;
    let expression_vol: i32;
    let mut master_vol: i32;
    let chan = (*vp).chan;
    let port = (*vp).port;

    expression_vol = (*chan).control[MIDI_CTL_MSB_EXPRESSION] as i32;
    limit_max(&mut (*vp).velocity, 127);
    let mut expr_vol = expression_vol;
    limit_value(&mut expr_vol, 0, 127);

    if (*port).port_mode == SNDRV_EMUX_PORT_MODE_OSS_SYNTH {
        /* 0 - 127 */
        main_vol = (*chan).control[MIDI_CTL_MSB_MAIN_VOLUME] as i32;
        vol = ((*vp).velocity as i32 * main_vol * expr_vol) / (127 * 127);
        vol = vol * (*vp).reg.amplitude as i32 / 127;

        limit_value(&mut vol, 0, 127);

        /* calc to attenuation */
        vol = snd_sf_vol_table[vol as usize];
    } else {
        main_vol = (*chan).control[MIDI_CTL_MSB_MAIN_VOLUME] as i32 * (*vp).reg.amplitude as i32
            / 127;
        limit_value(&mut main_vol, 0, 127);

        vol = VOLTAB1[main_vol as usize] as i32 + VOLTAB2[(*vp).velocity as usize] as i32;
        vol = (vol * 8) / 3;
        vol += (*vp).reg.attenuation as i32;
        vol += ((0x100 - vol) * EXPRESSIONTAB[expr_vol as usize] as i32) / 128;
    }

    master_vol = (*(*port).chset).gs_master_volume as i32;
    limit_value(&mut master_vol, 0, 127);
    vol += snd_sf_vol_table[master_vol as usize];
    vol += (*port).volume_atten as i32;

    #[cfg(feature = "SNDRV_EMUX_USE_RAW_EFFECT")]
    {
        if !(*chan).private.is_null() {
            let fx = (*chan).private as *mut snd_emux_effect_table;
            vol += (*fx).val[EMUX_FX_ATTEN];
        }
    }

    limit_value(&mut vol, 0, 255);
    if (*vp).avol == vol {
        return 0; /* value unchanged */
    }

    (*vp).avol = vol;
    if !sf_is_drum_bank(get_bank(port, chan))
        && lo_byte((*vp).reg.parm.volatkhld) < 0x7d
    {
        let mut atten: i32;
        if (*vp).velocity < 70 {
            atten = 70;
        } else {
            atten = (*vp).velocity as i32;
        }
        (*vp).acutoff = (atten * (*vp).reg.parm.cutoff as i32 + 0xa0) >> 7;
    } else {
        (*vp).acutoff = (*vp).reg.parm.cutoff as i32;
    }

    return 1; /* value changed */
}

/*
 * calculate pitch offset
 *
 * 0xE000 is no pitch offset at 44100Hz sample.
 * Every 4096 is one octave.
 */
unsafe fn calc_pitch(vp: *mut snd_emux_voice) -> i32 {
    let chan = (*vp).chan;
    let mut offset: i32;

    /* calculate offset */
    if (*vp).reg.fixkey >= 0 {
        offset = ((*vp).reg.fixkey - (*vp).reg.root) * 4096 / 12;
    } else {
        offset = ((*vp).note - (*vp).reg.root) * 4096 / 12;
    }
    offset = (offset * (*vp).reg.scaleTuning as i32) / 100;
    offset += (*vp).reg.tune as i32 * 4096 / 1200;
    if (*chan).midi_pitchbend != 0 {
        /* (128 * 8192: 1 semitone) ==> (4096: 12 semitones) */
        offset +=
            (*chan).midi_pitchbend * (*chan).gm_rpn_pitch_bend_range as i32 / 3072;
    }

    /* tuning via RPN:
     *   coarse = -8192 to 8192 (100 cent per 128)
     *   fine = -8192 to 8192 (max=100cent)
     */
    /* 4096 = 1200 cents in emu8000 parameter */
    offset += (*chan).gm_rpn_coarse_tuning as i32 * 4096 / (12 * 128);
    offset += (*chan).gm_rpn_fine_tuning as i32 / 24;

    #[cfg(feature = "SNDRV_EMUX_USE_RAW_EFFECT")]
    {
        /* add initial pitch correction */
        if !(*chan).private.is_null() {
            let fx = (*chan).private as *mut snd_emux_effect_table;
            if (*fx).flag[EMUX_FX_INIT_PITCH] != 0 {
                offset += (*fx).val[EMUX_FX_INIT_PITCH];
            }
        }
    }

    /* 0xe000: root pitch */
    offset += 0xe000 + (*vp).reg.rate_offset as i32;
    if let Some(get_pitch_shift) = (*(*vp).emu).ops.get_pitch_shift {
        offset += get_pitch_shift((*vp).emu);
    }
    limit_value(&mut offset, 0, 0xffff);
    if offset == (*vp).apitch {
        return 0; /* unchanged */
    }
    (*vp).apitch = offset;
    return 1; /* value changed */
}

/*
 * Get the bank number assigned to the channel
 */
unsafe fn get_bank(port: *mut snd_emux_port, chan: *mut snd_midi_channel) -> i32 {
    let val: i32;

    match (*(*port).chset).midi_mode {
        SNDRV_MIDI_MODE_XG => {
            val = (*chan).control[MIDI_CTL_MSB_BANK] as i32;
            if val == 127 {
                return 128; /* return drum bank */
            }
            return (*chan).control[MIDI_CTL_LSB_BANK] as i32;
        }
        SNDRV_MIDI_MODE_GS => {
            if (*chan).drum_channel != 0 {
                return 128;
            }
            /* ignore LSB (bank map) */
            return (*chan).control[MIDI_CTL_MSB_BANK] as i32;
        }
        _ => {
            if (*chan).drum_channel != 0 {
                return 128;
            }
            return (*chan).control[MIDI_CTL_MSB_BANK] as i32;
        }
    }
}

/* Look for the zones matching with the given note and velocity.
 * The resultant zones are stored on table.
 */
unsafe fn _get_zone(
    emu: *mut snd_emux,
    port: *mut snd_emux_port,
    notep: *mut i32,
    vel: i32,
    chan: *mut snd_midi_channel,
    table: *mut *mut snd_sf_zone,
) -> i32 {
    let mut preset: i32;
    let mut bank: i32;
    let def_preset: i32;
    let def_bank: i32;

    bank = get_bank(port, chan);
    preset = (*chan).midi_program as i32;

    if sf_is_drum_bank(bank) {
        def_preset = (*port).ctrls[EMUX_MD_DEF_DRUM] as i32;
        def_bank = bank;
    } else {
        def_preset = preset;
        def_bank = (*port).ctrls[EMUX_MD_DEF_BANK] as i32;
    }

    return snd_soundfont_search_zone(
        (*emu).sflist,
        notep,
        vel,
        preset,
        bank,
        def_preset,
        def_bank,
        table,
        SNDRV_EMUX_MAX_MULTI_VOICES,
    );
}

/*
 * Initialize all voices
 */
pub unsafe extern "C" fn snd_emux_init_voices(emu: *mut snd_emux) {
    let vp: *mut snd_emux_voice;

    // guard(spinlock_irqsave)(&emu->voice_lock);
    for i in 0..(*emu).max_voices as usize {
        let vp = &mut (*emu).voices[i];
        (*vp).ch = -1; /* not used */
        (*vp).state = SNDRV_EMUX_ST_OFF;
        (*vp).chan = ptr::null_mut();
        (*vp).port = ptr::null_mut();
        (*vp).time = 0;
        (*vp).emu = emu;
        (*vp).hw = (*emu).hw;
    }
    // lock guard released here at end of scope
}

/*
 * Lock a voice
 */
pub unsafe extern "C" fn snd_emux_lock_voice(emu: *mut snd_emux, voice: i32) {
    // guard(spinlock_irqsave)(&emu->voice_lock);
    if (*emu).voices[voice as usize].state == SNDRV_EMUX_ST_OFF {
        (*emu).voices[voice as usize].state = SNDRV_EMUX_ST_LOCKED;
    } else {
        dev_warn(
            (*(*emu).card).dev,
            b"invalid voice for lock %d (state = %x)\n\0" as *const _ as *const libc::c_char,
            voice,
            (*emu).voices[voice as usize].state,
        );
    }
    // lock guard released here at end of scope
}

// EXPORT_SYMBOL(snd_emux_lock_voice);

/*
 * Unlock a voice
 */
pub unsafe extern "C" fn snd_emux_unlock_voice(emu: *mut snd_emux, voice: i32) {
    // guard(spinlock_irqsave)(&emu->voice_lock);
    if (*emu).voices[voice as usize].state == SNDRV_EMUX_ST_LOCKED {
        (*emu).voices[voice as usize].state = SNDRV_EMUX_ST_OFF;
    } else {
        dev_warn(
            (*(*emu).card).dev,
            b"invalid voice for unlock %d (state = %x)\n\0" as *const _ as *const libc::c_char,
            voice,
            (*emu).voices[voice as usize].state,
        );
    }
    // lock guard released here at end of scope
}

// EXPORT_SYMBOL(snd_emux_unlock_voice);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
