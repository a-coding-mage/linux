// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025 Šerif Rami <ramiserifpersia@gmail.com>

// External dependencies from us144mkii.h and kernel subsystems are declared here:
// use crate::us144mkii::*;
// use kernel::usb::urb;
// use kernel::alsa::rawmidi::*;
// use kernel::workqueue::work_struct;
// etc.

extern "C" {
    fn container_of(ptr: *const core::ffi::c_void, member_offset: usize) -> *mut core::ffi::c_void;
    fn kfifo_out_spinlocked(
        fifo: *mut core::ffi::c_void,
        buf: *mut u8,
        len: usize,
        lock: *mut core::ffi::c_void,
    ) -> usize;
    fn kfifo_in_spinlocked(
        fifo: *mut core::ffi::c_void,
        buf: *const u8,
        len: usize,
        lock: *mut core::ffi::c_void,
    ) -> usize;
    fn kfifo_reset(fifo: *mut core::ffi::c_void);
    fn snd_rawmidi_receive(substream: *mut core::ffi::c_void, buf: *const u8, count: usize) -> i32;
    fn snd_rawmidi_transmit_peek(substream: *mut core::ffi::c_void, buf: *mut u8, count: usize) -> i32;
    fn snd_rawmidi_transmit(substream: *mut core::ffi::c_void, buf: *mut u8, count: usize) -> i32;
    fn schedule_work(work: *mut core::ffi::c_void);
    fn cancel_work_sync(work: *mut core::ffi::c_void);
    fn schedule_timeout_uninterruptible(timeout: u32);
    fn usb_get_urb(urb: *mut core::ffi::c_void);
    fn usb_put_urb(urb: *mut core::ffi::c_void);
    fn usb_anchor_urb(urb: *mut core::ffi::c_void, anchor: *mut core::ffi::c_void);
    fn usb_unanchor_urb(urb: *mut core::ffi::c_void);
    fn usb_submit_urb(urb: *mut core::ffi::c_void, mem_flags: u32) -> i32;
    fn usb_kill_anchored_urbs(anchor: *mut core::ffi::c_void);
    fn atomic_read(v: *const core::ffi::c_void) -> i32;
    fn atomic_xchg(v: *mut core::ffi::c_void, new: i32) -> i32;
    fn atomic_set(v: *mut core::ffi::c_void, i: i32);
    fn test_bit(nr: usize, addr: *const core::ffi::c_void) -> bool;
    fn set_bit(nr: usize, addr: *mut core::ffi::c_void);
    fn clear_bit(nr: usize, addr: *mut core::ffi::c_void);
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const i8, ...);
    fn dev_err_ratelimited(dev: *mut core::ffi::c_void, fmt: *const i8, ...);
    fn snd_rawmidi_new(
        card: *mut core::ffi::c_void,
        id: *const i8,
        device: i32,
        output_count: i32,
        input_count: i32,
        rmidi: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn snd_rawmidi_set_ops(rmidi: *mut core::ffi::c_void, stream: i32, ops: *const core::ffi::c_void);
    fn strscpy(dest: *mut i8, src: *const i8, size: usize) -> isize;
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
}

// Constants
const NUM_MIDI_IN_URBS: usize = 0; // Defined elsewhere in us144mkii.h
const NUM_MIDI_OUT_URBS: usize = 0; // Defined elsewhere in us144mkii.h
const GFP_KERNEL: u32 = 0; // Defined by kernel
const GFP_ATOMIC: u32 = 0; // Defined by kernel
const SNDRV_RAWMIDI_STREAM_INPUT: i32 = 0; // Defined by ALSA
const SNDRV_RAWMIDI_STREAM_OUTPUT: i32 = 1; // Defined by ALSA
const SNDRV_RAWMIDI_INFO_INPUT: u32 = 1 << 0;
const SNDRV_RAWMIDI_INFO_OUTPUT: u32 = 1 << 1;
const SNDRV_RAWMIDI_INFO_DUPLEX: u32 = 1 << 2;

// Error codes
const ENOENT: i32 = -2;
const ECONNRESET: i32 = -104;
const ESHUTDOWN: i32 = -108;
const EPROTO: i32 = -71;

