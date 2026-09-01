// SPDX-License-Identifier: GPL-2.0
// ff-protocol-former.c - a part of driver for RME Fireface series
//
// Copyright (c) 2019 Takashi Sakamoto

// C dependencies removed from executable Rust:
// #include <linux/delay.h>
// #include "ff.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type size_t = usize;
type bool_ = bool;
type __le32 = u32;

const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const ETIMEDOUT: c_int = 110;

const GFP_KERNEL: c_uint = 0;
const TCODE_READ_QUADLET_REQUEST: c_int = 0;
const TCODE_WRITE_QUADLET_REQUEST: c_int = 0;
const TCODE_READ_BLOCK_REQUEST: c_int = 0;
const TCODE_WRITE_BLOCK_REQUEST: c_int = 0;
const SND_FF_STREAM_MODE_COUNT: c_int = 0;
const SND_FF_MAXIMIM_MIDI_QUADS: c_int = 0;
const CIP_SFC_COUNT: c_int = 0;
const SCODE_800: c_int = 0;
const SNDRV_FIREWIRE_EVENT_FF400_MESSAGE: c_uint = 0;

#[repr(C)]
pub struct snd_ff {
    unit: *mut c_void,
    spec: *mut snd_ff_spec,
    tx_stream: amdtp_stream,
    rx_stream: amdtp_stream,
    tx_resources: fw_iso_resources,
    rx_resources: fw_iso_resources,
    msg_buf: *mut *mut __le32,
    rx_bytes: *mut c_int,
    tx_midi_substreams: *mut *mut snd_rawmidi_substream,
    msg_parser: *mut c_void,
    hwdep_wait: c_void,
    lock: c_void,
}

#[repr(C)]
pub struct snd_ff_spec {
    pcm_playback_channels: [c_uint; SND_FF_STREAM_MODE_COUNT as usize],
}

#[repr(C)]
pub struct amdtp_stream {
    data_block_quadlets: c_uint,
}

#[repr(C)]
pub struct fw_iso_resources {
    channel: c_uint,
    generation: c_uint,
    channels_mask: u64,
}

#[repr(C)]
pub struct fw_card {
    generation: c_uint,
}

#[repr(C)]
pub struct fw_device {
    max_speed: c_int,
    card: *mut fw_card,
}

pub enum snd_info_buffer {}
pub enum snd_rawmidi_substream {}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum snd_ff_clock_src {
    SND_FF_CLOCK_SRC_INTERNAL = 0,
    SND_FF_CLOCK_SRC_ADAT1 = 1,
    SND_FF_CLOCK_SRC_ADAT2 = 2,
    SND_FF_CLOCK_SRC_SPDIF = 3,
    SND_FF_CLOCK_SRC_WORD = 4,
    SND_FF_CLOCK_SRC_LTC = 5,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum snd_ff_stream_mode {
    __SND_FF_STREAM_MODE_PLACEHOLDER = 0,
}

#[repr(C)]
pub struct snd_firewire_event_ff400_message {
    type_: c_uint,
    message_count: c_uint,
}

#[repr(C)]
pub struct snd_ff_protocol {
    msg_parser_size: size_t,
    has_msg: Option<unsafe extern "C" fn(*mut snd_ff) -> bool_>,
    copy_msg_to_user: Option<unsafe extern "C" fn(*mut snd_ff, *mut c_char, c_long) -> c_long>,
    handle_msg: Option<unsafe extern "C" fn(*mut snd_ff, c_uint, *const __le32, size_t, u32)>,
    fill_midi_msg: Option<unsafe extern "C" fn(*mut snd_ff, *mut snd_rawmidi_substream, c_uint) -> c_int>,
    get_clock: Option<unsafe extern "C" fn(*mut snd_ff, *mut c_uint, *mut snd_ff_clock_src) -> c_int>,
    switch_fetching_mode: Option<unsafe extern "C" fn(*mut snd_ff, bool_) -> c_int>,
    allocate_resources: Option<unsafe extern "C" fn(*mut snd_ff, c_uint) -> c_int>,
    begin_session: Option<unsafe extern "C" fn(*mut snd_ff, c_uint) -> c_int>,
    finish_session: Option<unsafe extern "C" fn(*mut snd_ff)>,
    dump_status: Option<unsafe extern "C" fn(*mut snd_ff, *mut snd_info_buffer)>,
}

extern "C" {
    static amdtp_rate_table: [c_uint; CIP_SFC_COUNT as usize];

    fn snd_fw_transaction(unit: *mut c_void, tcode: c_int, offset: u64, buffer: *mut c_void, length: size_t, flags: c_uint) -> c_int;
    fn kcalloc(n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn msleep(msecs: c_uint);
    fn fw_iso_resources_allocate(resources: *mut fw_iso_resources, max_payload: c_uint, speed: c_int) -> c_int;
    fn fw_iso_resources_update(resources: *mut fw_iso_resources) -> c_int;
    fn fw_iso_resources_free(resources: *mut fw_iso_resources);
    fn amdtp_stream_get_max_payload(stream: *mut amdtp_stream) -> c_uint;
    fn fw_parent_device(unit: *mut c_void) -> *mut fw_device;
    fn snd_ff_stream_get_multiplier_mode(sfc: c_int, mode: *mut snd_ff_stream_mode) -> c_int;
    fn snd_rawmidi_transmit_peek(substream: *mut snd_rawmidi_substream, buffer: *mut u8, count: c_int) -> c_int;
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buffer: *mut u8, count: c_int) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_ff_proc_get_clk_label(src: snd_ff_clock_src) -> *const c_char;
    fn str_on_off(v: u32) -> *const c_char;
    fn wake_up(wait: *mut c_void);
    fn spin_unlock_irq(lock: *mut c_void);
    fn spin_lock_irq(lock: *mut c_void);
    fn copy_to_user(dst: *mut c_char, src: *const c_void, n: size_t) -> c_ulong;
}

#[inline]
fn cpu_to_le32(v: u32) -> __le32 {
    v.to_le()
}

#[inline]
fn le32_to_cpu(v: __le32) -> u32 {
    u32::from_le(v)
}

#[inline]
unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    ptr::read_volatile(p)
}

