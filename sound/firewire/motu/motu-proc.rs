// SPDX-License-Identifier: GPL-2.0-only
/*
 * motu-proc.c - a part of driver for MOTU FireWire series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// Translated from C. Declarations and definitions originally supplied by
// "./motu.h" are expected to be supplied by the surrounding Rust translation.

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static snd_motu_clock_rates: [c_uint; SND_MOTU_CLOCK_RATE_COUNT as usize];

    fn snd_motu_protocol_get_clock_rate(motu: *mut snd_motu, rate: *mut c_uint) -> c_int;
    fn snd_motu_protocol_get_clock_source(
        motu: *mut snd_motu,
        source: *mut snd_motu_clock_source,
    ) -> c_int;
    fn snd_motu_protocol_cache_packet_formats(motu: *mut snd_motu) -> c_int;

    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...) -> c_int;
    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        root: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(
        entry: *mut snd_info_entry,
        private_data: *mut c_void,
        op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
}

static clock_names: [*const c_char; (SND_MOTU_CLOCK_SOURCE_UNKNOWN + 1) as usize] = {
    let mut names = [core::ptr::null(); (SND_MOTU_CLOCK_SOURCE_UNKNOWN + 1) as usize];
    names[SND_MOTU_CLOCK_SOURCE_INTERNAL as usize] = b"Internal\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_ADAT_ON_DSUB as usize] =
        b"ADAT on Dsub-9pin interface\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT as usize] =
        b"ADAT on optical interface\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_A as usize] =
        b"ADAT on optical interface A\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_B as usize] =
        b"ADAT on optical interface B\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT as usize] =
        b"S/PDIF on optical interface\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_A as usize] =
        b"S/PDIF on optical interface A\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_B as usize] =
        b"S/PDIF on optical interface B\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX as usize] =
        b"S/PDIF on coaxial interface\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR as usize] =
        b"AESEBU on XLR interface\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_WORD_ON_BNC as usize] =
        b"Word clock on BNC interface\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_SPH as usize] =
        b"Source packet header\0".as_ptr() as *const c_char;
    names[SND_MOTU_CLOCK_SOURCE_UNKNOWN as usize] = b"Unknown\0".as_ptr() as *const c_char;
    names
};

unsafe extern "C" fn proc_read_clock(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let motu = (*entry).private_data as *mut snd_motu;
    let mut rate: c_uint = 0;
    let mut source: snd_motu_clock_source = 0;

    if snd_motu_protocol_get_clock_rate(motu, &mut rate) < 0 {
        return;
    }
    if snd_motu_protocol_get_clock_source(motu, &mut source) < 0 {
        return;
    }

    snd_iprintf(buffer, b"Rate:\t%d\n\0".as_ptr() as *const c_char, rate);
    snd_iprintf(
        buffer,
        b"Source:\t%s\n\0".as_ptr() as *const c_char,
        clock_names[source as usize],
    );
}

unsafe extern "C" fn proc_read_format(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let motu = (*entry).private_data as *mut snd_motu;
    let mut mode: c_uint;
    let mut formats: *mut snd_motu_packet_format;
    let mut i: c_int;

    if snd_motu_protocol_cache_packet_formats(motu) < 0 {
        return;
    }

    snd_iprintf(buffer, b"tx:\tmsg\tfixed\ttotal\n\0".as_ptr() as *const c_char);
    i = 0;
    while i < SND_MOTU_CLOCK_RATE_COUNT as c_int {
        mode = (i >> 1) as c_uint;

        formats = &mut (*motu).tx_packet_formats;
        snd_iprintf(
            buffer,
            b"%u:\t%u\t%u\t%u\n\0".as_ptr() as *const c_char,
            snd_motu_clock_rates[i as usize],
            (*formats).msg_chunks,
            (*(*motu).spec).tx_fixed_pcm_chunks[mode as usize],
            (*formats).pcm_chunks[mode as usize],
        );

        i += 1;
    }

    snd_iprintf(buffer, b"rx:\tmsg\tfixed\ttotal\n\0".as_ptr() as *const c_char);
    i = 0;
    while i < SND_MOTU_CLOCK_RATE_COUNT as c_int {
        mode = (i >> 1) as c_uint;

        formats = &mut (*motu).rx_packet_formats;
        snd_iprintf(
            buffer,
            b"%u:\t%u\t%u\t%u\n\0".as_ptr() as *const c_char,
            snd_motu_clock_rates[i as usize],
            (*formats).msg_chunks,
            (*(*motu).spec).rx_fixed_pcm_chunks[mode as usize],
            (*formats).pcm_chunks[mode as usize],
        );

        i += 1;
    }
}

unsafe fn add_node(
    motu: *mut snd_motu,
    root: *mut snd_info_entry,
    name: *const c_char,
    op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
) {
    let entry: *mut snd_info_entry;

    entry = snd_info_create_card_entry((*motu).card, name, root);
    if !entry.is_null() {
        snd_info_set_text_ops(entry, motu as *mut c_void, op);
    }
}

pub unsafe extern "C" fn snd_motu_proc_init(motu: *mut snd_motu) {
    let root: *mut snd_info_entry;

    /*
     * All nodes are automatically removed at snd_card_disconnect(),
     * by following to link list.
     */
    root = snd_info_create_card_entry(
        (*motu).card,
        b"firewire\0".as_ptr() as *const c_char,
        (*(*motu).card).proc_root,
    );
    if root.is_null() {
        return;
    }
    (*root).mode = S_IFDIR | 0o555;

    add_node(
        motu,
        root,
        b"clock\0".as_ptr() as *const c_char,
        Some(proc_read_clock),
    );
    add_node(
        motu,
        root,
        b"format\0".as_ptr() as *const c_char,
        Some(proc_read_format),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