// Placeholder struct definitions for external types
#[repr(C)]
pub struct work_struct {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct urb {
    pub context: *mut core::ffi::c_void,
    pub status: i32,
    pub actual_length: usize,
    pub transfer_buffer: *mut u8,
    pub transfer_buffer_length: usize,
}

#[repr(C)]
pub struct tascam_card {
    pub card: *mut core::ffi::c_void,
    pub rmidi: *mut core::ffi::c_void,
    pub midi_in_substream: *mut core::ffi::c_void,
    pub midi_out_substream: *mut core::ffi::c_void,
    pub midi_in_fifo: core::ffi::c_void,
    pub midi_out_lock: core::ffi::c_void,
    pub midi_in_lock: core::ffi::c_void,
    pub midi_in_anchor: core::ffi::c_void,
    pub midi_out_anchor: core::ffi::c_void,
    pub midi_in_active: core::ffi::c_void,
    pub midi_out_active: core::ffi::c_void,
    pub midi_in_urbs: [*mut urb; 0],
    pub midi_out_urbs: [*mut urb; 0],
    pub midi_out_urbs_in_flight: usize,
    pub midi_in_work: work_struct,
    pub midi_out_work: work_struct,
    pub midi_running_status: u8,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub rmidi: *mut core::ffi::c_void,
    pub private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>,
    pub drain: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
}

/*
 * tascam_midi_in_work_handler() - Deferred work for processing MIDI input.
 * @work: The work_struct instance.
 *
 * This function runs in a thread context. It safely reads raw USB data from
 * the kfifo, processes it by stripping protocol-specific padding bytes, and
 * passes the clean MIDI data to the ALSA rawmidi subsystem.
 */
unsafe fn tascam_midi_in_work_handler(work: *mut work_struct) {
    let tascam = container_of(work as *const _, 0) as *mut tascam_card;
    let mut buf: [u8; 9] = [0; 9];
    let mut clean_buf: [u8; 8] = [0; 8];
    let mut count: usize;
    let mut clean_count: usize;

    if (*tascam).midi_in_substream.is_null() {
        return;
    }

    loop {
        let n = kfifo_out_spinlocked(
            &mut (*tascam).midi_in_fifo as *mut _ as *mut core::ffi::c_void,
            buf.as_mut_ptr(),
            buf.len(),
            &mut (*tascam).midi_in_lock as *mut _ as *mut core::ffi::c_void,
        );
        if n != buf.len() {
            break;
        }

        clean_count = 0;
        count = 0;
        while count < 8 {
            if buf[count] != 0xfd {
                clean_buf[clean_count] = buf[count];
                clean_count += 1;
            }
            count += 1;
        }

        if clean_count > 0 {
            snd_rawmidi_receive(
                (*tascam).midi_in_substream,
                clean_buf.as_ptr(),
                clean_count,
            );
        }
    }
}

pub unsafe extern "C" fn tascam_midi_in_urb_complete(urb: *mut urb) {
    let tascam = (*urb).context as *mut tascam_card;
    let mut ret: i32;

    if tascam.is_null() {
        usb_put_urb(urb as *mut _);
        return;
    }

    if (*urb).status != 0 {
        if (*urb).status != ENOENT
            && (*urb).status != ECONNRESET
            && (*urb).status != ESHUTDOWN
            && (*urb).status != EPROTO
        {
            dev_err_ratelimited(
                (*(*tascam).card),
                b"MIDI IN URB failed: status %d\0".as_ptr() as *const i8,
                (*urb).status,
            );
        }
        usb_put_urb(urb as *mut _);
        return;
    }

    if atomic_read(&(*tascam).midi_in_active as *const _) != 0 && (*urb).actual_length > 0 {
        kfifo_in_spinlocked(
            &mut (*tascam).midi_in_fifo as *mut _ as *mut core::ffi::c_void,
            (*urb).transfer_buffer,
            (*urb).actual_length,
            &mut (*tascam).midi_in_lock as *mut _ as *mut core::ffi::c_void,
        );
        schedule_work(&mut (*tascam).midi_in_work as *mut _ as *mut core::ffi::c_void);
    }

    usb_get_urb(urb as *mut _);
    usb_anchor_urb(urb as *mut _, &mut (*tascam).midi_in_anchor as *mut _ as *mut core::ffi::c_void);
    ret = usb_submit_urb(urb as *mut _, GFP_ATOMIC);
    if ret < 0 {
        dev_err(
            (*(*tascam).card),
            b"Failed to resubmit MIDI IN URB: error %d\n\0".as_ptr() as *const i8,
            ret,
        );
        usb_unanchor_urb(urb as *mut _);
        usb_put_urb(urb as *mut _);
        return;
    }

    usb_put_urb(urb as *mut _);
}

/*
 * tascam_midi_in_open() - Opens the MIDI input substream.
 * @substream: The ALSA rawmidi substream to open.
 *
 * This function stores a reference to the MIDI input substream in the
 * driver's private data.
 *
 * Return: 0 on success.
 */
unsafe extern "C" fn tascam_midi_in_open(substream: *mut snd_rawmidi_substream) -> i32 {
    let tascam = (*(*substream).rmidi) as *mut tascam_card;

    (*tascam).midi_in_substream = substream as *mut core::ffi::c_void;
    0
}

/*
 * tascam_midi_in_close() - Closes the MIDI input substream.
 * @substream: The ALSA rawmidi substream to close.
 *
 * Return: 0 on success.
 */
unsafe extern "C" fn tascam_midi_in_close(_substream: *mut snd_rawmidi_substream) -> i32 {
    0
}

/*
 * tascam_midi_in_trigger() - Triggers MIDI input stream activity.
 * @substream: The ALSA rawmidi substream.
 * @up: Boolean indicating whether to start (1) or stop (0) the stream.
 *
 * This function starts or stops the MIDI input URBs based on the 'up'
 * parameter. When starting, it resets the kfifo and submits all MIDI input
 * URBs. When stopping, it kills all anchored MIDI input URBs and cancels the
 * associated workqueue.
 */
unsafe extern "C" fn tascam_midi_in_trigger(substream: *mut snd_rawmidi_substream, up: i32) {
    let tascam = (*(*substream).rmidi) as *mut tascam_card;
    let mut i: i32;
    let mut err: i32;

    if up != 0 {
        if atomic_xchg(&mut (*tascam).midi_in_active as *mut _ as *mut core::ffi::c_void, 1) == 0 {
            // scoped_guard(spinlock_irqsave, &tascam->midi_in_lock)
            {
                kfifo_reset(&mut (*tascam).midi_in_fifo as *mut _ as *mut core::ffi::c_void);
            }

            i = 0;
            while i < NUM_MIDI_IN_URBS as i32 {
                usb_get_urb((*tascam).midi_in_urbs[i as usize] as *mut _);
                usb_anchor_urb(
                    (*tascam).midi_in_urbs[i as usize] as *mut _,
                    &mut (*tascam).midi_in_anchor as *mut _ as *mut core::ffi::c_void,
                );
                err = usb_submit_urb(
                    (*tascam).midi_in_urbs[i as usize] as *mut _,
                    GFP_KERNEL,
                );
                if err < 0 {
                    dev_err(
                        (*(*tascam).card),
                        b"Failed to submit MIDI IN URB %d: %d\n\0".as_ptr() as *const i8,
                        i,
                        err,
                    );
                    usb_unanchor_urb((*tascam).midi_in_urbs[i as usize] as *mut _);
                    usb_put_urb((*tascam).midi_in_urbs[i as usize] as *mut _);
                }
                i += 1;
            }
        }
    } else {
        if atomic_xchg(&mut (*tascam).midi_in_active as *mut _ as *mut core::ffi::c_void, 0) == 1 {
            usb_kill_anchored_urbs(&mut (*tascam).midi_in_anchor as *mut _ as *mut core::ffi::c_void);
            cancel_work_sync(&mut (*tascam).midi_in_work as *mut _ as *mut core::ffi::c_void);
        }
    }
}

/*
 * tascam_midi_in_ops - ALSA rawmidi operations for MIDI input.
 *
 * This structure defines the callback functions for MIDI input stream
 * operations, including open, close, and trigger.
 */
static TASCAM_MIDI_IN_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(tascam_midi_in_open),
    close: Some(tascam_midi_in_close),
    trigger: Some(tascam_midi_in_trigger),
    drain: None,
};

