// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Midi synth routines for the Emu8k/Emu10k1
 *
 *  Copyright (C) 1999 Steve Ratcliffe
 *  Copyright (c) 1999-2000 Takashi Iwai <tiwai@suse.de>
 *
 *  Contains code based on awe_wave.c by Takashi Iwai
 */

// Requires: emux_voice.h types, linux/slab.h allocation functions

#[cfg(feature = "SNDRV_EMUX_USE_RAW_EFFECT")]
pub mod emux_effect {
    use std::ffi::c_void;

    // External types from emux_voice.h and other modules
    // TODO: Define or import from dependencies:
    // - struct snd_emux
    // - struct snd_midi_channel
    // - struct snd_emux_effect_table
    // - struct snd_emux_voice
    // - struct snd_emux_port
    // - EMUX_NUM_EFFECTS constant
    // - EMUX_FX_* constants
    // - STATE_IS_PLAYING macro
    // - SNDRV_EMUX_UPDATE_* flags
    // - SNDRV_SFNT_SAMPLE_8BITS flag

    // Offset of a field in a type via null pointer dereference
    // Original: ((long)(&((type)NULL)->tag) - (long)(NULL))
    unsafe fn xoffsetof<T>(offset_fn: fn(*const T) -> *const c_void) -> isize {
        let null_ptr: *const T = std::ptr::null();
        offset_fn(null_ptr) as isize - null_ptr as isize
    }

    // Effects table structure and constants
    const PARM_IS_BYTE: i32 = 1 << 0;
    const PARM_IS_WORD: i32 = 1 << 1;
    const PARM_IS_ALIGNED: i32 = 3 << 2;
    const PARM_IS_ALIGN_HI: i32 = 1 << 2;
    const PARM_IS_ALIGN_LO: i32 = 2 << 2;
    const PARM_IS_SIGNED: i32 = 1 << 4;

    const PARM_WORD: i32 = PARM_IS_WORD;
    const PARM_BYTE_LO: i32 = PARM_IS_BYTE | PARM_IS_ALIGN_LO;
    const PARM_BYTE_HI: i32 = PARM_IS_BYTE | PARM_IS_ALIGN_HI;
    const PARM_BYTE: i32 = PARM_IS_BYTE;
    const PARM_SIGN_LO: i32 = PARM_IS_BYTE | PARM_IS_ALIGN_LO | PARM_IS_SIGNED;
    const PARM_SIGN_HI: i32 = PARM_IS_BYTE | PARM_IS_ALIGN_HI | PARM_IS_SIGNED;

    struct EmuxParmDefs {
        type_: i32,
        low: i32,
        high: i32,
        offset: isize,
        update: i32,
    }

