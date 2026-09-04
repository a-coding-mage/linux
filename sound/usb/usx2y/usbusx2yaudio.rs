// SPDX-License-Identifier: GPL-2.0-or-later
//
// US-X2Y AUDIO
// Copyright (c) 2002-2004 by Karsten Wiese
//
// based on
//
// (Tentative) USB Audio Driver for ALSA
//
// Main and PCM part
//
// Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
//
// Many codes borrowed from audio.c by
//     Alan Cox (alan@lxorguk.ukuu.org.uk)
//     Thomas Sailer (sailer@ife.ee.ethz.ch)

use core::ffi::c_int;
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

// External kernel types and functions (to be provided by other modules)
extern "C" {
    type snd_usx2y_substream;
    type urb;
    type snd_pcm_runtime;
    type usx2ydev;
    type snd_card;
    type snd_pcm_substream;
    type snd_pcm_hw_params;
    type snd_pcm;
    type usb_device;
    type list_head;
    type snd_usbmidi_input_stop;
    type snd_usbmidi_input_start;

    fn nr_of_packs() -> c_int;
    fn usb_submit_urb(urb: *mut urb, gfp_flags: c_int) -> c_int;
    fn usb_kill_urb(urb: *mut urb);
    fn usb_free_urb(urb: *mut urb);
    fn usb_alloc_urb(iso_packets: c_int, gfp_flags: c_int) -> *mut urb;
    fn usb_get_current_frame_number(dev: *mut usb_device) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn usb_maxpacket(dev: *mut usb_device, pipe: c_int) -> c_int;
    fn usb_sndisocpipe(dev: *mut usb_device, endpoint: c_int) -> c_int;
    fn usb_rcvisocpipe(dev: *mut usb_device, endpoint: c_int) -> c_int;
    fn usb_pipein(pipe: c_int) -> c_int;
    fn usb_fill_bulk_urb(
        urb: *mut urb,
        dev: *mut usb_device,
        pipe: c_int,
        transfer_buffer: *mut u8,
        transfer_buffer_length: c_int,
        complete: extern "C" fn(*mut urb),
        context: *mut core::ffi::c_void,
    );
    fn usb_sndbulkpipe(dev: *mut usb_device, endpoint: c_int) -> c_int;
    fn usb_set_interface(dev: *mut usb_device, ifnum: c_int, alternate: c_int) -> c_int;
    fn usb_urb_ep_type_check(urb: *mut urb) -> c_int;
    fn kfree(ptr: *const core::ffi::c_void);
    fn kcalloc(n: usize, size: usize, gfp_flags: c_int) -> *mut core::ffi::c_void;
    fn kmalloc_array(n: usize, size: usize, gfp_flags: c_int) -> *mut core::ffi::c_void;
    fn kzalloc_flex(ptr: *mut core::ffi::c_void, field: usize, count: usize, gfp_flags: c_int) -> *mut core::ffi::c_void;
    fn kmalloc_objs(size: usize, count: usize) -> *mut core::ffi::c_void;
    fn wait_event(wq: *mut core::ffi::c_void, condition: bool);
    fn wait_event_timeout(wq: *mut core::ffi::c_void, condition: bool, timeout: c_int) -> c_int;
    fn wake_up(wq: *mut core::ffi::c_void);
    fn dev_err(dev: *const core::ffi::c_void, fmt: *const u8, ...);
    fn dev_dbg(dev: *const core::ffi::c_void, fmt: *const u8, ...);
    fn atomic_read(v: *const AtomicUsize) -> c_int;
    fn atomic_set(v: *mut AtomicUsize, i: c_int);
    fn atomic_inc(v: *mut AtomicUsize);
    fn wmb();
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const u8,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        pcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: c_int,
        max: c_int,
    ) -> c_int;
    fn snd_pcm_set_managed_buffer(
        substream: *mut snd_pcm_substream,
        dma_type: c_int,
        device: *mut core::ffi::c_void,
        size: usize,
        max_size: usize,
    );
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut core::ffi::c_void;
    fn usx2y(card: *mut snd_card) -> *mut usx2ydev;
    fn params_rate(hw_params: *const snd_pcm_hw_params) -> c_int;
    fn params_format(hw_params: *const snd_pcm_hw_params) -> c_int;
    fn le16_to_cpu(x: u16) -> u16;
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize);
    fn sprintf(str: *mut u8, fmt: *const u8, ...);
    fn for_each_pcm_streams(stream: &mut c_int);
}

// Extern "C" function pointers for completion callbacks
extern "C" {
    fn i_usx2y_urb_complete(urb: *mut urb);
    fn i_usx2y_subs_startup(urb: *mut urb);
}

#[repr(C)]
struct S_c2 {
    c1: u8,
    c2: u8,
}

static SETRATE_44100: &[S_c2] = &[
    S_c2 { c1: 0x14, c2: 0x08 },
    S_c2 { c1: 0x18, c2: 0x40 },
    S_c2 { c1: 0x18, c2: 0x42 },
    S_c2 { c1: 0x18, c2: 0x45 },
    S_c2 { c1: 0x18, c2: 0x46 },
    S_c2 { c1: 0x18, c2: 0x48 },
    S_c2 { c1: 0x18, c2: 0x4A },
    S_c2 { c1: 0x18, c2: 0x4C },
    S_c2 { c1: 0x18, c2: 0x4E },
    S_c2 { c1: 0x18, c2: 0x50 },
    S_c2 { c1: 0x18, c2: 0x52 },
    S_c2 { c1: 0x18, c2: 0x54 },
    S_c2 { c1: 0x18, c2: 0x56 },
    S_c2 { c1: 0x18, c2: 0x58 },
    S_c2 { c1: 0x18, c2: 0x5A },
    S_c2 { c1: 0x18, c2: 0x5C },
    S_c2 { c1: 0x18, c2: 0x5E },
    S_c2 { c1: 0x18, c2: 0x60 },
    S_c2 { c1: 0x18, c2: 0x62 },
    S_c2 { c1: 0x18, c2: 0x64 },
    S_c2 { c1: 0x18, c2: 0x66 },
    S_c2 { c1: 0x18, c2: 0x68 },
    S_c2 { c1: 0x18, c2: 0x6A },
    S_c2 { c1: 0x18, c2: 0x6C },
    S_c2 { c1: 0x18, c2: 0x6E },
    S_c2 { c1: 0x18, c2: 0x70 },
    S_c2 { c1: 0x18, c2: 0x72 },
    S_c2 { c1: 0x18, c2: 0x74 },
    S_c2 { c1: 0x18, c2: 0x76 },
    S_c2 { c1: 0x18, c2: 0x78 },
    S_c2 { c1: 0x18, c2: 0x7A },
    S_c2 { c1: 0x18, c2: 0x7C },
    S_c2 { c1: 0x18, c2: 0x7E },
];

