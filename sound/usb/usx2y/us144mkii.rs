// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025 Šerif Rami <ramiserifpersia@gmail.com>
//
// ALSA Driver for TASCAM US-144MKII Audio Interface

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

use core::ffi::c_int;

// External kernel types and functions (from us144mkii.h and other kernel modules)
// These would be provided by the kernel build environment

extern "C" {
    // Types from kernel headers
    type usb_interface;
    type usb_device;
    type usb_device_id;
    type snd_card;
    type urb;
    type usb_anchor;
    type timer_list;
    type work_struct;
    type pm_message_t;
    type spinlock_t;
    type kfifo;

    // USB core functions
    fn usb_alloc_urb(iso_packets: c_int, mem_flags: u32) -> *mut urb;
    fn usb_free_urb(urb: *mut urb);
    fn usb_alloc_coherent(
        dev: *mut usb_device,
        size: usize,
        mem_flags: u32,
        dma: *mut u64,
    ) -> *mut core::ffi::c_void;
    fn usb_free_coherent(
        dev: *mut usb_device,
        size: usize,
        addr: *mut core::ffi::c_void,
        dma: u64,
    );
    fn usb_kill_anchored_urbs(anchor: *mut usb_anchor);
    fn usb_get_intfdata(intf: *mut usb_interface) -> *mut core::ffi::c_void;
    fn usb_set_intfdata(intf: *mut usb_interface, data: *mut core::ffi::c_void);
    fn usb_get_dev(dev: *mut usb_device) -> *mut usb_device;
    fn usb_put_dev(dev: *mut usb_device);
    fn usb_rcvctrlpipe(dev: *mut usb_device, endpoint: c_int) -> c_int;
    fn usb_sndctrlpipe(dev: *mut usb_device, endpoint: c_int) -> c_int;
    fn usb_sndisocpipe(dev: *mut usb_device, endpoint: c_int) -> c_int;
    fn usb_rcvisocpipe(dev: *mut usb_device, endpoint: c_int) -> c_int;
    fn usb_rcvbulkpipe(dev: *mut usb_device, endpoint: c_int) -> c_int;
    fn usb_sndbulkpipe(dev: *mut usb_device, endpoint: c_int) -> c_int;
    fn usb_control_msg(
        dev: *mut usb_device,
        pipe: c_int,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut core::ffi::c_void,
        size: u16,
        timeout: c_int,
    ) -> c_int;
    fn usb_set_interface(dev: *mut usb_device, ifnum: c_int, alternate: c_int) -> c_int;
    fn usb_ifnum_to_if(dev: *mut usb_device, ifnum: c_int) -> *mut usb_interface;
    fn interface_to_usbdev(intf: *mut usb_interface) -> *mut usb_device;

    // Memory allocation
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *const core::ffi::c_void);
    fn kfifo_alloc(fifo: *mut kfifo, size: usize, flags: u32) -> c_int;
    fn kfifo_free(fifo: *mut kfifo);

    // ALSA functions
    fn snd_card_new(
        parent: *mut core::ffi::c_void,
        idx: c_int,
        xid: *const core::ffi::c_char,
        module: *mut core::ffi::c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_disconnect(card: *mut snd_card);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const core::ffi::c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut core::ffi::c_void,
    ) -> c_int;
    fn snd_pcm_suspend_all(pcm: *mut core::ffi::c_void) -> c_int;

    // Synchronization
    fn spin_lock_init(lock: *mut spinlock_t);
    fn init_usb_anchor(anchor: *mut usb_anchor);
    fn timer_setup(
        timer: *mut timer_list,
        callback: unsafe extern "C" fn(*mut timer_list),
        flags: c_int,
    );
    fn timer_delete_sync(timer: *mut timer_list) -> c_int;

    // Work queue and scheduling
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn schedule_work(work: *mut work_struct) -> bool;
    fn init_completion(x: *mut core::ffi::c_void);

    // Utilities
    fn dev_info(dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn dev_name(dev: *const core::ffi::c_void) -> *const core::ffi::c_char;
    fn strscpy(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char, count: usize) -> usize;
    fn le16_to_cpu(x: u16) -> u16;

    // Macros/functions from us144mkii.h that need external definition
    fn tascam_init_pcm(pcm: *mut core::ffi::c_void) -> c_int;
    fn tascam_create_midi(tascam: *mut core::ffi::c_void) -> c_int;
    fn tascam_create_controls(tascam: *mut core::ffi::c_void) -> c_int;
    fn tascam_stop_pcm_work_handler(work: *mut work_struct);
    fn tascam_capture_work_handler(work: *mut work_struct);
    fn tascam_midi_in_work(work: *mut work_struct);
    fn tascam_midi_out_work(work: *mut work_struct);
    fn tascam_midi_in_urb_complete(urb: *mut urb);
    fn tascam_midi_out_urb_complete(urb: *mut urb);
    fn playback_urb_complete(urb: *mut urb);
    fn feedback_urb_complete(urb: *mut urb);
    fn capture_urb_complete(urb: *mut urb);
    fn us144mkii_configure_device_for_rate(tascam: *mut core::ffi::c_void, rate: c_int);

    // Atomic operations
    fn atomic_set(v: *mut i32, i: i32);
    fn atomic_read(v: *const i32) -> i32;

    fn snprintf(
        str: *mut core::ffi::c_char,
        size: usize,
        format: *const core::ffi::c_char,
        ...
    ) -> c_int;

    // Macros as extern functions
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
}

