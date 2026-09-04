// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Linux USB driver
 *
 * Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
 */

use core::ffi::{c_int, c_uint, c_void};

// Opaque types for kernel structures
#[repr(C)]
pub struct urb {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_line6_pcm {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct usb_iso_packet_descriptor {
    pub offset: u32,
    pub length: u32,
    pub status: c_int,
}

#[repr(C)]
pub struct usb_line6 {
    _unused: [u8; 0],
}

// Constants
const GFP_ATOMIC: c_uint = 0x20;
const GFP_KERNEL: c_uint = 0xd0;
const SINGLE_DEPTH_NESTING: c_uint = 1;
const LINE6_ISO_PACKETS: usize = 1;
const LINE6_ISO_INTERVAL: c_int = 1;
const LINE6_STREAM_PCM: c_uint = 0;
const LINE6_FLAG_PAUSE_PLAYBACK: c_uint = 1;
const LINE6_STREAM_IMPULSE: c_uint = 2;
const LINE6_CAP_HWMON: c_uint = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_uint = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 11;
const URB_ISO_ASAP: c_uint = 0x0080;
const EXDEV: c_int = -71;
const EINVAL: c_int = -22;
const ENOMEM: c_int = -12;
const USB_ENDPOINT_NUMBER_MASK: c_uint = 0x0f;

// Conditional compilation simulation
const USE_CLEAR_BUFFER_WORKAROUND: bool = false;

// Helper functions for endian conversion
fn le16_to_cpu(val: u16) -> i16 {
    i16::from_le(val as i16)
}

fn cpu_to_le16(val: i16) -> u16 {
    val.to_le() as u16
}

fn clamp(val: i32, min: i32, max: i32) -> i16 {
    if val < min {
        min as i16
    } else if val > max {
        max as i16
    } else {
        val as i16
    }
}

// External functions from kernel
extern "C" {
    fn find_first_zero_bit(addr: *const core::ffi::c_ulong, nbits: c_uint) -> c_int;
    fn dev_err(dev: *mut c_void, fmt: *const c_int, ...);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn test_bit(nr: c_uint, addr: *const c_void) -> c_int;
    fn set_bit(nr: c_uint, addr: *mut c_void);
    fn clear_bit(nr: c_uint, addr: *mut c_void);
    fn test_and_clear_bit(nr: c_uint, addr: *mut c_void) -> c_int;
    fn spin_lock_nested(lock: *mut c_void, subclass: c_uint);
    fn spin_unlock(lock: *mut c_void);
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut core::ffi::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: core::ffi::c_ulong);
    fn usb_submit_urb(urb: *mut urb, mem_flags: c_uint) -> c_int;
    fn usb_alloc_urb(iso_packets: c_int, mem_flags: c_uint) -> *mut urb;
    fn usb_sndisocpipe(dev: *mut c_void, endpoint: c_uint) -> c_uint;
    fn usb_urb_ep_type_check(urb: *mut urb) -> c_int;
    fn get_substream(line6pcm: *mut snd_line6_pcm, stream: c_uint) -> *mut snd_pcm_substream;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_line6_pcm;
    fn snd_pcm_hw_constraint_ratdens(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        rats: *const c_void,
    ) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn kzalloc_objs(size: usize, count: c_int) -> *mut c_void;
    fn line6_capture_copy(line6pcm: *mut snd_line6_pcm, buffer: *mut u8, length: usize);
    fn line6_capture_check_period(line6pcm: *mut snd_line6_pcm, length: usize);