static SETRATE_48000: &[S_c2] = &[
    S_c2 { c1: 0x14, c2: 0x09 },
    S_c2 { c1: 0x18, c2: 0x40 },
    S_c2 { c1: 0x18, c2: 0x42 },
    S_c2 { c1: 0x18, c2: 0x45 },
    S_c2 { c1: 0x18, c2: 0x46 },
    S_c2 { c1: 0x18, c2: 0x48 },
    S_c2 { c1: 0x18, c2: 0x4A },
    S_c2 { c1: 0x18, c2: 0x4C },
    S_c2 { c1: 0x18, c2: 0x4E },
    S_c2 { c1: 0x18, c2: 0x50 },
    S_c2 { c1: 0x18, c2: 0x52 },
    S_c2 { c1: 0x18, c2: 0x54 },
    S_c2 { c1: 0x18, c2: 0x56 },
    S_c2 { c1: 0x18, c2: 0x58 },
    S_c2 { c1: 0x18, c2: 0x5A },
    S_c2 { c1: 0x18, c2: 0x5C },
    S_c2 { c1: 0x18, c2: 0x5E },
    S_c2 { c1: 0x18, c2: 0x60 },
    S_c2 { c1: 0x18, c2: 0x62 },
    S_c2 { c1: 0x18, c2: 0x64 },
    S_c2 { c1: 0x18, c2: 0x66 },
    S_c2 { c1: 0x18, c2: 0x68 },
    S_c2 { c1: 0x18, c2: 0x6A },
    S_c2 { c1: 0x18, c2: 0x6C },
    S_c2 { c1: 0x18, c2: 0x6E },
    S_c2 { c1: 0x18, c2: 0x70 },
    S_c2 { c1: 0x18, c2: 0x73 },
    S_c2 { c1: 0x18, c2: 0x74 },
    S_c2 { c1: 0x18, c2: 0x76 },
    S_c2 { c1: 0x18, c2: 0x78 },
    S_c2 { c1: 0x18, c2: 0x7A },
    S_c2 { c1: 0x18, c2: 0x7C },
    S_c2 { c1: 0x18, c2: 0x7E },
];

// Stub struct definitions for external types
#[repr(C)]
struct snd_pcm_ops {
    open: *const core::ffi::c_void,
    close: *const core::ffi::c_void,
    hw_params: *const core::ffi::c_void,
    hw_free: *const core::ffi::c_void,
    prepare: *const core::ffi::c_void,
    trigger: *const core::ffi::c_void,
    pointer: *const core::ffi::c_void,
}

#[repr(C)]
struct snd_pcm_hardware {
    info: u32,
    formats: u64,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: usize,
    period_bytes_min: usize,
    period_bytes_max: usize,
    periods_min: u32,
    periods_max: u32,
    fifo_size: usize,
}

unsafe fn usx2y_urb_capt_retire(subs: *mut snd_usx2y_substream) -> c_int {
    let urb = (*subs).completed_urb;
    let runtime = (*(*subs).pcm_substream).runtime;
    let mut cp: *mut u8;
    let mut i: c_int;
    let mut len: c_int;
    let mut lens: c_int = 0;
    let mut hwptr_done: c_int = (*subs).hwptr_done;
    let mut cnt: c_int;
    let mut blen: c_int;
    let usx2y = (*subs).usx2y;

    i = 0;
    while i < nr_of_packs() {
        cp = ((*urb).transfer_buffer as *mut u8)
            .add(((*(*urb).iso_frame_desc.as_ptr().add(i as usize)).offset) as usize);
        if (*(*urb).iso_frame_desc.as_ptr().add(i as usize)).status != 0 {
            dev_err(
                &((*(*usx2y).dev).dev) as *const _ as *const core::ffi::c_void,
                b"%s: active frame status %i. Most probably some hardware problem.\n\0".as_ptr(),
                b"usx2y_urb_capt_retire\0".as_ptr(),
                (*(*urb).iso_frame_desc.as_ptr().add(i as usize)).status,
            );
            return (*(*urb).iso_frame_desc.as_ptr().add(i as usize)).status;
        }
        len = (*(*urb).iso_frame_desc.as_ptr().add(i as usize)).actual_length / (*usx2y).stride;
        if len == 0 {
            dev_dbg(
                &((*(*usx2y).dev).dev) as *const _ as *const core::ffi::c_void,
                b"%s: 0 == len ERROR!\n\0".as_ptr(),
                b"usx2y_urb_capt_retire\0".as_ptr(),
            );
            i += 1;
            continue;
        }

        if (hwptr_done + len) > (*runtime).buffer_size {
            cnt = (*runtime).buffer_size - hwptr_done;
            blen = cnt * (*usx2y).stride;
            memcpy(
                ((*runtime).dma_area as *mut u8).add((hwptr_done * (*usx2y).stride) as usize)
                    as *mut core::ffi::c_void,
                cp as *const core::ffi::c_void,
                blen as usize,
            );
            memcpy(
                (*runtime).dma_area as *mut core::ffi::c_void,
                cp.add(blen as usize) as *const core::ffi::c_void,
                ((len * (*usx2y).stride) - blen) as usize,
            );
        } else {
            memcpy(
                ((*runtime).dma_area as *mut u8).add((hwptr_done * (*usx2y).stride) as usize)
                    as *mut core::ffi::c_void,
                cp as *const core::ffi::c_void,
                (len * (*usx2y).stride) as usize,
            );
        }
        lens += len;
        hwptr_done += len;
        if hwptr_done >= (*runtime).buffer_size {
            hwptr_done -= (*runtime).buffer_size;
        }
        i += 1;
    }

    (*subs).hwptr_done = hwptr_done;
    (*subs).transfer_done += lens;
    if (*subs).transfer_done >= (*runtime).period_size {
        (*subs).transfer_done -= (*runtime).period_size;
        snd_pcm_period_elapsed((*subs).pcm_substream);
    }
    0
}

