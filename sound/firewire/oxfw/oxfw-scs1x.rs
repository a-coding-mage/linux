// SPDX-License-Identifier: GPL-2.0-only
/*
 * oxfw-scs1x.rs - a part of driver for OXFW970/971 based devices
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 * Copyright (c) 2015 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// Rust translation of implementation depending on declarations from "oxfw.h".

use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = core::ffi::c_uchar;
type u64 = core::ffi::c_ulonglong;
type size_t = usize;
type bool_t = bool;
type __be64 = u64;

const HSS1394_ADDRESS: u64 = 0xc007dedadada;
const HSS1394_MAX_PACKET_SIZE: usize = 64;
const HSS1394_TAG_USER_DATA: u8 = 0x00;
const HSS1394_TAG_CHANGE_ADDRESS: u8 = 0xf1;

const RCODE_ADDRESS_ERROR: c_int = 0;
const RCODE_TYPE_ERROR: c_int = 0;
const RCODE_COMPLETE: c_int = 0;
const TCODE_WRITE_QUADLET_REQUEST: c_int = 0;
const TCODE_WRITE_BLOCK_REQUEST: c_int = 0;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 0;
const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 0;

#[repr(C)]
pub struct fw_address_handler {
    pub offset: c_ulonglong,
    pub length: size_t,
    pub address_callback: Option<
        unsafe extern "C" fn(
            *mut fw_card,
            *mut fw_request,
            c_int,
            c_int,
            c_int,
            c_int,
            c_ulonglong,
            *mut c_void,
            size_t,
            *mut c_void,
        ),
    >,
    pub callback_data: *mut c_void,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub rmidi: *mut snd_rawmidi,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_transaction {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_device {
    pub generation: c_int,
    pub card: *mut fw_card,
    pub node_id: c_int,
    pub max_speed: c_int,
}

#[repr(C)]
pub struct fw_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub card_dev: device,
    pub shortname: [core::ffi::c_char; 32],
}

#[repr(C)]
pub struct snd_oxfw {
    pub spec: *mut c_void,
    pub unit: *mut fw_unit,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_rawmidi)>,
    pub name: [core::ffi::c_char; 80],
    pub info_flags: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
    pub drain: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
}

#[repr(C)]
pub struct fw_scs1x {
    pub hss_handler: fw_address_handler,
    pub input_escape_count: u8,
    pub input: *mut snd_rawmidi_substream,

    /* For MIDI playback. */
    pub output: *mut snd_rawmidi_substream,
    pub output_idle: bool_t,
    pub output_status: u8,
    pub output_bytes: u8,
    pub output_escaped: bool_t,
    pub output_escape_high_nibble: bool_t,
    pub work: work_struct,
    pub idle_wait: wait_queue_head_t,
    pub buffer: [u8; HSS1394_MAX_PACKET_SIZE],
    pub transaction_running: bool_t,
    pub transaction: fw_transaction,
    pub transaction_bytes: c_uint,
    pub error: bool_t,
    pub fw_dev: *mut fw_device,
}

static SYSEX_ESCAPE_PREFIX: [u8; 7] = [
    0xf0,             /* SysEx begin */
    0x00, 0x01, 0x60, /* Stanton DJ */
    0x48, 0x53, 0x53, /* "HSS" */
];

