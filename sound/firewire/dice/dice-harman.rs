// SPDX-License-Identifier: GPL-2.0
// dice-harman.c - a part of driver for DICE based devices
//
// Copyright (c) 2021 Takashi Sakamoto

// C dependency: "dice.h".

pub unsafe fn snd_dice_detect_harman_formats(dice: *mut snd_dice) -> i32 {
    let mut i: i32;

    // Lexicon I-ONYX FW810s supports sampling transfer frequency up to
    // 96.0 kHz, 12 PCM channels and 1 MIDI channel in its first tx stream
    // , 10 PCM channels and 1 MIDI channel in its first rx stream for all
    // of the frequencies.
    i = 0;
    while i < 2 {
        (*dice).tx_pcm_chs[0][i as usize] = 12;
        (*dice).tx_midi_ports[0] = 1;
        (*dice).rx_pcm_chs[0][i as usize] = 10;
        (*dice).rx_midi_ports[0] = 1;

        i += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
