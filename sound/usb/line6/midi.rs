// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Linux USB driver
 *
 * Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
 */

// Bindings to Linux kernel and ALSA sound APIs
// #include <linux/slab.h>
// #include <linux/usb.h>
// #include <linux/export.h>
// #include <sound/core.h>
// #include <sound/rawmidi.h>

// Local module dependencies from driver.h and midi.h
use crate::driver::{usb_line6, snd_line6_midi, midi_buffer};
use crate::midi::{LINE6_FALLBACK_MAXPACKETSIZE, LINE6_MIDIBUF_READ_TX, MIDI_BUFFER_SIZE, LINE6_CAP_CONTROL_MIDI};

// External kernel/ALSA FFI functions and types
extern "C" {
    fn snd_rawmidi_receive(
        substream: *mut std::ffi::c_void,
        data: *const u8,
        length: i32,
    ) -> i32;

    fn snd_rawmidi_transmit_peek(
        substream: *mut std::ffi::c_void,
        data: *mut u8,
        count: i32,
    ) -> i32;

    fn snd_rawmidi_transmit_ack(substream: *mut std::ffi::c_void, count: i32);

    fn line6_midibuf_bytes_free(mb: *mut midi_buffer) -> i32;
    fn line6_midibuf_write(mb: *mut midi_buffer, data: *const u8, length: i32);
    fn line6_midibuf_read(
        mb: *mut midi_buffer,
        data: *mut u8,
        length: i32,
        read_type: i32,
    ) -> i32;
    fn line6_midibuf_destroy(mb: *mut midi_buffer);
    fn line6_midibuf_init(mb: *mut midi_buffer, size: i32, split: i32) -> i32;

    fn usb_alloc_urb(iso_packets: i32, mem_flags: u32) -> *mut std::ffi::c_void;
    fn usb_free_urb(urb: *mut std::ffi::c_void);
    fn usb_submit_urb(urb: *mut std::ffi::c_void, mem_flags: u32) -> i32;
    fn usb_urb_ep_type_check(urb: *mut std::ffi::c_void) -> i32;
    fn usb_sndintpipe(dev: *mut std::ffi::c_void, endpoint: u8) -> u32;
    fn usb_fill_int_urb(
        urb: *mut std::ffi::c_void,
        dev: *mut std::ffi::c_void,
        pipe: u32,
        transfer_buffer: *mut u8,
        buffer_length: i32,
        complete: unsafe extern "C" fn(*mut std::ffi::c_void),
        context: *mut std::ffi::c_void,
        interval: i32,
    );

    fn kmemdup(src: *const u8, len: i32, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut std::ffi::c_void);
    fn kzalloc_obj(size: u32, flags: u32) -> *mut std::ffi::c_void;

    fn snd_rawmidi_new(
        card: *mut std::ffi::c_void,
        id: *const u8,
        device: i32,
        output_count: i32,
        input_count: i32,
        rmidi_ret: *mut *mut std::ffi::c_void,
    ) -> i32;
    fn snd_rawmidi_set_ops(
        rmidi: *mut std::ffi::c_void,
        stream: i32,
        ops: *const std::ffi::c_void,
    );

    fn strscpy(dst: *mut u8, src: *const u8) -> i32;

    fn dev_err(dev: *mut std::ffi::c_void, format: *const u8, ...);

    fn init_waitqueue_head(wait: *mut std::ffi::c_void);
    fn spin_lock_init(lock: *mut std::ffi::c_void);
    fn wait_event_interruptible(wq: *mut std::ffi::c_void, condition: i32) -> i32;
}

// Macro equivalent for line6_rawmidi_substream_midi
fn line6_rawmidi_substream_midi(substream: *mut std::ffi::c_void) -> *mut snd_line6_midi {
    unsafe {
        let rmidi = *(substream as *mut *mut std::ffi::c_void);
        *(rmidi as *mut *mut snd_line6_midi) as *mut snd_line6_midi
    }
}

static send_midi_async: fn(*mut usb_line6, *mut u8, i32) -> i32;

/*
    Pass data received via USB to MIDI.
*/
pub extern "C" fn line6_midi_receive(line6: *mut usb_line6, data: *const u8, length: i32) {
    unsafe {
        let midi = (*line6).line6midi;
        if !(*midi).substream_receive.is_null() {
            snd_rawmidi_receive((*midi).substream_receive, data, length);
        }
    }
}

