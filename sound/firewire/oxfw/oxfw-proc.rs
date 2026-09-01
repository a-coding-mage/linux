// SPDX-License-Identifier: GPL-2.0-only
/*
 * oxfw_proc.c - a part of driver for OXFW970/971 based devices
 *
 * Copyright (c) 2014 Takashi Sakamoto
 */

// Rust translation of the implementation originally depending on "./oxfw.h".

use core::ffi::{c_char, c_int, c_uchar, c_void};
use core::mem::{size_of, MaybeUninit};
use core::ptr;

const AVC_GENERAL_PLUG_DIR_IN: c_int = 0;
const AVC_GENERAL_PLUG_DIR_OUT: c_int = 1;
const SND_OXFW_STREAM_FORMAT_ENTRIES: usize = 10;
const S_IFDIR: u32 = 0o040000;

#[repr(C)]
pub struct snd_card {
    pub proc_root: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
    pub mode: u32,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_oxfw_stream_formation {
    pub rate: c_int,
    pub pcm: c_int,
    pub midi: c_int,
}

#[repr(C)]
pub struct snd_oxfw {
    pub card: *mut snd_card,
    pub rx_stream_formats: [*mut u8; SND_OXFW_STREAM_FORMAT_ENTRIES],
    pub tx_stream_formats: [*mut u8; SND_OXFW_STREAM_FORMAT_ENTRIES],
    pub has_output: bool,
}

unsafe extern "C" {
    fn snd_oxfw_stream_get_current_formation(
        oxfw: *mut snd_oxfw,
        dir: c_int,
        formation: *mut snd_oxfw_stream_formation,
    ) -> c_int;
    fn snd_oxfw_stream_parse_format(
        format: *mut u8,
        formation: *mut snd_oxfw_stream_formation,
    ) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        root: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(
        entry: *mut snd_info_entry,
        private_data: *mut snd_oxfw,
        op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
}

unsafe extern "C" fn proc_read_formation(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let oxfw = (*entry).private_data as *mut snd_oxfw;
    let mut formation = MaybeUninit::<snd_oxfw_stream_formation>::uninit();
    let mut curr = MaybeUninit::<snd_oxfw_stream_formation>::uninit();
    let mut format: *mut u8;
    let mut flag: c_char;
    let mut i: c_int;
    let mut err: c_int;

    /* Show input. */
    err = snd_oxfw_stream_get_current_formation(
        oxfw,
        AVC_GENERAL_PLUG_DIR_IN,
        curr.as_mut_ptr(),
    );
    if err < 0 {
        return;
    }

    snd_iprintf(buffer, b"Input Stream to device:\n\0".as_ptr() as *const c_char);
    snd_iprintf(buffer, b"\tRate\tPCM\tMIDI\n\0".as_ptr() as *const c_char);
    i = 0;
    while i < SND_OXFW_STREAM_FORMAT_ENTRIES as c_int {
        format = (*oxfw).rx_stream_formats[i as usize];
        if format.is_null() {
            i += 1;
            continue;
        }

        err = snd_oxfw_stream_parse_format(format, formation.as_mut_ptr());
        if err < 0 {
            i += 1;
            continue;
        }

        if memcmp(
            formation.as_ptr() as *const c_void,
            curr.as_ptr() as *const c_void,
            size_of::<snd_oxfw_stream_formation>(),
        ) == 0
        {
            flag = b'*' as c_char;
        } else {
            flag = b' ' as c_char;
        }

        let formation_ref = &*formation.as_ptr();
        snd_iprintf(
            buffer,
            b"%c\t%d\t%d\t%d\n\0".as_ptr() as *const c_char,
            flag as c_int,
            formation_ref.rate,
            formation_ref.pcm,
            formation_ref.midi,
        );
        i += 1;
    }

    if !(*oxfw).has_output {
        return;
    }

    /* Show output. */
    err = snd_oxfw_stream_get_current_formation(
        oxfw,
        AVC_GENERAL_PLUG_DIR_OUT,
        curr.as_mut_ptr(),
    );
    if err < 0 {
        return;
    }

    snd_iprintf(
        buffer,
        b"Output Stream from device:\n\0".as_ptr() as *const c_char,
    );
    snd_iprintf(buffer, b"\tRate\tPCM\tMIDI\n\0".as_ptr() as *const c_char);
    i = 0;
    while i < SND_OXFW_STREAM_FORMAT_ENTRIES as c_int {
        format = (*oxfw).tx_stream_formats[i as usize];
        if format.is_null() {
            i += 1;
            continue;
        }

        err = snd_oxfw_stream_parse_format(format, formation.as_mut_ptr());
        if err < 0 {
            i += 1;
            continue;
        }

        if memcmp(
            formation.as_ptr() as *const c_void,
            curr.as_ptr() as *const c_void,
            size_of::<snd_oxfw_stream_formation>(),
        ) == 0
        {
            flag = b'*' as c_char;
        } else {
            flag = b' ' as c_char;
        }

        let formation_ref = &*formation.as_ptr();
        snd_iprintf(
            buffer,
            b"%c\t%d\t%d\t%d\n\0".as_ptr() as *const c_char,
            flag as c_int,
            formation_ref.rate,
            formation_ref.pcm,
            formation_ref.midi,
        );
        i += 1;
    }
}

unsafe extern "C" fn add_node(
    oxfw: *mut snd_oxfw,
    root: *mut snd_info_entry,
    name: *const c_char,
    op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
) {
    let entry: *mut snd_info_entry;

    entry = snd_info_create_card_entry((*oxfw).card, name, root);
    if !entry.is_null() {
        snd_info_set_text_ops(entry, oxfw, op);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_oxfw_proc_init(oxfw: *mut snd_oxfw) {
    let root: *mut snd_info_entry;

    /*
     * All nodes are automatically removed at snd_card_disconnect(),
     * by following to link list.
     */
    root = snd_info_create_card_entry(
        (*oxfw).card,
        b"firewire\0".as_ptr() as *const c_char,
        (*(*oxfw).card).proc_root,
    );
    if root.is_null() {
        return;
    }
    (*root).mode = S_IFDIR | 0o555;

    add_node(
        oxfw,
        root,
        b"formation\0".as_ptr() as *const c_char,
        Some(proc_read_formation),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
