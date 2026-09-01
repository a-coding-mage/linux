// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Advanced Linux Sound Architecture
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

use core::ffi::{c_char, c_int};

// Dependencies from <linux/init.h> and <sound/core.h>.
extern "C" {
    static SNDRV_CARDS: c_int;
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn snd_card_ref(idx: c_int) -> *mut snd_card;
    fn snd_card_unref(card: *mut snd_card);
}

#[repr(C)]
pub struct snd_card {
    pub longname: *const c_char,
}

unsafe extern "C" fn alsa_sound_last_init() -> c_int {
    let mut card: *mut snd_card;
    let mut idx: c_int;
    let mut ok: c_int = 0;

    // KERN_INFO log-level string prefix from the C source is dependency-provided.
    printk(b"ALSA device list:\n\0".as_ptr() as *const c_char);
    idx = 0;
    while idx < SNDRV_CARDS {
        card = snd_card_ref(idx);
        if !card.is_null() {
            printk(b"  #%i: %s\n\0".as_ptr() as *const c_char, idx, (*card).longname);
            snd_card_unref(card);
            ok += 1;
        }
        idx += 1;
    }
    if ok == 0 {
        printk(b"  No soundcards found.\n\0".as_ptr() as *const c_char);
    }
    0
}

// C registration macro preserved from source:
// late_initcall_sync(alsa_sound_last_init);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