// Constants (from us144mkii.h)
const SNDRV_CARDS: usize = 32;
const NUM_PLAYBACK_URBS: usize = 2;
const NUM_FEEDBACK_URBS: usize = 2;
const NUM_CAPTURE_URBS: usize = 2;
const NUM_MIDI_IN_URBS: usize = 4;
const NUM_MIDI_OUT_URBS: usize = 4;
const PLAYBACK_URB_PACKETS: c_int = 10;
const FEEDBACK_URB_PACKETS: c_int = 10;
const BYTES_PER_FRAME: usize = 12;
const FEEDBACK_PACKET_SIZE: usize = 3;
const CAPTURE_URB_SIZE: usize = 65536;
const MIDI_IN_BUF_SIZE: usize = 64;
const MIDI_OUT_BUF_SIZE: usize = 64;
const CAPTURE_RING_BUFFER_SIZE: usize = 131072;
const RAW_BYTES_PER_DECODE_BLOCK: usize = 512;
const FRAMES_PER_DECODE_BLOCK: usize = 512;
const DECODED_CHANNELS_PER_FRAME: usize = 10;
const DECODED_SAMPLE_SIZE: usize = 4;
const MIDI_IN_FIFO_SIZE: usize = 64;
const GFP_KERNEL: u32 = 0xd0;
const URB_ISO_ASAP: u32 = 1 << 0;
const URB_NO_TRANSFER_DMA_MAP: u32 = 1 << 2;
const USB_CTRL_TIMEOUT_MS: c_int = 5000;
const VENDOR_REQ_MODE_CONTROL: u8 = 0xb0;
const VENDOR_REQ_DEEP_SLEEP: u8 = 0xb1;
const RT_H2D_VENDOR_DEV: u8 = 0x40;
const RT_D2H_VENDOR_DEV: u8 = 0xc0;
const MODE_VAL_HANDSHAKE_READ: u16 = 0x0000;
const EP_AUDIO_OUT: c_int = 0x01;
const EP_PLAYBACK_FEEDBACK: c_int = 0x82;
const EP_AUDIO_IN: c_int = 0x82;
const EP_MIDI_IN: c_int = 0x86;
const EP_MIDI_OUT: c_int = 0x02;
const USB_VID_TASCAM: u16 = 0x0644;
const USB_PID_TASCAM_US144: u16 = 0x8006;
const USB_PID_TASCAM_US144MKII: u16 = 0x8007;
const DRIVER_NAME: &[u8] = b"snd-usb-tascam-us144\0";

// Module parameters
static mut index: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
static mut id: [*mut core::ffi::c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS];
static mut enable: [bool; SNDRV_CARDS] = {
    let mut arr = [false; SNDRV_CARDS];
    arr[0] = true;
    arr
};
static mut dev_idx: c_int = 0;

// Forward declarations of probed functions
extern "C" {
    fn tascam_probe(intf: *mut usb_interface, usb_id: *const usb_device_id) -> c_int;
    fn tascam_disconnect(intf: *mut usb_interface);
    fn tascam_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int;
    fn tascam_resume(intf: *mut usb_interface) -> c_int;
}

