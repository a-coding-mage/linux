// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Linux USB driver
 *
 * Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
 */

// Requires: linux/slab.h, sound/core.h, sound/pcm.h, sound/pcm_params.h
// Requires: capture.h, driver.h, pcm.h

use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_void;
use core::ptr;

// Type declarations from other files
#[repr(C)]
pub struct snd_line6_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct urb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_iso_packet_descriptor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_line6 {
    _private: [u8; 0],
}

// Constants from headers
const LINE6_ISO_PACKETS: usize = 1;
const LINE6_ISO_INTERVAL: c_uint = 1;
const LINE6_STREAM_IMPULSE: c_ulong = 0;
const LINE6_STREAM_PCM: c_ulong = 1;
const LINE6_STREAM_CAPTURE_HELPER: u32 = 2;
const GFP_ATOMIC: c_int = 32;
const GFP_KERNEL: c_int = 208;
const URB_ISO_ASAP: u32 = 0x80;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const USB_ENDPOINT_NUMBER_MASK: u32 = 0x0f;
const EINVAL: c_int = -22;
const ENOMEM: c_int = -12;
const EXDEV: c_int = -19;

// External C functions
extern "C" {
    fn find_first_zero_bit(addr: *const c_ulong, nbits: c_uint) -> c_int;
    fn set_bit(nr: c_ulong, addr: *mut c_ulong);
    fn clear_bit(nr: c_ulong, addr: *mut c_ulong);
    fn test_bit(nr: c_ulong, addr: *const c_ulong) -> c_int;
    fn test_and_clear_bit(nr: c_ulong, addr: *mut c_ulong) -> c_int;

    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn usb_submit_urb(urb: *mut urb, mem_flags: c_int) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn get_substream(
        line6pcm: *mut snd_line6_pcm,
        stream: c_int,
    ) -> *mut snd_pcm_substream;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_line6_pcm;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_hw_constraint_ratdens(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        ratdens: *mut c_void,
    ) -> c_int;
    fn line6_pcm_acquire(line6pcm: *mut snd_line6_pcm, stream: u32, silent: bool) -> c_int;
    fn line6_pcm_release(line6pcm: *mut snd_line6_pcm, stream: u32);
    fn usb_alloc_urb(iso_packets: c_int, mem_flags: c_int) -> *mut urb;
    fn usb_rcvisocpipe(dev: *mut c_void, endpoint: u32) -> u32;
    fn usb_urb_ep_type_check(urb: *mut urb) -> c_int;
    fn kzalloc_objs(size: usize, gfp: c_int) -> *mut c_void;

    fn snd_line6_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut c_void) -> c_int;
    fn snd_line6_hw_free(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_line6_prepare(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_line6_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
    fn snd_line6_pointer(substream: *mut snd_pcm_substream) -> c_ulong;

    fn spin_unlock(lock: *mut c_void);
    fn spin_lock(lock: *mut c_void);
}

unsafe fn submit_audio_in_urb(line6pcm: *mut snd_line6_pcm) -> c_int {
    let mut index: c_int;
    let mut i: c_int;
    let mut urb_size: c_int;
    let mut ret: c_int;
    let urb_in: *mut urb;

    index = find_first_zero_bit(&(*line6pcm).in_active_urbs, (*(*line6pcm).line6).iso_buffers);

    if index < 0 || index >= (*(*line6pcm).line6).iso_buffers {
        dev_err(
            (*(*line6pcm).line6).ifcdev as *mut c_void,
            b"no free URB found\n".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    urb_in = (*line6pcm).in_urbs.add(index as usize) as *mut urb;
    urb_size = 0;

    i = 0;
    while i < LINE6_ISO_PACKETS as c_int {
        let fin: *mut usb_iso_packet_descriptor = ptr::addr_of_mut!((*urb_in).iso_frame_desc[i as usize]);
        (*fin).offset = urb_size;
        (*fin).length = (*line6pcm).max_packet_size_in;
        urb_size += (*line6pcm).max_packet_size_in;
        i += 1;
    }

    (*urb_in).transfer_buffer = ((*line6pcm).in_buffer as *mut u8)
        .add(index as usize * LINE6_ISO_PACKETS * (*line6pcm).max_packet_size_in as usize) as *mut c_void;
    (*urb_in).transfer_buffer_length = urb_size;
    (*urb_in).context = line6pcm as *mut c_void;

    ret = usb_submit_urb(urb_in, GFP_ATOMIC);

    if ret == 0 {
        set_bit(index as c_ulong, &mut (*line6pcm).in_active_urbs);
    } else {
        dev_err(
            (*(*line6pcm).line6).ifcdev as *mut c_void,
            b"URB in #%d submission failed (%d)\n".as_ptr() as *const c_char,
            index,
            ret,
        );
    }

    0
}

pub unsafe extern "C" fn line6_submit_audio_in_all_urbs(line6pcm: *mut snd_line6_pcm) -> c_int {
    let mut ret: c_int = 0;
    let mut i: c_int = 0;

    while i < (*(*line6pcm).line6).iso_buffers {
        ret = submit_audio_in_urb(line6pcm);
        if ret < 0 {
            break;
        }
        i += 1;
    }

    ret
}

pub unsafe extern "C" fn line6_capture_copy(
    line6pcm: *mut snd_line6_pcm,
    fbuf: *mut c_char,
    fsize: c_int,
) {
    let substream: *mut snd_pcm_substream =
        get_substream(line6pcm, SNDRV_PCM_STREAM_CAPTURE);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let bytes_per_frame: c_int = (*(*line6pcm).properties).bytes_per_channel
        * (*(*line6pcm).properties).capture_hw.channels_max;
    let frames: c_int = fsize / bytes_per_frame;

    if runtime.is_null() {
        return;
    }

    if (*line6pcm).in_pos_done + frames > (*runtime).buffer_size {
        let mut len: c_int;

        len = (*runtime).buffer_size - (*line6pcm).in_pos_done;

        if len > 0 {
            memcpy(
                ((*runtime).dma_area as *mut u8)
                    .add((*line6pcm).in_pos_done as usize * bytes_per_frame as usize)
                    as *mut c_void,
                fbuf as *const c_void,
                (len * bytes_per_frame) as usize,
            );
            memcpy(
                (*runtime).dma_area as *mut c_void,
                (fbuf as *mut u8)
                    .add(len as usize * bytes_per_frame as usize)
                    as *const c_void,
                ((frames - len) * bytes_per_frame) as usize,
            );
        } else {
            dev_err(
                (*(*line6pcm).line6).ifcdev as *mut c_void,
                b"driver bug: len = %d\n".as_ptr() as *const c_char,
                len,
            );
        }
    } else {
        memcpy(
            ((*runtime).dma_area as *mut u8)
                .add((*line6pcm).in_pos_done as usize * bytes_per_frame as usize)
                as *mut c_void,
            fbuf as *const c_void,
            fsize as usize,
        );
    }

    (*line6pcm).in_pos_done += frames;
    if (*line6pcm).in_pos_done >= (*runtime).buffer_size {
        (*line6pcm).in_pos_done -= (*runtime).buffer_size;
    }
}

pub unsafe extern "C" fn line6_capture_check_period(
    line6pcm: *mut snd_line6_pcm,
    length: c_int,
) {
    let substream: *mut snd_pcm_substream =
        get_substream(line6pcm, SNDRV_PCM_STREAM_CAPTURE);

    (*line6pcm).in_bytes += length;
    if (*line6pcm).in_bytes >= (*line6pcm).in_period {
        (*line6pcm).in_bytes %= (*line6pcm).in_period;
        spin_unlock(&mut (*line6pcm).in_lock as *mut c_void);
        snd_pcm_period_elapsed(substream);
        spin_lock(&mut (*line6pcm).in_lock as *mut c_void);
    }
}

unsafe extern "C" fn audio_in_callback(urb: *mut urb) {
    let mut i: c_int;
    let mut index: c_int;
    let mut length: c_int = 0;
    let mut shutdown: c_int = 0;
    let line6pcm: *mut snd_line6_pcm = (*urb).context as *mut snd_line6_pcm;

    (*line6pcm).in_last_frame = (*urb).start_frame;

    index = 0;
    while index < (*(*line6pcm).line6).iso_buffers {
        if urb == (*line6pcm).in_urbs.add(index as usize) as *mut urb {
            break;
        }
        index += 1;
    }

    spin_lock(&mut (*line6pcm).in_lock as *mut c_void);

    i = 0;
    while i < LINE6_ISO_PACKETS as c_int {
        let fbuf: *mut c_char;
        let fsize: c_int;
        let fin: *mut usb_iso_packet_descriptor = ptr::addr_of_mut!((*urb).iso_frame_desc[i as usize]);

        if (*fin).status == -EXDEV {
            shutdown = 1;
            break;
        }

        fbuf = ((*urb).transfer_buffer as *mut u8).add((*fin).offset as usize) as *mut c_char;
        fsize = (*fin).actual_length;

        if fsize > (*line6pcm).max_packet_size_in {
            dev_err(
                (*(*line6pcm).line6).ifcdev as *mut c_void,
                b"driver and/or device bug: packet too large (%d > %d)\n".as_ptr() as *const c_char,
                fsize,
                (*line6pcm).max_packet_size_in,
            );
        }

        length += fsize;

        // BUILD_BUG_ON_MSG(LINE6_ISO_PACKETS != 1, ...)
        // Compile-time check: LINE6_ISO_PACKETS must equal 1

        (*line6pcm).prev_fbuf = fbuf;
        (*line6pcm).prev_fsize = fsize
            / ((*(*line6pcm).properties).bytes_per_channel
                * (*(*line6pcm).properties).capture_hw.channels_max);

        if (test_bit(LINE6_STREAM_IMPULSE, &(*line6pcm).in_running) == 0)
            && (test_bit(LINE6_STREAM_PCM, &(*line6pcm).in_running) != 0)
            && (fsize > 0)
        {
            line6_capture_copy(line6pcm, fbuf, fsize);
        }

        i += 1;
    }

    clear_bit(index as c_ulong, &mut (*line6pcm).in_active_urbs);

    if test_and_clear_bit(index as c_ulong, &mut (*line6pcm).in_unlink_urbs) != 0 {
        shutdown = 1;
    }

    if shutdown == 0 {
        submit_audio_in_urb(line6pcm);

        if (test_bit(LINE6_STREAM_IMPULSE, &(*line6pcm).in_running) == 0)
            && (test_bit(LINE6_STREAM_PCM, &(*line6pcm).in_running) != 0)
        {
            line6_capture_check_period(line6pcm, length);
        }
    }

    spin_unlock(&mut (*line6pcm).in_lock as *mut c_void);
}

unsafe extern "C" fn snd_line6_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let mut err: c_int;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let line6pcm: *mut snd_line6_pcm = snd_pcm_substream_chip(substream);

    err = snd_pcm_hw_constraint_ratdens(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &mut (*(*line6pcm).properties).rates as *mut c_void,
    );
    if err < 0 {
        return err;
    }

    line6_pcm_acquire(line6pcm, LINE6_STREAM_CAPTURE_HELPER, false);

    (*runtime).hw = (*(*line6pcm).properties).capture_hw;
    0
}

unsafe extern "C" fn snd_line6_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let line6pcm: *mut snd_line6_pcm = snd_pcm_substream_chip(substream);

    line6_pcm_release(line6pcm, LINE6_STREAM_CAPTURE_HELPER);
    0
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut c_void) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_ulong>,
}