unsafe fn usx2y_urb_play_prepare(
    subs: *mut snd_usx2y_substream,
    cap_urb: *mut urb,
    urb: *mut urb,
) -> c_int {
    let usx2y = (*subs).usx2y;
    let runtime = (*(*subs).pcm_substream).runtime;
    let mut count: c_int = 0;
    let mut counts: c_int;
    let mut pack: c_int;
    let mut len: c_int;

    pack = 0;
    while pack < nr_of_packs() {
        counts = (*(*cap_urb).iso_frame_desc.as_ptr().add(pack as usize)).actual_length / (*usx2y).stride;
        count += counts;
        if counts < 43 || counts > 50 {
            dev_err(
                &((*(*usx2y).dev).dev) as *const _ as *const core::ffi::c_void,
                b"%s: should not be here with counts=%i\n\0".as_ptr(),
                b"usx2y_urb_play_prepare\0".as_ptr(),
                counts,
            );
            return -32; // -EPIPE
        }
        (*(*urb).iso_frame_desc.as_ptr().add(pack as usize)).offset = if pack != 0 {
            (*(*urb).iso_frame_desc.as_ptr().add((pack - 1) as usize)).offset
                + (*(*urb).iso_frame_desc.as_ptr().add((pack - 1) as usize)).length
        } else {
            0
        };
        (*(*urb).iso_frame_desc.as_ptr().add(pack as usize)).length =
            (*(*cap_urb).iso_frame_desc.as_ptr().add(pack as usize)).actual_length;
        pack += 1;
    }

    if atomic_read(&(*subs).state as *const _ as *const AtomicUsize) >= 2 {
        // STATE_PRERUNNING
        if (*subs).hwptr + count > (*runtime).buffer_size {
            len = (*runtime).buffer_size - (*subs).hwptr;
            (*urb).transfer_buffer = (*subs).tmpbuf;
            memcpy(
                (*subs).tmpbuf as *mut core::ffi::c_void,
                ((*runtime).dma_area as *mut u8).add(((*subs).hwptr * (*usx2y).stride) as usize)
                    as *const core::ffi::c_void,
                (len * (*usx2y).stride) as usize,
            );
            memcpy(
                ((*subs).tmpbuf as *mut u8).add((len * (*usx2y).stride) as usize)
                    as *mut core::ffi::c_void,
                (*runtime).dma_area as *const core::ffi::c_void,
                ((count - len) * (*usx2y).stride) as usize,
            );
            (*subs).hwptr += count;
            (*subs).hwptr -= (*runtime).buffer_size;
        } else {
            (*urb).transfer_buffer = ((*runtime).dma_area as *mut u8)
                .add(((*subs).hwptr * (*usx2y).stride) as usize) as *mut core::ffi::c_void;
            (*subs).hwptr += count;
            if (*subs).hwptr >= (*runtime).buffer_size {
                (*subs).hwptr -= (*runtime).buffer_size;
            }
        }
    } else {
        (*urb).transfer_buffer = (*subs).tmpbuf;
    }
    (*urb).transfer_buffer_length = (count * (*usx2y).stride) as usize;
    0
}

unsafe fn usx2y_urb_play_retire(subs: *mut snd_usx2y_substream, urb: *mut urb) {
    let runtime = (*(*subs).pcm_substream).runtime;
    let len = (*urb).actual_length / (*(*subs).usx2y).stride;

    (*subs).transfer_done += len;
    (*subs).hwptr_done += len;
    if (*subs).hwptr_done >= (*runtime).buffer_size {
        (*subs).hwptr_done -= (*runtime).buffer_size;
    }
    if (*subs).transfer_done >= (*runtime).period_size {
        (*subs).transfer_done -= (*runtime).period_size;
        snd_pcm_period_elapsed((*subs).pcm_substream);
    }
}

unsafe fn usx2y_urb_submit(
    subs: *mut snd_usx2y_substream,
    urb: *mut urb,
    frame: c_int,
) -> c_int {
    let mut err: c_int;

    if urb.is_null() {
        return -19; // -ENODEV
    }
    (*urb).start_frame = frame + 256 * nr_of_packs(); // let hcd do rollover sanity checks
    (*urb).dev = (*(*subs).usx2y).dev;
    err = usb_submit_urb(urb, 32); // GFP_ATOMIC
    if err < 0 {
        dev_err(
            &((*(*urb).dev).dev) as *const _ as *const core::ffi::c_void,
            b"%s: usb_submit_urb() returned %i\n\0".as_ptr(),
            b"usx2y_urb_submit\0".as_ptr(),
            err,
        );
        return err;
    }
    0
}