pub unsafe fn tascam_free_urbs(tascam: *mut core::ffi::c_void) {
    let tascam = tascam as *mut crate::TascamCard;

    usb_kill_anchored_urbs(&mut (*tascam).playback_anchor);
    for i in 0..NUM_PLAYBACK_URBS {
        if !(*tascam).playback_urbs[i].is_null() {
            usb_free_coherent(
                (*tascam).dev,
                (*tascam).playback_urb_alloc_size,
                (*(*tascam).playback_urbs[i]).transfer_buffer,
                (*(*tascam).playback_urbs[i]).transfer_dma,
            );
            usb_free_urb((*tascam).playback_urbs[i]);
            (*tascam).playback_urbs[i] = core::ptr::null_mut();
        }
    }

    usb_kill_anchored_urbs(&mut (*tascam).feedback_anchor);
    for i in 0..NUM_FEEDBACK_URBS {
        if !(*tascam).feedback_urbs[i].is_null() {
            usb_free_coherent(
                (*tascam).dev,
                (*tascam).feedback_urb_alloc_size,
                (*(*tascam).feedback_urbs[i]).transfer_buffer,
                (*(*tascam).feedback_urbs[i]).transfer_dma,
            );
            usb_free_urb((*tascam).feedback_urbs[i]);
            (*tascam).feedback_urbs[i] = core::ptr::null_mut();
        }
    }

    usb_kill_anchored_urbs(&mut (*tascam).capture_anchor);
    for i in 0..NUM_CAPTURE_URBS {
        if !(*tascam).capture_urbs[i].is_null() {
            usb_free_coherent(
                (*tascam).dev,
                (*tascam).capture_urb_alloc_size,
                (*(*tascam).capture_urbs[i]).transfer_buffer,
                (*(*tascam).capture_urbs[i]).transfer_dma,
            );
            usb_free_urb((*tascam).capture_urbs[i]);
            (*tascam).capture_urbs[i] = core::ptr::null_mut();
        }
    }

    usb_kill_anchored_urbs(&mut (*tascam).midi_in_anchor);
    for i in 0..NUM_MIDI_IN_URBS {
        if !(*tascam).midi_in_urbs[i].is_null() {
            usb_free_coherent(
                (*tascam).dev,
                MIDI_IN_BUF_SIZE,
                (*(*tascam).midi_in_urbs[i]).transfer_buffer,
                (*(*tascam).midi_in_urbs[i]).transfer_dma,
            );
            usb_free_urb((*tascam).midi_in_urbs[i]);
            (*tascam).midi_in_urbs[i] = core::ptr::null_mut();
        }
    }

    usb_kill_anchored_urbs(&mut (*tascam).midi_out_anchor);
    for i in 0..NUM_MIDI_OUT_URBS {
        if !(*tascam).midi_out_urbs[i].is_null() {
            usb_free_coherent(
                (*tascam).dev,
                MIDI_OUT_BUF_SIZE,
                (*(*tascam).midi_out_urbs[i]).transfer_buffer,
                (*(*tascam).midi_out_urbs[i]).transfer_dma,
            );
            usb_free_urb((*tascam).midi_out_urbs[i]);
            (*tascam).midi_out_urbs[i] = core::ptr::null_mut();
        }
    }

    kfree((*tascam).capture_routing_buffer as *const _);
    (*tascam).capture_routing_buffer = core::ptr::null_mut();
    kfree((*tascam).capture_decode_dst_block as *const _);
    (*tascam).capture_decode_dst_block = core::ptr::null_mut();
    kfree((*tascam).capture_decode_raw_block as *const _);
    (*tascam).capture_decode_raw_block = core::ptr::null_mut();
    kfree((*tascam).capture_ring_buffer as *const _);
    (*tascam).capture_ring_buffer = core::ptr::null_mut();
}

