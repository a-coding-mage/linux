// SPDX-License-Identifier: GPL-2.0-only
/*
 * digi00x-proc.c - a part of driver for Digidesign Digi 002/003 family
 *
 * Copyright (c) 2014-2015 Takashi Sakamoto
 */

// Rust translation of the original C implementation. External types,
// constants, and functions are supplied by the surrounding driver/kernel code.

use core::ffi::{c_char, c_int, c_uint, c_void};

type __be32 = u32;

#[repr(C)]
pub struct snd_dg00x {
    pub unit: *mut c_void,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub proc_root: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
    pub mode: c_uint,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

type snd_dg00x_optical_mode = c_uint;
type snd_dg00x_clock = c_uint;

const SND_DG00X_CLOCK_INTERNAL: snd_dg00x_clock = 0;
const SND_DG00X_CLOCK_SPDIF: snd_dg00x_clock = 1;
const SND_DG00X_CLOCK_ADAT: snd_dg00x_clock = 2;
const SND_DG00X_CLOCK_WORD: snd_dg00x_clock = 3;

const SND_DG00X_OPT_IFACE_MODE_ADAT: snd_dg00x_optical_mode = 0;
const SND_DG00X_OPT_IFACE_MODE_SPDIF: snd_dg00x_optical_mode = 1;

extern "C" {
    static TCODE_READ_QUADLET_REQUEST: c_int;
    static DG00X_ADDR_BASE: u64;
    static DG00X_OFFSET_OPT_IFACE_MODE: u64;
    static S_IFDIR: c_uint;

    fn snd_fw_transaction(
        unit: *mut c_void,
        tcode: c_int,
        offset: u64,
        buffer: *mut c_void,
        length: usize,
        flags: c_uint,
    ) -> c_int;
    fn be32_to_cpu(data: __be32) -> u32;
    fn snd_dg00x_stream_get_local_rate(dg00x: *mut snd_dg00x, rate: *mut c_uint) -> c_int;
    fn snd_dg00x_stream_get_clock(dg00x: *mut snd_dg00x, clock: *mut snd_dg00x_clock) -> c_int;
    fn snd_dg00x_stream_check_external_clock(dg00x: *mut snd_dg00x, detect: *mut bool) -> c_int;
    fn snd_dg00x_stream_get_external_rate(dg00x: *mut snd_dg00x, rate: *mut c_uint) -> c_int;
    fn snd_iprintf(buf: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        parent: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(
        entry: *mut snd_info_entry,
        private_data: *mut snd_dg00x,
        read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer),
    );
}

unsafe fn get_optical_iface_mode(
    dg00x: *mut snd_dg00x,
    mode: *mut snd_dg00x_optical_mode,
) -> c_int {
    let mut data: __be32 = 0;
    let err: c_int;

    err = snd_fw_transaction(
        (*dg00x).unit,
        TCODE_READ_QUADLET_REQUEST,
        DG00X_ADDR_BASE + DG00X_OFFSET_OPT_IFACE_MODE,
        &mut data as *mut __be32 as *mut c_void,
        core::mem::size_of_val(&data),
        0,
    );
    if err >= 0 {
        *mode = (be32_to_cpu(data) & 0x01) as snd_dg00x_optical_mode;
    }

    err
}

unsafe extern "C" fn proc_read_clock(entry: *mut snd_info_entry, buf: *mut snd_info_buffer) {
    let source_name: [*const c_char; 4] = [
        b"internal\0".as_ptr() as *const c_char,
        b"s/pdif\0".as_ptr() as *const c_char,
        b"adat\0".as_ptr() as *const c_char,
        b"word clock\0".as_ptr() as *const c_char,
    ];
    let optical_name: [*const c_char; 2] = [
        b"adat\0".as_ptr() as *const c_char,
        b"s/pdif\0".as_ptr() as *const c_char,
    ];
    let dg00x: *mut snd_dg00x = (*entry).private_data as *mut snd_dg00x;
    let mut mode: snd_dg00x_optical_mode = 0;
    let mut rate: c_uint = 0;
    let mut clock: snd_dg00x_clock = 0;
    let mut detect: bool = false;

    if get_optical_iface_mode(dg00x, &mut mode) < 0 {
        return;
    }
    if snd_dg00x_stream_get_local_rate(dg00x, &mut rate) < 0 {
        return;
    }
    if snd_dg00x_stream_get_clock(dg00x, &mut clock) < 0 {
        return;
    }

    snd_iprintf(
        buf,
        b"Optical mode: %s\n\0".as_ptr() as *const c_char,
        optical_name[mode as usize],
    );
    snd_iprintf(
        buf,
        b"Sampling Rate: %d\n\0".as_ptr() as *const c_char,
        rate,
    );
    snd_iprintf(
        buf,
        b"Clock Source: %s\n\0".as_ptr() as *const c_char,
        source_name[clock as usize],
    );

    if clock == SND_DG00X_CLOCK_INTERNAL {
        return;
    }

    if snd_dg00x_stream_check_external_clock(dg00x, &mut detect) < 0 {
        return;
    }
    snd_iprintf(
        buf,
        b"External source: %s\n\0".as_ptr() as *const c_char,
        if detect {
            b"detected\0".as_ptr() as *const c_char
        } else {
            b"not\0".as_ptr() as *const c_char
        },
    );
    if !detect {
        return;
    }

    if snd_dg00x_stream_get_external_rate(dg00x, &mut rate) >= 0 {
        snd_iprintf(
            buf,
            b"External sampling rate: %d\n\0".as_ptr() as *const c_char,
            rate,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_dg00x_proc_init(dg00x: *mut snd_dg00x) {
    let mut root: *mut snd_info_entry;
    let entry: *mut snd_info_entry;

    /*
     * All nodes are automatically removed at snd_card_disconnect(),
     * by following to link list.
     */
    root = snd_info_create_card_entry(
        (*dg00x).card,
        b"firewire\0".as_ptr() as *const c_char,
        (*(*dg00x).card).proc_root,
    );
    if root.is_null() {
        return;
    }

    (*root).mode = S_IFDIR | 0o555;

    entry = snd_info_create_card_entry(
        (*dg00x).card,
        b"clock\0".as_ptr() as *const c_char,
        root,
    );
    if !entry.is_null() {
        snd_info_set_text_ops(entry, dg00x, proc_read_clock);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
