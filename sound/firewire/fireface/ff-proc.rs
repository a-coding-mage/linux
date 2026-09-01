// SPDX-License-Identifier: GPL-2.0-only
/*
 * ff-proc.c - a part of driver for RME Fireface series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto
 */

// Dependencies from "./ff.h" and the Linux kernel proc/info APIs are expected
// to be supplied by the surrounding translation unit.

use core::ffi::{c_char, c_uint};
use core::ptr;

extern "C" {
    static SND_FF_PROC_NODE_FIREWIRE: c_char;
    static SND_FF_PROC_NODE_STATUS: c_char;

    static S_IFDIR: c_uint;

    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        root: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(
        entry: *mut snd_info_entry,
        private: *mut snd_ff,
        op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
}

#[repr(C)]
pub struct snd_ff_clock_src {
    _private: c_uint,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub proc_root: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut snd_ff,
    pub mode: c_uint,
}

#[repr(C)]
pub struct snd_ff_protocol {
    pub dump_status: unsafe extern "C" fn(*mut snd_ff, *mut snd_info_buffer),
}

#[repr(C)]
pub struct snd_ff_spec {
    pub protocol: *mut snd_ff_protocol,
}

#[repr(C)]
pub struct snd_ff {
    pub card: *mut snd_card,
    pub spec: *mut snd_ff_spec,
}

#[no_mangle]
pub unsafe extern "C" fn snd_ff_proc_get_clk_label(src: c_uint) -> *const c_char {
    static LABEL_INTERNAL: &[u8] = b"Internal\0";
    static LABEL_SPDIF: &[u8] = b"S/PDIF\0";
    static LABEL_ADAT1: &[u8] = b"ADAT1\0";
    static LABEL_ADAT2: &[u8] = b"ADAT2\0";
    static LABEL_WORD: &[u8] = b"Word\0";
    static LABEL_LTC: &[u8] = b"LTC\0";

    let labels: [*const c_char; 6] = [
        LABEL_INTERNAL.as_ptr() as *const c_char,
        LABEL_SPDIF.as_ptr() as *const c_char,
        LABEL_ADAT1.as_ptr() as *const c_char,
        LABEL_ADAT2.as_ptr() as *const c_char,
        LABEL_WORD.as_ptr() as *const c_char,
        LABEL_LTC.as_ptr() as *const c_char,
    ];

    if src as usize >= LABELS.len() {
        return ptr::null();
    }

    LABELS[src as usize]
}

unsafe extern "C" fn proc_dump_status(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ff = (*entry).private_data;

    ((*(*(*ff).spec).protocol).dump_status)(ff, buffer);
}

unsafe fn add_node(
    ff: *mut snd_ff,
    root: *mut snd_info_entry,
    name: *const c_char,
    op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
) {
    let entry: *mut snd_info_entry;

    entry = snd_info_create_card_entry((*ff).card, name, root);
    if !entry.is_null() {
        snd_info_set_text_ops(entry, ff, op);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_ff_proc_init(ff: *mut snd_ff) {
    let root: *mut snd_info_entry;

    /*
     * All nodes are automatically removed at snd_card_disconnect(),
     * by following to link list.
     */
    root = snd_info_create_card_entry(
        (*ff).card,
        &SND_FF_PROC_NODE_FIREWIRE as *const c_char,
        (*(*ff).card).proc_root,
    );
    if root.is_null() {
        return;
    }
    (*root).mode = S_IFDIR | 0o555;

    add_node(
        ff,
        root,
        &SND_FF_PROC_NODE_STATUS as *const c_char,
        Some(proc_dump_status),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