    // Effects table - requires parm_offset calculations
    // These offset values would be derived from struct soundfont_voice_parm layout
    static PARM_DEFS: &[EmuxParmDefs] = &[
        // env1 delay
        EmuxParmDefs { type_: PARM_WORD, low: 0, high: 0x8000, offset: 0, update: 0 },
        // env1 attack
        EmuxParmDefs { type_: PARM_BYTE_LO, low: 1, high: 0x80, offset: 0, update: 0 },
        // env1 hold
        EmuxParmDefs { type_: PARM_BYTE_HI, low: 0, high: 0x7e, offset: 0, update: 0 },
        // env1 decay
        EmuxParmDefs { type_: PARM_BYTE_LO, low: 1, high: 0x7f, offset: 0, update: 0 },
        // env1 release
        EmuxParmDefs { type_: PARM_BYTE_LO, low: 1, high: 0x7f, offset: 0, update: 0 },
        // env1 sustain
        EmuxParmDefs { type_: PARM_BYTE_HI, low: 0, high: 0x7f, offset: 0, update: 0 },
        // env1 pitch
        EmuxParmDefs { type_: PARM_BYTE_HI, low: 0, high: 0xff, offset: 0, update: 0 },
        // env1 fc
        EmuxParmDefs { type_: PARM_BYTE_LO, low: 0, high: 0xff, offset: 0, update: 0 },

        // env2 delay
        EmuxParmDefs { type_: PARM_WORD, low: 0, high: 0x8000, offset: 0, update: 0 },
        // env2 attack
        EmuxParmDefs { type_: PARM_BYTE_LO, low: 1, high: 0x80, offset: 0, update: 0 },
        // env2 hold
        EmuxParmDefs { type_: PARM_BYTE_HI, low: 0, high: 0x7e, offset: 0, update: 0 },
        // env2 decay
        EmuxParmDefs { type_: PARM_BYTE_LO, low: 1, high: 0x7f, offset: 0, update: 0 },
        // env2 release
        EmuxParmDefs { type_: PARM_BYTE_LO, low: 1, high: 0x7f, offset: 0, update: 0 },
        // env2 sustain
        EmuxParmDefs { type_: PARM_BYTE_HI, low: 0, high: 0x7f, offset: 0, update: 0 },

        // lfo1 delay
        EmuxParmDefs { type_: PARM_WORD, low: 0, high: 0x8000, offset: 0, update: 0 },
        // lfo1 freq
        EmuxParmDefs { type_: PARM_BYTE_LO, low: 0, high: 0xff, offset: 0, update: 0 /* SNDRV_EMUX_UPDATE_TREMFREQ */ },
        // lfo1 vol
        EmuxParmDefs { type_: PARM_SIGN_HI, low: -128, high: 127, offset: 0, update: 0 /* SNDRV_EMUX_UPDATE_TREMFREQ */ },
        // lfo1 pitch
        EmuxParmDefs { type_: PARM_SIGN_HI, low: -128, high: 127, offset: 0, update: 0 /* SNDRV_EMUX_UPDATE_FMMOD */ },
        // lfo1 cutoff
        EmuxParmDefs { type_: PARM_BYTE_LO, low: 0, high: 0xff, offset: 0, update: 0 /* SNDRV_EMUX_UPDATE_FMMOD */ },

        // lfo2 delay
        EmuxParmDefs { type_: PARM_WORD, low: 0, high: 0x8000, offset: 0, update: 0 },
        // lfo2 freq
        EmuxParmDefs { type_: PARM_BYTE_LO, low: 0, high: 0xff, offset: 0, update: 0 /* SNDRV_EMUX_UPDATE_FM2FRQ2 */ },
        // lfo2 pitch
        EmuxParmDefs { type_: PARM_SIGN_HI, low: -128, high: 127, offset: 0, update: 0 /* SNDRV_EMUX_UPDATE_FM2FRQ2 */ },

        // initial pitch
        EmuxParmDefs { type_: PARM_WORD, low: 0, high: 0xffff, offset: -1, update: 0 /* SNDRV_EMUX_UPDATE_PITCH */ },
        // chorus
        EmuxParmDefs { type_: PARM_BYTE, low: 0, high: 0xff, offset: 0, update: 0 },
        // reverb
        EmuxParmDefs { type_: PARM_BYTE, low: 0, high: 0xff, offset: 0, update: 0 },
        // cutoff
        EmuxParmDefs { type_: PARM_BYTE, low: 0, high: 0xff, offset: 0, update: 0 /* SNDRV_EMUX_UPDATE_VOLUME */ },
        // resonance
        EmuxParmDefs { type_: PARM_BYTE, low: 0, high: 15, offset: 0, update: 0 /* SNDRV_EMUX_UPDATE_Q */ },

        // sample start
        EmuxParmDefs { type_: PARM_WORD, low: 0, high: 0xffff, offset: -1, update: 0 },
        // loop start
        EmuxParmDefs { type_: PARM_WORD, low: 0, high: 0xffff, offset: -1, update: 0 },
        // loop end
        EmuxParmDefs { type_: PARM_WORD, low: 0, high: 0xffff, offset: -1, update: 0 },
        // coarse sample start
        EmuxParmDefs { type_: PARM_WORD, low: 0, high: 0xffff, offset: -1, update: 0 },
        // coarse loop start
        EmuxParmDefs { type_: PARM_WORD, low: 0, high: 0xffff, offset: -1, update: 0 },
        // coarse loop end
        EmuxParmDefs { type_: PARM_WORD, low: 0, high: 0xffff, offset: -1, update: 0 },
        // initial attenuation
        EmuxParmDefs { type_: PARM_BYTE, low: 0, high: 0xff, offset: -1, update: 0 /* SNDRV_EMUX_UPDATE_VOLUME */ },
    ];

    // Set byte effect value
    fn effect_set_byte(
        valp: *mut u8,
        chan: *const c_void,
        type_: usize,
    ) {
        // Requires external access to chan->private (snd_emux_effect_table)
        // and parm_defs[type_] structure
        unsafe {
            // TODO: Retrieve fx = chan->private
            // let fx = ...;
            // let mut effect: i16 = fx->val[type_] as i16;
            //
            // if fx->flag[type_] == EMUX_FX_FLAG_ADD {
            //     if PARM_DEFS[type_].type_ & PARM_IS_SIGNED != 0 {
            //         effect += *(valp as *const i8) as i16;
            //     } else {
            //         effect += *valp as i16;
            //     }
            // }
            // if effect < PARM_DEFS[type_].low as i16 {
            //     effect = PARM_DEFS[type_].low as i16;
            // } else if effect > PARM_DEFS[type_].high as i16 {
            //     effect = PARM_DEFS[type_].high as i16;
            // }
            // *valp = effect as u8;
        }
    }