pub unsafe fn tascam_alloc_urbs(tascam: *mut core::ffi::c_void) -> c_int {
    let tascam = tascam as *mut crate::TascamCard;
    let mut max_packet_size: usize;

    max_packet_size = ((96000 / 8000) + 2) * BYTES_PER_FRAME;
    (*tascam).playback_urb_alloc_size = max_packet_size * PLAYBACK_URB_PACKETS as usize;

    for i in 0..NUM_PLAYBACK_URBS {
        let urb = usb_alloc_urb(PLAYBACK_URB_PACKETS, GFP_KERNEL);

        if urb.is_null() {
            goto_error(tascam);
            return -12; // -ENOMEM
        }
        (*tascam).playback_urbs[i] = urb;

        (*urb).transfer_buffer = usb_alloc_coherent(
            (*tascam).dev,
            (*tascam).playback_urb_alloc_size,
            GFP_KERNEL,
            &mut (*urb).transfer_dma,
        );
        if (*urb).transfer_buffer.is_null() {
            goto_error(tascam);
            return -12; // -ENOMEM
        }

        (*urb).dev = (*tascam).dev;
        (*urb).pipe = usb_sndisocpipe((*tascam).dev, EP_AUDIO_OUT);
        (*urb).transfer_flags = URB_ISO_ASAP | URB_NO_TRANSFER_DMA_MAP;
        (*urb).interval = 1;
        (*urb).context = tascam as *mut _;
        (*urb).complete = Some(playback_urb_complete);
    }

    (*tascam).feedback_urb_alloc_size = FEEDBACK_PACKET_SIZE * FEEDBACK_URB_PACKETS as usize;

    for i in 0..NUM_FEEDBACK_URBS {
        let f_urb = usb_alloc_urb(FEEDBACK_URB_PACKETS, GFP_KERNEL);

        if f_urb.is_null() {
            goto_error(tascam);
            return -12; // -ENOMEM
        }
        (*tascam).feedback_urbs[i] = f_urb;

        (*f_urb).transfer_buffer = usb_alloc_coherent(
            (*tascam).dev,
            (*tascam).feedback_urb_alloc_size,
            GFP_KERNEL,
            &mut (*f_urb).transfer_dma,
        );
        if (*f_urb).transfer_buffer.is_null() {
            goto_error(tascam);
            return -12; // -ENOMEM
        }

        (*f_urb).dev = (*tascam).dev;
        (*f_urb).pipe = usb_rcvisocpipe((*tascam).dev, EP_PLAYBACK_FEEDBACK);
        (*f_urb).transfer_flags = URB_ISO_ASAP | URB_NO_TRANSFER_DMA_MAP;
        (*f_urb).interval = 4;
        (*f_urb).context = tascam as *mut _;
        (*f_urb).complete = Some(feedback_urb_complete);
    }

    (*tascam).capture_urb_alloc_size = CAPTURE_URB_SIZE;
    for i in 0..NUM_CAPTURE_URBS {
        let c_urb = usb_alloc_urb(0, GFP_KERNEL);

        if c_urb.is_null() {
            goto_error(tascam);
            return -12; // -ENOMEM
        }
        (*tascam).capture_urbs[i] = c_urb;

        (*c_urb).transfer_buffer = usb_alloc_coherent(
            (*tascam).dev,
            (*tascam).capture_urb_alloc_size,
            GFP_KERNEL,
            &mut (*c_urb).transfer_dma,
        );
        if (*c_urb).transfer_buffer.is_null() {
            goto_error(tascam);
            return -12; // -ENOMEM
        }

        usb_fill_bulk_urb(
            c_urb,
            (*tascam).dev,
            usb_rcvbulkpipe((*tascam).dev, EP_AUDIO_IN),
            (*c_urb).transfer_buffer,
            (*tascam).capture_urb_alloc_size,
            Some(capture_urb_complete),
            tascam as *mut _,
        );
        (*c_urb).transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    }

    for i in 0..NUM_MIDI_IN_URBS {
        let m_urb = usb_alloc_urb(0, GFP_KERNEL);

        if m_urb.is_null() {
            goto_error(tascam);
            return -12; // -ENOMEM
        }
        (*tascam).midi_in_urbs[i] = m_urb;
        (*m_urb).transfer_buffer = usb_alloc_coherent(
            (*tascam).dev,
            MIDI_IN_BUF_SIZE,
            GFP_KERNEL,
            &mut (*m_urb).transfer_dma,
        );
        if (*m_urb).transfer_buffer.is_null() {
            goto_error(tascam);
            return -12; // -ENOMEM
        }
        usb_fill_bulk_urb(
            m_urb,
            (*tascam).dev,
            usb_rcvbulkpipe((*tascam).dev, EP_MIDI_IN),
            (*m_urb).transfer_buffer,
            MIDI_IN_BUF_SIZE,
            Some(tascam_midi_in_urb_complete),
            tascam as *mut _,
        );
        (*m_urb).transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    }

    for i in 0..NUM_MIDI_OUT_URBS {
        let m_urb = usb_alloc_urb(0, GFP_KERNEL);

        if m_urb.is_null() {
            goto_error(tascam);
            return -12; // -ENOMEM
        }
        (*tascam).midi_out_urbs[i] = m_urb;
        (*m_urb).transfer_buffer = usb_alloc_coherent(
            (*tascam).dev,
            MIDI_OUT_BUF_SIZE,
            GFP_KERNEL,
            &mut (*m_urb).transfer_dma,
        );
        if (*m_urb).transfer_buffer.is_null() {
            goto_error(tascam);
            return -12; // -ENOMEM
        }
        usb_fill_bulk_urb(
            m_urb,
            (*tascam).dev,
            usb_sndbulkpipe((*tascam).dev, EP_MIDI_OUT),
            (*m_urb).transfer_buffer,
            0,
            Some(tascam_midi_out_urb_complete),
            tascam as *mut _,
        );
        (*m_urb).transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    }

    (*tascam).capture_ring_buffer =
        kmalloc(CAPTURE_RING_BUFFER_SIZE, GFP_KERNEL) as *mut _;
    if (*tascam).capture_ring_buffer.is_null() {
        goto_error(tascam);
        return -12; // -ENOMEM
    }

    (*tascam).capture_decode_raw_block =
        kmalloc(RAW_BYTES_PER_DECODE_BLOCK, GFP_KERNEL) as *mut _;
    if (*tascam).capture_decode_raw_block.is_null() {
        goto_error(tascam);
        return -12; // -ENOMEM
    }

    (*tascam).capture_decode_dst_block = kmalloc(
        FRAMES_PER_DECODE_BLOCK * DECODED_CHANNELS_PER_FRAME * DECODED_SAMPLE_SIZE,
        GFP_KERNEL,
    ) as *mut _;
    if (*tascam).capture_decode_dst_block.is_null() {
        goto_error(tascam);
        return -12; // -ENOMEM
    }

    (*tascam).capture_routing_buffer = kmalloc(
        FRAMES_PER_DECODE_BLOCK * DECODED_CHANNELS_PER_FRAME * DECODED_SAMPLE_SIZE,
        GFP_KERNEL,
    ) as *mut _;
    if (*tascam).capture_routing_buffer.is_null() {
        goto_error(tascam);
        return -12; // -ENOMEM
    }

    0
}

