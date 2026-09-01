// SPDX-License-Identifier: GPL-2.0
// dice-weiss.c - a part of driver for DICE based devices
//
// Copyright (c) 2023 Rolf Anderegg and Michele Perrone

// Rust translation of the implementation that depends on declarations from
// "dice.h" in the original C source.

use core::ffi::{c_int, c_uint};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
struct dice_weiss_spec {
    tx_pcm_chs: [[c_uint; SND_DICE_RATE_MODE_COUNT as usize]; MAX_STREAMS as usize],
    rx_pcm_chs: [[c_uint; SND_DICE_RATE_MODE_COUNT as usize]; MAX_STREAMS as usize],
}

// Weiss DAC202: 192kHz 2-channel DAC
static dac202: dice_weiss_spec = dice_weiss_spec {
    tx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
    rx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
};

// Weiss MAN301: 192kHz 2-channel music archive network player
static man301: dice_weiss_spec = dice_weiss_spec {
    tx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
    rx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
};

// Weiss INT202: 192kHz unidirectional 2-channel digital Firewire nterface
static int202: dice_weiss_spec = dice_weiss_spec {
    tx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
    rx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
};

// Weiss INT203: 192kHz bidirectional 2-channel digital Firewire nterface
static int203: dice_weiss_spec = dice_weiss_spec {
    tx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
    rx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
};

// Weiss ADC2: 192kHz A/D converter with microphone preamps and line nputs
static adc2: dice_weiss_spec = dice_weiss_spec {
    tx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
    rx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
};

// Weiss DAC2/Minerva: 192kHz 2-channel DAC
static dac2_minerva: dice_weiss_spec = dice_weiss_spec {
    tx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
    rx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
};

// Weiss Vesta: 192kHz 2-channel Firewire to AES/EBU interface
static vesta: dice_weiss_spec = dice_weiss_spec {
    tx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
    rx_pcm_chs: [[2, 2, 2], [0, 0, 0]],
};

// Weiss AFI1: 192kHz 24-channel Firewire to ADAT or AES/EBU interface
static afi1: dice_weiss_spec = dice_weiss_spec {
    tx_pcm_chs: [[24, 16, 8], [0, 0, 0]],
    rx_pcm_chs: [[24, 16, 8], [0, 0, 0]],
};

#[repr(C)]
struct snd_dice_detect_weiss_formats_entry {
    model_id: u32,
    spec: *const dice_weiss_spec,
}

extern "C" {
    fn fw_csr_iterator_init(it: *mut fw_csr_iterator, directory: *const core::ffi::c_void);
    fn fw_csr_iterator_next(it: *mut fw_csr_iterator, key: *mut c_int, val: *mut c_int) -> bool;
}

pub unsafe extern "C" fn snd_dice_detect_weiss_formats(dice: *mut snd_dice) -> c_int {
    static entries: [snd_dice_detect_weiss_formats_entry; 9] = [
        snd_dice_detect_weiss_formats_entry {
            model_id: 0x000007,
            spec: &dac202,
        },
        snd_dice_detect_weiss_formats_entry {
            model_id: 0x000008,
            spec: &dac202,
        }, // Maya edition: same audio I/O as DAC202.
        snd_dice_detect_weiss_formats_entry {
            model_id: 0x000006,
            spec: &int202,
        },
        snd_dice_detect_weiss_formats_entry {
            model_id: 0x00000a,
            spec: &int203,
        },
        snd_dice_detect_weiss_formats_entry {
            model_id: 0x00000b,
            spec: &man301,
        },
        snd_dice_detect_weiss_formats_entry {
            model_id: 0x000001,
            spec: &adc2,
        },
        snd_dice_detect_weiss_formats_entry {
            model_id: 0x000003,
            spec: &dac2_minerva,
        },
        snd_dice_detect_weiss_formats_entry {
            model_id: 0x000002,
            spec: &vesta,
        },
        snd_dice_detect_weiss_formats_entry {
            model_id: 0x000004,
            spec: &afi1,
        },
    ];
    let mut entry: *const snd_dice_detect_weiss_formats_entry;
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
    while i < entries.len() {
        entry = entries.as_ptr().add(i);
        if (*entry).model_id == model_id as u32 {
            break;
        }
        i += 1;
    }
    if i == entries.len() {
        return -ENODEV;
    }

    entry = entries.as_ptr().add(i);
    ptr::copy_nonoverlapping(
        (*(*entry).spec).tx_pcm_chs.as_ptr() as *const c_uint,
        (*dice).tx_pcm_chs.as_mut_ptr() as *mut c_uint,
        MAX_STREAMS as usize * SND_DICE_RATE_MODE_COUNT as usize,
    );
    ptr::copy_nonoverlapping(
        (*(*entry).spec).rx_pcm_chs.as_ptr() as *const c_uint,
        (*dice).rx_pcm_chs.as_mut_ptr() as *mut c_uint,
        MAX_STREAMS as usize * SND_DICE_RATE_MODE_COUNT as usize,
    );

    let _ = size_of::<c_uint>();

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
