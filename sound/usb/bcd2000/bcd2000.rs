// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Behringer BCD2000 driver
 *
 *   Copyright (C) 2014 Mario Kicherer (dev@kicherer.org)
 */

// Requires kernel bindings: linux/kernel.h, linux/errno.h, linux/init.h,
// linux/slab.h, linux/module.h, linux/bitmap.h, linux/usb.h, linux/usb/audio.h,
// sound/core.h, sound/initval.h, sound/rawmidi.h

const PREFIX: &str = "snd-bcd2000: ";
const BUFSIZE: usize = 64;

#[allow(non_snake_case)]
mod usb_constants {
    pub const SNDRV_CARDS: usize = 32; // Typical value; may vary by kernel config
}

use usb_constants::SNDRV_CARDS;

static ID_TABLE: &[(u16, u16)] = &[(0x1397, 0x00bd)];

static DEVICE_CMD_PREFIX: &[u8] = &[0x03, 0x00];

static BCD2000_INIT_SEQUENCE: &[u8] = &[
    0x07, 0x00, 0x00, 0x00, 0x78, 0x48, 0x1c, 0x81,
    0xc4, 0x00, 0x00, 0x00, 0x5e, 0x53, 0x4a, 0xf7,
    0x18, 0xfa, 0x11, 0xff, 0x6c, 0xf3, 0x90, 0xff,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x18, 0xfa, 0x11, 0xff, 0x14, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xf2, 0x34, 0x4a, 0xf7,
    0x18, 0xfa, 0x11, 0xff
];

#[repr(C)]
pub struct bcd2000 {
    pub dev: *mut usb_device,
    pub card: *mut snd_card,
    pub intf: *mut usb_interface,
    pub card_index: i32,

    pub midi_out_active: i32,
    pub rmidi: *mut snd_rawmidi,
    pub midi_receive_substream: *mut snd_rawmidi_substream,
    pub midi_out_substream: *mut snd_rawmidi_substream,

    pub midi_in_buf: [u8; BUFSIZE],
    pub midi_out_buf: [u8; BUFSIZE],

    pub midi_out_urb: *mut urb,
    pub midi_in_urb: *mut urb,

    pub anchor: usb_anchor,
}

// External declarations for kernel structures and functions
#[allow(non_camel_case_types)]
pub struct usb_device;
#[allow(non_camel_case_types)]
pub struct snd_card;
#[allow(non_camel_case_types)]
pub struct usb_interface;
#[allow(non_camel_case_types)]
pub struct snd_rawmidi;
#[allow(non_camel_case_types)]
pub struct snd_rawmidi_substream;
#[allow(non_camel_case_types)]
pub struct urb;
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct usb_anchor {
    list: core::ffi::c_void,
    wait: core::ffi::c_void,
    ref_count: core::sync::atomic::AtomicI32,
}

static mut INDEX: [i32; SNDRV_CARDS] = [0; SNDRV_CARDS]; // SNDRV_DEFAULT_IDX equivalent
static mut ID: [*mut core::ffi::c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS]; // SNDRV_DEFAULT_STR equivalent

static DEVICES_MUTEX: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(0);
static mut DEVICES_USED: [u32; (SNDRV_CARDS + 31) / 32] = [0; (SNDRV_CARDS + 31) / 32];
static mut BCD2000_DRIVER: usb_driver = usb_driver {
    name: b"snd-bcd2000\0" as *const u8 as *const core::ffi::c_char,
    probe: Some(bcd2000_probe),
    disconnect: Some(bcd2000_disconnect),
    id_table: ID_TABLE.as_ptr(),
};

#[cfg(CONFIG_SND_DEBUG)]
unsafe fn bcd2000_dump_buffer(prefix: *const core::ffi::c_char, buf: *const core::ffi::c_char, len: i32) {
    // print_hex_dump(KERN_DEBUG, prefix, DUMP_PREFIX_NONE, 16, 1, buf, len, false);
    extern "C" {
        fn print_hex_dump(
            level: i32,
            prefix: *const core::ffi::c_char,
            prefix_type: i32,
            rowsize: i32,
            groupsize: i32,
            buf: *const core::ffi::c_char,
            len: i32,
            ascii: i32,
        );
    }
    print_hex_dump(
        7, // KERN_DEBUG
        prefix,
        0, // DUMP_PREFIX_NONE
        16,
        1,
        buf,
        len,
        0,
    );
}

