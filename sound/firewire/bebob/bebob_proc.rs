// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob_proc.c - a part of driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// C dependency: ./bebob.h

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = u32;
type u64 = u64;

const GFP_KERNEL: c_uint = 0;
const S_IFDIR: c_uint = 0o040000;

extern "C" {
    static snd_bebob_rate_table: [c_uint; SND_BEBOB_STRM_FMT_ENTRIES];

    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);

    fn snd_bebob_read_block(
        unit: *mut c_void,
        addr: u64,
        buf: *mut c_void,
        size: usize,
    ) -> c_int;
    fn snd_bebob_stream_get_clock_src(
        bebob: *mut snd_bebob,
        src: *mut snd_bebob_clock_type,
    ) -> c_int;

    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        root: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(
        entry: *mut snd_info_entry,
        private_data: *mut snd_bebob,
        op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...) -> c_int;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

extern "C" {
    type snd_card;
    type snd_info_buffer;
    type snd_info_entry;
    type snd_bebob;
    type snd_bebob_meter_spec;
    type snd_bebob_rate_spec;
    type snd_bebob_clock_spec;
    type snd_bebob_stream_formation;
}

type snd_bebob_clock_type = c_uint;

extern "C" {
    static SND_BEBOB_STRM_FMT_ENTRIES: usize;
}

/* contents of information register */
#[repr(C, packed)]
struct hw_info {
    manufacturer: u64,
    protocol_ver: u32,
    bld_ver: u32,
    guid: [u32; 2],
    model_id: u32,
    model_rev: u32,
    fw_date: u64,
    fw_time: u64,
    fw_id: u32,
    fw_ver: u32,
    base_addr: u32,
    max_size: u32,
    bld_date: u64,
    bld_time: u64,
    /* may not used in product
    dbg_date: u64,
    dbg_time: u64,
    dbg_id: u32,
    dbg_version: u32,
    */
}

unsafe extern "C" fn proc_read_hw_info(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let bebob: *mut snd_bebob = (*entry).private_data as *mut snd_bebob;
    let info: *mut hw_info;

    info = kmalloc(core::mem::size_of::<hw_info>(), GFP_KERNEL) as *mut hw_info;
    if info.is_null() {
        return;
    }
    core::ptr::write_bytes(info as *mut u8, 0, core::mem::size_of::<hw_info>());

    if snd_bebob_read_block(
        (*bebob).unit,
        0,
        info as *mut c_void,
        core::mem::size_of::<hw_info>(),
    ) < 0
    {
        kfree(info as *const c_void);
        return;
    }

    snd_iprintf(
        buffer,
        b"Manufacturer:\t%.8s\n\0".as_ptr() as *const c_char,
        &(*info).manufacturer as *const u64 as *const c_char,
    );
    snd_iprintf(
        buffer,
        b"Protocol Ver:\t%d\n\0".as_ptr() as *const c_char,
        (*info).protocol_ver,
    );
    snd_iprintf(
        buffer,
        b"Build Ver:\t%d\n\0".as_ptr() as *const c_char,
        (*info).bld_ver,
    );
    snd_iprintf(
        buffer,
        b"GUID:\t\t0x%.8X%.8X\n\0".as_ptr() as *const c_char,
        (*info).guid[0],
        (*info).guid[1],
    );
    snd_iprintf(
        buffer,
        b"Model ID:\t0x%02X\n\0".as_ptr() as *const c_char,
        (*info).model_id,
    );
    snd_iprintf(
        buffer,
        b"Model Rev:\t%d\n\0".as_ptr() as *const c_char,
        (*info).model_rev,
    );
    snd_iprintf(
        buffer,
        b"Firmware Date:\t%.8s\n\0".as_ptr() as *const c_char,
        &(*info).fw_date as *const u64 as *const c_char,
    );
    snd_iprintf(
        buffer,
        b"Firmware Time:\t%.8s\n\0".as_ptr() as *const c_char,
        &(*info).fw_time as *const u64 as *const c_char,
    );
    snd_iprintf(
        buffer,
        b"Firmware ID:\t0x%X\n\0".as_ptr() as *const c_char,
        (*info).fw_id,
    );
    snd_iprintf(
        buffer,
        b"Firmware Ver:\t%d\n\0".as_ptr() as *const c_char,
        (*info).fw_ver,
    );
    snd_iprintf(
        buffer,
        b"Base Addr:\t0x%X\n\0".as_ptr() as *const c_char,
        (*info).base_addr,
    );
    snd_iprintf(
        buffer,
        b"Max Size:\t%d\n\0".as_ptr() as *const c_char,
        (*info).max_size,
    );
    snd_iprintf(
        buffer,
        b"Loader Date:\t%.8s\n\0".as_ptr() as *const c_char,
        &(*info).bld_date as *const u64 as *const c_char,
    );
    snd_iprintf(
        buffer,
        b"Loader Time:\t%.8s\n\0".as_ptr() as *const c_char,
        &(*info).bld_time as *const u64 as *const c_char,
    );

    kfree(info as *const c_void);
}