const FORMER_REG_SYNC_STATUS: u64 = 0x0000801c0000u64;
/* For block write request. */
const FORMER_REG_FETCH_PCM_FRAMES: u64 = 0x0000801c0000u64;
const FORMER_REG_CLOCK_CONFIG: u64 = 0x0000801c0004u64;

#[repr(C)]
struct clock_rate_entry {
    rate: c_uint,
    mask: u32,
}

#[repr(C)]
struct clock_src_entry {
    src: snd_ff_clock_src,
    mask: u32,
}

unsafe extern "C" fn parse_clock_bits(data: u32, rate: *mut c_uint, src: *mut snd_ff_clock_src) -> c_int {
    static rate_entries: [clock_rate_entry; 9] = [
        clock_rate_entry { rate: 32000, mask: 0x00000002 },
        clock_rate_entry { rate: 44100, mask: 0x00000000 },
        clock_rate_entry { rate: 48000, mask: 0x00000006 },
        clock_rate_entry { rate: 64000, mask: 0x0000000a },
        clock_rate_entry { rate: 88200, mask: 0x00000008 },
        clock_rate_entry { rate: 96000, mask: 0x0000000e },
        clock_rate_entry { rate: 128000, mask: 0x00000012 },
        clock_rate_entry { rate: 176400, mask: 0x00000010 },
        clock_rate_entry { rate: 192000, mask: 0x00000016 },
    ];
    static clk_entries: [clock_src_entry; 5] = [
        clock_src_entry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_ADAT1, mask: 0x00000000 },
        clock_src_entry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_ADAT2, mask: 0x00000400 },
        clock_src_entry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_SPDIF, mask: 0x00000c00 },
        clock_src_entry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_WORD, mask: 0x00001000 },
        clock_src_entry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_LTC, mask: 0x00001800 },
    ];
    let mut i: usize;

    i = 0;
    while i < rate_entries.len() {
        let rate_entry = &rate_entries[i];
        if (data & 0x0000001e) == rate_entry.mask {
            *rate = rate_entry.rate;
            break;
        }
        i += 1;
    }
    if i == rate_entries.len() {
        return -EIO;
    }

    if (data & 0x00000001) != 0 {
        *src = snd_ff_clock_src::SND_FF_CLOCK_SRC_INTERNAL;
    } else {
        i = 0;
        while i < clk_entries.len() {
            let clk_entry = &clk_entries[i];
            if (data & 0x00001c00) == clk_entry.mask {
                *src = clk_entry.src;
                break;
            }
            i += 1;
        }
        if i == clk_entries.len() {
            return -EIO;
        }
    }

    0
}

unsafe extern "C" fn former_get_clock(ff: *mut snd_ff, rate: *mut c_uint, src: *mut snd_ff_clock_src) -> c_int {
    let mut reg: __le32 = 0;
    let data: u32;
    let err: c_int;

    err = snd_fw_transaction((*ff).unit, TCODE_READ_QUADLET_REQUEST, FORMER_REG_CLOCK_CONFIG, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0);
    if err < 0 {
        return err;
    }
    data = le32_to_cpu(reg);

    parse_clock_bits(data, rate, src)
}

