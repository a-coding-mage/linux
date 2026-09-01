// SPDX-License-Identifier: GPL-2.0
/*
 * dice-tc_electronic.c - a part of driver for DICE based devices
 *
 * Copyright (c) 2018 Takashi Sakamoto
 */

// Rust translation of implementation depending on declarations from "dice.h".

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
struct dice_tc_spec {
    tx_pcm_chs: [[c_uint; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
    rx_pcm_chs: [[c_uint; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
    has_midi: bool,
}

static DESKTOP_KONNEKT6: dice_tc_spec = dice_tc_spec {
    tx_pcm_chs: [[6, 6, 2], [0, 0, 0]],
    rx_pcm_chs: [[6, 6, 4], [0, 0, 0]],
    has_midi: false,
};

static IMPACT_TWIN: dice_tc_spec = dice_tc_spec {
    tx_pcm_chs: [[14, 10, 6], [0, 0, 0]],
    rx_pcm_chs: [[14, 10, 6], [0, 0, 0]],
    has_midi: true,
};

static KONNEKT_8: dice_tc_spec = dice_tc_spec {
    tx_pcm_chs: [[4, 4, 3], [0, 0, 0]],
    rx_pcm_chs: [[4, 4, 3], [0, 0, 0]],
    has_midi: true,
};

static KONNEKT_24D: dice_tc_spec = dice_tc_spec {
    tx_pcm_chs: [[16, 16, 6], [0, 0, 0]],
    rx_pcm_chs: [[16, 16, 6], [0, 0, 0]],
    has_midi: true,
};

static KONNEKT_LIVE: dice_tc_spec = dice_tc_spec {
    tx_pcm_chs: [[16, 16, 6], [0, 0, 0]],
    rx_pcm_chs: [[16, 16, 6], [0, 0, 0]],
    has_midi: true,
};

static STUDIO_KONNEKT_48: dice_tc_spec = dice_tc_spec {
    tx_pcm_chs: [[16, 16, 8], [16, 16, 7]],
    rx_pcm_chs: [[16, 16, 8], [14, 14, 7]],
    has_midi: true,
};

static DIGITAL_KONNEKT_X32: dice_tc_spec = dice_tc_spec {
    tx_pcm_chs: [[16, 16, 4], [0, 0, 0]],
    rx_pcm_chs: [[16, 16, 4], [0, 0, 0]],
    has_midi: false,
};

#[repr(C)]
struct dice_tc_entry {
    model_id: u32,
    spec: *const dice_tc_spec,
}

pub unsafe fn snd_dice_detect_tcelectronic_formats(dice: *mut snd_dice) -> c_int {
    static ENTRIES: [dice_tc_entry; 7] = [
        dice_tc_entry {
            model_id: 0x00000020,
            spec: &KONNEKT_24D,
        },
        dice_tc_entry {
            model_id: 0x00000021,
            spec: &KONNEKT_8,
        },
        dice_tc_entry {
            model_id: 0x00000022,
            spec: &STUDIO_KONNEKT_48,
        },
        dice_tc_entry {
            model_id: 0x00000023,
            spec: &KONNEKT_LIVE,
        },
        dice_tc_entry {
            model_id: 0x00000024,
            spec: &DESKTOP_KONNEKT6,
        },
        dice_tc_entry {
            model_id: 0x00000027,
            spec: &IMPACT_TWIN,
        },
        dice_tc_entry {
            model_id: 0x00000030,
            spec: &DIGITAL_KONNEKT_X32,
        },
    ];
    let mut entry: *const dice_tc_entry;
    let mut it: fw_csr_iterator = core::mem::zeroed();
    let mut key: c_int = 0;
    let mut val: c_int = 0;
    let mut model_id: c_int;
    let mut i: usize;

    model_id = 0;
    fw_csr_iterator_init(&mut it, (*(*dice).unit).directory);
    while fw_csr_iterator_next(&mut it, &mut key, &mut val) {
        if key == CSR_MODEL {
            model_id = val;
            break;
        }
    }

    i = 0;
    while i < ENTRIES.len() {
        entry = ENTRIES.as_ptr().add(i);
        if (*entry).model_id == model_id as u32 {
            break;
        }
        i += 1;
    }
    if i == ENTRIES.len() {
        return -ENODEV;
    }

    entry = ENTRIES.as_ptr().add(i);
    core::ptr::copy_nonoverlapping(
        (*(*entry).spec).tx_pcm_chs.as_ptr() as *const u8,
        (*dice).tx_pcm_chs.as_mut_ptr() as *mut u8,
        MAX_STREAMS * SND_DICE_RATE_MODE_COUNT * core::mem::size_of::<c_uint>(),
    );
    core::ptr::copy_nonoverlapping(
        (*(*entry).spec).rx_pcm_chs.as_ptr() as *const u8,
        (*dice).rx_pcm_chs.as_mut_ptr() as *mut u8,
        MAX_STREAMS * SND_DICE_RATE_MODE_COUNT * core::mem::size_of::<c_uint>(),
    );

    if (*(*entry).spec).has_midi {
        (*dice).tx_midi_ports[0] = 1;
        (*dice).rx_midi_ports[0] = 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