unsafe extern "C" fn proc_read_meters(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let bebob: *mut snd_bebob = (*entry).private_data as *mut snd_bebob;
    let spec: *const snd_bebob_meter_spec = (*(*bebob).spec).meter;
    let buf: *mut u32;
    let mut i: c_uint;
    let mut c: c_uint;
    let channels: c_uint;
    let size: c_uint;

    if spec.is_null() {
        return;
    }

    channels = (*spec).num * 2;
    size = channels * core::mem::size_of::<u32>() as c_uint;
    buf = kmalloc(size as usize, GFP_KERNEL) as *mut u32;
    if buf.is_null() {
        return;
    }

    if ((*spec).get.unwrap())(bebob, buf, size) < 0 {
        kfree(buf as *const c_void);
        return;
    }

    i = 0;
    c = 1;
    while i < channels {
        snd_iprintf(
            buffer,
            b"%s %d:\t%d\n\0".as_ptr() as *const c_char,
            *(*spec).labels.add((i / 2) as usize),
            c,
            *buf.add(i as usize),
        );
        c += 1;
        if i + 1 < channels - 1
            && strcmp(
                *(*spec).labels.add((i / 2) as usize),
                *(*spec).labels.add(((i + 1) / 2) as usize),
            ) != 0
        {
            c = 1;
        }
        i += 1;
    }

    kfree(buf as *const c_void);
}

unsafe extern "C" fn proc_read_formation(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let bebob: *mut snd_bebob = (*entry).private_data as *mut snd_bebob;
    let mut formation: *mut snd_bebob_stream_formation;
    let mut i: c_uint;

    snd_iprintf(
        buffer,
        b"Output Stream from device:\n\0".as_ptr() as *const c_char,
    );
    snd_iprintf(buffer, b"\tRate\tPCM\tMIDI\n\0".as_ptr() as *const c_char);
    formation = (*bebob).tx_stream_formations;
    i = 0;
    while (i as usize) < SND_BEBOB_STRM_FMT_ENTRIES {
        snd_iprintf(
            buffer,
            b"\t%d\t%d\t%d\n\0".as_ptr() as *const c_char,
            snd_bebob_rate_table[i as usize],
            (*formation.add(i as usize)).pcm,
            (*formation.add(i as usize)).midi,
        );
        i += 1;
    }

    snd_iprintf(
        buffer,
        b"Input Stream to device:\n\0".as_ptr() as *const c_char,
    );
    snd_iprintf(buffer, b"\tRate\tPCM\tMIDI\n\0".as_ptr() as *const c_char);
    formation = (*bebob).rx_stream_formations;
    i = 0;
    while (i as usize) < SND_BEBOB_STRM_FMT_ENTRIES {
        snd_iprintf(
            buffer,
            b"\t%d\t%d\t%d\n\0".as_ptr() as *const c_char,
            snd_bebob_rate_table[i as usize],
            (*formation.add(i as usize)).pcm,
            (*formation.add(i as usize)).midi,
        );
        i += 1;
    }
}

unsafe extern "C" fn proc_read_clock(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    static CLK_LABELS: [*const c_char; 3] = [
        b"Internal\0".as_ptr() as *const c_char,
        b"External\0".as_ptr() as *const c_char,
        b"SYT-Match\0".as_ptr() as *const c_char,
    ];
    let bebob: *mut snd_bebob = (*entry).private_data as *mut snd_bebob;
    let rate_spec: *const snd_bebob_rate_spec = (*(*bebob).spec).rate;
    let clk_spec: *const snd_bebob_clock_spec = (*(*bebob).spec).clock;
    let mut src: snd_bebob_clock_type = 0;
    let mut rate: c_uint = 0;

    if ((*rate_spec).get.unwrap())(bebob, &mut rate) >= 0 {
        snd_iprintf(
            buffer,
            b"Sampling rate: %d\n\0".as_ptr() as *const c_char,
            rate,
        );
    }

    if snd_bebob_stream_get_clock_src(bebob, &mut src) >= 0 {
        if !clk_spec.is_null() {
            snd_iprintf(
                buffer,
                b"Clock Source: %s\n\0".as_ptr() as *const c_char,
                CLK_LABELS[src as usize],
            );
        } else {
            snd_iprintf(
                buffer,
                b"Clock Source: %s (MSU-dest: %d)\n\0".as_ptr() as *const c_char,
                CLK_LABELS[src as usize],
                (*bebob).sync_input_plug,
            );
        }
    }
}

unsafe extern "C" fn add_node(
    bebob: *mut snd_bebob,
    root: *mut snd_info_entry,
    name: *const c_char,
    op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
) {
    let entry: *mut snd_info_entry;

    entry = snd_info_create_card_entry((*bebob).card, name, root);
    if !entry.is_null() {
        snd_info_set_text_ops(entry, bebob, op);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_bebob_proc_init(bebob: *mut snd_bebob) {
    let root: *mut snd_info_entry;

    /*
     * All nodes are automatically removed at snd_card_disconnect(),
     * by following to link list.
     */
    root = snd_info_create_card_entry(
        (*bebob).card,
        b"firewire\0".as_ptr() as *const c_char,
        (*(*bebob).card).proc_root,
    );
    if root.is_null() {
        return;
    }
    (*root).mode = S_IFDIR | 0o555;

    add_node(
        bebob,
        root,
        b"clock\0".as_ptr() as *const c_char,
        Some(proc_read_clock),
    );
    add_node(
        bebob,
        root,
        b"firmware\0".as_ptr() as *const c_char,
        Some(proc_read_hw_info),
    );
    add_node(
        bebob,
        root,
        b"formation\0".as_ptr() as *const c_char,
        Some(proc_read_formation),
    );

    if !(*(*bebob).spec).meter.is_null() {
        add_node(
            bebob,
            root,
            b"meter\0".as_ptr() as *const c_char,
            Some(proc_read_meters),
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