unsafe extern "C" fn former_switch_fetching_mode(ff: *mut snd_ff, enable: bool_) -> c_int {
    let mut count: c_uint;
    let reg: *mut __le32;
    let mut i: c_int;
    let err: c_int;

    count = 0;
    i = 0;
    while i < SND_FF_STREAM_MODE_COUNT {
        let v = (*(*ff).spec).pcm_playback_channels[i as usize];
        if count < v {
            count = v;
        }
        i += 1;
    }

    reg = kcalloc(count as size_t, size_of::<__le32>(), GFP_KERNEL) as *mut __le32;
    if reg.is_null() {
        return -ENOMEM;
    }

    if !enable {
        /*
         * Each quadlet is corresponding to data channels in a data
         * blocks in reverse order. Precisely, quadlets for available
         * data channels should be enabled. Here, I take second best
         * to fetch PCM frames from all of data channels regardless of
         * stf.
         */
        i = 0;
        while i < count as c_int {
            *reg.add(i as usize) = cpu_to_le32(0x00000001);
            i += 1;
        }
    }

    err = snd_fw_transaction((*ff).unit, TCODE_WRITE_BLOCK_REQUEST, FORMER_REG_FETCH_PCM_FRAMES, reg as *mut c_void, size_of::<__le32>() * count as size_t, 0);
    kfree(reg as *mut c_void);
    err
}

unsafe extern "C" fn dump_clock_config(ff: *mut snd_ff, buffer: *mut snd_info_buffer) {
    let mut reg: __le32 = 0;
    let data: u32;
    let mut rate: c_uint = 0;
    let mut src: snd_ff_clock_src = snd_ff_clock_src::SND_FF_CLOCK_SRC_INTERNAL;
    let label: *const c_char;
    let err: c_int;

    err = snd_fw_transaction((*ff).unit, TCODE_READ_BLOCK_REQUEST, FORMER_REG_CLOCK_CONFIG, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0);
    if err < 0 {
        return;
    }
    data = le32_to_cpu(reg);

    snd_iprintf(buffer, b"Output S/PDIF format: %s (Emphasis: %s)\n\0".as_ptr() as *const c_char,
        if (data & 0x00000020) != 0 { b"Professional\0".as_ptr() } else { b"Consumer\0".as_ptr() } as *const c_char,
        str_on_off(data & 0x00000040));

    snd_iprintf(buffer, b"Optical output interface format: %s\n\0".as_ptr() as *const c_char,
        if (data & 0x00000100) != 0 { b"S/PDIF\0".as_ptr() } else { b"ADAT\0".as_ptr() } as *const c_char);

    snd_iprintf(buffer, b"Word output single speed: %s\n\0".as_ptr() as *const c_char,
        str_on_off(data & 0x00002000));

    snd_iprintf(buffer, b"S/PDIF input interface: %s\n\0".as_ptr() as *const c_char,
        if (data & 0x00000200) != 0 { b"Optical\0".as_ptr() } else { b"Coaxial\0".as_ptr() } as *const c_char);

    if parse_clock_bits(data, &mut rate, &mut src) < 0 {
        return;
    }
    label = snd_ff_proc_get_clk_label(src);
    if label.is_null() {
        return;
    }

    snd_iprintf(buffer, b"Clock configuration: %d %s\n\0".as_ptr() as *const c_char, rate, label);
}

#[repr(C)]
struct sync_clk_entry {
    label: *const c_char,
    locked_mask: u32,
    synced_mask: u32,
}

#[repr(C)]
struct label_mask_entry {
    label: *const c_char,
    mask: u32,
}