    pub fn snd_line6_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut c_void) -> c_int;
    pub fn snd_line6_hw_free(substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_line6_prepare(substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_line6_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
    pub fn snd_line6_pointer(substream: *mut snd_pcm_substream) -> core::ffi::c_ulong;
}

/*
    Software stereo volume control.
*/
unsafe fn change_volume(urb_out: *mut urb, volume: *const [i32; 2], bytes_per_frame: c_int) {
    let mut chn = 0;

    if (*volume)[0] == 256 && (*volume)[1] == 256 {
        return; /* maximum volume - no change */
    }

    if bytes_per_frame == 4 {
        let mut p = (*urb_out).transfer_buffer as *mut u16;
        let buf_end =
            ((*urb_out).transfer_buffer as usize + (*urb_out).transfer_buffer_length) as *mut u16;

        while p < buf_end {
            let pv = le16_to_cpu(*p as u16);
            let val = ((pv as i32 * (*volume)[chn & 1]) >> 8) as i32;
            let pv = clamp(val, -0x8000, 0x7fff);
            *p = cpu_to_le16(pv);
            chn += 1;
            p = p.offset(1);
        }
    } else if bytes_per_frame == 6 {
        let mut p = (*urb_out).transfer_buffer as *mut u8;
        let buf_end =
            ((*urb_out).transfer_buffer as usize + (*urb_out).transfer_buffer_length) as *mut u8;

        while p < buf_end {
            let val: i32 = *p as i32
                + ((*p.offset(1) as i32) << 8)
                + ((*(p.offset(2)) as i8 as i32) << 16);
            let val = ((val * (*volume)[chn & 1]) >> 8);
            let val = clamp(val, -0x800000, 0x7fffff) as i32;
            *p = (val & 0xff) as u8;
            *p.offset(1) = ((val >> 8) & 0xff) as u8;
            *p.offset(2) = ((val >> 16) & 0xff) as u8;
            chn += 1;
            p = p.offset(3);
        }
    }
}

/*
    Create signal for impulse response test.
*/
unsafe fn create_impulse_test_signal(
    line6pcm: *mut snd_line6_pcm,
    urb_out: *mut urb,
    bytes_per_frame: c_int,
) {
    let frames = (*urb_out).transfer_buffer_length / bytes_per_frame as usize;

    if bytes_per_frame == 4 {
        let mut pi = (*line6pcm).prev_fbuf as *mut i16;
        let mut po = (*urb_out).transfer_buffer as *mut i16;

        for _ in 0..frames {
            *po = *pi;
            *po.offset(1) = 0;
            pi = pi.offset(2);
            po = po.offset(2);
        }
    } else if bytes_per_frame == 6 {
        let mut pi = (*line6pcm).prev_fbuf as *mut u8;
        let mut po = (*urb_out).transfer_buffer as *mut u8;

        for _ in 0..frames {
            for j in 0..(bytes_per_frame as usize / 2) {
                *po.offset(j as isize) = *pi.offset(j as isize);
            }

            for j in (bytes_per_frame as usize / 2)..bytes_per_frame as usize {
                *po.offset(j as isize) = 0;
            }

            pi = pi.offset(bytes_per_frame as isize);
            po = po.offset(bytes_per_frame as isize);
        }
    }

    (*line6pcm).impulse_count -= 1;
    if (*line6pcm).impulse_count <= 0 {
        *((*urb_out).transfer_buffer as *mut u8).offset(bytes_per_frame as isize - 1) =
            (*line6pcm).impulse_volume;
        (*line6pcm).impulse_count = (*line6pcm).impulse_period;
    }
}

/*
    Add signal to buffer for software monitoring.
*/
unsafe fn add_monitor_signal(
    urb_out: *mut urb,
    signal: *mut u8,
    volume: c_int,
    bytes_per_frame: c_int,
) {
    if volume == 0 {
        return; /* zero volume - no change */
    }

    if bytes_per_frame == 4 {
        let mut pi = signal as *mut u16;
        let mut po = (*urb_out).transfer_buffer as *mut u16;
        let buf_end =
            ((*urb_out).transfer_buffer as usize + (*urb_out).transfer_buffer_length) as *mut u16;

        while po < buf_end {
            let pov = le16_to_cpu(*po as u16);
            let piv = le16_to_cpu(*pi as u16);
            let val = pov as i32 + ((piv as i32 * volume) >> 8);
            let pov = clamp(val, -0x8000, 0x7fff);
            *po = cpu_to_le16(pov);
            pi = pi.offset(1);
            po = po.offset(1);
        }
    }

    /*
       We don't need to handle devices with 6 bytes per frame here
       since they all support hardware monitoring.
     */
}

/*
    Find a free URB, prepare audio data, and submit URB.
    must be called in line6pcm->out.lock context
*/
unsafe fn submit_audio_out_urb(line6pcm: *mut snd_line6_pcm) -> c_int {
    let index: c_uint;
    let mut i: c_int;
    let urb_size: usize;
    let urb_frames: usize;
    let ret: c_int;
    let bytes_per_frame: c_int = ((*(*line6pcm).properties).bytes_per_channel
        * (*(*line6pcm).properties).playback_hw.channels_max)
        as c_int;
    let frame_increment: c_int = (*(*(*line6pcm).properties).rates.rats.as_ptr()).num_min as c_int;
    let frame_factor: c_int =
        ((*(*(*line6pcm).properties).rates.rats.as_ptr()).den as c_int)
            * (((*(*line6pcm).line6).intervals_per_second / LINE6_ISO_INTERVAL) as c_int);
    let urb_out: *mut urb;

    index = find_first_zero_bit(&(*line6pcm).out.active_urbs, (*(*line6pcm).line6).iso_buffers)
        as c_uint;

    if (index as c_int) < 0 || (index as c_int) >= (*(*line6pcm).line6).iso_buffers {
        dev_err(
            (*(*line6pcm).line6).ifcdev,
            "no free URB found\n" as *const _ as *const c_int,
        );
        return -EINVAL;
    }

    urb_out = *(*line6pcm)
        .out
        .urbs
        .offset(index as isize) as *mut urb;
    let mut urb_size_mut = 0usize;

    /* TODO: this may not work for LINE6_ISO_PACKETS != 1 */
    i = 0;
    while i < LINE6_ISO_PACKETS as c_int {
        /* compute frame size for given sampling rate */
        let mut fsize: c_int = 0;
        let fout = &mut (*urb_out).iso_frame_desc[i as usize];

        fsize = (*line6pcm).prev_fsize;
        if fsize == 0 {
            let n: c_int;

            (*line6pcm).out.count += frame_increment;
            n = (*line6pcm).out.count / frame_factor;
            (*line6pcm).out.count -= n * frame_factor;
            fsize = n;
        }

        fsize *= bytes_per_frame;

        fout.offset = urb_size_mut as u32;
        fout.length = fsize as u32;
        urb_size_mut += fsize as usize;
        i += 1;
    }
    urb_size = urb_size_mut;

    if urb_size == 0 {
        /* can't determine URB size */
        dev_err(
            (*(*line6pcm).line6).ifcdev,
            "driver bug: urb_size = 0\n" as *const _ as *const c_int,
        );
        return -EINVAL;
    }

    urb_frames = urb_size / bytes_per_frame as usize;
    (*urb_out).transfer_buffer = (*line6pcm)
        .out
        .buffer
        .offset((index as usize * LINE6_ISO_PACKETS * (*line6pcm).max_packet_size_out) as isize)
        as *mut c_void;
    (*urb_out).transfer_buffer_length = urb_size;
    (*urb_out).context = line6pcm as *mut c_void;

    if test_bit(LINE6_STREAM_PCM, &(*line6pcm).out.running as *const _ as *const c_void) != 0
        && test_bit(
            LINE6_FLAG_PAUSE_PLAYBACK,
            &(*line6pcm).flags as *const _ as *const c_void,
        ) == 0
    {
        let runtime: *mut snd_pcm_runtime =
            (*get_substream(line6pcm, SNDRV_PCM_STREAM_PLAYBACK)).runtime;

        if (*line6pcm).out.pos + urb_frames as core::ffi::c_ulong > (*runtime).buffer_size {
            /*
               The transferred area goes over buffer boundary,
               copy the data to the temp buffer.
             */
            let len: core::ffi::c_ulong;

            len = (*runtime).buffer_size - (*line6pcm).out.pos;

            if len > 0 {
                memcpy(
                    (*urb_out).transfer_buffer,
                    ((*runtime).dma_area as usize
                        + ((*line6pcm).out.pos * bytes_per_frame as core::ffi::c_ulong) as usize)
                        as *const c_void,
                    (len as c_int * bytes_per_frame) as usize,
                );
                memcpy(
                    ((*urb_out).transfer_buffer as usize + (len as c_int * bytes_per_frame) as usize)
                        as *mut c_void,
                    (*runtime).dma_area,
                    ((urb_frames as c_int - len as c_int) * bytes_per_frame) as usize,
                );
            } else {
                dev_err(
                    (*(*line6pcm).line6).ifcdev,
                    "driver bug: len = %d\n" as *const _ as *const c_int,
                    len,
                );
            }
        } else {
            memcpy(
                (*urb_out).transfer_buffer,
                ((*runtime).dma_area as usize
                    + ((*line6pcm).out.pos * bytes_per_frame as core::ffi::c_ulong) as usize)
                    as *const c_void,
                (*urb_out).transfer_buffer_length,
            );
        }

        (*line6pcm).out.pos += urb_frames as core::ffi::c_ulong;
        if (*line6pcm).out.pos >= (*runtime).buffer_size {
            (*line6pcm).out.pos -= (*runtime).buffer_size;
        }

        change_volume(urb_out, &(*line6pcm).volume_playback, bytes_per_frame);
    } else {
        memset(
            (*urb_out).transfer_buffer,
            0,
            (*urb_out).transfer_buffer_length,
        );
    }

    spin_lock_nested(
        &mut (*line6pcm).in_.lock as *mut _ as *mut c_void,
        SINGLE_DEPTH_NESTING,
    );
    if !(*line6pcm).prev_fbuf.is_null() {
        if test_bit(LINE6_STREAM_IMPULSE, &(*line6pcm).out.running as *const _ as *const c_void)
            != 0
        {
            create_impulse_test_signal(line6pcm, urb_out, bytes_per_frame);
            if test_bit(
                LINE6_STREAM_PCM,
                &(*line6pcm).in_.running as *const _ as *const c_void,
            ) != 0
            {
                line6_capture_copy(
                    line6pcm,
                    (*urb_out).transfer_buffer as *mut u8,
                    (*urb_out).transfer_buffer_length,
                );
                line6_capture_check_period(line6pcm, (*urb_out).transfer_buffer_length);
            }
        } else {
            if ((*(*(*line6pcm).line6).properties).capabilities & LINE6_CAP_HWMON) == 0
                && ((*line6pcm).out.running != 0) as c_int != 0
                && ((*line6pcm).in_.running != 0) as c_int != 0
            {
                add_monitor_signal(
                    urb_out,
                    (*line6pcm).prev_fbuf as *mut u8,
                    (*line6pcm).volume_monitor,
                    bytes_per_frame,
                );
            }
        }
        (*line6pcm).prev_fbuf = core::ptr::null_mut();
        (*line6pcm).prev_fsize = 0;
    }
    spin_unlock(&mut (*line6pcm).in_.lock as *mut _ as *mut c_void);

    ret = usb_submit_urb(urb_out, GFP_ATOMIC);

    if ret == 0 {
        set_bit(index, &mut (*line6pcm).out.active_urbs as *mut _ as *mut c_void);
    } else {
        dev_err(
            (*(*line6pcm).line6).ifcdev,
            "URB out #%d submission failed (%d)\n" as *const _ as *const c_int,
            index,
            ret,
        );
    }

    0
}

/*
    Submit all currently available playback URBs.
    must be called in line6pcm->out.lock context
 */
pub unsafe fn line6_submit_audio_out_all_urbs(line6pcm: *mut snd_line6_pcm) -> c_int {
    let mut ret: c_int = 0;
    let mut i: c_int = 0;

    while i < (*(*line6pcm).line6).iso_buffers {
        ret = submit_audio_out_urb(line6pcm);
        if ret < 0 {
            break;
        }
        i += 1;
    }

    ret
}

/*
    Callback for completed playback URB.
*/
extern "C" fn audio_out_callback(urb: *mut urb) {
    let mut i: c_int;
    let index: c_int;
    let mut length: c_int = 0;
    let mut shutdown: c_int = 0;
    let flags: core::ffi::c_ulong;
    let line6pcm: *mut snd_line6_pcm = unsafe { (*urb).context as *mut snd_line6_pcm };
    let substream: *mut snd_pcm_substream = unsafe { get_substream(line6pcm, SNDRV_PCM_STREAM_PLAYBACK) };
    let bytes_per_frame: c_int =
        unsafe { ((*(*line6pcm).properties).bytes_per_channel * (*(*line6pcm).properties).playback_hw.channels_max) as c_int };

    if USE_CLEAR_BUFFER_WORKAROUND {
        unsafe {
            memset(
                (*urb).transfer_buffer,
                0,
                (*urb).transfer_buffer_length,
            );
        }
    }

    unsafe {
        (*line6pcm).out.last_frame = (*urb).start_frame;

        /* find index of URB */
        let mut idx: c_int = 0;
        while idx < (*(*line6pcm).line6).iso_buffers {
            if urb == *(*line6pcm).out.urbs.offset(idx as isize) {
                index = idx;
                break;
            }
            idx += 1;
        }

        if idx >= (*(*line6pcm).line6).iso_buffers {
            return; /* URB has been unlinked asynchronously */
        }

        i = 0;
        while i < LINE6_ISO_PACKETS as c_int {
            length += (*urb).iso_frame_desc[i as usize].length as c_int;
            i += 1;
        }

        spin_lock_irqsave(&mut (*line6pcm).out.lock as *mut _ as *mut c_void, &mut flags as *mut core::ffi::c_ulong);

        if test_bit(LINE6_STREAM_PCM, &(*line6pcm).out.running as *const _ as *const c_void) != 0
        {
            let runtime: *mut snd_pcm_runtime = (*substream).runtime;

            (*line6pcm).out.pos_done += (length / bytes_per_frame) as core::ffi::c_ulong;

            if (*line6pcm).out.pos_done >= (*runtime).buffer_size {
                (*line6pcm).out.pos_done -= (*runtime).buffer_size;
            }
        }

        clear_bit(index as c_uint, &mut (*line6pcm).out.active_urbs as *mut _ as *mut c_void);

        i = 0;
        while i < LINE6_ISO_PACKETS as c_int {
            if (*urb).iso_frame_desc[i as usize].status == EXDEV {
                shutdown = 1;
                break;
            }
            i += 1;
        }

        if test_and_clear_bit(index as c_uint, &mut (*line6pcm).out.unlink_urbs as *mut _ as *mut c_void)
            != 0
        {
            shutdown = 1;
        }

        if shutdown == 0 {
            submit_audio_out_urb(line6pcm);

            if test_bit(LINE6_STREAM_PCM, &(*line6pcm).out.running as *const _ as *const c_void)
                != 0
            {
                (*line6pcm).out.bytes += length as core::ffi::c_ulong;
                if (*line6pcm).out.bytes >= (*line6pcm).out.period {
                    (*line6pcm).out.bytes %= (*line6pcm).out.period;
                    spin_unlock(&mut (*line6pcm).out.lock as *mut _ as *mut c_void);
                    snd_pcm_period_elapsed(substream);
                    spin_lock_irqsave(&mut (*line6pcm).out.lock as *mut _ as *mut c_void, &mut flags as *mut core::ffi::c_ulong);
                }
            }
        }
        spin_unlock_irqrestore(&mut (*line6pcm).out.lock as *mut _ as *mut c_void, flags);
    }
}

/* open playback callback */
extern "C" fn snd_line6_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let err: c_int;
    let runtime: *mut snd_pcm_runtime = unsafe { (*substream).runtime };
    let line6pcm: *mut snd_line6_pcm = unsafe { snd_pcm_substream_chip(substream) };

    unsafe {
        err = snd_pcm_hw_constraint_ratdens(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            &(*(*line6pcm).properties).rates as *const _ as *const c_void,
        );
        if err < 0 {
            return err;
        }

        (*runtime).hw = (*(*line6pcm).properties).playback_hw;
    }
    0
}

/* close playback callback */
extern "C" fn snd_line6_playback_close(_substream: *mut snd_pcm_substream) -> c_int {
    0
}

/* playback operators */
#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<extern "C" fn(*mut snd_pcm_substream, *mut c_void) -> c_int>,
    pub hw_free: Option<extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<extern "C" fn(*mut snd_pcm_substream) -> core::ffi::c_ulong>,
}