unsafe extern "C" {
    static fw_high_memory_region: c_void;

    fn snd_rawmidi_receive(stream: *mut snd_rawmidi_substream, buffer: *const u8, count: size_t) -> c_int;
    fn snd_rawmidi_transmit(stream: *mut snd_rawmidi_substream, buffer: *mut u8, count: size_t) -> c_int;
    fn fw_send_response(card: *mut fw_card, request: *mut fw_request, rcode: c_int);
    fn rcode_is_permanent_error(rcode: c_int) -> bool_t;
    fn schedule_work(work: *mut work_struct) -> bool_t;
    fn wake_up(wait: *mut wait_queue_head_t);
    fn smp_rmb();
    fn fw_send_request(
        card: *mut fw_card,
        transaction: *mut fw_transaction,
        tcode: c_int,
        node_id: c_int,
        generation: c_int,
        speed: c_int,
        offset: u64,
        payload: *mut u8,
        length: c_uint,
        callback: unsafe extern "C" fn(*mut fw_card, c_int, *mut c_void, size_t, *mut c_void),
        callback_data: *mut c_void,
    );
    fn cpu_to_be64(value: u64) -> __be64;
    fn snd_fw_transaction(
        unit: *mut fw_unit,
        tcode: c_int,
        offset: u64,
        buffer: *mut __be64,
        length: size_t,
        flags: c_int,
    ) -> c_int;
    fn fw_core_remove_address_handler(handler: *mut fw_address_handler);
    fn fw_core_add_address_handler(handler: *mut fw_address_handler, region: *const c_void) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const core::ffi::c_char,
        device: c_int,
        output_count: c_int,
        input_count: c_int,
        rmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snprintf(
        s: *mut core::ffi::c_char,
        maxlen: size_t,
        format: *const core::ffi::c_char,
        ...
    ) -> c_int;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn init_waitqueue_head(wait: *mut wait_queue_head_t);
    fn wait_event(wait: *mut wait_queue_head_t, condition: bool_t);
}

unsafe fn midi_input_escaped_byte(stream: *mut snd_rawmidi_substream, byte: u8) {
    let mut nibbles: [u8; 2] = [0; 2];

    nibbles[0] = byte >> 4;
    nibbles[1] = byte & 0x0f;
    unsafe {
        snd_rawmidi_receive(stream, nibbles.as_ptr(), 2);
    }
}

unsafe fn midi_input_byte(scs: *mut fw_scs1x, stream: *mut snd_rawmidi_substream, byte: u8) {
    let eox: u8 = 0xf7;

    unsafe {
        if (*scs).input_escape_count > 0 {
            midi_input_escaped_byte(stream, byte);
            (*scs).input_escape_count = (*scs).input_escape_count.wrapping_sub(1);
            if (*scs).input_escape_count == 0 {
                snd_rawmidi_receive(stream, &eox, size_of::<u8>());
            }
        } else if byte == 0xf9 {
            snd_rawmidi_receive(stream, SYSEX_ESCAPE_PREFIX.as_ptr(), SYSEX_ESCAPE_PREFIX.len());
            midi_input_escaped_byte(stream, 0x00);
            midi_input_escaped_byte(stream, 0xf9);
            (*scs).input_escape_count = 3;
        } else {
            snd_rawmidi_receive(stream, &byte, 1);
        }
    }
}

unsafe fn midi_input_packet(
    scs: *mut fw_scs1x,
    stream: *mut snd_rawmidi_substream,
    data: *const u8,
    bytes: c_uint,
) {
    let mut i: c_uint;
    let eox: u8 = 0xf7;

    unsafe {
        if *data.add(0) == HSS1394_TAG_USER_DATA {
            i = 1;
            while i < bytes {
                midi_input_byte(scs, stream, *data.add(i as usize));
                i += 1;
            }
        } else {
            snd_rawmidi_receive(stream, SYSEX_ESCAPE_PREFIX.as_ptr(), SYSEX_ESCAPE_PREFIX.len());
            i = 0;
            while i < bytes {
                midi_input_escaped_byte(stream, *data.add(i as usize));
                i += 1;
            }
            snd_rawmidi_receive(stream, &eox, size_of::<u8>());
        }
    }
}

unsafe extern "C" fn handle_hss(
    card: *mut fw_card,
    request: *mut fw_request,
    tcode: c_int,
    _destination: c_int,
    _source: c_int,
    _generation: c_int,
    offset: c_ulonglong,
    data: *mut c_void,
    length: size_t,
    callback_data: *mut c_void,
) {
    let scs = callback_data as *mut fw_scs1x;
    let stream: *mut snd_rawmidi_substream;
    let rcode: c_int;

    unsafe {
        if offset != (*scs).hss_handler.offset {
            rcode = RCODE_ADDRESS_ERROR;
        } else if tcode != TCODE_WRITE_QUADLET_REQUEST && tcode != TCODE_WRITE_BLOCK_REQUEST {
            rcode = RCODE_TYPE_ERROR;
        } else {
            if length >= 1 {
                stream = ptr::read_volatile(&(*scs).input);
                if !stream.is_null() {
                    midi_input_packet(scs, stream, data as *const u8, length as c_uint);
                }
            }
            rcode = RCODE_COMPLETE;
        }
        fw_send_response(card, request, rcode);
    }
}