unsafe extern "C" fn dump_sync_status(ff: *mut snd_ff, buffer: *mut snd_info_buffer) {
    let clk_entries: [sync_clk_entry; 4] = [
        sync_clk_entry { label: b"WDClk\0".as_ptr() as *const c_char, locked_mask: 0x40000000, synced_mask: 0x20000000 },
        sync_clk_entry { label: b"S/PDIF\0".as_ptr() as *const c_char, locked_mask: 0x00080000, synced_mask: 0x00040000 },
        sync_clk_entry { label: b"ADAT1\0".as_ptr() as *const c_char, locked_mask: 0x00000400, synced_mask: 0x00001000 },
        sync_clk_entry { label: b"ADAT2\0".as_ptr() as *const c_char, locked_mask: 0x00000800, synced_mask: 0x00002000 },
    ];
    let referred_entries: [label_mask_entry; 5] = [
        label_mask_entry { label: b"ADAT1\0".as_ptr() as *const c_char, mask: 0x00000000 },
        label_mask_entry { label: b"ADAT2\0".as_ptr() as *const c_char, mask: 0x00400000 },
        label_mask_entry { label: b"S/PDIF\0".as_ptr() as *const c_char, mask: 0x00c00000 },
        label_mask_entry { label: b"WDclk\0".as_ptr() as *const c_char, mask: 0x01000000 },
        label_mask_entry { label: b"TCO\0".as_ptr() as *const c_char, mask: 0x01400000 },
    ];
    let rate_entries: [clock_rate_entry; 9] = [
        clock_rate_entry { rate: 32000, mask: 0x02000000 },
        clock_rate_entry { rate: 44100, mask: 0x04000000 },
        clock_rate_entry { rate: 48000, mask: 0x06000000 },
        clock_rate_entry { rate: 64000, mask: 0x08000000 },
        clock_rate_entry { rate: 88200, mask: 0x0a000000 },
        clock_rate_entry { rate: 96000, mask: 0x0c000000 },
        clock_rate_entry { rate: 128000, mask: 0x0e000000 },
        clock_rate_entry { rate: 176400, mask: 0x10000000 },
        clock_rate_entry { rate: 192000, mask: 0x12000000 },
    ];
    let mut reg: [__le32; 2] = [0; 2];
    let mut data: [u32; 2] = [0; 2];
    let mut i: usize;
    let err: c_int;

    err = snd_fw_transaction((*ff).unit, TCODE_READ_BLOCK_REQUEST, FORMER_REG_SYNC_STATUS, reg.as_mut_ptr() as *mut c_void, size_of::<[__le32; 2]>(), 0);
    if err < 0 {
        return;
    }
    data[0] = le32_to_cpu(reg[0]);
    data[1] = le32_to_cpu(reg[1]);

    snd_iprintf(buffer, b"External source detection:\n\0".as_ptr() as *const c_char);

    i = 0;
    while i < clk_entries.len() {
        let state: *const c_char;
        let clk_entry = &clk_entries[i];
        if (data[0] & clk_entry.locked_mask) != 0 {
            if (data[0] & clk_entry.synced_mask) != 0 {
                state = b"sync\0".as_ptr() as *const c_char;
            } else {
                state = b"lock\0".as_ptr() as *const c_char;
            }
        } else {
            state = b"none\0".as_ptr() as *const c_char;
        }

        snd_iprintf(buffer, b"%s: %s\n\0".as_ptr() as *const c_char, clk_entry.label, state);
        i += 1;
    }

    snd_iprintf(buffer, b"Referred clock:\n\0".as_ptr() as *const c_char);

    if (data[1] & 0x00000001) != 0 {
        snd_iprintf(buffer, b"Internal\n\0".as_ptr() as *const c_char);
    } else {
        let mut rate: c_uint;
        let mut label: *const c_char;

        i = 0;
        label = ptr::null();
        while i < referred_entries.len() {
            let referred_entry = &referred_entries[i];
            if (data[0] & 0x1e0000) == referred_entry.mask {
                label = referred_entry.label;
                break;
            }
            i += 1;
        }
        if i == referred_entries.len() {
            label = b"none\0".as_ptr() as *const c_char;
        }

        i = 0;
        rate = 0;
        while i < rate_entries.len() {
            let rate_entry = &rate_entries[i];
            if (data[0] & 0x1e000000) == rate_entry.mask {
                rate = rate_entry.rate;
                break;
            }
            i += 1;
        }
        if i == rate_entries.len() {
            rate = 0;
        }

        snd_iprintf(buffer, b"%s %d\n\0".as_ptr() as *const c_char, label, rate);
    }
}

unsafe extern "C" fn former_dump_status(ff: *mut snd_ff, buffer: *mut snd_info_buffer) {
    dump_clock_config(ff, buffer);
    dump_sync_status(ff, buffer);
}

unsafe extern "C" fn former_fill_midi_msg(ff: *mut snd_ff, substream: *mut snd_rawmidi_substream, port: c_uint) -> c_int {
    let buf: *mut u8 = *(*ff).msg_buf.add(port as usize) as *mut u8;
    let len: c_int;
    let mut i: c_int;

    len = snd_rawmidi_transmit_peek(substream, buf, SND_FF_MAXIMIM_MIDI_QUADS);
    if len <= 0 {
        return len;
    }

    // One quadlet includes one byte.
    i = len - 1;
    while i >= 0 {
        *(*ff).msg_buf.add(port as usize).add(i as usize) = cpu_to_le32(*buf.add(i as usize) as u32);
        i -= 1;
    }
    *(*ff).rx_bytes.add(port as usize) = len;

    len
}

const FF800_STF: u64 = 0x0000fc88f000;
const FF800_RX_PACKET_FORMAT: u64 = 0x0000fc88f004;
const FF800_ALLOC_TX_STREAM: u64 = 0x0000fc88f008;
const FF800_ISOC_COMM_START: u64 = 0x0000fc88f00c;
const FF800_TX_S800_FLAG: u32 = 0x00000800;
const FF800_ISOC_COMM_STOP: u64 = 0x0000fc88f010;