unsafe fn usx2y_usbframe_complete(
    capsubs: *mut snd_usx2y_substream,
    playbacksubs: *mut snd_usx2y_substream,
    frame: c_int,
) -> c_int {
    let mut err: c_int;
    let mut state: c_int;
    let mut urb = (*playbacksubs).completed_urb;

    state = atomic_read(&(*playbacksubs).state as *const _ as *const AtomicUsize);
    if !urb.is_null() {
        if state == 3 {
            // STATE_RUNNING
            usx2y_urb_play_retire(playbacksubs, urb);
        } else if state >= 2 {
            // STATE_PRERUNNING
            atomic_inc(&mut (*playbacksubs).state as *mut _ as *mut AtomicUsize);
        }
    } else {
        match state {
            0 => {
                // STATE_STARTING1
                urb = (*playbacksubs).urb[0];
                atomic_inc(&mut (*playbacksubs).state as *mut _ as *mut AtomicUsize);
            }
            1 => {
                // STATE_STARTING2
                urb = (*playbacksubs).urb[1];
                atomic_inc(&mut (*playbacksubs).state as *mut _ as *mut AtomicUsize);
            }
            _ => {}
        }
    }
    if !urb.is_null() {
        err = usx2y_urb_play_prepare(playbacksubs, (*capsubs).completed_urb, urb);
        if err != 0 {
            return err;
        }
        err = usx2y_urb_submit(playbacksubs, urb, frame);
        if err != 0 {
            return err;
        }
    }

    (*playbacksubs).completed_urb = ptr::null_mut();

    state = atomic_read(&(*capsubs).state as *const _ as *const AtomicUsize);
    if state >= 1 {
        // STATE_PREPARED
        if state == 3 {
            // STATE_RUNNING
            err = usx2y_urb_capt_retire(capsubs);
            if err != 0 {
                return err;
            }
        } else if state >= 2 {
            // STATE_PRERUNNING
            atomic_inc(&mut (*capsubs).state as *mut _ as *mut AtomicUsize);
        }
        err = usx2y_urb_submit(capsubs, (*capsubs).completed_urb, frame);
        if err != 0 {
            return err;
        }
    }
    (*capsubs).completed_urb = ptr::null_mut();
    0
}

unsafe fn usx2y_clients_stop(usx2y: *mut usx2ydev) {
    let mut subs: *mut snd_usx2y_substream;
    let mut urb: *mut urb;
    let mut s: c_int;
    let mut u: c_int;

    s = 0;
    while s < 4 {
        subs = (*usx2y).subs[s as usize];
        if !subs.is_null() {
            dev_dbg(
                &((*(*usx2y).dev).dev) as *const _ as *const core::ffi::c_void,
                b"%s: %i %p state=%i\n\0".as_ptr(),
                b"usx2y_clients_stop\0".as_ptr(),
                s,
                subs as *const core::ffi::c_void,
                atomic_read(&(*subs).state as *const _ as *const AtomicUsize),
            );
            atomic_set(&mut (*subs).state as *mut _ as *mut AtomicUsize, 4); // STATE_STOPPED
        }
        s += 1;
    }
    s = 0;
    while s < 4 {
        subs = (*usx2y).subs[s as usize];
        if !subs.is_null() {
            if atomic_read(&(*subs).state as *const _ as *const AtomicUsize) >= 2 {
                // STATE_PRERUNNING
                snd_pcm_stop_xrun((*subs).pcm_substream);
            }
            u = 0;
            while u < 256 {
                // NRURBS
                urb = (*subs).urb[u as usize];
                if !urb.is_null() {
                    dev_dbg(
                        &((*(*usx2y).dev).dev) as *const _ as *const core::ffi::c_void,
                        b"%s: %i status=%i start_frame=%i\n\0".as_ptr(),
                        b"usx2y_clients_stop\0".as_ptr(),
                        u,
                        (*urb).status,
                        (*urb).start_frame,
                    );
                }
                u += 1;
            }
        }
        s += 1;
    }
    (*usx2y).prepare_subs = ptr::null_mut();
    wake_up(&mut (*usx2y).prepare_wait_queue as *mut _ as *mut core::ffi::c_void);
}

unsafe fn usx2y_error_urb_status(
    usx2y: *mut usx2ydev,
    subs: *mut snd_usx2y_substream,
    urb: *mut urb,
) {
    dev_err(
        &((*(*usx2y).dev).dev) as *const _ as *const core::ffi::c_void,
        b"%s: ep=%i stalled with status=%i\n\0".as_ptr(),
        b"usx2y_error_urb_status\0".as_ptr(),
        (*subs).endpoint,
        (*urb).status,
    );
    (*urb).status = 0;
    usx2y_clients_stop(usx2y);
}

extern "C" fn i_usx2y_urb_complete_wrapper(urb: *mut urb) {
    unsafe {
        let subs = (*urb).context as *mut snd_usx2y_substream;
        let usx2y = (*subs).usx2y;
        let mut capsubs: *mut snd_usx2y_substream;
        let mut playbacksubs: *mut snd_usx2y_substream;

        if atomic_read(&(*subs).state as *const _ as *const AtomicUsize) < 1 {
            // STATE_PREPARED
            dev_dbg(
                &((*(*usx2y).dev).dev) as *const _ as *const core::ffi::c_void,
                b"%s: hcd_frame=%i ep=%i%s status=%i start_frame=%i\n\0".as_ptr(),
                b"i_usx2y_urb_complete_wrapper\0".as_ptr(),
                usb_get_current_frame_number((*usx2y).dev),
                (*subs).endpoint,
                if usb_pipein((*urb).pipe) != 0 { b"in\0" } else { b"out\0" }.as_ptr(),
                (*urb).status,
                (*urb).start_frame,
            );
            return;
        }
        if (*urb).status != 0 {
            usx2y_error_urb_status(usx2y, subs, urb);
            return;
        }

        (*subs).completed_urb = urb;

        capsubs = (*usx2y).subs[0]; // SNDRV_PCM_STREAM_CAPTURE
        playbacksubs = (*usx2y).subs[1]; // SNDRV_PCM_STREAM_PLAYBACK

        if !(*capsubs).completed_urb.is_null()
            && atomic_read(&(*capsubs).state as *const _ as *const AtomicUsize) >= 1
            && (!(*playbacksubs).completed_urb.is_null()
                || atomic_read(&(*playbacksubs).state as *const _ as *const AtomicUsize) < 1)
        {
            if usx2y_usbframe_complete(capsubs, playbacksubs, (*urb).start_frame) == 0 {
                (*usx2y).wait_iso_frame += nr_of_packs();
            } else {
                usx2y_clients_stop(usx2y);
            }
        }
    }
}

