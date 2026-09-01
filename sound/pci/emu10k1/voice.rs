// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   Lee Revell <rlrevell@joe-job.com>
 *                   Oswald Buddenhagen <oswald.buddenhagen@gmx.de>
 *                   Creative Labs, Inc.
 *
 *  Routines for control of EMU10K1 chips - voice manager
 */

// C dependencies:
// #include <linux/time.h>
// #include <linux/export.h>
// #include <sound/core.h>
// #include <sound/emu10k1.h>

use core::ptr;

use crate::*;

/* Previously the voice allocator started at 0 every time.  The new voice
 * allocator uses a round robin scheme.  The next free voice is tracked in
 * the card record and each allocation begins where the last left off.  The
 * hardware requires stereo interleaved voices be aligned to an even/odd
 * boundary.
 *							--rlrevell
 */

unsafe fn voice_alloc(
    emu: *mut snd_emu10k1,
    type_: i32,
    number: i32,
    epcm: *mut snd_emu10k1_pcm,
    rvoice: *mut *mut snd_emu10k1_voice,
) -> i32 {
    let mut voice: *mut snd_emu10k1_voice;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut skip: i32;

    i = (*emu).next_free_voice;
    j = 0;
    'next: while j < NUM_G {
        /*
        dev_dbg(emu->card->dev, "i %d j %d next free %d!\n",
               i, j, emu->next_free_voice);
        */

        /* stereo voices must be even/odd */
        if number > 1 && (i % 2) != 0 {
            skip = 1;
            i = (i + skip) % NUM_G;
            j += skip;
            continue;
        }

        k = 0;
        while k < number {
            voice = &mut (*emu).voices[(i + k) as usize];
            if (*voice).r#use != 0 {
                skip = k + 1;
                i = (i + skip) % NUM_G;
                j += skip;
                continue 'next;
            }
            k += 1;
        }

        k = 0;
        while k < number {
            voice = &mut (*emu).voices[(i + k) as usize];
            (*voice).r#use = type_;
            (*voice).epcm = epcm;
            /* dev_dbg(emu->card->dev, "allocated voice %d\n", i + k); */
            k += 1;
        }
        voice = &mut (*emu).voices[(i + number - 1) as usize];
        (*voice).last = 1;

        *rvoice = &mut (*emu).voices[i as usize];
        (*emu).next_free_voice = (i + number) % NUM_G;
        return 0;
    }
    -ENOMEM // -EBUSY would have been better
}

unsafe fn voice_free(emu: *mut snd_emu10k1, mut pvoice: *mut snd_emu10k1_voice) {
    if (*pvoice).dirty != 0 {
        snd_emu10k1_voice_init(emu, (*pvoice).number);
    }
    (*pvoice).interrupt = None;
    (*pvoice).r#use = 0;
    (*pvoice).dirty = 0;
    (*pvoice).last = 0;
    (*pvoice).epcm = ptr::null_mut();
}

pub unsafe extern "C" fn snd_emu10k1_voice_alloc(
    emu: *mut snd_emu10k1,
    type_: i32,
    count: i32,
    channels: i32,
    epcm: *mut snd_emu10k1_pcm,
    rvoice: *mut *mut snd_emu10k1_voice,
) -> i32 {
    let mut result: i32 = 0;

    if snd_BUG_ON(rvoice.is_null()) != 0 {
        return -EINVAL;
    }
    if snd_BUG_ON(count == 0) != 0 {
        return -EINVAL;
    }
    if snd_BUG_ON(channels == 0) != 0 {
        return -EINVAL;
    }

    let _guard = spinlock_irqsave(&mut (*emu).voice_lock);
    let mut got: i32 = 0;
    while got < channels {
        result = voice_alloc(emu, type_, count, epcm, rvoice.add(got as usize));
        if result == 0 {
            got += 1;
            /*
            dev_dbg(emu->card->dev, "voice alloc - %i, %i of %i\n",
                    rvoice[got - 1]->number, got, want);
            */
            continue;
        }
        if type_ != EMU10K1_SYNTH && (*emu).get_synth_voice.is_some() {
            /* free a voice from synth */
            result = ((*emu).get_synth_voice.unwrap())(emu);
            if result >= 0 {
                voice_free(emu, &mut (*emu).voices[result as usize]);
                continue;
            }
        }
        let mut i: i32 = 0;
        while i < got {
            let mut j: i32 = 0;
            while j < count {
                voice_free(emu, (*rvoice.add(i as usize)).add(j as usize));
                j += 1;
            }
            *rvoice.add(i as usize) = ptr::null_mut();
            i += 1;
        }
        break;
    }

    result
}

// EXPORT_SYMBOL(snd_emu10k1_voice_alloc);

pub unsafe extern "C" fn snd_emu10k1_voice_free(
    emu: *mut snd_emu10k1,
    mut pvoice: *mut snd_emu10k1_voice,
) -> i32 {
    let mut last: i32;

    if snd_BUG_ON(pvoice.is_null()) != 0 {
        return -EINVAL;
    }
    let _guard = spinlock_irqsave(&mut (*emu).voice_lock);
    loop {
        last = (*pvoice).last;
        voice_free(emu, pvoice);
        pvoice = pvoice.add(1);
        if last != 0 {
            break;
        }
    }
    0
}

// EXPORT_SYMBOL(snd_emu10k1_voice_free);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