/*
    Read data from MIDI buffer and transmit them via USB.
*/
unsafe fn line6_midi_transmit(substream: *mut std::ffi::c_void) {
    let line6 = (*line6_rawmidi_substream_midi(substream)).line6;
    let line6midi = (*line6).line6midi;
    let mb = &mut (*line6midi).midibuf_out;
    let mut chunk: [u8; LINE6_FALLBACK_MAXPACKETSIZE as usize] = [0; LINE6_FALLBACK_MAXPACKETSIZE as usize];
    let mut req: i32;
    let mut done: i32;

    loop {
        req = {
            let free_bytes = line6_midibuf_bytes_free(mb);
            let max_packet = (*line6).max_packet_size;
            if free_bytes < max_packet && free_bytes < LINE6_FALLBACK_MAXPACKETSIZE {
                free_bytes
            } else if max_packet < LINE6_FALLBACK_MAXPACKETSIZE {
                max_packet
            } else {
                LINE6_FALLBACK_MAXPACKETSIZE
            }
        };
        done = snd_rawmidi_transmit_peek(substream, chunk.as_mut_ptr(), req);

        if done == 0 {
            break;
        }

        line6_midibuf_write(mb, chunk.as_ptr(), done);
        snd_rawmidi_transmit_ack(substream, done);
    }

    loop {
        done = line6_midibuf_read(
            mb,
            chunk.as_mut_ptr(),
            LINE6_FALLBACK_MAXPACKETSIZE,
            LINE6_MIDIBUF_READ_TX,
        );

        if done == 0 {
            break;
        }

        send_midi_async(line6, chunk.as_mut_ptr(), done);
    }
}

/*
    Notification of completion of MIDI transmission.
*/
unsafe extern "C" fn midi_sent(urb: *mut std::ffi::c_void) {
    let status: i32;
    let mut num: i32;
    let line6 = urb as *mut usb_line6;

    status = *(urb as *mut i32 + 4); // offset for status field in urb
    kfree(*(urb as *mut *mut std::ffi::c_void)); // urb->transfer_buffer
    usb_free_urb(urb);

    if status == -ESHUTDOWN {
        return;
    }

    // guard(spinlock_irqsave)(&line6->line6midi->lock);
    // Acquire spinlock
    let midi = (*line6).line6midi;
    (*midi).num_active_send_urbs -= 1;
    num = (*midi).num_active_send_urbs;

    if num == 0 {
        line6_midi_transmit((*midi).substream_transmit);
        num = (*midi).num_active_send_urbs;
    }

    if num == 0 {
        // wake_up(&line6->line6midi->send_wait);
    }
}

/*
    Send an asynchronous MIDI message.
    Assumes that line6->line6midi->lock is held
    (i.e., this function is serialized).
*/
static fn send_midi_async(line6: *mut usb_line6, data: *mut u8, length: i32) -> i32 {
    const GFP_ATOMIC: u32 = 0x20;
    const ENOMEM: i32 = -12;
    const ESHUTDOWN: i32 = -108;

    unsafe {
        let urb: *mut std::ffi::c_void;
        let retval: i32;
        let transfer_buffer: *mut u8;

        urb = usb_alloc_urb(0, GFP_ATOMIC);

        if urb.is_null() {
            return ENOMEM;
        }

        transfer_buffer = kmemdup(data, length, GFP_ATOMIC);

        if transfer_buffer.is_null() {
            usb_free_urb(urb);
            return ENOMEM;
        }

        usb_fill_int_urb(
            urb,
            (*line6).usbdev,
            usb_sndintpipe((*line6).usbdev, (*(*line6).properties).ep_ctrl_w),
            transfer_buffer,
            length,
            midi_sent,
            line6 as *mut std::ffi::c_void,
            (*line6).interval,
        );

        *(urb as *mut i32 + 9) = 0; // urb->actual_length = 0

        retval = usb_urb_ep_type_check(urb);
        if retval < 0 {
            goto_error(urb, retval)
        } else {
            retval = usb_submit_urb(urb, GFP_ATOMIC);
            if retval < 0 {
                goto_error(urb, retval)
            } else {
                (*(*line6).line6midi).num_active_send_urbs += 1;
                return 0;
            }
        }
    }
}

fn goto_error(urb: *mut std::ffi::c_void, retval: i32) -> i32 {
    unsafe {
        dev_err(
            std::ptr::null_mut(),
            b"usb_submit_urb failed\n" as *const u8,
        );
        usb_free_urb(urb);
    }
    retval
}

static fn line6_midi_output_open(_substream: *mut std::ffi::c_void) -> i32 {
    0
}

static fn line6_midi_output_close(_substream: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn line6_midi_output_trigger(substream: *mut std::ffi::c_void, _up: i32) {
    let line6 = (*line6_rawmidi_substream_midi(substream)).line6;

    (*(*line6).line6midi).substream_transmit = substream;
    // guard(spinlock_irqsave)(&line6->line6midi->lock);
    // Acquire spinlock

    if (*(*line6).line6midi).num_active_send_urbs == 0 {
        line6_midi_transmit(substream);
    }
}

unsafe fn line6_midi_output_drain(substream: *mut std::ffi::c_void) {
    let line6 = (*line6_rawmidi_substream_midi(substream)).line6;
    let midi = (*line6).line6midi;

    // wait_event_interruptible(midi->send_wait, midi->num_active_send_urbs == 0);
    while (*midi).num_active_send_urbs != 0 {
        // Wait on send_wait queue
    }
}

static fn line6_midi_input_open(_substream: *mut std::ffi::c_void) -> i32 {
    0
}

static fn line6_midi_input_close(_substream: *mut std::ffi::c_void) -> i32 {
    0
}

unsafe fn line6_midi_input_trigger(substream: *mut std::ffi::c_void, up: i32) {
    let line6 = (*line6_rawmidi_substream_midi(substream)).line6;

    if up != 0 {
        (*(*line6).line6midi).substream_receive = substream;
    } else {
        (*(*line6).line6midi).substream_receive = std::ptr::null_mut();
    }
}

#[repr(C)]
struct snd_rawmidi_ops {
    open: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> i32>,
    close: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> i32>,
    trigger: Option<unsafe extern "C" fn(*mut std::ffi::c_void, i32)>,
    drain: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
}

static LINE6_MIDI_OUTPUT_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(line6_midi_output_open),
    close: Some(line6_midi_output_close),
    trigger: Some(line6_midi_output_trigger),
    drain: Some(line6_midi_output_drain),
};