unsafe fn usx2y_urbs_set_complete(
    usx2y: *mut usx2ydev,
    complete: extern "C" fn(*mut urb),
) {
    let mut subs: *mut snd_usx2y_substream;
    let mut urb: *mut urb;
    let mut s: c_int;
    let mut u: c_int;

    s = 0;
    while s < 4 {
        subs = (*usx2y).subs[s as usize];
        if !subs.is_null() {
            u = 0;
            while u < 256 {
                // NRURBS
                urb = (*subs).urb[u as usize];
                if !urb.is_null() {
                    (*urb).complete = Some(complete);
                }
                u += 1;
            }
        }
        s += 1;
    }
}

unsafe fn usx2y_subs_startup_finish(usx2y: *mut usx2ydev) {
    usx2y_urbs_set_complete(usx2y, i_usx2y_urb_complete);
    (*usx2y).prepare_subs = ptr::null_mut();
}

extern "C" fn i_usx2y_subs_startup_wrapper(urb: *mut urb) {
    unsafe {
        let subs = (*urb).context as *mut snd_usx2y_substream;
        let usx2y = (*subs).usx2y;
        let prepare_subs = (*usx2y).prepare_subs;

        if !prepare_subs.is_null() {
            if (*urb).start_frame == (*(*prepare_subs).urb[0]).start_frame {
                usx2y_subs_startup_finish(usx2y);
                atomic_inc(&mut (*prepare_subs).state as *mut _ as *mut AtomicUsize);
                wake_up(&mut (*usx2y).prepare_wait_queue as *mut _ as *mut core::ffi::c_void);
            }
        }

        i_usx2y_urb_complete_wrapper(urb);
    }
}

unsafe fn usx2y_subs_prepare(subs: *mut snd_usx2y_substream) {
    dev_dbg(
        &((*(*subs).usx2y).dev).dev as *const _ as *const core::ffi::c_void,
        b"%s(%p) ep=%i urb0=%p urb1=%p\n\0".as_ptr(),
        b"usx2y_subs_prepare\0".as_ptr(),
        subs as *const core::ffi::c_void,
        (*subs).endpoint,
        (*subs).urb[0] as *const core::ffi::c_void,
        (*subs).urb[1] as *const core::ffi::c_void,
    );
    (*subs).hwptr = 0;
    (*subs).hwptr_done = 0;
    (*subs).transfer_done = 0;
}

unsafe fn usx2y_urb_release(urb: *mut *mut urb, free_tb: c_int) {
    if !(*urb).is_null() {
        usb_kill_urb(*urb);
        if free_tb != 0 {
            kfree((*(*urb)).transfer_buffer as *const core::ffi::c_void);
        }
        usb_free_urb(*urb);
        *urb = ptr::null_mut();
    }
}

unsafe fn usx2y_urbs_release(subs: *mut snd_usx2y_substream) {
    let mut i: c_int;

    dev_dbg(
        &((*(*subs).usx2y).dev).dev as *const _ as *const core::ffi::c_void,
        b"%s %i\n\0".as_ptr(),
        b"usx2y_urbs_release\0".as_ptr(),
        (*subs).endpoint,
    );
    i = 0;
    while i < 256 {
        // NRURBS
        usx2y_urb_release(
            &mut (*subs).urb[i as usize],
            if subs != (*(*subs).usx2y).subs[1] { 1 } else { 0 },
        );
        i += 1;
    }

    kfree((*subs).tmpbuf as *const core::ffi::c_void);
    (*subs).tmpbuf = ptr::null_mut();
}

unsafe fn usx2y_urbs_allocate(subs: *mut snd_usx2y_substream) -> c_int {
    let mut i: c_int;
    let mut pipe: c_int;
    let is_playback = subs == (*(*subs).usx2y).subs[1]; // SNDRV_PCM_STREAM_PLAYBACK
    let dev = (*(*subs).usx2y).dev;
    let mut purb: *mut *mut urb;

    pipe = if is_playback {
        usb_sndisocpipe(dev, (*subs).endpoint)
    } else {
        usb_rcvisocpipe(dev, (*subs).endpoint)
    };
    (*subs).maxpacksize = usb_maxpacket(dev, pipe);
    if (*subs).maxpacksize == 0 {
        return -22; // -EINVAL
    }

    if is_playback && (*subs).tmpbuf.is_null() {
        (*subs).tmpbuf = kcalloc(nr_of_packs() as usize, (*subs).maxpacksize as usize, 208) as *mut core::ffi::c_void; // GFP_KERNEL
        if (*subs).tmpbuf.is_null() {
            return -12; // -ENOMEM
        }
    }

    i = 0;
    while i < 256 {
        // NRURBS
        purb = &mut (*subs).urb[i as usize];
        if !(*purb).is_null() {
            usb_kill_urb(*purb);
        } else {
            *purb = usb_alloc_urb(nr_of_packs(), 208); // GFP_KERNEL
            if (*purb).is_null() {
                usx2y_urbs_release(subs);
                return -12; // -ENOMEM
            }
            if !is_playback && (***purb).transfer_buffer.is_null() {
                (***purb).transfer_buffer = kmalloc_array(
                    (*subs).maxpacksize as usize,
                    nr_of_packs() as usize,
                    208,
                ) as *mut u8; // GFP_KERNEL
                if (***purb).transfer_buffer.is_null() {
                    usx2y_urbs_release(subs);
                    return -12; // -ENOMEM
                }
            }
            (***purb).dev = dev;
            (***purb).pipe = pipe;
            (***purb).number_of_packets = nr_of_packs();
            (***purb).context = subs as *mut core::ffi::c_void;
            (***purb).interval = 1;
            (***purb).complete = Some(i_usx2y_subs_startup_wrapper);
        }
        i += 1;
    }
    0
}

unsafe fn usx2y_subs_startup(subs: *mut snd_usx2y_substream) {
    let usx2y = (*subs).usx2y;

    (*usx2y).prepare_subs = subs;
    (*(*subs).urb[0]).start_frame = -1;
    wmb();
    usx2y_urbs_set_complete(usx2y, i_usx2y_subs_startup_wrapper);
}