#[no_mangle]
pub static snd_line6_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_line6_capture_open),
    close: Some(snd_line6_capture_close),
    hw_params: Some(snd_line6_hw_params),
    hw_free: Some(snd_line6_hw_free),
    prepare: Some(snd_line6_prepare),
    trigger: Some(snd_line6_trigger),
    pointer: Some(snd_line6_pointer),
};

pub unsafe extern "C" fn line6_create_audio_in_urbs(line6pcm: *mut snd_line6_pcm) -> c_int {
    let line6: *mut usb_line6 = (*line6pcm).line6;
    let mut i: c_int;

    (*line6pcm).in_urbs =
        kzalloc_objs(core::mem::size_of::<*mut urb>() * (*line6).iso_buffers as usize, GFP_KERNEL)
            as *mut *mut urb;
    if (*line6pcm).in_urbs.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*line6).iso_buffers {
        let urb: *mut urb;

        urb = usb_alloc_urb(LINE6_ISO_PACKETS as c_int, GFP_KERNEL);

        if urb.is_null() {
            return -ENOMEM;
        }

        *(*line6pcm).in_urbs.add(i as usize) = urb;

        (*urb).dev = (*line6).usbdev;
        (*urb).pipe = usb_rcvisocpipe(
            (*line6).usbdev,
            (*(*line6).properties).ep_audio_r & USB_ENDPOINT_NUMBER_MASK,
        );
        (*urb).transfer_flags = URB_ISO_ASAP;
        (*urb).start_frame = -1;
        (*urb).number_of_packets = LINE6_ISO_PACKETS as c_int;
        (*urb).interval = LINE6_ISO_INTERVAL;
        (*urb).error_count = 0;
        (*urb).complete = Some(audio_in_callback);
        if usb_urb_ep_type_check(urb) != 0 {
            return -EINVAL;
        }

        i += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