unsafe extern "C" fn scs_write_callback(
    _card: *mut fw_card,
    rcode: c_int,
    _data: *mut c_void,
    _length: size_t,
    callback_data: *mut c_void,
) {
    let scs = callback_data as *mut fw_scs1x;

    unsafe {
        if !rcode_is_permanent_error(rcode) {
            /* Don't retry for this data. */
            if rcode == RCODE_COMPLETE {
                (*scs).transaction_bytes = 0;
            }
        } else {
            (*scs).error = true;
        }

        (*scs).transaction_running = false;
        schedule_work(&mut (*scs).work);
    }
}

fn is_valid_running_status(status: u8) -> bool {
    status >= 0x80 && status <= 0xef
}

fn is_one_byte_cmd(status: u8) -> bool {
    status == 0xf6 || status >= 0xf8
}

fn is_two_bytes_cmd(status: u8) -> bool {
    (status >= 0xc0 && status <= 0xdf) || status == 0xf1 || status == 0xf3
}

fn is_three_bytes_cmd(status: u8) -> bool {
    (status >= 0x80 && status <= 0xbf) || (status >= 0xe0 && status <= 0xef) || status == 0xf2
}

fn is_invalid_cmd(status: u8) -> bool {
    status == 0xf4 || status == 0xf5 || status == 0xf9 || status == 0xfd
}

unsafe extern "C" fn scs_output_work(work: *mut work_struct) {
    let scs = work as *mut fw_scs1x;
    let stream: *mut snd_rawmidi_substream;
    let mut i: c_uint;
    let mut byte: u8 = 0;
    let generation: c_int;

    unsafe {
        if (*scs).transaction_running {
            return;
        }

        stream = ptr::read_volatile(&(*scs).output);
        if stream.is_null() || (*scs).error {
            (*scs).output_idle = true;
            wake_up(&mut (*scs).idle_wait);
            return;
        }

        if (*scs).transaction_bytes > 0 {
            goto_retry(scs);
            return;
        }

        i = (*scs).output_bytes as c_uint;
        loop {
            if snd_rawmidi_transmit(stream, &mut byte, 1) != 1 {
                (*scs).output_bytes = i as u8;
                (*scs).output_idle = true;
                wake_up(&mut (*scs).idle_wait);
                return;
            }
            /*
             * Convert from real MIDI to what I think the device expects (no
             * running status, one command per packet, unescaped SysExs).
             */
            if (*scs).output_escaped && byte < 0x80 {
                if (*scs).output_escape_high_nibble {
                    if (i as usize) < HSS1394_MAX_PACKET_SIZE {
                        (*scs).buffer[i as usize] = byte << 4;
                        (*scs).output_escape_high_nibble = false;
                    }
                } else {
                    (*scs).buffer[i as usize] |= byte & 0x0f;
                    i += 1;
                    (*scs).output_escape_high_nibble = true;
                }
            } else if byte < 0x80 {
                if i == 1 {
                    if !is_valid_running_status((*scs).output_status) {
                        continue;
                    }
                    (*scs).buffer[0] = HSS1394_TAG_USER_DATA;
                    (*scs).buffer[i as usize] = (*scs).output_status;
                    i += 1;
                }
                (*scs).buffer[i as usize] = byte;
                i += 1;
                if (i == 3 && is_two_bytes_cmd((*scs).output_status))
                    || (i == 4 && is_three_bytes_cmd((*scs).output_status))
                {
                    break;
                }
                if (i as usize) == 1 + SYSEX_ESCAPE_PREFIX.len()
                    && libc_memcmp(
                        (*scs).buffer.as_ptr().add(1) as *const c_void,
                        SYSEX_ESCAPE_PREFIX.as_ptr() as *const c_void,
                        SYSEX_ESCAPE_PREFIX.len(),
                    ) == 0
                {
                    (*scs).output_escaped = true;
                    (*scs).output_escape_high_nibble = true;
                    i = 0;
                }
                if (i as usize) >= HSS1394_MAX_PACKET_SIZE {
                    i = 1;
                }
            } else if byte == 0xf7 {
                if (*scs).output_escaped {
                    if i >= 1
                        && (*scs).output_escape_high_nibble
                        && (*scs).buffer[0] != HSS1394_TAG_CHANGE_ADDRESS
                    {
                        break;
                    }
                } else if i > 1 && (*scs).output_status == 0xf0 {
                    (*scs).buffer[i as usize] = 0xf7;
                    i += 1;
                    break;
                }
                i = 1;
                (*scs).output_escaped = false;
            } else if !is_invalid_cmd(byte) && byte < 0xf8 {
                i = 1;
                (*scs).buffer[0] = HSS1394_TAG_USER_DATA;
                (*scs).buffer[i as usize] = byte;
                i += 1;
                (*scs).output_status = byte;
                (*scs).output_escaped = false;
                if is_one_byte_cmd(byte) {
                    break;
                }
            }
        }
        (*scs).output_bytes = 1;
        (*scs).output_escaped = false;

        (*scs).transaction_bytes = i;
        generation = (*(*scs).fw_dev).generation;
        smp_rmb(); /* node_id vs. generation */
        (*scs).transaction_running = true;
        fw_send_request(
            (*(*scs).fw_dev).card,
            &mut (*scs).transaction,
            TCODE_WRITE_BLOCK_REQUEST,
            (*(*scs).fw_dev).node_id,
            generation,
            (*(*scs).fw_dev).max_speed,
            HSS1394_ADDRESS,
            (*scs).buffer.as_mut_ptr(),
            (*scs).transaction_bytes,
            scs_write_callback,
            scs as *mut c_void,
        );
    }
}