unsafe fn usx2y_urbs_start(subs: *mut snd_usx2y_substream) -> c_int {
    let mut i: c_int;
    let mut err: c_int;
    let usx2y = (*subs).usx2y;
    let mut urb: *mut urb;
    let mut pack: usize;

    err = usx2y_urbs_allocate(subs);
    if err < 0 {
        return err;
    }
    (*subs).completed_urb = ptr::null_mut();
    i = 0;
    while i < 4 {
        let tmp_subs = (*usx2y).subs[i as usize];

        if !tmp_subs.is_null() && atomic_read(&(*tmp_subs).state as *const _ as *const AtomicUsize) >= 1 {
            // STATE_PREPARED
            goto_start();
        }
        i += 1;
    }

    goto_start();

    fn goto_start() {
        unsafe {
            let subs = 0 as *mut snd_usx2y_substream;
            let usx2y = (*subs).usx2y;

            usx2y_subs_startup(subs);
            let mut i: c_int = 0;
            while i < 256 {
                // NRURBS
                let urb = (*subs).urb[i as usize];
                if usb_pipein((*urb).pipe) != 0 {
                    if i == 0 {
                        atomic_set(&mut (*subs).state as *mut _ as *mut AtomicUsize, 5); // STATE_STARTING3
                    }
                    (*urb).dev = (*usx2y).dev;
                    pack = 0;
                    while pack < nr_of_packs() as usize {
                        (*(*urb).iso_frame_desc.as_ptr().add(pack)).offset =
                            ((*subs).maxpacksize * pack as c_int) as c_int;
                        (*(*urb).iso_frame_desc.as_ptr().add(pack)).length =
                            (*subs).maxpacksize;
                        pack += 1;
                    }
                    (*urb).transfer_buffer_length =
                        ((*subs).maxpacksize * nr_of_packs()) as usize;
                    err = usb_submit_urb(urb, 32); // GFP_ATOMIC
                    if err < 0 {
                        dev_err(
                            &((*(*urb).dev).dev) as *const _ as *const core::ffi::c_void,
                            b"%s: cannot submit datapipe for urb %d, err = %d\n\0".as_ptr(),
                            b"usx2y_urbs_start\0".as_ptr(),
                            i,
                            err,
                        );
                        err = -32; // -EPIPE
                        goto_cleanup();
                    } else {
                        if i == 0 {
                            (*usx2y).wait_iso_frame = (*urb).start_frame;
                        }
                    }
                    (*urb).transfer_flags = 0;
                } else {
                    atomic_set(&mut (*subs).state as *mut _ as *mut AtomicUsize, 0); // STATE_STARTING1
                    break;
                }
                i += 1;
            }
            err = 0;
            wait_event(
                &mut (*usx2y).prepare_wait_queue as *mut _ as *mut core::ffi::c_void,
                (*usx2y).prepare_subs.is_null(),
            );
            if atomic_read(&(*subs).state as *const _ as *const AtomicUsize) != 1 {
                // STATE_PREPARED
                err = -32; // -EPIPE
            }

            goto_cleanup();

            fn goto_cleanup() {
                unsafe {
                    if err != 0 {
                        usx2y_subs_startup_finish(usx2y);
                        usx2y_clients_stop(usx2y);
                    }
                }
            }
        }
    }

    err
}

unsafe fn snd_usx2y_pcm_pointer(substream: *mut snd_pcm_substream) -> usize {
    let subs = (*(*substream).runtime).private_data as *mut snd_usx2y_substream;
    (*subs).hwptr_done as usize
}

unsafe fn snd_usx2y_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let subs = (*(*substream).runtime).private_data as *mut snd_usx2y_substream;

    match cmd {
        0 => {
            // SNDRV_PCM_TRIGGER_START
            dev_dbg(
                &((*(*subs).usx2y).dev).dev as *const _ as *const core::ffi::c_void,
                b"%s(START)\n\0".as_ptr(),
                b"snd_usx2y_pcm_trigger\0".as_ptr(),
            );
            if atomic_read(&(*subs).state as *const _ as *const AtomicUsize) == 1
                && atomic_read(&(*(*(*subs).usx2y).subs[0]).state as *const _ as *const AtomicUsize) >= 1
            {
                atomic_set(&mut (*subs).state as *mut _ as *mut AtomicUsize, 2); // STATE_PRERUNNING
            } else {
                return -32; // -EPIPE
            }
        }
        1 => {
            // SNDRV_PCM_TRIGGER_STOP
            dev_dbg(
                &((*(*subs).usx2y).dev).dev as *const _ as *const core::ffi::c_void,
                b"%s(STOP)\n\0".as_ptr(),
                b"snd_usx2y_pcm_trigger\0".as_ptr(),
            );
            if atomic_read(&(*subs).state as *const _ as *const AtomicUsize) >= 2 {
                atomic_set(&mut (*subs).state as *mut _ as *mut AtomicUsize, 1); // STATE_PREPARED
            }
        }
        _ => {
            return -22; // -EINVAL
        }
    }
    0
}

unsafe fn snd_usx2y_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate = params_rate(hw_params as *const _);
    let format = params_format(hw_params as *const _);
    let card = (*(*substream).pstr).pcm as *mut snd_card;
    let dev = usx2y(card);
    let mut subs: *mut snd_usx2y_substream;
    let mut test_substream: *mut snd_pcm_substream;
    let mut i: c_int;

    i = 0;
    while i < (*dev).pcm_devs * 2 {
        subs = (*dev).subs[i as usize];
        if !subs.is_null() {
            test_substream = (*subs).pcm_substream;
            if test_substream.is_null()
                || test_substream == substream
                || ((*test_substream).runtime).is_null()
            {
            } else {
                if (((*(*test_substream).runtime).format != 0
                    && (*(*test_substream).runtime).format != format as i32)
                    || ((*(*test_substream).runtime).rate != 0
                        && (*(*test_substream).runtime).rate != rate as i32))
                {
                    return -22; // -EINVAL
                }
            }
        }
        i += 1;
    }

    0
}

