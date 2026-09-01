// SPDX-License-Identifier: GPL-2.0
/*
 * dice-mytek.c - a part of driver for DICE based devices
 *
 * Copyright (c) 2018 Melvin Vermeeren
 */

// Depends on declarations from "dice.h".

#[repr(C)]
struct dice_mytek_spec {
    tx_pcm_chs: [[u32; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
    rx_pcm_chs: [[u32; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
}

static stereo_192_dsd_dac: dice_mytek_spec = dice_mytek_spec {
    /* AES, TOSLINK, SPDIF, ADAT inputs on device */
    tx_pcm_chs: [[8, 8, 8], [0, 0, 0]],
    /* PCM 44.1-192, native DSD64/DSD128 to device */
    rx_pcm_chs: [[4, 4, 4], [0, 0, 0]],
};

/*
 * Mytek has a few other firewire-capable devices, though newer models appear
 * to lack the port more often than not. As I don't have access to any of them
 * they are missing here. An example is the Mytek 8x192 ADDA, which is DICE.
 */

#[no_mangle]
pub unsafe extern "C" fn snd_dice_detect_mytek_formats(dice: *mut snd_dice) -> i32 {
    let mut i: i32;
    let dev: *const dice_mytek_spec;

    dev = &stereo_192_dsd_dac;

    core::ptr::copy_nonoverlapping(
        (*dev).tx_pcm_chs.as_ptr() as *const u8,
        (*dice).tx_pcm_chs.as_mut_ptr() as *mut u8,
        MAX_STREAMS * SND_DICE_RATE_MODE_COUNT * core::mem::size_of::<u32>(),
    );
    core::ptr::copy_nonoverlapping(
        (*dev).rx_pcm_chs.as_ptr() as *const u8,
        (*dice).rx_pcm_chs.as_mut_ptr() as *mut u8,
        MAX_STREAMS * SND_DICE_RATE_MODE_COUNT * core::mem::size_of::<u32>(),
    );

    i = 0;
    while i < MAX_STREAMS as i32 {
        (*dice).tx_midi_ports[i as usize] = 0;
        (*dice).rx_midi_ports[i as usize] = 0;
        i += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