    // Set word effect value
    fn effect_set_word(
        valp: *mut u16,
        chan: *const c_void,
        type_: usize,
    ) {
        // Requires external access to chan->private (snd_emux_effect_table)
        // and parm_defs[type_] structure
        unsafe {
            // TODO: Retrieve fx = chan->private
            // let fx = ...;
            // let mut effect: i32 = *((&fx->val[type_] as *const _) as *const u16) as i32;
            //
            // if fx->flag[type_] == EMUX_FX_FLAG_ADD {
            //     effect += *valp as i32;
            // }
            // if effect < PARM_DEFS[type_].low {
            //     effect = PARM_DEFS[type_].low;
            // } else if effect > PARM_DEFS[type_].high {
            //     effect = PARM_DEFS[type_].high;
            // }
            // *valp = effect as u16;
        }
    }

    // Address offset
    fn effect_get_offset(
        chan: *const c_void,
        lo: i32,
        hi: i32,
        mode: i32,
    ) -> i32 {
        unsafe {
            let mut addr: i32 = 0;
            // TODO: Retrieve fx = chan->private
            // let fx = ...;
            //
            // if fx->flag[hi as usize] != 0 {
            //     addr = fx->val[hi as usize] as i16 as i32;
            // }
            // addr = addr << 15;
            // if fx->flag[lo as usize] != 0 {
            //     addr += fx->val[lo as usize] as i16 as i32;
            // }
            // if (mode & SNDRV_SFNT_SAMPLE_8BITS) == 0 {
            //     addr /= 2;
            // }
            addr
        }
    }

    #[cfg(feature = "CONFIG_SND_SEQUENCER_OSS")]
    // Change effects - for OSS sequencer compatibility
    pub fn snd_emux_send_effect_oss(
        port: *const c_void,
        chan: *const c_void,
        type_: i32,
        val: i32,
    ) {
        let mut mode;

        if (type_ & 0x40) != 0 {
            mode = 0; // EMUX_FX_FLAG_OFF
        } else if (type_ & 0x80) != 0 {
            mode = 1; // EMUX_FX_FLAG_ADD
        } else {
            mode = 0; // EMUX_FX_FLAG_SET
        }

        snd_emux_send_effect(port, chan, type_ & 0x3f, val, mode);
    }

    // Modify the effect value.
    // if update is necessary, call emu8000_control
    pub fn snd_emux_send_effect(
        port: *const c_void,
        chan: *const c_void,
        type_: i32,
        val: i32,
        mode: i32,
    ) {
        unsafe {
            // TODO: Requires external function definitions and types:
            // - snd_emux_port structure
            // - snd_emux structure
            // - snd_emux_effect_table structure
            // - snd_emux_voice structure
            // - STATE_IS_PLAYING macro
            // - snd_emux_update_channel function
            // - EMUX_NUM_EFFECTS constant
            // - SNDRV_LITTLE_ENDIAN compile-time flag
            //
            // let emu = (*port).emu;
            // let fx = chan.private;
            // if emu.is_null() || fx.is_null() {
            //     return;
            // }
            // if type_ < 0 || type_ >= EMUX_NUM_EFFECTS {
            //     return;
            // }
            //
            // (*fx).val[type_ as usize] = val;
            // (*fx).flag[type_ as usize] = mode;
            //
            // if PARM_DEFS[type_ as usize].update == 0 {
            //     return;
            // }
            // let mut offset = PARM_DEFS[type_ as usize].offset;
            // if offset < 0 {
            //     return;
            // }
            //
            // #[cfg(target_endian = "little")]
            // {
            //     if (PARM_DEFS[type_ as usize].type_ & PARM_IS_ALIGN_HI) != 0 {
            //         offset += 1;
            //     }
            // }
            // #[cfg(not(target_endian = "little"))]
            // {
            //     if (PARM_DEFS[type_ as usize].type_ & PARM_IS_ALIGN_LO) != 0 {
            //         offset += 1;
            //     }
            // }
            //
            // // scoped_guard(spinlock_irqsave, &emu->voice_lock) {
            // //     for i in 0..emu->max_voices {
            // //         let vp = &emu->voices[i];
            // //         if !STATE_IS_PLAYING(vp->state) || vp->chan != chan {
            // //             continue;
            // //         }
            // //         let srcp = (vp->reg.parm as *mut u8).add(offset as usize);
            // //         let origp = (vp->zone->v.parm as *mut u8).add(offset as usize);
            // //         if (PARM_DEFS[i].type_ & PARM_IS_BYTE) != 0 {
            // //             *srcp = *origp;
            // //             effect_set_byte(srcp, chan, type_ as usize);
            // //         } else {
            // //             *(srcp as *mut u16) = *(origp as *const u16);
            // //             effect_set_word(srcp as *mut u16, chan, type_ as usize);
            // //         }
            // //     }
            // // }
            //
            // snd_emux_update_channel(port, chan, PARM_DEFS[type_ as usize].update);
        }
    }