const FF800_TX_PACKET_ISOC_CH: u64 = 0x0000801c0008;

unsafe extern "C" fn allocate_tx_resources(ff: *mut snd_ff) -> c_int {
    let mut reg: __le32;
    let mut count: c_uint;
    let mut tx_isoc_channel: c_uint = 0;
    let mut err: c_int;

    reg = cpu_to_le32((*ff).tx_stream.data_block_quadlets);
    err = snd_fw_transaction((*ff).unit, TCODE_WRITE_QUADLET_REQUEST, FF800_ALLOC_TX_STREAM, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0);
    if err < 0 {
        return err;
    }

    // Wait till the format of tx packet is available.
    count = 0;
    while {
        count += 1;
        count < 10
    } {
        let data: u32;
        err = snd_fw_transaction((*ff).unit, TCODE_READ_QUADLET_REQUEST, FF800_TX_PACKET_ISOC_CH, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0);
        if err < 0 {
            return err;
        }

        data = le32_to_cpu(reg);
        if data != 0xffffffff {
            tx_isoc_channel = data;
            break;
        }

        msleep(50);
    }
    if count >= 10 {
        return -ETIMEDOUT;
    }

    // NOTE: this is a makeshift to start OHCI 1394 IR context in the
    // channel. On the other hand, 'struct fw_iso_resources.allocated' is
    // not true and it's not deallocated at stop.
    (*ff).tx_resources.channel = tx_isoc_channel;

    0
}

unsafe extern "C" fn ff800_allocate_resources(ff: *mut snd_ff, rate: c_uint) -> c_int {
    let mut data: u32;
    let mut reg: __le32;
    let mut err: c_int;

    reg = cpu_to_le32(rate);
    err = snd_fw_transaction((*ff).unit, TCODE_WRITE_QUADLET_REQUEST, FF800_STF, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0);
    if err < 0 {
        return err;
    }

    // If starting isochronous communication immediately, change of STF has
    // no effect. In this case, the communication runs based on former STF.
    // Let's sleep for a bit.
    msleep(100);

    // Controllers should allocate isochronous resources for rx stream.
    err = fw_iso_resources_allocate(&mut (*ff).rx_resources,
        amdtp_stream_get_max_payload(&mut (*ff).rx_stream),
        (*fw_parent_device((*ff).unit)).max_speed);
    if err < 0 {
        return err;
    }

    // Set isochronous channel and the number of quadlets of rx packets.
    // This should be done before the allocation of tx resources to avoid
    // periodical noise.
    data = (*ff).rx_stream.data_block_quadlets << 3;
    data = (data << 8) | (*ff).rx_resources.channel;
    reg = cpu_to_le32(data);
    err = snd_fw_transaction((*ff).unit, TCODE_WRITE_QUADLET_REQUEST, FF800_RX_PACKET_FORMAT, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0);
    if err < 0 {
        return err;
    }

    allocate_tx_resources(ff)
}

unsafe extern "C" fn ff800_begin_session(ff: *mut snd_ff, _rate: c_uint) -> c_int {
    let generation: c_uint = (*ff).rx_resources.generation;
    let mut reg: __le32;

    if generation != (*(*fw_parent_device((*ff).unit)).card).generation {
        let err: c_int = fw_iso_resources_update(&mut (*ff).rx_resources);
        if err < 0 {
            return err;
        }
    }

    reg = cpu_to_le32(0x80000000);
    reg |= cpu_to_le32((*ff).tx_stream.data_block_quadlets);
    if (*fw_parent_device((*ff).unit)).max_speed == SCODE_800 {
        reg |= cpu_to_le32(FF800_TX_S800_FLAG);
    }
    snd_fw_transaction((*ff).unit, TCODE_WRITE_QUADLET_REQUEST, FF800_ISOC_COMM_START, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0)
}

unsafe extern "C" fn ff800_finish_session(ff: *mut snd_ff) {
    let mut reg: __le32;

    reg = cpu_to_le32(0x80000000);
    snd_fw_transaction((*ff).unit, TCODE_WRITE_QUADLET_REQUEST, FF800_ISOC_COMM_STOP, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0);
}

// Fireface 800 doesn't allow drivers to register lower 4 bytes of destination
// address.
// A write transaction to clear registered higher 4 bytes of destination address
// has an effect to suppress asynchronous transaction from device.
unsafe extern "C" fn ff800_handle_midi_msg(ff: *mut snd_ff, _offset: c_uint, buf: *const __le32, length: size_t, _tstamp: u32) {
    let mut i: size_t;

    i = 0;
    while i < length / 4 {
        let mut byte: u8 = (le32_to_cpu(*buf.add(i)) & 0xff) as u8;
        let substream: *mut snd_rawmidi_substream;

        substream = READ_ONCE((*ff).tx_midi_substreams.add(0));
        if !substream.is_null() {
            snd_rawmidi_receive(substream, &mut byte, 1);
        }
        i += 1;
    }
}