unsafe fn goto_retry(scs: *mut fw_scs1x) {
    unsafe {
        (*scs).transaction_running = true;
        let generation = (*(*scs).fw_dev).generation;
        smp_rmb(); /* node_id vs. generation */
        fw_send_request(
            (*(*scs).fw_dev).card,
            &mut (*scs).transaction,
            TCODE_WRITE_BLOCK_REQUEST,
            (*(*scs).fw_dev).node_id,
            generation,
            (*(*scs).fw_dev).max_speed,
            HSS1394_ADDRESS,
            (*scs).buffer.as_mut_ptr(),
            (*scs).transaction_bytes,
            scs_write_callback,
            scs as *mut c_void,
        );
    }
}

unsafe extern "C" {
    fn libc_memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
}

unsafe extern "C" fn midi_capture_open(_stream: *mut snd_rawmidi_substream) -> c_int {
    0
}

unsafe extern "C" fn midi_capture_close(_stream: *mut snd_rawmidi_substream) -> c_int {
    0
}

unsafe extern "C" fn midi_capture_trigger(stream: *mut snd_rawmidi_substream, up: c_int) {
    unsafe {
        let scs = (*(*stream).rmidi).private_data as *mut fw_scs1x;

        if up != 0 {
            (*scs).input_escape_count = 0;
            ptr::write_volatile(&mut (*scs).input, stream);
        } else {
            ptr::write_volatile(&mut (*scs).input, ptr::null_mut());
        }
    }
}

unsafe extern "C" fn midi_playback_open(_stream: *mut snd_rawmidi_substream) -> c_int {
    0
}

unsafe extern "C" fn midi_playback_close(_stream: *mut snd_rawmidi_substream) -> c_int {
    0
}

unsafe extern "C" fn midi_playback_trigger(stream: *mut snd_rawmidi_substream, up: c_int) {
    unsafe {
        let scs = (*(*stream).rmidi).private_data as *mut fw_scs1x;

        if up != 0 {
            (*scs).output_status = 0;
            (*scs).output_bytes = 1;
            (*scs).output_escaped = false;
            (*scs).output_idle = false;
            (*scs).transaction_bytes = 0;
            (*scs).error = false;

            ptr::write_volatile(&mut (*scs).output, stream);
            schedule_work(&mut (*scs).work);
        } else {
            ptr::write_volatile(&mut (*scs).output, ptr::null_mut());
        }
    }
}