pub unsafe extern "C" fn tascam_midi_out_urb_complete(urb: *mut urb) {
    let tascam = (*urb).context as *mut tascam_card;
    let mut i: usize;
    let mut urb_index: i32 = -1;

    if (*urb).status != 0 {
        if (*urb).status != ENOENT
            && (*urb).status != ECONNRESET
            && (*urb).status != ESHUTDOWN
        {
            dev_err_ratelimited(
                (*(*tascam).card),
                b"MIDI OUT URB failed: %d\n\0".as_ptr() as *const i8,
                (*urb).status,
            );
        }
        usb_put_urb(urb as *mut _);
        return;
    }

    if tascam.is_null() {
        usb_put_urb(urb as *mut _);
        return;
    }

    i = 0;
    while i < NUM_MIDI_OUT_URBS {
        if (*tascam).midi_out_urbs[i] == urb {
            urb_index = i as i32;
            break;
        }
        i += 1;
    }

    if urb_index < 0 {
        dev_err_ratelimited(
            (*(*tascam).card),
            b"Unknown MIDI OUT URB completed!\n\0".as_ptr() as *const i8,
        );
        usb_put_urb(urb as *mut _);
        return;
    }

    // scoped_guard(spinlock_irqsave, &tascam->midi_out_lock)
    {
        clear_bit(urb_index as usize, &mut (*tascam).midi_out_urbs_in_flight as *mut _ as *mut core::ffi::c_void);
    }

    if atomic_read(&(*tascam).midi_out_active as *const _) != 0 {
        schedule_work(&mut (*tascam).midi_out_work as *mut _ as *mut core::ffi::c_void);
    }

    usb_put_urb(urb as *mut _);
}