    // Copy wavetable registers to voice table
    pub fn snd_emux_setup_effect(vp: *const c_void) {
        unsafe {
            // TODO: Requires external structure definitions:
            // - snd_emux_voice with chan field
            // - snd_midi_channel with private field
            // - snd_emux_effect_table structure
            // - EMUX_FX_END constant
            // - STATE_IS_PLAYING macro or similar
            //
            // let chan = (*vp).chan;
            // let fx = (*chan).private;
            // if fx.is_null() {
            //     return;
            // }
            //
            // for i in 0..EMUX_FX_END {
            //     if (*fx).flag[i] == 0 {
            //         continue;
            //     }
            //     let mut offset = PARM_DEFS[i].offset;
            //     if offset < 0 {
            //         continue;
            //     }
            //     #[cfg(target_endian = "little")]
            //     {
            //         if (PARM_DEFS[i].type_ & PARM_IS_ALIGN_HI) != 0 {
            //             offset += 1;
            //         }
            //     }
            //     #[cfg(not(target_endian = "little"))]
            //     {
            //         if (PARM_DEFS[i].type_ & PARM_IS_ALIGN_LO) != 0 {
            //             offset += 1;
            //         }
            //     }
            //     let srcp = ((*vp).reg.parm as *mut u8).add(offset as usize);
            //     if (PARM_DEFS[i].type_ & PARM_IS_BYTE) != 0 {
            //         effect_set_byte(srcp, chan, i);
            //     } else {
            //         effect_set_word(srcp as *mut u16, chan, i);
            //     }
            // }
            //
            // (*vp).reg.start += effect_get_offset(chan, EMUX_FX_SAMPLE_START,
            //                                       EMUX_FX_COARSE_SAMPLE_START,
            //                                       (*vp).reg.sample_mode);
            //
            // (*vp).reg.loopstart += effect_get_offset(chan, EMUX_FX_LOOP_START,
            //                                           EMUX_FX_COARSE_LOOP_START,
            //                                           (*vp).reg.sample_mode);
            //
            // (*vp).reg.loopend += effect_get_offset(chan, EMUX_FX_LOOP_END,
            //                                         EMUX_FX_COARSE_LOOP_END,
            //                                         (*vp).reg.sample_mode);
        }
    }

    // Effect table - create
    pub fn snd_emux_create_effect(p: *const c_void) {
        unsafe {
            // TODO: Requires external functions and types:
            // - snd_emux_port structure with effect, chset fields
            // - snd_emux_channel_set with max_channels and channels
            // - snd_emux_effect_table type
            // - kzalloc_objs or similar allocation function
            //
            // let effect = kzalloc_objs(struct snd_emux_effect_table, (*p).chset.max_channels);
            // (*p).effect = effect;
            // if !effect.is_null() {
            //     for i in 0..(*p).chset.max_channels {
            //         (*p).chset.channels[i].private = effect.add(i);
            //     }
            // } else {
            //     for i in 0..(*p).chset.max_channels {
            //         (*p).chset.channels[i].private = std::ptr::null_mut();
            //     }
            // }
        }
    }

    pub fn snd_emux_delete_effect(p: *const c_void) {
        unsafe {
            // TODO: Requires external:
            // - snd_emux_port structure with effect field
            // - kfree function
            //
            // kfree((*p).effect);
            // (*p).effect = std::ptr::null_mut();
        }
    }

    pub fn snd_emux_clear_effect(p: *const c_void) {
        unsafe {
            // TODO: Requires external:
            // - snd_emux_port structure
            // - snd_emux_effect_table type and size
            //
            // if !(*p).effect.is_null() {
            //     let size = std::mem::size_of::<snd_emux_effect_table>() * (*p).chset.max_channels;
            //     std::ptr::write_bytes((*p).effect, 0, size);
            // }
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