unsafe fn goto_error(tascam: *mut crate::TascamCard) {
    let card = (*tascam).card;
    dev_err(
        &mut (*card).dev as *mut _ as *mut _,
        b"Failed to allocate URBs\n" as *const _ as *const _,
    );
    tascam_free_urbs(tascam as *mut _);
}

pub unsafe fn tascam_stop_work_handler(work: *mut work_struct) {
    let tascam = crate::container_of(work, crate::TascamCard, stop_work);

    usb_kill_anchored_urbs(&mut (*tascam).playback_anchor);
    usb_kill_anchored_urbs(&mut (*tascam).feedback_anchor);
    usb_kill_anchored_urbs(&mut (*tascam).capture_anchor);
    atomic_set(&mut (*tascam).active_urbs, 0);
}

pub unsafe fn tascam_card_private_free(card: *mut snd_card) {
    let tascam = (*card).private_data as *mut crate::TascamCard;

    if !tascam.is_null() {
        kfifo_free(&mut (*tascam).midi_in_fifo);
        if !(*tascam).dev.is_null() {
            usb_put_dev((*tascam).dev);
            (*tascam).dev = core::ptr::null_mut();
        }
    }
}

pub unsafe fn tascam_suspend(intf: *mut usb_interface, _message: pm_message_t) -> c_int {
    let tascam = usb_get_intfdata(intf) as *mut crate::TascamCard;

    if tascam.is_null() {
        return 0;
    }

    snd_pcm_suspend_all((*tascam).pcm as *mut _);

    cancel_work_sync(&mut (*tascam).stop_work);
    cancel_work_sync(&mut (*tascam).capture_work);
    cancel_work_sync(&mut (*tascam).midi_in_work);
    cancel_work_sync(&mut (*tascam).midi_out_work);
    cancel_work_sync(&mut (*tascam).stop_pcm_work);
    usb_kill_anchored_urbs(&mut (*tascam).playback_anchor);
    usb_kill_anchored_urbs(&mut (*tascam).capture_anchor);
    usb_kill_anchored_urbs(&mut (*tascam).feedback_anchor);
    usb_kill_anchored_urbs(&mut (*tascam).midi_in_anchor);
    usb_kill_anchored_urbs(&mut (*tascam).midi_out_anchor);

    dev_info(&mut (*intf).dev as *mut _ as *mut _, b"sending deep sleep command\n" as *const _ as *const _);
    let err = usb_control_msg(
        (*tascam).dev,
        usb_sndctrlpipe((*tascam).dev, 0),
        VENDOR_REQ_DEEP_SLEEP,
        RT_H2D_VENDOR_DEV,
        0x0000,
        0x0000,
        core::ptr::null_mut(),
        0,
        USB_CTRL_TIMEOUT_MS,
    );
    if err < 0 {
        dev_err(&mut (*intf).dev as *mut _ as *mut _, b"deep sleep command failed: %d\n" as *const _ as *const _, err);
    }

    0
}

pub unsafe fn tascam_resume(intf: *mut usb_interface) -> c_int {
    let tascam = usb_get_intfdata(intf) as *mut crate::TascamCard;
    let mut err: c_int;

    if tascam.is_null() {
        return 0;
    }

    dev_info(&mut (*intf).dev as *mut _ as *mut _, b"resuming TASCAM US-144MKII\n" as *const _ as *const _);

    err = usb_set_interface((*tascam).dev, 0, 1);
    if err < 0 {
        dev_err(
            &mut (*intf).dev as *mut _ as *mut _,
            b"resume: failed to set alt setting on intf 0: %d\n" as *const _ as *const _,
            err,
        );
        return err;
    }
    err = usb_set_interface((*tascam).dev, 1, 1);
    if err < 0 {
        dev_err(
            &mut (*intf).dev as *mut _ as *mut _,
            b"resume: failed to set alt setting on intf 1: %d\n" as *const _ as *const _,
            err,
        );
        return err;
    }

    if (*tascam).current_rate > 0 {
        us144mkii_configure_device_for_rate(tascam as *mut _, (*tascam).current_rate);
    }

    0
}