/*
 * tascam_midi_out_work_handler() - Deferred work for sending MIDI data
 * @work: The work_struct instance.
 *
 * This function handles the proprietary output protocol: take the raw MIDI
 * message bytes from the application, place them at the start of a 9-byte
 * buffer, pad the rest with 0xFD, and add a terminator byte (0x00).
 * This function pulls as many bytes as will fit into one packet from the
 * ALSA buffer and sends them.
 */
unsafe fn tascam_midi_out_work_handler(work: *mut work_struct) {
    let tascam = container_of(work as *const _, 0) as *mut tascam_card;
    let substream = (*tascam).midi_out_substream;
    let mut i: i32;

    if substream.is_null() || atomic_read(&(*tascam).midi_out_active as *const _) == 0 {
        return;
    }

    loop {
        let mut probe_buf: [u8; 1] = [0; 1];
        if snd_rawmidi_transmit_peek(substream, probe_buf.as_mut_ptr(), 1) != 1 {
            break;
        }

        let mut urb_index: i32;
        let urb: *mut urb;
        let buf: *mut u8;
        let bytes_to_send: i32;

        // scoped_guard(spinlock_irqsave, &tascam->midi_out_lock)
        {
            urb_index = -1;
            i = 0;
            while i < NUM_MIDI_OUT_URBS as i32 {
                if !test_bit(i as usize, &(*tascam).midi_out_urbs_in_flight as *const _ as *const core::ffi::c_void) {
                    urb_index = i;
                    break;
                }
                i += 1;
            }

            if urb_index < 0 {
                return;
            }

            urb = (*tascam).midi_out_urbs[urb_index as usize];
            buf = (*urb).transfer_buffer;
            bytes_to_send = snd_rawmidi_transmit(substream, buf, 8);

            if bytes_to_send <= 0 {
                break;
            }

            if bytes_to_send < 9 {
                memset(
                    buf.add(bytes_to_send as usize) as *mut core::ffi::c_void,
                    0xfd,
                    (9 - bytes_to_send) as usize,
                );
            }
            *buf.add(8) = 0xe0;

            set_bit(urb_index as usize, &mut (*tascam).midi_out_urbs_in_flight as *mut _ as *mut core::ffi::c_void);
            (*urb).transfer_buffer_length = 9;
        }

        usb_get_urb(urb as *mut _);
        usb_anchor_urb(urb as *mut _, &mut (*tascam).midi_out_anchor as *mut _ as *mut core::ffi::c_void);
        if usb_submit_urb(urb as *mut _, GFP_KERNEL) < 0 {
            dev_err_ratelimited(
                (*(*tascam).card),
                b"Failed to submit MIDI OUT URB %d\n\0".as_ptr() as *const i8,
                urb_index,
            );
            // scoped_guard(spinlock_irqsave, &tascam->midi_out_lock)
            {
                clear_bit(urb_index as usize, &mut (*tascam).midi_out_urbs_in_flight as *mut _ as *mut core::ffi::c_void);
            }
            usb_unanchor_urb(urb as *mut _);
            usb_put_urb(urb as *mut _);
            break;
        }
    }
}

/*
 * tascam_midi_out_open() - Opens the MIDI output substream.
 * @substream: The ALSA rawmidi substream to open.
 *
 * This function stores a reference to the MIDI output substream in the
 * driver's private data and initializes the MIDI running status.
 *
 * Return: 0 on success.
 */
