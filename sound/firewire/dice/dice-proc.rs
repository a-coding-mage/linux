// SPDX-License-Identifier: GPL-2.0-only
/*
 * dice_proc.c - a part of driver for Dice based devices
 *
 * Copyright (c) Clemens Ladisch
 * Copyright (c) 2014 Takashi Sakamoto
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Constants, types, and helpers are supplied by dice.h and kernel/ALSA headers.

#[repr(C)]
pub struct snd_card {
    proc_root: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_dice {
    unit: *mut c_void,
    card: *mut snd_card,
    tx_pcm_chs: [[u32; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
    tx_midi_ports: [u32; MAX_STREAMS],
    rx_pcm_chs: [[u32; SND_DICE_RATE_MODE_COUNT]; MAX_STREAMS],
    rx_midi_ports: [u32; MAX_STREAMS],
}

#[repr(C)]
pub struct snd_info_entry {
    private_data: *mut c_void,
    mode: c_uint,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn snd_fw_transaction(
        unit: *mut c_void,
        tcode: c_int,
        offset: u64,
        buffer: *mut c_void,
        length: c_uint,
        flags: c_int,
    ) -> c_int;
    fn be32_to_cpus(p: *mut u32);
    fn cpu_to_le32s(p: *mut u32);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        root: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(
        entry: *mut snd_info_entry,
        private_data: *mut snd_dice,
        op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
}

unsafe fn dice_proc_read_mem(
    dice: *mut snd_dice,
    buffer: *mut c_void,
    offset_q: c_uint,
    quadlets: c_uint,
) -> c_int {
    let mut i: c_uint;
    let err: c_int;

    err = snd_fw_transaction(
        (*dice).unit,
        TCODE_READ_BLOCK_REQUEST,
        DICE_PRIVATE_SPACE.wrapping_add(4u64.wrapping_mul(offset_q as u64)),
        buffer,
        4u32.wrapping_mul(quadlets),
        0,
    );
    if err < 0 {
        return err;
    }

    i = 0;
    while i < quadlets {
        be32_to_cpus((buffer as *mut u32).add(i as usize));
        i += 1;
    }

    0
}

unsafe fn str_from_array(
    strs: *const *const c_char,
    count: c_uint,
    i: c_uint,
) -> *const c_char {
    if i < count {
        return *strs.add(i as usize);
    }

    c"(unknown)".as_ptr()
}

unsafe fn dice_proc_fixup_string(s: *mut c_char, size: c_uint) {
    let mut i: c_uint;

    i = 0;
    while i < size {
        cpu_to_le32s(s.add(i as usize) as *mut u32);
        i += 4;
    }

    i = 0;
    while i < size.wrapping_sub(2) {
        if *s.add(i as usize) == 0 {
            return;
        }
        if *s.add(i as usize) == b'\\' as c_char && *s.add(i as usize + 1) == b'\\' as c_char {
            *s.add(i as usize + 2) = 0;
            return;
        }
        i += 1;
    }
    *s.add(size as usize - 1) = 0;
}

#[repr(C)]
struct tx_rx_header {
    number: u32,
    size: u32,
}

#[repr(C)]
struct dice_proc_global {
    owner_hi: u32,
    owner_lo: u32,
    notification: u32,
    nick_name: [c_char; NICK_NAME_SIZE],
    clock_select: u32,
    enable: u32,
    status: u32,
    extended_status: u32,
    sample_rate: u32,
    version: u32,
    clock_caps: u32,
    clock_source_names: [c_char; CLOCK_SOURCE_NAMES_SIZE],
}

#[repr(C)]
struct dice_proc_tx {
    iso: u32,
    number_audio: u32,
    number_midi: u32,
    speed: u32,
    names: [c_char; TX_NAMES_SIZE],
    ac3_caps: u32,
    ac3_enable: u32,
}

#[repr(C)]
struct dice_proc_rx {
    iso: u32,
    seq_start: u32,
    number_audio: u32,
    number_midi: u32,
    names: [c_char; RX_NAMES_SIZE],
    ac3_caps: u32,
    ac3_enable: u32,
}

#[repr(C)]
struct dice_proc_ext_sync {
    clock_source: u32,
    locked: u32,
    rate: u32,
    adat_user_data: u32,
}

#[repr(C)]
union dice_proc_buf {
    global: core::mem::ManuallyDrop<dice_proc_global>,
    tx: core::mem::ManuallyDrop<dice_proc_tx>,
    rx: core::mem::ManuallyDrop<dice_proc_rx>,
    ext_sync: core::mem::ManuallyDrop<dice_proc_ext_sync>,
}

unsafe extern "C" fn dice_proc_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    static SECTION_NAMES: [*const c_char; 5] = [
        c"global".as_ptr(),
        c"tx".as_ptr(),
        c"rx".as_ptr(),
        c"ext_sync".as_ptr(),
        c"unused2".as_ptr(),
    ];
    static CLOCK_SOURCES: [*const c_char; 13] = [
        c"aes1".as_ptr(),
        c"aes2".as_ptr(),
        c"aes3".as_ptr(),
        c"aes4".as_ptr(),
        c"aes".as_ptr(),
        c"adat".as_ptr(),
        c"tdif".as_ptr(),
        c"wc".as_ptr(),
        c"arx1".as_ptr(),
        c"arx2".as_ptr(),
        c"arx3".as_ptr(),
        c"arx4".as_ptr(),
        c"internal".as_ptr(),
    ];
    static RATES: [*const c_char; 11] = [
        c"32000".as_ptr(),
        c"44100".as_ptr(),
        c"48000".as_ptr(),
        c"88200".as_ptr(),
        c"96000".as_ptr(),
        c"176400".as_ptr(),
        c"192000".as_ptr(),
        c"any low".as_ptr(),
        c"any mid".as_ptr(),
        c"any high".as_ptr(),
        c"none".as_ptr(),
    ];
    let dice = (*entry).private_data as *mut snd_dice;
    let mut sections: [u32; 5 * 2] = [0; 5 * 2];
    let mut tx_rx_header: tx_rx_header = core::mem::zeroed();
    let mut buf: dice_proc_buf = core::mem::zeroed();
    let mut quadlets: c_uint;
    let mut stream: c_uint;
    let mut i: c_uint;

    if dice_proc_read_mem(
        dice,
        sections.as_mut_ptr() as *mut c_void,
        0,
        sections.len() as c_uint,
    ) < 0
    {
        return;
    }
    snd_iprintf(buffer, c"sections:\n".as_ptr());
    i = 0;
    while (i as usize) < SECTION_NAMES.len() {
        snd_iprintf(
            buffer,
            c"  %s: offset %u, size %u\n".as_ptr(),
            SECTION_NAMES[i as usize],
            sections[i as usize * 2],
            sections[i as usize * 2 + 1],
        );
        i += 1;
    }

    quadlets = core::cmp::min(sections[1], (core::mem::size_of::<dice_proc_global>() / 4) as u32);
    if dice_proc_read_mem(
        dice,
        &mut buf.global as *mut _ as *mut c_void,
        sections[0],
        quadlets,
    ) < 0
    {
        return;
    }
    snd_iprintf(buffer, c"global:\n".as_ptr());
    snd_iprintf(
        buffer,
        c"  owner: %04x:%04x%08x\n".as_ptr(),
        (*buf.global).owner_hi >> 16,
        (*buf.global).owner_hi & 0xffff,
        (*buf.global).owner_lo,
    );
    snd_iprintf(buffer, c"  notification: %08x\n".as_ptr(), (*buf.global).notification);
    dice_proc_fixup_string((*buf.global).nick_name.as_mut_ptr(), NICK_NAME_SIZE as c_uint);
    snd_iprintf(buffer, c"  nick name: %s\n".as_ptr(), (*buf.global).nick_name.as_ptr());
    snd_iprintf(
        buffer,
        c"  clock select: %s %s\n".as_ptr(),
        str_from_array(
            CLOCK_SOURCES.as_ptr(),
            CLOCK_SOURCES.len() as c_uint,
            (*buf.global).clock_select & CLOCK_SOURCE_MASK,
        ),
        str_from_array(
            RATES.as_ptr(),
            RATES.len() as c_uint,
            ((*buf.global).clock_select & CLOCK_RATE_MASK) >> CLOCK_RATE_SHIFT,
        ),
    );
    snd_iprintf(buffer, c"  enable: %u\n".as_ptr(), (*buf.global).enable);
    snd_iprintf(
        buffer,
        c"  status: %slocked %s\n".as_ptr(),
        if (*buf.global).status & STATUS_SOURCE_LOCKED != 0 {
            c"".as_ptr()
        } else {
            c"un".as_ptr()
        },
        str_from_array(
            RATES.as_ptr(),
            RATES.len() as c_uint,
            ((*buf.global).status & STATUS_NOMINAL_RATE_MASK) >> CLOCK_RATE_SHIFT,
        ),
    );
    snd_iprintf(buffer, c"  ext status: %08x\n".as_ptr(), (*buf.global).extended_status);
    snd_iprintf(buffer, c"  sample rate: %u\n".as_ptr(), (*buf.global).sample_rate);
    if quadlets >= 90 {
        snd_iprintf(
            buffer,
            c"  version: %u.%u.%u.%u\n".as_ptr(),
            ((*buf.global).version >> 24) & 0xff,
            ((*buf.global).version >> 16) & 0xff,
            ((*buf.global).version >> 8) & 0xff,
            ((*buf.global).version >> 0) & 0xff,
        );
        snd_iprintf(buffer, c"  clock caps:".as_ptr());
        i = 0;
        while i <= 6 {
            if (*buf.global).clock_caps & (1u32 << i) != 0 {
                snd_iprintf(buffer, c" %s".as_ptr(), RATES[i as usize]);
            }
            i += 1;
        }
        i = 0;
        while i <= 12 {
            if (*buf.global).clock_caps & (1u32 << (16 + i)) != 0 {
                snd_iprintf(buffer, c" %s".as_ptr(), CLOCK_SOURCES[i as usize]);
            }
            i += 1;
        }
        snd_iprintf(buffer, c"\n".as_ptr());
        dice_proc_fixup_string(
            (*buf.global).clock_source_names.as_mut_ptr(),
            CLOCK_SOURCE_NAMES_SIZE as c_uint,
        );
        snd_iprintf(
            buffer,
            c"  clock source names: %s\n".as_ptr(),
            (*buf.global).clock_source_names.as_ptr(),
        );
    }

    if dice_proc_read_mem(dice, &mut tx_rx_header as *mut _ as *mut c_void, sections[2], 2) < 0 {
        return;
    }
    quadlets = core::cmp::min(tx_rx_header.size, (core::mem::size_of::<dice_proc_tx>() / 4) as u32);
    stream = 0;
    while stream < tx_rx_header.number {
        if dice_proc_read_mem(
            dice,
            &mut buf.tx as *mut _ as *mut c_void,
            sections[2].wrapping_add(2).wrapping_add(stream.wrapping_mul(tx_rx_header.size)),
            quadlets,
        ) < 0
        {
            break;
        }
        snd_iprintf(buffer, c"tx %u:\n".as_ptr(), stream);
        snd_iprintf(buffer, c"  iso channel: %d\n".as_ptr(), (*buf.tx).iso as c_int);
        snd_iprintf(buffer, c"  audio channels: %u\n".as_ptr(), (*buf.tx).number_audio);
        snd_iprintf(buffer, c"  midi ports: %u\n".as_ptr(), (*buf.tx).number_midi);
        snd_iprintf(buffer, c"  speed: S%u\n".as_ptr(), 100u32 << (*buf.tx).speed);
        if quadlets >= 68 {
            dice_proc_fixup_string((*buf.tx).names.as_mut_ptr(), TX_NAMES_SIZE as c_uint);
            snd_iprintf(buffer, c"  names: %s\n".as_ptr(), (*buf.tx).names.as_ptr());
        }
        if quadlets >= 70 {
            snd_iprintf(buffer, c"  ac3 caps: %08x\n".as_ptr(), (*buf.tx).ac3_caps);
            snd_iprintf(buffer, c"  ac3 enable: %08x\n".as_ptr(), (*buf.tx).ac3_enable);
        }
        stream += 1;
    }

    if dice_proc_read_mem(dice, &mut tx_rx_header as *mut _ as *mut c_void, sections[4], 2) < 0 {
        return;
    }
    quadlets = core::cmp::min(tx_rx_header.size, (core::mem::size_of::<dice_proc_rx>() / 4) as u32);
    stream = 0;
    while stream < tx_rx_header.number {
        if dice_proc_read_mem(
            dice,
            &mut buf.rx as *mut _ as *mut c_void,
            sections[4].wrapping_add(2).wrapping_add(stream.wrapping_mul(tx_rx_header.size)),
            quadlets,
        ) < 0
        {
            break;
        }
        snd_iprintf(buffer, c"rx %u:\n".as_ptr(), stream);
        snd_iprintf(buffer, c"  iso channel: %d\n".as_ptr(), (*buf.rx).iso as c_int);
        snd_iprintf(buffer, c"  sequence start: %u\n".as_ptr(), (*buf.rx).seq_start);
        snd_iprintf(buffer, c"  audio channels: %u\n".as_ptr(), (*buf.rx).number_audio);
        snd_iprintf(buffer, c"  midi ports: %u\n".as_ptr(), (*buf.rx).number_midi);
        if quadlets >= 68 {
            dice_proc_fixup_string((*buf.rx).names.as_mut_ptr(), RX_NAMES_SIZE as c_uint);
            snd_iprintf(buffer, c"  names: %s\n".as_ptr(), (*buf.rx).names.as_ptr());
        }
        if quadlets >= 70 {
            snd_iprintf(buffer, c"  ac3 caps: %08x\n".as_ptr(), (*buf.rx).ac3_caps);
            snd_iprintf(buffer, c"  ac3 enable: %08x\n".as_ptr(), (*buf.rx).ac3_enable);
        }
        stream += 1;
    }

    quadlets = core::cmp::min(sections[7], (core::mem::size_of::<dice_proc_ext_sync>() / 4) as u32);
    if quadlets >= 4 {
        if dice_proc_read_mem(dice, &mut buf.ext_sync as *mut _ as *mut c_void, sections[6], 4) < 0 {
            return;
        }
        snd_iprintf(buffer, c"ext status:\n".as_ptr());
        snd_iprintf(
            buffer,
            c"  clock source: %s\n".as_ptr(),
            str_from_array(
                CLOCK_SOURCES.as_ptr(),
                CLOCK_SOURCES.len() as c_uint,
                (*buf.ext_sync).clock_source,
            ),
        );
        snd_iprintf(buffer, c"  locked: %u\n".as_ptr(), (*buf.ext_sync).locked);
        snd_iprintf(
            buffer,
            c"  rate: %s\n".as_ptr(),
            str_from_array(RATES.as_ptr(), RATES.len() as c_uint, (*buf.ext_sync).rate),
        );
        snd_iprintf(buffer, c"  adat user data: ".as_ptr());
        if (*buf.ext_sync).adat_user_data & ADAT_USER_DATA_NO_DATA != 0 {
            snd_iprintf(buffer, c"-\n".as_ptr());
        } else {
            snd_iprintf(buffer, c"%x\n".as_ptr(), (*buf.ext_sync).adat_user_data);
        }
    }
}

unsafe extern "C" fn dice_proc_read_formation(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    static RATE_LABELS: [*const c_char; SND_DICE_RATE_MODE_COUNT] = {
        let mut labels = [core::ptr::null(); SND_DICE_RATE_MODE_COUNT];
        labels[SND_DICE_RATE_MODE_LOW] = c"low".as_ptr();
        labels[SND_DICE_RATE_MODE_MIDDLE] = c"middle".as_ptr();
        labels[SND_DICE_RATE_MODE_HIGH] = c"high".as_ptr();
        labels
    };
    let dice = (*entry).private_data as *mut snd_dice;
    let mut i: c_int;
    let mut j: c_int;

    snd_iprintf(buffer, c"Output stream from unit:\n".as_ptr());
    i = 0;
    while i < SND_DICE_RATE_MODE_COUNT as c_int {
        snd_iprintf(buffer, c"\t%s".as_ptr(), RATE_LABELS[i as usize]);
        i += 1;
    }
    snd_iprintf(buffer, c"\tMIDI\n".as_ptr());
    i = 0;
    while i < MAX_STREAMS as c_int {
        snd_iprintf(buffer, c"Tx %u:".as_ptr(), i as c_uint);
        j = 0;
        while j < SND_DICE_RATE_MODE_COUNT as c_int {
            snd_iprintf(buffer, c"\t%u".as_ptr(), (*dice).tx_pcm_chs[i as usize][j as usize]);
            j += 1;
        }
        snd_iprintf(buffer, c"\t%u\n".as_ptr(), (*dice).tx_midi_ports[i as usize]);
        i += 1;
    }

    snd_iprintf(buffer, c"Input stream to unit:\n".as_ptr());
    i = 0;
    while i < SND_DICE_RATE_MODE_COUNT as c_int {
        snd_iprintf(buffer, c"\t%s".as_ptr(), RATE_LABELS[i as usize]);
        i += 1;
    }
    snd_iprintf(buffer, c"\n".as_ptr());
    i = 0;
    while i < MAX_STREAMS as c_int {
        snd_iprintf(buffer, c"Rx %u:".as_ptr(), i as c_uint);
        j = 0;
        while j < SND_DICE_RATE_MODE_COUNT as c_int {
            snd_iprintf(buffer, c"\t%u".as_ptr(), (*dice).rx_pcm_chs[i as usize][j as usize]);
            j += 1;
        }
        snd_iprintf(buffer, c"\t%u\n".as_ptr(), (*dice).rx_midi_ports[i as usize]);
        i += 1;
    }
}

unsafe fn add_node(
    dice: *mut snd_dice,
    root: *mut snd_info_entry,
    name: *const c_char,
    op: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
) {
    let entry: *mut snd_info_entry;

    entry = snd_info_create_card_entry((*dice).card, name, root);
    if !entry.is_null() {
        snd_info_set_text_ops(entry, dice, op);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dice_create_proc(dice: *mut snd_dice) {
    let root: *mut snd_info_entry;

    /*
     * All nodes are automatically removed at snd_card_disconnect(),
     * by following to link list.
     */
    root = snd_info_create_card_entry((*dice).card, c"firewire".as_ptr(), (*(*dice).card).proc_root);
    if root.is_null() {
        return;
    }
    (*root).mode = S_IFDIR | 0o555;

    add_node(dice, root, c"dice".as_ptr(), Some(dice_proc_read));
    add_node(
        dice,
        root,
        c"formation".as_ptr(),
        Some(dice_proc_read_formation),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