#[no_mangle]
pub static snd_ff_protocol_ff800: snd_ff_protocol = snd_ff_protocol {
    msg_parser_size: 0,
    has_msg: None,
    copy_msg_to_user: None,
    handle_msg: Some(ff800_handle_midi_msg),
    fill_midi_msg: Some(former_fill_midi_msg),
    get_clock: Some(former_get_clock),
    switch_fetching_mode: Some(former_switch_fetching_mode),
    allocate_resources: Some(ff800_allocate_resources),
    begin_session: Some(ff800_begin_session),
    finish_session: Some(ff800_finish_session),
    dump_status: Some(former_dump_status),
};

const FF400_STF: u64 = 0x000080100500u64;
const FF400_RX_PACKET_FORMAT: u64 = 0x000080100504u64;
const FF400_ISOC_COMM_START: u64 = 0x000080100508u64;
const FF400_TX_PACKET_FORMAT: u64 = 0x00008010050cu64;
const FF400_ISOC_COMM_STOP: u64 = 0x000080100510u64;

// Fireface 400 manages isochronous channel number in 3 bit field. Therefore,
// we can allocate between 0 and 7 channel.
unsafe extern "C" fn ff400_allocate_resources(ff: *mut snd_ff, rate: c_uint) -> c_int {
    let mut reg: __le32;
    let mut mode: snd_ff_stream_mode = snd_ff_stream_mode::__SND_FF_STREAM_MODE_PLACEHOLDER;
    let mut i: c_int;
    let mut err: c_int;

    // Check whether the given value is supported or not.
    i = 0;
    while i < CIP_SFC_COUNT {
        if amdtp_rate_table[i as usize] == rate {
            break;
        }
        i += 1;
    }
    if i >= CIP_SFC_COUNT {
        return -EINVAL;
    }

    // Set the number of data blocks transferred in a second.
    reg = cpu_to_le32(rate);
    err = snd_fw_transaction((*ff).unit, TCODE_WRITE_QUADLET_REQUEST, FF400_STF, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0);
    if err < 0 {
        return err;
    }

    msleep(100);

    err = snd_ff_stream_get_multiplier_mode(i, &mut mode);
    if err < 0 {
        return err;
    }

    // Keep resources for in-stream.
    (*ff).tx_resources.channels_mask = 0x00000000000000ffu64;
    err = fw_iso_resources_allocate(&mut (*ff).tx_resources,
        amdtp_stream_get_max_payload(&mut (*ff).tx_stream),
        (*fw_parent_device((*ff).unit)).max_speed);
    if err < 0 {
        return err;
    }

    // Keep resources for out-stream.
    (*ff).rx_resources.channels_mask = 0x00000000000000ffu64;
    err = fw_iso_resources_allocate(&mut (*ff).rx_resources,
        amdtp_stream_get_max_payload(&mut (*ff).rx_stream),
        (*fw_parent_device((*ff).unit)).max_speed);
    if err < 0 {
        fw_iso_resources_free(&mut (*ff).tx_resources);
    }

    err
}

unsafe extern "C" fn ff400_begin_session(ff: *mut snd_ff, _rate: c_uint) -> c_int {
    let generation: c_uint = (*ff).rx_resources.generation;
    let mut reg: __le32;
    let mut err: c_int;

    if generation != (*(*fw_parent_device((*ff).unit)).card).generation {
        err = fw_iso_resources_update(&mut (*ff).tx_resources);
        if err < 0 {
            return err;
        }

        err = fw_iso_resources_update(&mut (*ff).rx_resources);
        if err < 0 {
            return err;
        }
    }

    // Set isochronous channel and the number of quadlets of received
    // packets.
    reg = cpu_to_le32((((*ff).rx_stream.data_block_quadlets << 3) << 8) | (*ff).rx_resources.channel);
    err = snd_fw_transaction((*ff).unit, TCODE_WRITE_QUADLET_REQUEST, FF400_RX_PACKET_FORMAT, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0);
    if err < 0 {
        return err;
    }

    // Set isochronous channel and the number of quadlets of transmitted
    // packet.
    // TODO: investigate the purpose of this 0x80.
    reg = cpu_to_le32((0x80u32 << 24) |
        ((*ff).tx_resources.channel << 5) |
        ((*ff).tx_stream.data_block_quadlets));
    err = snd_fw_transaction((*ff).unit, TCODE_WRITE_QUADLET_REQUEST, FF400_TX_PACKET_FORMAT, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0);
    if err < 0 {
        return err;
    }

    // Allow to transmit packets.
    reg = cpu_to_le32(0x00000001);
    snd_fw_transaction((*ff).unit, TCODE_WRITE_QUADLET_REQUEST, FF400_ISOC_COMM_START, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0)
}