unsafe extern "C" fn midi_playback_drain(stream: *mut snd_rawmidi_substream) {
    unsafe {
        let scs = (*(*stream).rmidi).private_data as *mut fw_scs1x;

        wait_event(&mut (*scs).idle_wait, (*scs).output_idle);
    }
}

unsafe fn register_address(oxfw: *mut snd_oxfw) -> c_int {
    unsafe {
        let scs = (*oxfw).spec as *mut fw_scs1x;
        let mut data: __be64;

        data = cpu_to_be64(((HSS1394_TAG_CHANGE_ADDRESS as u64) << 56) | (*scs).hss_handler.offset);
        snd_fw_transaction(
            (*oxfw).unit,
            TCODE_WRITE_BLOCK_REQUEST,
            HSS1394_ADDRESS,
            &mut data,
            size_of::<__be64>(),
            0,
        )
    }
}

unsafe extern "C" fn remove_scs1x(rmidi: *mut snd_rawmidi) {
    unsafe {
        let scs = (*rmidi).private_data as *mut fw_scs1x;

        fw_core_remove_address_handler(&mut (*scs).hss_handler);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_oxfw_scs1x_update(oxfw: *mut snd_oxfw) {
    unsafe {
        register_address(oxfw);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_oxfw_scs1x_add(oxfw: *mut snd_oxfw) -> c_int {
    static MIDI_CAPTURE_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_capture_open),
        close: Some(midi_capture_close),
        trigger: Some(midi_capture_trigger),
        drain: None,
    };
    static MIDI_PLAYBACK_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_playback_open),
        close: Some(midi_playback_close),
        trigger: Some(midi_playback_trigger),
        drain: Some(midi_playback_drain),
    };
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let scs: *mut fw_scs1x;
    let mut err: c_int;

    unsafe {
        scs = devm_kzalloc(
            &mut (*(*oxfw).card).card_dev,
            size_of::<fw_scs1x>(),
            GFP_KERNEL,
        ) as *mut fw_scs1x;
        if scs.is_null() {
            return -ENOMEM;
        }
        (*scs).fw_dev = fw_parent_device((*oxfw).unit);
        (*oxfw).spec = scs as *mut c_void;

        /* Allocate own handler for imcoming asynchronous transaction. */
        (*scs).hss_handler.length = HSS1394_MAX_PACKET_SIZE;
        (*scs).hss_handler.address_callback = Some(handle_hss);
        (*scs).hss_handler.callback_data = scs as *mut c_void;
        err = fw_core_add_address_handler(
            &mut (*scs).hss_handler,
            &fw_high_memory_region as *const c_void,
        );
        if err < 0 {
            return err;
        }

        err = register_address(oxfw);
        if err < 0 {
            fw_core_remove_address_handler(&mut (*scs).hss_handler);
            return err;
        }

        /* Use unique name for backward compatibility to scs1x module. */
        err = snd_rawmidi_new(
            (*oxfw).card,
            c"SCS.1x".as_ptr(),
            0,
            1,
            1,
            &mut rmidi,
        );
        if err < 0 {
            fw_core_remove_address_handler(&mut (*scs).hss_handler);
            return err;
        }
        (*rmidi).private_data = scs as *mut c_void;
        (*rmidi).private_free = Some(remove_scs1x);

        snprintf(
            (*rmidi).name.as_mut_ptr(),
            (*rmidi).name.len(),
            c"%s MIDI".as_ptr(),
            (*(*oxfw).card).shortname.as_ptr(),
        );

        (*rmidi).info_flags =
            SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_DUPLEX;
        snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &MIDI_CAPTURE_OPS);
        snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &MIDI_PLAYBACK_OPS);

        INIT_WORK(&mut (*scs).work, scs_output_work);
        init_waitqueue_head(&mut (*scs).idle_wait);
        (*scs).output_idle = true;

        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