static LINE6_MIDI_INPUT_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(line6_midi_input_open),
    close: Some(line6_midi_input_close),
    trigger: Some(line6_midi_input_trigger),
    drain: None,
};

/* Create a MIDI device */
unsafe fn snd_line6_new_midi(line6: *mut usb_line6, rmidi_ret: *mut *mut std::ffi::c_void) -> i32 {
    const SNDRV_RAWMIDI_STREAM_OUTPUT: i32 = 0;
    const SNDRV_RAWMIDI_STREAM_INPUT: i32 = 1;
    const SNDRV_RAWMIDI_INFO_OUTPUT: i32 = 0x00000001;
    const SNDRV_RAWMIDI_INFO_INPUT: i32 = 0x00000002;
    const SNDRV_RAWMIDI_INFO_DUPLEX: i32 = 0x00000004;

    let mut rmidi: *mut std::ffi::c_void;
    let err: i32;

    err = snd_rawmidi_new(
        (*line6).card,
        b"Line 6 MIDI" as *const u8,
        0,
        1,
        1,
        rmidi_ret,
    );
    if err < 0 {
        return err;
    }

    rmidi = *rmidi_ret;
    strscpy(
        (rmidi as *mut *mut u8).add(0) as *mut u8,
        (*(*line6).properties).id as *const u8,
    );
    strscpy(
        (rmidi as *mut *mut u8).add(1) as *mut u8,
        (*(*line6).properties).name as *const u8,
    );

    // Set info_flags
    let flags_ptr = (rmidi as *mut i32).add(5);
    *flags_ptr = SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX;

    snd_rawmidi_set_ops(
        rmidi,
        SNDRV_RAWMIDI_STREAM_OUTPUT,
        &LINE6_MIDI_OUTPUT_OPS as *const snd_rawmidi_ops as *const std::ffi::c_void,
    );
    snd_rawmidi_set_ops(
        rmidi,
        SNDRV_RAWMIDI_STREAM_INPUT,
        &LINE6_MIDI_INPUT_OPS as *const snd_rawmidi_ops as *const std::ffi::c_void,
    );
    0
}

/* MIDI device destructor */
unsafe extern "C" fn snd_line6_midi_free(rmidi: *mut std::ffi::c_void) {
    let line6midi = *(rmidi as *mut *mut snd_line6_midi);

    line6_midibuf_destroy(&mut (*line6midi).midibuf_in);
    line6_midibuf_destroy(&mut (*line6midi).midibuf_out);
    kfree(line6midi as *mut std::ffi::c_void);
}

/*
    Initialize the Line 6 MIDI subsystem.
*/
#[no_mangle]
pub extern "C" fn line6_init_midi(line6: *mut usb_line6) -> i32 {
    const ENOMEM: i32 = -12;

    unsafe {
        let mut err: i32;
        let mut rmidi: *mut std::ffi::c_void;
        let mut line6midi: *mut snd_line6_midi;

        if ((*(*line6).properties).capabilities & LINE6_CAP_CONTROL_MIDI) == 0 {
            // skip MIDI initialization and report success
            return 0;
        }

        err = snd_line6_new_midi(line6, &mut rmidi as *mut _);
        if err < 0 {
            return err;
        }

        line6midi = kzalloc_obj(std::mem::size_of::<snd_line6_midi>() as u32, 0x20) as *mut snd_line6_midi;
        if line6midi.is_null() {
            return ENOMEM;
        }

        *(rmidi as *mut *mut snd_line6_midi) = line6midi;
        *(rmidi as *mut *const std::ffi::c_void).add(1) = snd_line6_midi_free as *const std::ffi::c_void;

        init_waitqueue_head(&mut (*line6midi).send_wait as *mut std::ffi::c_void);
        spin_lock_init(&mut (*line6midi).lock as *mut std::ffi::c_void);
        (*line6midi).line6 = line6;

        err = line6_midibuf_init(&mut (*line6midi).midibuf_in, MIDI_BUFFER_SIZE, 0);
        if err < 0 {
            return err;
        }

        err = line6_midibuf_init(&mut (*line6midi).midibuf_out, MIDI_BUFFER_SIZE, 1);
        if err < 0 {
            return err;
        }

        (*line6).line6midi = line6midi;
        return 0;
    }
}

// EXPORT_SYMBOL_GPL(line6_init_midi);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