pub unsafe fn tascam_error_timer(t: *mut timer_list) {
    let tascam = crate::container_of(t, crate::TascamCard, error_timer);

    if atomic_read(&(*tascam).midi_in_active) != 0 {
        schedule_work(&mut (*tascam).midi_in_work);
    }
    if atomic_read(&(*tascam).midi_out_active) != 0 {
        schedule_work(&mut (*tascam).midi_out_work);
    }
}

pub unsafe fn tascam_probe(
    intf: *mut usb_interface,
    _usb_id: *const usb_device_id,
) -> c_int {
    let dev = interface_to_usbdev(intf);
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut tascam: *mut crate::TascamCard;
    let mut err: c_int;

    if (*dev).speed != 0 { // USB_SPEED_HIGH
        dev_info(
            &mut (*dev).dev as *mut _ as *mut _,
            b"Device is connected to a USB 1.1 port, this is not supported.\n" as *const _ as *const _,
        );
    }

    if (*(*intf).cur_altsetting).desc.bInterfaceNumber == 1 {
        let intf_zero = usb_ifnum_to_if(dev, 0);

        if intf_zero.is_null() {
            return -19; // -ENODEV
        }
        tascam = usb_get_intfdata(intf_zero) as *mut crate::TascamCard;
        if !tascam.is_null() {
            usb_set_intfdata(intf, tascam as *mut _);
            (*tascam).iface1 = intf;
        }
        return 0;
    }

    if dev_idx >= SNDRV_CARDS as c_int {
        dev_err(&mut (*dev).dev as *mut _ as *mut _, b"Too many TASCAM devices present" as *const _ as *const _);
        return -19; // -ENODEV
    }

    if !enable[dev_idx as usize] {
        dev_info(&mut (*dev).dev as *mut _ as *mut _, b"TASCAM US-144MKII device disabled" as *const _ as *const _);
        return -2; // -ENOENT
    }

    let handshake_buf = kmalloc(1, GFP_KERNEL) as *mut u8;
    if handshake_buf.is_null() {
        return -12; // -ENOMEM
    }

    err = usb_control_msg(
        dev,
        usb_rcvctrlpipe(dev, 0),
        VENDOR_REQ_MODE_CONTROL,
        RT_D2H_VENDOR_DEV,
        MODE_VAL_HANDSHAKE_READ,
        0x0000,
        handshake_buf as *mut core::ffi::c_void,
        1,
        USB_CTRL_TIMEOUT_MS,
    );
    if err < 0 {
        dev_err(&mut (*dev).dev as *mut _ as *mut _, b"Handshake read failed with %d\n" as *const _ as *const _, err);
        kfree(handshake_buf as *const _);
        return err;
    }

    if *handshake_buf != 0x12 && *handshake_buf != 0x16 && *handshake_buf != 0x30
        && *handshake_buf != 0x32
    {
        dev_err(
            &mut (*dev).dev as *mut _ as *mut _,
            b"Unexpected handshake value: 0x%x\n" as *const _ as *const _,
            *handshake_buf,
        );
        kfree(handshake_buf as *const _);
        return -19; // -ENODEV
    }

    kfree(handshake_buf as *const _);

    err = usb_set_interface(dev, 0, 1);
    if err < 0 {
        dev_err(
            &mut (*dev).dev as *mut _ as *mut _,
            b"Failed to set alt setting 1 on interface 0: %d\n" as *const _ as *const _,
            err,
        );
        return err;
    }

    err = usb_set_interface(dev, 1, 1);
    if err < 0 {
        dev_err(
            &mut (*dev).dev as *mut _ as *mut _,
            b"Failed to set alt setting 1 on interface 1: %d\n" as *const _ as *const _,
            err,
        );
        return err;
    }

    err = snd_card_new(
        &mut (*dev).dev as *mut _ as *mut core::ffi::c_void,
        index[dev_idx as usize],
        id[dev_idx as usize] as *const core::ffi::c_char,
        core::ptr::null_mut(),
        core::mem::size_of::<crate::TascamCard>(),
        &mut card,
    );
    if err < 0 {
        dev_err(&mut (*dev).dev as *mut _ as *mut _, b"Failed to create sound card instance\n" as *const _ as *const _);
        return err;
    }

    tascam = (*card).private_data as *mut crate::TascamCard;
    (*card).private_free = Some(tascam_card_private_free);
    (*tascam).dev = usb_get_dev(dev);
    (*tascam).card = card;
    (*tascam).iface0 = intf;
    (*tascam).digital_out_source = 1;
    (*tascam).capture_34_source = 1;

    spin_lock_init(&mut (*tascam).lock);
    spin_lock_init(&mut (*tascam).midi_in_lock);
    spin_lock_init(&mut (*tascam).midi_out_lock);
    init_usb_anchor(&mut (*tascam).playback_anchor);
    init_usb_anchor(&mut (*tascam).capture_anchor);
    init_usb_anchor(&mut (*tascam).feedback_anchor);
    init_usb_anchor(&mut (*tascam).midi_in_anchor);
    init_usb_anchor(&mut (*tascam).midi_out_anchor);

    timer_setup(&mut (*tascam).error_timer, tascam_error_timer, 0);

    INIT_WORK(&mut (*tascam).stop_work, tascam_stop_work_handler);
    INIT_WORK(&mut (*tascam).stop_pcm_work, tascam_stop_pcm_work_handler);
    INIT_WORK(&mut (*tascam).capture_work, tascam_capture_work_handler);
    init_completion(&mut (*tascam).midi_out_drain_completion as *mut _ as *mut _);

    if kfifo_alloc(&mut (*tascam).midi_in_fifo, MIDI_IN_FIFO_SIZE, GFP_KERNEL) != 0 {
        snd_card_free(card);
        return -12; // -ENOMEM
    }

    strscpy(
        (*card).driver.as_mut_ptr(),
        DRIVER_NAME.as_ptr() as *const _,
        core::mem::size_of_val(&(*card).driver),
    );
    if le16_to_cpu((*dev).descriptor.idProduct) == USB_PID_TASCAM_US144 {
        strscpy(
            (*card).shortname.as_mut_ptr(),
            b"TASCAM US-144\0" as *const _ as *const _,
            core::mem::size_of_val(&(*card).shortname),
        );
    } else if le16_to_cpu((*dev).descriptor.idProduct) == USB_PID_TASCAM_US144MKII {
        strscpy(
            (*card).shortname.as_mut_ptr(),
            b"TASCAM US-144MKII\0" as *const _ as *const _,
            core::mem::size_of_val(&(*card).shortname),
        );
    } else {
        strscpy(
            (*card).shortname.as_mut_ptr(),
            b"TASCAM Unknown\0" as *const _ as *const _,
            core::mem::size_of_val(&(*card).shortname),
        );
    }
    snprintf(
        (*card).longname.as_mut_ptr(),
        core::mem::size_of_val(&(*card).longname),
        b"%s (%04x:%04x) at %s\0" as *const _ as *const _,
        (*card).shortname.as_ptr(),
        USB_VID_TASCAM,
        (*dev).descriptor.idProduct,
        dev_name(&mut (*dev).dev as *const _),
    );

    err = snd_pcm_new(
        card,
        b"US144MKII PCM\0" as *const _ as *const _,
        0,
        1,
        1,
        &mut (*tascam).pcm as *mut _ as *mut *mut core::ffi::c_void,
    );
    if err < 0 {
        goto_probe_error(tascam, card);
        return err;
    }
    (*(*tascam).pcm as *mut crate::SndPcm).private_data = tascam as *mut _;
    strscpy(
        (*(*tascam).pcm as *mut crate::SndPcm).name.as_mut_ptr(),
        b"US144MKII PCM\0" as *const _ as *const _,
        core::mem::size_of_val(&(*(*tascam).pcm as *mut crate::SndPcm).name),
    );

    err = tascam_init_pcm((*tascam).pcm as *mut _);
    if err < 0 {
        goto_probe_error(tascam, card);
        return err;
    }

    err = tascam_create_midi(tascam as *mut _);
    if err < 0 {
        goto_probe_error(tascam, card);
        return err;
    }

    err = tascam_create_controls(tascam as *mut _);
    if err < 0 {
        goto_probe_error(tascam, card);
        return err;
    }

    err = tascam_alloc_urbs(tascam as *mut _);
    if err < 0 {
        goto_probe_error(tascam, card);
        return err;
    }

    err = snd_card_register(card);
    if err < 0 {
        goto_probe_error(tascam, card);
        return err;
    }

    usb_set_intfdata(intf, tascam as *mut _);

    dev_idx += 1;
    0
}