unsafe extern "C" fn tascam_midi_out_open(substream: *mut snd_rawmidi_substream) -> i32 {
    let tascam = (*(*substream).rmidi) as *mut tascam_card;

    (*tascam).midi_out_substream = substream as *mut core::ffi::c_void;
    (*tascam).midi_running_status = 0;
    0
}

/*
 * tascam_midi_out_close() - Closes the MIDI output substream.
 * @substream: The ALSA rawmidi substream to close.
 *
 * Return: 0 on success.
 */
unsafe extern "C" fn tascam_midi_out_close(_substream: *mut snd_rawmidi_substream) -> i32 {
    0
}

/*
 * tascam_midi_out_drain() - Drains the MIDI output stream.
 * @substream: The ALSA rawmidi substream.
 *
 * This function cancels any pending MIDI output work and kills all
 * anchored MIDI output URBs, ensuring all data is sent or discarded.
 */
unsafe extern "C" fn tascam_midi_out_drain(substream: *mut snd_rawmidi_substream) {
    let tascam = (*(*substream).rmidi) as *mut tascam_card;
    let mut in_flight: bool = true;

    while in_flight {
        in_flight = false;
        let mut i: i32 = 0;
        while i < NUM_MIDI_OUT_URBS as i32 {
            if test_bit(i as usize, &(*tascam).midi_out_urbs_in_flight as *const _ as *const core::ffi::c_void) {
                in_flight = true;
                break;
            }
            i += 1;
        }
        if in_flight {
            schedule_timeout_uninterruptible(1);
        }
    }

    cancel_work_sync(&mut (*tascam).midi_out_work as *mut _ as *mut core::ffi::c_void);
    usb_kill_anchored_urbs(&mut (*tascam).midi_out_anchor as *mut _ as *mut core::ffi::c_void);
}

/*
 * tascam_midi_out_trigger() - Triggers MIDI output stream activity.
 * @substream: The ALSA rawmidi substream.
 * @up: Boolean indicating whether to start (1) or stop (0) the stream.
 *
 * This function starts or stops the MIDI output workqueue based on the
 * 'up' parameter.
 */
unsafe extern "C" fn tascam_midi_out_trigger(substream: *mut snd_rawmidi_substream, up: i32) {
    let tascam = (*(*substream).rmidi) as *mut tascam_card;

    if up != 0 {
        atomic_set(&mut (*tascam).midi_out_active as *mut _ as *mut core::ffi::c_void, 1);
        schedule_work(&mut (*tascam).midi_out_work as *mut _ as *mut core::ffi::c_void);
    } else {
        atomic_set(&mut (*tascam).midi_out_active as *mut _ as *mut core::ffi::c_void, 0);
    }
}

/*
 * tascam_midi_out_ops - ALSA rawmidi operations for MIDI output.
 *
 * This structure defines the callback functions for MIDI output stream
 * operations, including open, close, trigger, and drain.
 */
static TASCAM_MIDI_OUT_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(tascam_midi_out_open),
    close: Some(tascam_midi_out_close),
    trigger: Some(tascam_midi_out_trigger),
    drain: Some(tascam_midi_out_drain),
};

pub unsafe extern "C" fn tascam_create_midi(tascam: *mut tascam_card) -> i32 {
    let mut err: i32;
    let mut rmidi: *mut core::ffi::c_void = core::ptr::null_mut();

    err = snd_rawmidi_new(
        (*tascam).card,
        b"US144MKII MIDI\0".as_ptr() as *const i8,
        0,
        1,
        1,
        &mut rmidi as *mut _,
    );
    if err < 0 {
        return err;
    }

    (*tascam).rmidi = rmidi;

    strscpy(
        (*rmidi as *mut i8).add(0),
        b"US144MKII MIDI\0".as_ptr() as *const i8,
        16,
    );
    let rmidi_struct = rmidi as *mut core::ffi::c_void;
    *(rmidi_struct.add(core::mem::size_of::<*mut core::ffi::c_void>()) as *mut *mut core::ffi::c_void) = tascam as *mut _;

    snd_rawmidi_set_ops(
        rmidi,
        SNDRV_RAWMIDI_STREAM_INPUT,
        &TASCAM_MIDI_IN_OPS as *const _ as *const core::ffi::c_void,
    );
    snd_rawmidi_set_ops(
        rmidi,
        SNDRV_RAWMIDI_STREAM_OUTPUT,
        &TASCAM_MIDI_OUT_OPS as *const _ as *const core::ffi::c_void,
    );

    let rmidi_flags = rmidi as *mut u32;
    *rmidi_flags = SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_DUPLEX;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