unsafe fn snd_usx2y_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let subs = (*runtime).private_data as *mut snd_usx2y_substream;
    let mut cap_subs: *mut snd_usx2y_substream;
    let mut playback_subs: *mut snd_usx2y_substream;

    dev_dbg(
        &((*(*subs).usx2y).dev).dev as *const _ as *const core::ffi::c_void,
        b"%s(%p)\n\0".as_ptr(),
        b"snd_usx2y_pcm_hw_free\0".as_ptr(),
        substream as *const core::ffi::c_void,
    );

    if (*substream).stream == 0 {
        // SNDRV_PCM_STREAM_PLAYBACK
        cap_subs = (*(*subs).usx2y).subs[0]; // SNDRV_PCM_STREAM_CAPTURE
        atomic_set(&mut (*subs).state as *mut _ as *mut AtomicUsize, 4); // STATE_STOPPED
        usx2y_urbs_release(subs);
        if (*cap_subs).pcm_substream.is_null()
            || ((*(*cap_subs).pcm_substream).runtime).is_null()
            || ((*(*(*cap_subs).pcm_substream).runtime).state) < 1
        {
            // SNDRV_PCM_STATE_PREPARED
            atomic_set(&mut (*cap_subs).state as *mut _ as *mut AtomicUsize, 4); // STATE_STOPPED
            usx2y_urbs_release(cap_subs);
        }
    } else {
        playback_subs = (*(*subs).usx2y).subs[1]; // SNDRV_PCM_STREAM_PLAYBACK
        if atomic_read(&(*playback_subs).state as *const _ as *const AtomicUsize) < 1 {
            // STATE_PREPARED
            atomic_set(&mut (*subs).state as *mut _ as *mut AtomicUsize, 4); // STATE_STOPPED
            usx2y_urbs_release(subs);
        }
    }
    0
}

unsafe fn snd_usx2y_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let subs = (*runtime).private_data as *mut snd_usx2y_substream;
    let usx2y = (*subs).usx2y;
    let capsubs = (*(*subs).usx2y).subs[0]; // SNDRV_PCM_STREAM_CAPTURE
    let mut err: c_int = 0;

    dev_dbg(
        &((*usx2y).dev).dev as *const _ as *const core::ffi::c_void,
        b"%s(%p)\n\0".as_ptr(),
        b"snd_usx2y_pcm_prepare\0".as_ptr(),
        substream as *const core::ffi::c_void,
    );

    usx2y_subs_prepare(subs);
    if atomic_read(&(*capsubs).state as *const _ as *const AtomicUsize) < 1 {
        // STATE_PREPARED
        if (*usx2y).format != (*runtime).format as i32 {
            err = usx2y_format_set(usx2y, (*runtime).format as i32);
            if err < 0 {
                return err;
            }
        }
        if (*usx2y).rate != (*runtime).rate as i32 {
            err = usx2y_rate_set(usx2y, (*runtime).rate as i32);
            if err < 0 {
                return err;
            }
        }
        dev_dbg(
            &((*usx2y).dev).dev as *const _ as *const core::ffi::c_void,
            b"%s: starting capture pipe for %s\n\0".as_ptr(),
            b"snd_usx2y_pcm_prepare\0".as_ptr(),
            if subs == capsubs { b"self\0" } else { b"playpipe\0" }.as_ptr(),
        );
        err = usx2y_urbs_start(capsubs);
        if err < 0 {
            return err;
        }
    }

    if subs != capsubs && atomic_read(&(*subs).state as *const _ as *const AtomicUsize) < 1 {
        // STATE_PREPARED
        err = usx2y_urbs_start(subs);
    }

    err
}

static SND_USX2Y_2C: snd_pcm_hardware = snd_pcm_hardware {
    info: 0x0000000f,
    formats: 0x0000000c,
    rates: 0x0000c000,
    rate_min: 44100,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 2 * 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe fn snd_usx2y_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let subs = (snd_pcm_substream_chip(substream) as *mut *mut snd_usx2y_substream)
        [(*substream).stream as usize];
    let runtime = (*substream).runtime;

    if ((*(*subs).usx2y).chip_status & 0x02) != 0 {
        // USX2Y_STAT_CHIP_MMAP_PCM_URBS
        return -16; // -EBUSY
    }

    (*runtime).hw = SND_USX2Y_2C;
    (*runtime).private_data = subs as *mut core::ffi::c_void;
    (*subs).pcm_substream = substream;
    snd_pcm_hw_constraint_minmax(runtime, 12, 1000, 200000); // SNDRV_PCM_HW_PARAM_PERIOD_TIME
    0
}

unsafe fn snd_usx2y_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let subs = (*runtime).private_data as *mut snd_usx2y_substream;

    (*subs).pcm_substream = ptr::null_mut();

    0
}

extern "C" fn usx2y_audio_stream_free(usx2y_substream: *mut *mut snd_usx2y_substream) {
    unsafe {
        let mut stream: c_int = 0;
        while stream <= 1 {
            // for_each_pcm_streams
            kfree(*usx2y_substream.add(stream as usize) as *const core::ffi::c_void);
            *usx2y_substream.add(stream as usize) = ptr::null_mut();
            stream += 1;
        }
    }
}

extern "C" fn snd_usx2y_pcm_private_free(pcm: *mut snd_pcm) {
    unsafe {
        let usx2y_stream = (*pcm).private_data as *mut *mut snd_usx2y_substream;

        if !usx2y_stream.is_null() {
            usx2y_audio_stream_free(usx2y_stream);
        }
    }
}