unsafe extern "C" fn ff400_finish_session(ff: *mut snd_ff) {
    let mut reg: __le32;

    reg = cpu_to_le32(0x80000000);
    snd_fw_transaction((*ff).unit, TCODE_WRITE_QUADLET_REQUEST, FF400_ISOC_COMM_STOP, &mut reg as *mut _ as *mut c_void, size_of::<__le32>(), 0);
}

unsafe extern "C" fn parse_midi_msg(ff: *mut snd_ff, quad: u32, port: c_uint) {
    let substream: *mut snd_rawmidi_substream = READ_ONCE((*ff).tx_midi_substreams.add(port as usize));

    if !substream.is_null() {
        let mut byte: u8 = ((quad >> (16 * port)) & 0x000000ff) as u8;

        snd_rawmidi_receive(substream, &mut byte, 1);
    }
}

const FF400_QUEUE_SIZE: usize = 32;

#[repr(C)]
#[derive(Clone, Copy)]
struct ff400_msg {
    msg: u32,
    tstamp: u32,
}

#[repr(C)]
struct ff400_msg_parser {
    msgs: [ff400_msg; FF400_QUEUE_SIZE],
    push_pos: size_t,
    pull_pos: size_t,
}

unsafe extern "C" fn ff400_has_msg(ff: *mut snd_ff) -> bool_ {
    let parser: *mut ff400_msg_parser = (*ff).msg_parser as *mut ff400_msg_parser;

    (*parser).push_pos != (*parser).pull_pos
}

// For Fireface 400, lower 4 bytes of destination address is configured by bit
// flag in quadlet register (little endian) at 0x'0000'801'0051c. Drivers can
// select one of 4 options:
//
// bit flags: offset of destination address
//  - 0x04000000: 0x'....'....'0000'0000
//  - 0x08000000: 0x'....'....'0000'0080
//  - 0x10000000: 0x'....'....'0000'0100
//  - 0x20000000: 0x'....'....'0000'0180
//
// Drivers can suppress the device to transfer asynchronous transactions by
// using below 2 bits.
//  - 0x01000000: suppress transmission
//  - 0x02000000: suppress transmission
//
// Actually, the register is write-only and includes the other options such as
// input attenuation. This driver allocates destination address with '0000'0000
// in its lower offset and expects userspace application to configure the
// register for it.

// When the message is for signal level operation, the upper 4 bits in MSB expresses the pair of
// stereo physical port.
// - 0: Microphone input 0/1
// - 1: Line input 0/1
// - [2-4]: Line output 0-5
// - 5: Headphone output 0/1
// - 6: S/PDIF output 0/1
// - [7-10]: ADAT output 0-7
//
// The value of signal level can be detected by mask of 0x00fffc00. For signal level of microphone
// input:
//
// - 0:    0.0 dB
// - 10: +10.0 dB
// - 11: +11.0 dB
// - 12: +12.0 dB
// - ...
// - 63: +63.0 dB:
// - 64: +64.0 dB:
// - 65: +65.0 dB:
//
// For signal level of line input:
//
// - 0:  0.0 dB
// - 1: +0.5 dB
// - 2: +1.0 dB
// - 3: +1.5 dB
// - ...
// - 34: +17.0 dB:
// - 35: +17.5 dB:
// - 36: +18.0 dB:
//
// For signal level of any type of output:
//
// - 63: -infinite
// - 62: -58.0 dB
// - 61: -56.0 dB
// - 60: -54.0 dB
// - 59: -53.0 dB
// - 58: -52.0 dB
// - ...
// - 7: -1.0 dB
// - 6:  0.0 dB
// - 5: +1.0 dB
// - ...
// - 2: +4.0 dB
// - 1: +5.0 dB
// - 0: +6.0 dB
//
// When the message is not for signal level operation, it's for MIDI bytes. When matching to
// FF400_MSG_FLAG_IS_MIDI_PORT_0, one MIDI byte can be detected by mask of 0x000000ff. When
// matching to FF400_MSG_FLAG_IS_MIDI_PORT_1, one MIDI byte can be detected by mask of 0x00ff0000.
const FF400_MSG_FLAG_IS_SIGNAL_LEVEL: u32 = 0x04000000;
const FF400_MSG_FLAG_IS_RIGHT_CHANNEL: u32 = 0x08000000;
const FF400_MSG_FLAG_IS_STEREO_PAIRED: u32 = 0x02000000;
const FF400_MSG_MASK_STEREO_PAIR: u32 = 0xf0000000;
const FF400_MSG_MASK_SIGNAL_LEVEL: u32 = 0x00fffc00;
const FF400_MSG_FLAG_IS_MIDI_PORT_0: u32 = 0x00000100;
const FF400_MSG_MASK_MIDI_PORT_0: u32 = 0x000000ff;
const FF400_MSG_FLAG_IS_MIDI_PORT_1: u32 = 0x01000000;
const FF400_MSG_MASK_MIDI_PORT_1: u32 = 0x00ff0000;