unsafe fn goto_probe_error(tascam: *mut crate::TascamCard, card: *mut snd_card) {
    tascam_free_urbs(tascam as *mut _);
    snd_card_free(card);
}

pub unsafe fn tascam_disconnect(intf: *mut usb_interface) {
    let tascam = usb_get_intfdata(intf) as *mut crate::TascamCard;

    if tascam.is_null() {
        return;
    }

    if (*(*intf).cur_altsetting).desc.bInterfaceNumber == 0 {
        snd_card_disconnect((*tascam).card);

        usb_kill_anchored_urbs(&mut (*tascam).playback_anchor);
        usb_kill_anchored_urbs(&mut (*tascam).capture_anchor);
        usb_kill_anchored_urbs(&mut (*tascam).feedback_anchor);
        usb_kill_anchored_urbs(&mut (*tascam).midi_in_anchor);
        usb_kill_anchored_urbs(&mut (*tascam).midi_out_anchor);

        cancel_work_sync(&mut (*tascam).stop_work);
        cancel_work_sync(&mut (*tascam).capture_work);
        cancel_work_sync(&mut (*tascam).midi_in_work);
        cancel_work_sync(&mut (*tascam).midi_out_work);
        cancel_work_sync(&mut (*tascam).stop_pcm_work);
        timer_delete_sync(&mut (*tascam).error_timer);
        tascam_free_urbs(tascam as *mut _);
        snd_card_free((*tascam).card);
        dev_idx -= 1;
    }
}