pub const SND_LINE6_PLAYBACK_OPS: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_line6_playback_open),
    close: Some(snd_line6_playback_close),
    hw_params: Some(snd_line6_hw_params),
    hw_free: Some(snd_line6_hw_free),
    prepare: Some(snd_line6_prepare),
    trigger: Some(snd_line6_trigger),
    pointer: Some(snd_line6_pointer),
};

pub unsafe fn line6_create_audio_out_urbs(line6pcm: *mut snd_line6_pcm) -> c_int {
    let line6: *mut usb_line6 = (*line6pcm).line6;
    let mut i: c_int;

    (*line6pcm).out.urbs = kzalloc_objs(
        core::mem::size_of::<*mut urb>(),
        (*line6).iso_buffers,
    ) as *mut *mut urb;
    if (*line6pcm).out.urbs.is_null() {
        return -ENOMEM;
    }

    /* create audio URBs and fill in constant values: */
    i = 0;
    while i < (*line6).iso_buffers {
        let urb: *mut urb;

        /* URB for audio out: */
        urb = usb_alloc_urb(LINE6_ISO_PACKETS as c_int, GFP_KERNEL);
        *(*line6pcm).out.urbs.offset(i as isize) = urb;

        if urb.is_null() {
            return -ENOMEM;
        }

        (*urb).dev = (*line6).usbdev;
        (*urb).pipe = usb_sndisocpipe(
            (*line6).usbdev as *mut c_void,
            ((*(*line6).properties).ep_audio_w & USB_ENDPOINT_NUMBER_MASK) as c_uint,
        );
        (*urb).transfer_flags = URB_ISO_ASAP;
        (*urb).start_frame = -1;
        (*urb).number_of_packets = LINE6_ISO_PACKETS as c_int;
        (*urb).interval = LINE6_ISO_INTERVAL;
        (*urb).error_count = 0;
        (*urb).complete = Some(audio_out_callback);
        if usb_urb_ep_type_check(urb) != 0 {
            return -EINVAL;
        }
        i += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
