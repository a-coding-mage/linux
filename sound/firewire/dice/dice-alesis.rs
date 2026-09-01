// SPDX-License-Identifier: GPL-2.0
/*
 * dice-alesis.c - a part of driver for DICE based devices
 *
 * Copyright (c) 2018 Takashi Sakamoto
 */

// Translated from C. Symbols provided by dice.h are expected to be supplied by
// the surrounding Rust translation unit.

use core::ffi::{c_int, c_uint};
use core::mem::size_of;
use core::ptr;

static ALESIS_IO14_TX_PCM_CHS: [[c_uint; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS] = [
    [6, 6, 4], /* Tx0 = Analog + S/PDIF. */
    [8, 4, 0], /* Tx1 = ADAT1. */
];

static ALESIS_IO26_TX_PCM_CHS: [[c_uint; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS] = [
    [10, 10, 4], /* Tx0 = Analog + S/PDIF. */
    [16, 4, 0],  /* Tx1 = ADAT1 + ADAT2 (available at low rate). */
];

unsafe extern "C" {
    fn snd_dice_transaction_read_tx(
        dice: *mut snd_dice,
        offset: c_uint,
        buf: *mut __be32,
        len: usize,
    ) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_detect_alesis_formats(dice: *mut snd_dice) -> c_int {
    let mut reg: __be32 = 0;
    let data: u32;
    let mut i: c_int;
    let err: c_int;

    err = snd_dice_transaction_read_tx(
        dice,
        TX_NUMBER_AUDIO,
        &mut reg,
        size_of::<__be32>(),
    );
    if err < 0 {
        return err;
    }
    data = u32::from_be(reg);

    if data == 4 || data == 6 {
        ptr::copy_nonoverlapping(
            ALESIS_IO14_TX_PCM_CHS.as_ptr() as *const c_uint,
            (*dice).tx_pcm_chs.as_mut_ptr() as *mut c_uint,
            MAX_STREAMS * SND_DICE_RATE_MODE_COUNT,
        );
    } else {
        ptr::copy_nonoverlapping(
            ALESIS_IO26_TX_PCM_CHS.as_ptr() as *const c_uint,
            (*dice).tx_pcm_chs.as_mut_ptr() as *mut c_uint,
            MAX_STREAMS * SND_DICE_RATE_MODE_COUNT,
        );
    }

    i = 0;
    while i < SND_DICE_RATE_MODE_COUNT as c_int {
        (*dice).rx_pcm_chs[0][i as usize] = 8;
        i += 1;
    }

    (*dice).tx_midi_ports[0] = 1;
    (*dice).rx_midi_ports[0] = 1;

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_detect_alesis_mastercontrol_formats(
    dice: *mut snd_dice,
) -> c_int {
    let mut i: c_int;

    (*dice).tx_pcm_chs[0][SND_DICE_RATE_MODE_LOW] = 16;
    (*dice).tx_pcm_chs[1][SND_DICE_RATE_MODE_LOW] = 12;
    (*dice).tx_pcm_chs[0][SND_DICE_RATE_MODE_MIDDLE] = 12;
    (*dice).tx_pcm_chs[1][SND_DICE_RATE_MODE_MIDDLE] = 4;
    (*dice).tx_pcm_chs[0][SND_DICE_RATE_MODE_HIGH] = 8;
    (*dice).tx_pcm_chs[1][SND_DICE_RATE_MODE_HIGH] = 0;

    i = 0;
    while i < SND_DICE_RATE_MODE_COUNT as c_int {
        (*dice).rx_pcm_chs[0][i as usize] = 6;
        (*dice).rx_pcm_chs[1][i as usize] = 0;
        i += 1;
    }

    i = 0;
    while i < MAX_STREAMS as c_int {
        (*dice).tx_midi_ports[i as usize] = 2;
        (*dice).rx_midi_ports[i as usize] = 2;
        i += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