unsafe extern "C" fn ff400_handle_msg(ff: *mut snd_ff, _offset: c_uint, buf: *const __le32, length: size_t, tstamp: u32) {
    let mut need_hwdep_wake_up: bool_ = false;
    let mut i: size_t;

    i = 0;
    while i < length / 4 {
        let quad: u32 = le32_to_cpu(*buf.add(i));

        if (quad & FF400_MSG_FLAG_IS_SIGNAL_LEVEL) != 0 {
            let parser: *mut ff400_msg_parser = (*ff).msg_parser as *mut ff400_msg_parser;

            (*parser).msgs[(*parser).push_pos].msg = quad;
            (*parser).msgs[(*parser).push_pos].tstamp = tstamp;
            (*parser).push_pos += 1;
            if (*parser).push_pos >= FF400_QUEUE_SIZE {
                (*parser).push_pos = 0;
            }

            need_hwdep_wake_up = true;
        } else if (quad & FF400_MSG_FLAG_IS_MIDI_PORT_0) != 0 {
            parse_midi_msg(ff, quad, 0);
        } else if (quad & FF400_MSG_FLAG_IS_MIDI_PORT_1) != 0 {
            parse_midi_msg(ff, quad, 1);
        }
        i += 1;
    }

    if need_hwdep_wake_up {
        wake_up(&mut (*ff).hwdep_wait as *mut _ as *mut c_void);
    }
}

unsafe extern "C" fn ff400_copy_msg_to_user(ff: *mut snd_ff, buf: *mut c_char, mut count: c_long) -> c_long {
    let mut ev: snd_firewire_event_ff400_message = snd_firewire_event_ff400_message {
        type_: SNDRV_FIREWIRE_EVENT_FF400_MESSAGE,
        message_count: 0,
    };
    let parser: *mut ff400_msg_parser = (*ff).msg_parser as *mut ff400_msg_parser;
    let mut consumed: c_long = 0;
    let mut ret: c_long = 0;

    if count < size_of::<snd_firewire_event_ff400_message>() as c_long || (*parser).pull_pos == (*parser).push_pos {
        return 0;
    }

    count -= size_of::<snd_firewire_event_ff400_message>() as c_long;
    consumed += size_of::<snd_firewire_event_ff400_message>() as c_long;

    while count >= size_of::<ff400_msg>() as c_long && (*parser).pull_pos != (*parser).push_pos {
        spin_unlock_irq(&mut (*ff).lock as *mut _ as *mut c_void);
        if copy_to_user(buf.add(consumed as usize), (*parser).msgs.as_ptr().add((*parser).pull_pos) as *const c_void, size_of::<ff400_msg>()) != 0 {
            ret = -EFAULT as c_long;
        }
        spin_lock_irq(&mut (*ff).lock as *mut _ as *mut c_void);
        if ret != 0 {
            return ret;
        }

        (*parser).pull_pos += 1;
        if (*parser).pull_pos >= FF400_QUEUE_SIZE {
            (*parser).pull_pos = 0;
        }
        ev.message_count += 1;
        count -= size_of::<ff400_msg>() as c_long;
        consumed += size_of::<ff400_msg>() as c_long;
    }

    spin_unlock_irq(&mut (*ff).lock as *mut _ as *mut c_void);
    if copy_to_user(buf, &ev as *const _ as *const c_void, size_of::<snd_firewire_event_ff400_message>()) != 0 {
        ret = -EFAULT as c_long;
    }
    spin_lock_irq(&mut (*ff).lock as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }

    consumed
}

#[no_mangle]
pub static snd_ff_protocol_ff400: snd_ff_protocol = snd_ff_protocol {
    msg_parser_size: size_of::<ff400_msg_parser>(),
    has_msg: Some(ff400_has_msg),
    copy_msg_to_user: Some(ff400_copy_msg_to_user),
    handle_msg: Some(ff400_handle_msg),
    fill_midi_msg: Some(former_fill_midi_msg),
    get_clock: Some(former_get_clock),
    switch_fetching_mode: Some(former_switch_fetching_mode),
    allocate_resources: Some(ff400_allocate_resources),
    begin_session: Some(ff400_begin_session),
    finish_session: Some(ff400_finish_session),
    dump_status: Some(former_dump_status),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