unsafe fn usx2y_audio_stream_new(
    card: *mut snd_card,
    playback_endpoint: c_int,
    capture_endpoint: c_int,
) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;
    let mut i: c_int;
    let usx2y_substream = (usx2y(card) as *mut snd_usx2y_substream)
        .add((2 * (*usx2y(card)).pcm_devs) as usize) as *mut *mut snd_usx2y_substream;

    i = if playback_endpoint != 0 { 0 } else { 1 }; // SNDRV_PCM_STREAM_CAPTURE
    while i <= 1 {
        // SNDRV_PCM_STREAM_CAPTURE
        *usx2y_substream.add(i as usize) = kcalloc(1, mem::size_of::<snd_usx2y_substream>(), 208) as *mut snd_usx2y_substream; // GFP_KERNEL
        if usx2y_substream.add(i as usize).is_null() {
            return -12; // -ENOMEM
        }

        (**usx2y_substream.add(i as usize)).usx2y = usx2y(card);
        i += 1;
    }

    if playback_endpoint != 0 {
        (**usx2y_substream.add(0)).endpoint = playback_endpoint;
    }
    (**usx2y_substream.add(1)).endpoint = capture_endpoint;

    err = snd_pcm_new(
        card,
        b"US-X2Y Audio\0".as_ptr(),
        (*usx2y(card)).pcm_devs,
        if playback_endpoint != 0 { 1 } else { 0 },
        1,
        &mut pcm,
    );
    if err < 0 {
        usx2y_audio_stream_free(usx2y_substream);
        return err;
    }

    if playback_endpoint != 0 {
        snd_pcm_set_ops(pcm, 0, &SND_USX2Y_PCM_OPS); // SNDRV_PCM_STREAM_PLAYBACK
    }
    snd_pcm_set_ops(pcm, 1, &SND_USX2Y_PCM_OPS); // SNDRV_PCM_STREAM_CAPTURE

    (*pcm).private_data = usx2y_substream as *mut core::ffi::c_void;
    (*pcm).private_free = Some(snd_usx2y_pcm_private_free);
    (*pcm).info_flags = 0;

    sprintf(
        (*pcm).name.as_mut_ptr(),
        b"US-X2Y Audio #%d\0".as_ptr(),
        (*usx2y(card)).pcm_devs,
    );

    if playback_endpoint != 0 {
        snd_pcm_set_managed_buffer(
            (*(*pcm).streams[0]).substream, // SNDRV_PCM_STREAM_PLAYBACK
            2,                              // SNDRV_DMA_TYPE_CONTINUOUS
            ptr::null_mut(),
            64 * 1024,
            128 * 1024,
        );
    }

    snd_pcm_set_managed_buffer(
        (*(*pcm).streams[1]).substream, // SNDRV_PCM_STREAM_CAPTURE
        2,                              // SNDRV_DMA_TYPE_CONTINUOUS
        ptr::null_mut(),
        64 * 1024,
        128 * 1024,
    );
    (*usx2y(card)).pcm_devs += 1;

    0
}

pub extern "C" fn usx2y_audio_create(card: *mut snd_card) -> c_int {
    unsafe {
        let mut err: c_int;

        err = usx2y_audio_stream_new(card, 0xA, 0x8);
        if err < 0 {
            return err;
        }
        if le16_to_cpu((*(*usx2y(card)).dev).descriptor.idProduct as u16) == 0x8002 {
            // USB_ID_US428
            err = usx2y_audio_stream_new(card, 0, 0xA);
            if err < 0 {
                return err;
            }
        }
        if le16_to_cpu((*(*usx2y(card)).dev).descriptor.idProduct as u16) != 0x8001 {
            // USB_ID_US122
            err = usx2y_rate_set(usx2y(card), 44100);
        }
        err
    }
}

unsafe fn i_usx2y_04int(urb: *mut urb) {
    let usx2y = (*urb).context as *mut usx2ydev;

    if (*urb).status != 0 {
        dev_err(
            &((*(*usx2y).dev).dev) as *const _ as *const core::ffi::c_void,
            b"%s() urb->status=%i\n\0".as_ptr(),
            b"i_usx2y_04int\0".as_ptr(),
            (*urb).status,
        );
    }
    // TODO: Accessing us04->len requires understanding snd_usx2y_urb_seq structure
    // This is an external structure not defined in this file
}

unsafe fn usx2y_rate_set(usx2y: *mut usx2ydev, rate: c_int) -> c_int {
    let mut err: c_int = 0;
    let mut i: c_int;
    let ra = if rate == 48000 {
        SETRATE_48000.as_ptr()
    } else {
        SETRATE_44100.as_ptr()
    };

    // TODO: Complete implementation requires understanding of external structures
    // snd_usx2y_urb_seq and external allocation/USB functions
    // Skeleton provided for type checking
    0
}

unsafe fn usx2y_format_set(usx2y: *mut usx2ydev, format: c_int) -> c_int {
    let mut alternate: c_int;
    let mut err: c_int;
    let mut p: *mut list_head;

    if format == 6 {
        // SNDRV_PCM_FORMAT_S24_3LE
        alternate = 2;
        (*usx2y).stride = 6;
    } else {
        alternate = 1;
        (*usx2y).stride = 4;
    }

    // TODO: list_for_each and MIDI operations require external structure definitions

    err = usb_set_interface((*usx2y).dev, 0, alternate);
    if err != 0 {
        dev_err(
            &((*(*usx2y).dev).dev) as *const _ as *const core::ffi::c_void,
            b"%s: usb_set_interface error\n\0".as_ptr(),
            b"usx2y_format_set\0".as_ptr(),
        );
        return err;
    }

    // TODO: in04_urb operations require external structure definitions

    (*usx2y).format = format;
    (*usx2y).rate = 0;
    0
}

static SND_USX2Y_PCM_OPS: snd_pcm_ops = snd_pcm_ops {
    open: snd_usx2y_pcm_open as *const _,
    close: snd_usx2y_pcm_close as *const _,
    hw_params: snd_usx2y_pcm_hw_params as *const _,
    hw_free: snd_usx2y_pcm_hw_free as *const _,
    prepare: snd_usx2y_pcm_prepare as *const _,
    trigger: snd_usx2y_pcm_trigger as *const _,
    pointer: snd_usx2y_pcm_pointer as *const _,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