const TASCAM_DEVICE_COUNT: usize = 2;
static mut tascam_usb_ids: [usb_device_id; TASCAM_DEVICE_COUNT + 1] = [
    usb_device_id {
        idVendor: USB_VID_TASCAM,
        idProduct: USB_PID_TASCAM_US144,
    },
    usb_device_id {
        idVendor: USB_VID_TASCAM,
        idProduct: USB_PID_TASCAM_US144MKII,
    },
    usb_device_id {
        idVendor: 0,
        idProduct: 0,
    },
];

#[repr(C)]
struct usb_device_id {
    idVendor: u16,
    idProduct: u16,
}

// Helper function for container_of pattern
#[inline]
pub unsafe fn container_of<T, U>(ptr: *mut U, member_offset: usize) -> *mut T {
    (ptr as *mut u8).offset(-(member_offset as isize)) as *mut T
}

// Structure definitions (partial, from us144mkii.h)
#[repr(C)]
pub struct TascamCard {
    pub dev: *mut usb_device,
    pub card: *mut snd_card,
    pub pcm: *mut core::ffi::c_void,
    pub iface0: *mut usb_interface,
    pub iface1: *mut usb_interface,
    pub lock: spinlock_t,
    pub midi_in_lock: spinlock_t,
    pub midi_out_lock: spinlock_t,
    pub playback_urbs: [*mut urb; NUM_PLAYBACK_URBS],
    pub feedback_urbs: [*mut urb; NUM_FEEDBACK_URBS],
    pub capture_urbs: [*mut urb; NUM_CAPTURE_URBS],
    pub midi_in_urbs: [*mut urb; NUM_MIDI_IN_URBS],
    pub midi_out_urbs: [*mut urb; NUM_MIDI_OUT_URBS],
    pub playback_anchor: usb_anchor,
    pub capture_anchor: usb_anchor,
    pub feedback_anchor: usb_anchor,
    pub midi_in_anchor: usb_anchor,
    pub midi_out_anchor: usb_anchor,
    pub playback_urb_alloc_size: usize,
    pub feedback_urb_alloc_size: usize,
    pub capture_urb_alloc_size: usize,
    pub error_timer: timer_list,
    pub stop_work: work_struct,
    pub stop_pcm_work: work_struct,
    pub capture_work: work_struct,
    pub midi_in_work: work_struct,
    pub midi_out_work: work_struct,
    pub midi_out_drain_completion: core::ffi::c_void,
    pub midi_in_fifo: kfifo,
    pub capture_ring_buffer: *mut core::ffi::c_void,
    pub capture_decode_raw_block: *mut core::ffi::c_void,
    pub capture_decode_dst_block: *mut core::ffi::c_void,
    pub capture_routing_buffer: *mut core::ffi::c_void,
    pub active_urbs: i32,
    pub midi_in_active: i32,
    pub midi_out_active: i32,
    pub digital_out_source: i32,
    pub capture_34_source: i32,
    pub current_rate: c_int,
}

#[repr(C)]
pub struct SndPcm {
    pub private_data: *mut core::ffi::c_void,
    pub name: [core::ffi::c_char; 80],
}

// Stub for usb_fill_bulk_urb macro behavior
unsafe fn usb_fill_bulk_urb(
    urb: *mut urb,
    dev: *mut usb_device,
    pipe: c_int,
    transfer_buffer: *mut core::ffi::c_void,
    transfer_buffer_length: usize,
    complete: Option<unsafe extern "C" fn(*mut urb)>,
    context: *mut core::ffi::c_void,
) {
    (*urb).dev = dev;
    (*urb).pipe = pipe;
    (*urb).transfer_buffer = transfer_buffer;
    (*urb).transfer_buffer_length = transfer_buffer_length as _;
    (*urb).complete = complete;
    (*urb).context = context;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
