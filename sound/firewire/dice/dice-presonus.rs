// SPDX-License-Identifier: GPL-2.0
// dice-presonus.c - a part of driver for DICE based devices
//
// Copyright (c) 2019 Takashi Sakamoto

// Translated from C. Declarations normally supplied by "dice.h" are expected
// to be provided by the surrounding crate/bindings.

use core::ffi::{c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct snd_dice {
    pub unit: *mut fw_unit,
    pub tx_pcm_chs: [[c_uint; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
    pub rx_pcm_chs: [[c_uint; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
    pub tx_midi_ports: [c_uint; MAX_STREAMS],
    pub rx_midi_ports: [c_uint; MAX_STREAMS],
}

#[repr(C)]
pub struct fw_unit {
    pub directory: *const c_void,
}

#[repr(C)]
pub struct fw_csr_iterator {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn fw_csr_iterator_init(it: *mut fw_csr_iterator, directory: *const c_void);
    fn fw_csr_iterator_next(it: *mut fw_csr_iterator, key: *mut c_int, val: *mut c_int) -> bool;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

unsafe extern "Rust" {
    static MAX_STREAMS: usize;
    static SND_DICE_RATE_MODE_COUNT: usize;
    static CSR_MODEL: c_int;
    static ENODEV: c_int;
}

#[repr(C)]
struct dice_presonus_spec {
    tx_pcm_chs: [[c_uint; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
    rx_pcm_chs: [[c_uint; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
    has_midi: bool,
}

static dice_presonus_firesutio: dice_presonus_spec = dice_presonus_spec {
    tx_pcm_chs: [[16, 16, 0], [10, 2, 0]],
    rx_pcm_chs: [[16, 16, 0], [10, 2, 0]],
    has_midi: true,
};

#[repr(C)]
struct snd_dice_detect_presonus_formats_entry {
    model_id: u32,
    spec: *const dice_presonus_spec,
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_detect_presonus_formats(dice: *mut snd_dice) -> c_int {
    static entries: [snd_dice_detect_presonus_formats_entry; 1] = [
        snd_dice_detect_presonus_formats_entry {
            model_id: 0x000008,
            spec: ptr::addr_of!(dice_presonus_firesutio),
        },
    ];
    let mut entry: *const snd_dice_detect_presonus_formats_entry;
    let mut it: fw_csr_iterator = core::mem::zeroed();
    let mut key: c_int = 0;
    let mut val: c_int = 0;
    let mut model_id: c_int;
    let mut i: usize;

    model_id = 0;
    fw_csr_iterator_init(ptr::addr_of_mut!(it), (*(*dice).unit).directory);
    while fw_csr_iterator_next(ptr::addr_of_mut!(it), ptr::addr_of_mut!(key), ptr::addr_of_mut!(val)) {
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
    memcpy(
        (*dice).tx_pcm_chs.as_mut_ptr() as *mut c_void,
        (*(*entry).spec).tx_pcm_chs.as_ptr() as *const c_void,
        MAX_STREAMS * SND_DICE_RATE_MODE_COUNT * size_of::<c_uint>(),
    );
    memcpy(
        (*dice).rx_pcm_chs.as_mut_ptr() as *mut c_void,
        (*(*entry).spec).rx_pcm_chs.as_ptr() as *const c_void,
        MAX_STREAMS * SND_DICE_RATE_MODE_COUNT * size_of::<c_uint>(),
    );

    if (*(*entry).spec).has_midi {
        (*dice).tx_midi_ports[0] = 1;
        (*dice).rx_midi_ports[0] = 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