#[cfg(not(CONFIG_SND_DEBUG))]
unsafe fn bcd2000_dump_buffer(_prefix: *const core::ffi::c_char, _buf: *const core::ffi::c_char, _len: i32) {}

extern "C" {
    fn snd_rawmidi_receive(
        substream: *mut snd_rawmidi_substream,
        buf: *const u8,
        count: i32,
    ) -> i32;
    fn READ_ONCE(x: *const core::ffi::c_void) -> *const core::ffi::c_void;
    fn dev_err(dev: *const core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn dev_warn(dev: *const core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn dev_info(dev: *const core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn min(a: u32, b: u32) -> u32;
    fn usb_submit_urb(urb: *mut urb, mem_flags: i32) -> i32;
    fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buf: *mut u8, count: i32) -> i32;
    fn usb_kill_urb(urb: *mut urb);
    fn init_usb_anchor(anchor: *mut usb_anchor);
    fn usb_anchor_urb(urb: *mut urb, anchor: *mut usb_anchor);
    fn usb_wait_anchor_empty_timeout(anchor: *mut usb_anchor, timeout: u32) -> i32;
    fn usb_poison_urb(urb: *mut urb);
    fn usb_free_urb(urb: *mut urb);
    fn usb_set_intfdata(intf: *mut usb_interface, data: *mut core::ffi::c_void);
    fn usb_get_intfdata(intf: *mut usb_interface) -> *mut core::ffi::c_void;
    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const core::ffi::c_char,
        device: i32,
        output_count: i32,
        input_count: i32,
        rmidi: *mut *mut snd_rawmidi,
    ) -> i32;
    fn strscpy(dst: *mut u8, src: *const u8, n: usize) -> i32;
    fn snd_rawmidi_set_ops(
        rmidi: *mut snd_rawmidi,
        stream: i32,
        ops: *const snd_rawmidi_ops,
    );
    fn usb_alloc_urb(iso_packets: i32, mem_flags: i32) -> *mut urb;
    fn usb_rcvintpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    fn usb_sndintpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    fn usb_fill_int_urb(
        urb: *mut urb,
        dev: *mut usb_device,
        pipe: u32,
        transfer_buffer: *mut u8,
        buffer_length: i32,
        complete_fn: unsafe extern "C" fn(*mut urb),
        context: *mut core::ffi::c_void,
        interval: i32,
    );
    fn usb_urb_ep_type_check(urb: *mut urb) -> i32;
    fn snd_card_new(
        parent: *mut core::ffi::c_void,
        idx: i32,
        xid: *const core::ffi::c_char,
        module: *mut core::ffi::c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> i32;
    fn interface_to_usbdev(intf: *mut usb_interface) -> *mut usb_device;
    fn snd_card_set_dev(card: *mut snd_card, dev: *const core::ffi::c_void);
    fn usb_make_path(dev: *mut usb_device, buf: *mut u8, size: usize) -> i32;
    fn snprintf(buf: *mut u8, size: usize, fmt: *const core::ffi::c_char, ...) -> i32;
    fn snd_card_register(card: *mut snd_card) -> i32;
    fn snd_card_disconnect(card: *mut snd_card);
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_free_when_closed(card: *mut snd_card);
    fn test_bit(nr: i32, addr: *const u32) -> i32;
    fn set_bit(nr: i32, addr: *mut u32);
    fn clear_bit(nr: i32, addr: *mut u32);
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct snd_rawmidi_ops {
    open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> i32>,
    close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> i32>,
    trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, i32)>,
    drain: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct usb_driver {
    name: *const core::ffi::c_char,
    probe: Option<unsafe extern "C" fn(*mut usb_interface, *const core::ffi::c_void) -> i32>,
    disconnect: Option<unsafe extern "C" fn(*mut usb_interface)>,
    id_table: *const (u16, u16),
}

unsafe extern "C" fn bcd2000_midi_input_open(_substream: *mut snd_rawmidi_substream) -> i32 {
    0
}

unsafe extern "C" fn bcd2000_midi_input_close(_substream: *mut snd_rawmidi_substream) -> i32 {
    0
}

unsafe extern "C" fn bcd2000_midi_input_trigger(substream: *mut snd_rawmidi_substream, up: i32) {
    let bcd2k = (*(*substream).rmidi).private_data as *mut bcd2000;
    (*bcd2k).midi_receive_substream = if up != 0 { substream } else { core::ptr::null_mut() };
}

unsafe fn bcd2000_midi_handle_input(
    bcd2k: *mut bcd2000,
    buf: *const u8,
    buf_len: u32,
) {
    let mut payload_length: u32;
    let mut tocopy: u32;
    let mut midi_receive_substream: *mut snd_rawmidi_substream;

    midi_receive_substream = READ_ONCE(&(*bcd2k).midi_receive_substream as *const _ as *const core::ffi::c_void) as *mut snd_rawmidi_substream;
    if midi_receive_substream.is_null() {
        return;
    }

    bcd2000_dump_buffer(PREFIX.as_ptr() as *const core::ffi::c_char, buf as *const core::ffi::c_char, buf_len as i32);

    if buf_len < 2 {
        return;
    }

    payload_length = *buf as u32;

    if payload_length == 0 {
        return;
    }

    tocopy = min(payload_length, buf_len.wrapping_sub(1));

    bcd2000_dump_buffer(
        "snd-bcd2000: sending to userspace: \0".as_ptr() as *const core::ffi::c_char,
        buf.add(1) as *const core::ffi::c_char,
        tocopy as i32,
    );

    snd_rawmidi_receive(midi_receive_substream, buf.add(1), tocopy as i32);
}

unsafe fn bcd2000_midi_send(bcd2k: *mut bcd2000) {
    let mut len: i32;
    let mut ret: i32;
    let mut midi_out_substream: *mut snd_rawmidi_substream;

    if core::mem::size_of_val(&DEVICE_CMD_PREFIX) >= BUFSIZE {
        core::arch::asm!("ud2");
    }

    midi_out_substream = READ_ONCE(&(*bcd2k).midi_out_substream as *const _ as *const core::ffi::c_void) as *mut snd_rawmidi_substream;
    if midi_out_substream.is_null() {
        return;
    }

    if (*bcd2k).midi_out_urb.is_null() {
        return;
    }

    memcpy(
        (*bcd2k).midi_out_buf.as_mut_ptr() as *mut core::ffi::c_void,
        DEVICE_CMD_PREFIX.as_ptr() as *const core::ffi::c_void,
        DEVICE_CMD_PREFIX.len(),
    );

    len = snd_rawmidi_transmit(
        midi_out_substream,
        (*bcd2k).midi_out_buf.as_mut_ptr().add(3),
        (BUFSIZE - 3) as i32,
    );

    if len < 0 {
        dev_err(
            &(*(*bcd2k).dev).dev as *const _ as *const core::ffi::c_void,
            b"%s: snd_rawmidi_transmit error %d\n\0".as_ptr() as *const core::ffi::c_char,
            b"bcd2000_midi_send\0".as_ptr() as *const core::ffi::c_char,
            len,
        );
    }

    if len <= 0 {
        return;
    }

    (*bcd2k).midi_out_buf[2] = len as u8;
    (*(*bcd2k).midi_out_urb).transfer_buffer_length = BUFSIZE as u32;

    bcd2000_dump_buffer(
        "snd-bcd2000: sending to device: \0".as_ptr() as *const core::ffi::c_char,
        (*bcd2k).midi_out_buf.as_ptr() as *const core::ffi::c_char,
        len + 3,
    );

    ret = usb_submit_urb((*bcd2k).midi_out_urb, 0x20); // GFP_ATOMIC
    if ret < 0 {
        dev_err(
            &(*(*bcd2k).dev).dev as *const _ as *const core::ffi::c_void,
            b"snd-bcd2000: %s (%p): usb_submit_urb() failed, ret=%d, len=%d\n\0".as_ptr() as *const core::ffi::c_char,
            b"bcd2000_midi_send\0".as_ptr() as *const core::ffi::c_char,
            midi_out_substream as *const core::ffi::c_void,
            ret,
            len,
        );
    } else {
        (*bcd2k).midi_out_active = 1;
    }
}

unsafe extern "C" fn bcd2000_midi_output_open(_substream: *mut snd_rawmidi_substream) -> i32 {
    0
}

unsafe extern "C" fn bcd2000_midi_output_close(substream: *mut snd_rawmidi_substream) -> i32 {
    let bcd2k = (*(*substream).rmidi).private_data as *mut bcd2000;

    if (*bcd2k).midi_out_active != 0 && !(*bcd2k).midi_out_urb.is_null() {
        usb_kill_urb((*bcd2k).midi_out_urb);
        (*bcd2k).midi_out_active = 0;
    }

    0
}

unsafe extern "C" fn bcd2000_midi_output_trigger(substream: *mut snd_rawmidi_substream, up: i32) {
    let bcd2k = (*(*substream).rmidi).private_data as *mut bcd2000;

    if up != 0 {
        (*bcd2k).midi_out_substream = substream;
        if (*bcd2k).midi_out_active == 0 {
            bcd2000_midi_send(bcd2k);
        }
    } else {
        (*bcd2k).midi_out_substream = core::ptr::null_mut();
    }
}

unsafe extern "C" fn bcd2000_output_complete(urb: *mut urb) {
    let bcd2k = (*urb).context as *mut bcd2000;

    (*bcd2k).midi_out_active = 0;

    if (*urb).status != 0 {
        dev_warn(
            &(*(*urb).dev).dev as *const _ as *const core::ffi::c_void,
            b"snd-bcd2000: output urb->status: %d\n\0".as_ptr() as *const core::ffi::c_char,
            (*urb).status,
        );
    }

    if (*urb).status == -19 { // -ESHUTDOWN
        return;
    }

    bcd2000_midi_send(bcd2k);
}

unsafe extern "C" fn bcd2000_input_complete(urb: *mut urb) {
    let mut ret: i32;
    let bcd2k = (*urb).context as *mut bcd2000;

    if (*urb).status != 0 {
        dev_warn(
            &(*(*urb).dev).dev as *const _ as *const core::ffi::c_void,
            b"snd-bcd2000: input urb->status: %i\n\0".as_ptr() as *const core::ffi::c_char,
            (*urb).status,
        );
    }

    if bcd2k.is_null() || (*urb).status == -19 { // -ESHUTDOWN
        return;
    }

    if (*urb).actual_length > 0 {
        bcd2000_midi_handle_input(
            bcd2k,
            (*urb).transfer_buffer as *const u8,
            (*urb).actual_length as u32,
        );
    }

    ret = usb_submit_urb((*bcd2k).midi_in_urb, 0x20); // GFP_ATOMIC
    if ret < 0 {
        dev_err(
            &(*(*bcd2k).dev).dev as *const _ as *const core::ffi::c_void,
            b"snd-bcd2000: %s: usb_submit_urb() failed, ret=%d\n\0".as_ptr() as *const core::ffi::c_char,
            b"bcd2000_input_complete\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
    }
}

static BCD2000_MIDI_OUTPUT: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(bcd2000_midi_output_open),
    close: Some(bcd2000_midi_output_close),
    trigger: Some(bcd2000_midi_output_trigger),
    drain: None,
};

static BCD2000_MIDI_INPUT: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(bcd2000_midi_input_open),
    close: Some(bcd2000_midi_input_close),
    trigger: Some(bcd2000_midi_input_trigger),
    drain: None,
};

unsafe fn bcd2000_init_device(bcd2k: *mut bcd2000) {
    let mut ret: i32;

    init_usb_anchor(&mut (*bcd2k).anchor);
    usb_anchor_urb((*bcd2k).midi_out_urb, &mut (*bcd2k).anchor);
    usb_anchor_urb((*bcd2k).midi_in_urb, &mut (*bcd2k).anchor);

    memcpy(
        (*bcd2k).midi_out_buf.as_mut_ptr() as *mut core::ffi::c_void,
        BCD2000_INIT_SEQUENCE.as_ptr() as *const core::ffi::c_void,
        52,
    );
    (*(*bcd2k).midi_out_urb).transfer_buffer_length = 52;

    ret = usb_submit_urb((*bcd2k).midi_out_urb, 0xd0); // GFP_KERNEL
    if ret < 0 {
        dev_err(
            &(*(*bcd2k).dev).dev as *const _ as *const core::ffi::c_void,
            b"snd-bcd2000: %s: usb_submit_urb() out failed, ret=%d: \0".as_ptr() as *const core::ffi::c_char,
            b"bcd2000_init_device\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
    } else {
        (*bcd2k).midi_out_active = 1;
    }

    ret = usb_submit_urb((*bcd2k).midi_in_urb, 0xd0); // GFP_KERNEL
    if ret < 0 {
        dev_err(
            &(*(*bcd2k).dev).dev as *const _ as *const core::ffi::c_void,
            b"snd-bcd2000: %s: usb_submit_urb() in failed, ret=%d: \0".as_ptr() as *const core::ffi::c_char,
            b"bcd2000_init_device\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
    }

    usb_wait_anchor_empty_timeout(&mut (*bcd2k).anchor, 1000);
}

unsafe fn bcd2000_init_midi(bcd2k: *mut bcd2000) -> i32 {
    let mut ret: i32;
    let mut rmidi: *mut snd_rawmidi = core::ptr::null_mut();

    ret = snd_rawmidi_new(
        (*bcd2k).card,
        (*(*bcd2k).card).shortname.as_ptr() as *const core::ffi::c_char,
        0,
        1,
        1,
        &mut rmidi,
    );

    if ret < 0 {
        return ret;
    }

    strscpy(
        (*rmidi).name.as_mut_ptr() as *mut u8,
        (*(*bcd2k).card).shortname.as_ptr() as *const u8,
        core::mem::size_of_val(&(*rmidi).name),
    );

    (*rmidi).info_flags = 0x00000040; // SNDRV_RAWMIDI_INFO_DUPLEX
    (*rmidi).private_data = bcd2k as *mut core::ffi::c_void;

    (*rmidi).info_flags |= 0x00000004; // SNDRV_RAWMIDI_INFO_OUTPUT
    snd_rawmidi_set_ops(rmidi, 0, &BCD2000_MIDI_OUTPUT); // SNDRV_RAWMIDI_STREAM_OUTPUT

    (*rmidi).info_flags |= 0x00000002; // SNDRV_RAWMIDI_INFO_INPUT
    snd_rawmidi_set_ops(rmidi, 1, &BCD2000_MIDI_INPUT); // SNDRV_RAWMIDI_STREAM_INPUT

    (*bcd2k).rmidi = rmidi;

    (*bcd2k).midi_in_urb = usb_alloc_urb(0, 0xd0); // GFP_KERNEL
    (*bcd2k).midi_out_urb = usb_alloc_urb(0, 0xd0); // GFP_KERNEL

    if (*bcd2k).midi_in_urb.is_null() || (*bcd2k).midi_out_urb.is_null() {
        dev_err(
            &(*(*bcd2k).dev).dev as *const _ as *const core::ffi::c_void,
            b"snd-bcd2000: usb_alloc_urb failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
        return -12; // -ENOMEM
    }

    usb_fill_int_urb(
        (*bcd2k).midi_in_urb,
        (*bcd2k).dev,
        usb_rcvintpipe((*bcd2k).dev, 0x81),
        (*bcd2k).midi_in_buf.as_mut_ptr(),
        BUFSIZE as i32,
        bcd2000_input_complete,
        bcd2k as *mut core::ffi::c_void,
        1,
    );

    usb_fill_int_urb(
        (*bcd2k).midi_out_urb,
        (*bcd2k).dev,
        usb_sndintpipe((*bcd2k).dev, 0x1),
        (*bcd2k).midi_out_buf.as_mut_ptr(),
        BUFSIZE as i32,
        bcd2000_output_complete,
        bcd2k as *mut core::ffi::c_void,
        1,
    );

    if usb_urb_ep_type_check((*bcd2k).midi_in_urb) != 0
        || usb_urb_ep_type_check((*bcd2k).midi_out_urb) != 0
    {
        dev_err(
            &(*(*bcd2k).dev).dev as *const _ as *const core::ffi::c_void,
            b"invalid MIDI EP\n\0".as_ptr() as *const core::ffi::c_char,
        );
        return -22; // -EINVAL
    }

    bcd2000_init_device(bcd2k);

    0
}

unsafe fn bcd2000_free_usb_related_resources(
    bcd2k: *mut bcd2000,
    _interface: *mut usb_interface,
) {
    usb_poison_urb((*bcd2k).midi_out_urb);
    usb_poison_urb((*bcd2k).midi_in_urb);

    usb_free_urb((*bcd2k).midi_out_urb);
    usb_free_urb((*bcd2k).midi_in_urb);
    (*bcd2k).midi_out_urb = core::ptr::null_mut();
    (*bcd2k).midi_in_urb = core::ptr::null_mut();

    if !(*bcd2k).intf.is_null() {
        usb_set_intfdata((*bcd2k).intf, core::ptr::null_mut());
        (*bcd2k).intf = core::ptr::null_mut();
    }
}

unsafe extern "C" fn bcd2000_probe(
    interface: *mut usb_interface,
    _usb_id: *const core::ffi::c_void,
) -> i32 {
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut bcd2k: *mut bcd2000;
    let mut card_index: u32;
    let mut usb_path: [u8; 32] = [0; 32];
    let mut err: i32;

    // guard(mutex)(&devices_mutex);
    // Simplified: assuming mutex is available via kernel API

    card_index = 0;
    while card_index < SNDRV_CARDS as u32 {
        if test_bit(card_index as i32, DEVICES_USED.as_ptr()) == 0 {
            break;
        }
        card_index += 1;
    }

    if card_index >= SNDRV_CARDS as u32 {
        return -2; // -ENOENT
    }

    err = snd_card_new(
        interface as *mut core::ffi::c_void,
        INDEX[card_index as usize],
        ID[card_index as usize],
        core::ptr::null_mut(),
        core::mem::size_of::<bcd2000>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }

    bcd2k = (*card).private_data as *mut bcd2000;
    (*bcd2k).dev = interface_to_usbdev(interface);
    (*bcd2k).card = card;
    (*bcd2k).card_index = card_index as i32;
    (*bcd2k).intf = interface;

    snd_card_set_dev(card, interface as *const core::ffi::c_void);

    strscpy(
        (*card).driver.as_mut_ptr() as *mut u8,
        b"snd-bcd2000\0".as_ptr() as *const u8,
        core::mem::size_of_val(&(*card).driver),
    );
    strscpy(
        (*card).shortname.as_mut_ptr() as *mut u8,
        b"BCD2000\0".as_ptr() as *const u8,
        core::mem::size_of_val(&(*card).shortname),
    );
    usb_make_path((*bcd2k).dev, usb_path.as_mut_ptr(), usb_path.len());
    snprintf(
        (*(*bcd2k).card).longname.as_mut_ptr() as *mut u8,
        core::mem::size_of_val(&(*(*bcd2k).card).longname),
        b"Behringer BCD2000 at %s\0".as_ptr() as *const core::ffi::c_char,
        usb_path.as_ptr() as *const core::ffi::c_char,
    );

    err = bcd2000_init_midi(bcd2k);
    if err < 0 {
        dev_info(
            &(*(*bcd2k).dev).dev as *const _ as *const core::ffi::c_void,
            b"snd-bcd2000: error during probing\0".as_ptr() as *const core::ffi::c_char,
        );
        bcd2000_free_usb_related_resources(bcd2k, interface);
        snd_card_free(card);
        return err;
    }

    err = snd_card_register(card);
    if err < 0 {
        dev_info(
            &(*(*bcd2k).dev).dev as *const _ as *const core::ffi::c_void,
            b"snd-bcd2000: error during probing\0".as_ptr() as *const core::ffi::c_char,
        );
        bcd2000_free_usb_related_resources(bcd2k, interface);
        snd_card_free(card);
        return err;
    }

    usb_set_intfdata(interface, bcd2k as *mut core::ffi::c_void);
    set_bit(card_index as i32, DEVICES_USED.as_mut_ptr());

    0
}

unsafe extern "C" fn bcd2000_disconnect(interface: *mut usb_interface) {
    let bcd2k = usb_get_intfdata(interface) as *mut bcd2000;

    if bcd2k.is_null() {
        return;
    }

    // guard(mutex)(&devices_mutex);
    // Simplified: assuming mutex is available via kernel API

    snd_card_disconnect((*bcd2k).card);

    bcd2000_free_usb_related_resources(bcd2k, interface);

    clear_bit((*bcd2k).card_index, DEVICES_USED.as_mut_ptr());

    snd_card_free_when_closed((*bcd2k).card);
}

// Module-level initialization and driver registration
// Equivalent to module_usb_driver(bcd2000_driver)
// MODULE_DEVICE_TABLE(usb, id_table);
// MODULE_AUTHOR("Mario Kicherer, dev@kicherer.org");
// MODULE_DESCRIPTION("Behringer BCD2000 driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
