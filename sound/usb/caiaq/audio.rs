// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (c) 2006-2008 Daniel Mack, Karsten Wiese
*/

// #include <linux/device.h>
// #include <linux/spinlock.h>
// #include <linux/slab.h>
// #include <linux/init.h>
// #include <linux/usb.h>
// #include <sound/core.h>
// #include <sound/pcm.h>
// #include "device.h"
// #include "audio.h"

const N_URBS: usize = 32;
const CLOCK_DRIFT_TOLERANCE: i32 = 5;
const FRAMES_PER_URB: usize = 8;
const BYTES_PER_FRAME: usize = 512;
const CHANNELS_PER_STREAM: usize = 2;
const BYTES_PER_SAMPLE: usize = 3;
const BYTES_PER_SAMPLE_USB: usize = 4;
const MAX_BUFFER_SIZE: usize = 128 * 1024;
const MAX_ENDPOINT_SIZE: usize = 512;

const ENDPOINT_CAPTURE: u32 = 2;
const ENDPOINT_PLAYBACK: u32 = 6;

fn make_checkbyte(cdev: *const SndUsbCaiaqdev, stream: u32, i: usize) -> u8 {
    ((stream << 1) | (!(i / (unsafe { (*cdev).n_streams as usize } * BYTES_PER_SAMPLE_USB)) as u32 & 1)) as u8
}

// Static const struct snd_pcm_hardware
#[repr(C)]
pub struct SndPcmHardware {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
}

static SND_USB_CAIAQ_PCM_HARDWARE: SndPcmHardware = SndPcmHardware {
    info: 0, // SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER
    formats: 0, // SNDRV_PCM_FMTBIT_S24_3BE
    rates: 0, // SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000
    rate_min: 44100,
    rate_max: 0,
    channels_min: CHANNELS_PER_STREAM as u32,
    channels_max: CHANNELS_PER_STREAM as u32,
    buffer_bytes_max: MAX_BUFFER_SIZE,
    period_bytes_min: 128,
    period_bytes_max: MAX_BUFFER_SIZE,
    periods_min: 1,
    periods_max: 1024,
};

// External types and structures
pub struct SndUsbCaiaqdev {
    // Members to be defined in device.h
}

pub struct SndPcmSubstream {
    // Members to be defined in core.h
}

pub struct UsbIsoPacketDescriptor {
    // Members to be defined in usb.h
}

pub struct Urb {
    // Members to be defined in usb.h
}

pub struct SndUsbCaiaqCbInfo {
    // Members to be defined in device.h
}

pub struct Device {
    // Members to be defined in device.h
}

// External function declarations
extern "C" {
    fn caiaqdev_to_dev(cdev: *const SndUsbCaiaqdev) -> *const Device;
    fn dev_dbg(dev: *const Device, format: *const i8, ...);
    fn dev_err(dev: *const Device, format: *const i8, ...);
    fn dev_warn(dev: *const Device, format: *const i8, ...);
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn usb_submit_urb(urb: *mut Urb, mem_flags: u32) -> i32;
    fn usb_kill_urb(urb: *mut Urb);
    fn kmalloc_objs(size: usize) -> *mut core::ffi::c_void;
    fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn usb_alloc_urb(iso_packets: i32, mem_flags: u32) -> *mut Urb;
    fn usb_free_urb(urb: *mut Urb);
    fn usb_sndisocpipe(dev: *mut core::ffi::c_void, ep: u32) -> u32;
    fn usb_rcvisocpipe(dev: *mut core::ffi::c_void, ep: u32) -> u32;
    fn snd_pcm_new(card: *mut core::ffi::c_void, name: *const i8, device: i32,
                   playback_count: i32, capture_count: i32, rpcm: *mut *mut core::ffi::c_void) -> i32;
    fn snd_pcm_set_ops(pcm: *mut core::ffi::c_void, direction: i32, ops: *const SndPcmOps);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut core::ffi::c_void, dtype: i32,
                                      dev: *mut core::ffi::c_void, prealloc: usize, max: usize);
    fn snd_pcm_limit_hw_rates(runtime: *mut core::ffi::c_void);
    fn snd_pcm_rate_to_rate_bit(rate: u32) -> u32;
    fn frames_to_bytes(runtime: *const core::ffi::c_void, frames: u32) -> usize;
    fn bytes_to_frames(runtime: *const core::ffi::c_void, bytes: usize) -> u32;
    fn snd_pcm_lib_period_bytes(substream: *const SndPcmSubstream) -> usize;
    fn snd_pcm_period_elapsed(substream: *mut SndPcmSubstream);
    fn wait_event_timeout(wq: *mut core::ffi::c_void, condition: i32, timeout: u32) -> u32;
    fn wake_up(wq: *mut core::ffi::c_void);
    fn snd_usb_caiaq_set_audio_params(cdev: *mut SndUsbCaiaqdev, rate: u32, sample_bits: u32, bpp: u32) -> i32;
    fn snd_pcm_substream_chip(substream: *const SndPcmSubstream) -> *mut SndUsbCaiaqdev;
    fn test_bit(nr: usize, addr: *const core::ffi::c_void) -> i32;
    fn test_and_set_bit(nr: usize, addr: *mut core::ffi::c_void) -> i32;
    fn clear_bit(nr: usize, addr: *mut core::ffi::c_void);
    fn strscpy(dest: *mut i8, src: *const i8, sz: usize) -> isize;
}

#[repr(C)]
pub struct SndPcmOps {
    open: *const extern "C" fn(*mut SndPcmSubstream) -> i32,
    close: *const extern "C" fn(*mut SndPcmSubstream) -> i32,
    hw_free: *const extern "C" fn(*mut SndPcmSubstream) -> i32,
    prepare: *const extern "C" fn(*mut SndPcmSubstream) -> i32,
    trigger: *const extern "C" fn(*mut SndPcmSubstream, i32) -> i32,
    pointer: *const extern "C" fn(*mut SndPcmSubstream) -> u32,
}

const MAX_STREAMS: usize = 128; // Define based on actual value from device.h

unsafe fn activate_substream(cdev: *mut SndUsbCaiaqdev, sub: *mut SndPcmSubstream) {
    // guard(spinlock)(&cdev->spinlock);
    let sub_ref = &*sub;

    if sub_ref.stream == 0 { // SNDRV_PCM_STREAM_PLAYBACK
        // (*cdev).sub_playback[sub_ref.number] = sub;
    } else {
        // (*cdev).sub_capture[sub_ref.number] = sub;
    }
}

unsafe fn deactivate_substream(cdev: *mut SndUsbCaiaqdev, sub: *mut SndPcmSubstream) {
    // guard(spinlock_irqsave)(&cdev->spinlock);
    let sub_ref = &*sub;

    if sub_ref.stream == 0 { // SNDRV_PCM_STREAM_PLAYBACK
        // (*cdev).sub_playback[sub_ref.number] = core::ptr::null_mut();
    } else {
        // (*cdev).sub_capture[sub_ref.number] = core::ptr::null_mut();
    }
}

unsafe fn all_substreams_zero(subs: *const *mut SndPcmSubstream) -> i32 {
    for i in 0..MAX_STREAMS {
        if !(*subs.add(i)).is_null() {
            return 0;
        }
    }
    1
}

unsafe fn stream_start(cdev: *mut SndUsbCaiaqdev) -> i32 {
    let cdev_ref = &mut *cdev;
    let dev = caiaqdev_to_dev(cdev);

    dev_dbg(dev, b"%s(%p)\n\0".as_ptr() as *const i8, cdev);

    if cdev_ref.streaming != 0 {
        return -22; // -EINVAL
    }

    memset(&mut cdev_ref.sub_playback as *mut _ as *mut core::ffi::c_void, 0,
           std::mem::size_of_val(&cdev_ref.sub_playback));
    memset(&mut cdev_ref.sub_capture as *mut _ as *mut core::ffi::c_void, 0,
           std::mem::size_of_val(&cdev_ref.sub_capture));
    cdev_ref.input_panic = 0;
    cdev_ref.output_panic = 0;
    cdev_ref.first_packet = 4;
    cdev_ref.streaming = 1;
    cdev_ref.warned = 0;

    for i in 0..N_URBS {
        let ret = usb_submit_urb(cdev_ref.data_urbs_in[i], 0x20); // GFP_ATOMIC
        if ret != 0 {
            dev_err(dev, b"unable to trigger read #%d! (ret %d)\n\0".as_ptr() as *const i8,
                   i, ret);
            cdev_ref.streaming = 0;
            return -32; // -EPIPE
        }
    }

    0
}

unsafe fn stream_stop(cdev: *mut SndUsbCaiaqdev) {
    let cdev_ref = &mut *cdev;
    let dev = caiaqdev_to_dev(cdev);

    dev_dbg(dev, b"%s(%p)\n\0".as_ptr() as *const i8, cdev);
    if cdev_ref.streaming == 0 {
        return;
    }

    cdev_ref.streaming = 0;

    for i in 0..N_URBS {
        usb_kill_urb(cdev_ref.data_urbs_in[i]);

        if test_bit(i, &cdev_ref.outurb_active_mask as *const _ as *const core::ffi::c_void) != 0 {
            usb_kill_urb(cdev_ref.data_urbs_out[i]);
        }
    }

    cdev_ref.outurb_active_mask = 0;
}

unsafe extern "C" fn snd_usb_caiaq_substream_open(substream: *mut SndPcmSubstream) -> i32 {
    let cdev = snd_pcm_substream_chip(substream);
    let dev = caiaqdev_to_dev(cdev);

    dev_dbg(dev, b"%s(%p)\n\0".as_ptr() as *const i8, substream);
    // (*substream).runtime.hw = (*cdev).pcm_info;
    snd_pcm_limit_hw_rates((*substream).runtime as *mut core::ffi::c_void);

    0
}

unsafe extern "C" fn snd_usb_caiaq_substream_close(substream: *mut SndPcmSubstream) -> i32 {
    let cdev = snd_pcm_substream_chip(substream);
    let dev = caiaqdev_to_dev(cdev);

    dev_dbg(dev, b"%s(%p)\n\0".as_ptr() as *const i8, substream);
    if all_substreams_zero((*cdev).sub_playback.as_ptr()) != 0 &&
       all_substreams_zero((*cdev).sub_capture.as_ptr()) != 0 {
        stream_stop(cdev);
        (*cdev).pcm_info.rates = (*cdev).samplerates;
    }

    0
}

unsafe extern "C" fn snd_usb_caiaq_pcm_hw_free(sub: *mut SndPcmSubstream) -> i32 {
    let cdev = snd_pcm_substream_chip(sub);
    deactivate_substream(cdev, sub);
    0
}

unsafe extern "C" fn snd_usb_caiaq_pcm_prepare(substream: *mut SndPcmSubstream) -> i32 {
    let mut bytes_per_sample: i32;
    let mut bpp: i32;
    let mut ret: i32;
    let index = (*substream).number as usize;
    let cdev = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let dev = caiaqdev_to_dev(cdev);

    dev_dbg(dev, b"%s(%p)\n\0".as_ptr() as *const i8, substream);

    if (*substream).stream == 0 { // SNDRV_PCM_STREAM_PLAYBACK
        let mut out_pos: i32;

        match (*cdev).spec.data_alignment {
            0 | 2 => {
                out_pos = (BYTES_PER_SAMPLE + 1) as i32;
            }
            3 | _ => {
                out_pos = 0;
            }
        }

        (*cdev).period_out_count[index] = out_pos;
        (*cdev).audio_out_buf_pos[index] = out_pos;
    } else {
        let mut in_pos: i32;

        match (*cdev).spec.data_alignment {
            0 => {
                in_pos = (BYTES_PER_SAMPLE + 2) as i32;
            }
            2 => {
                in_pos = BYTES_PER_SAMPLE as i32;
            }
            3 | _ => {
                in_pos = 0;
            }
        }

        (*cdev).period_in_count[index] = in_pos;
        (*cdev).audio_in_buf_pos[index] = in_pos;
    }

    if (*cdev).streaming != 0 {
        return 0;
    }

    (*cdev).pcm_info.rates = snd_pcm_rate_to_rate_bit((*runtime).rate);
    snd_pcm_limit_hw_rates(runtime as *mut core::ffi::c_void);

    bytes_per_sample = BYTES_PER_SAMPLE as i32;
    if (*cdev).spec.data_alignment >= 2 {
        bytes_per_sample += 1;
    }

    bpp = ((((*runtime).rate / 8000) as i32 + CLOCK_DRIFT_TOLERANCE)
        * bytes_per_sample * (CHANNELS_PER_STREAM as i32) * ((*cdev).n_streams as i32)) as i32;

    if bpp > (MAX_ENDPOINT_SIZE as i32) {
        bpp = MAX_ENDPOINT_SIZE as i32;
    }

    ret = snd_usb_caiaq_set_audio_params(cdev, (*runtime).rate,
                                         (*runtime).sample_bits, bpp as u32);
    if ret != 0 {
        return ret;
    }

    ret = stream_start(cdev);
    if ret != 0 {
        return ret;
    }

    (*cdev).output_running = 0;
    wait_event_timeout((*cdev).prepare_wait_queue as *mut core::ffi::c_void,
                      (*cdev).output_running, 100); // HZ
    if (*cdev).output_running == 0 {
        stream_stop(cdev);
        return -32; // -EPIPE
    }

    0
}

unsafe extern "C" fn snd_usb_caiaq_pcm_trigger(sub: *mut SndPcmSubstream, cmd: i32) -> i32 {
    let cdev = snd_pcm_substream_chip(sub);
    let dev = caiaqdev_to_dev(cdev);

    dev_dbg(dev, b"%s(%p) cmd %d\n\0".as_ptr() as *const i8, sub, cmd);

    match cmd {
        0 | 1 => { // SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE
            activate_substream(cdev, sub);
        }
        2 | 3 => { // SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH
            deactivate_substream(cdev, sub);
        }
        _ => {
            return -22; // -EINVAL
        }
    }

    0
}

unsafe extern "C" fn snd_usb_caiaq_pcm_pointer(sub: *mut SndPcmSubstream) -> u32 {
    let index = (*sub).number as usize;
    let cdev = snd_pcm_substream_chip(sub);

    // guard(spinlock)(&cdev->spinlock);

    if (*cdev).input_panic != 0 || (*cdev).output_panic != 0 {
        return 0xffffffff; // SNDRV_PCM_POS_XRUN
    }

    if (*sub).stream == 0 { // SNDRV_PCM_STREAM_PLAYBACK
        bytes_to_frames((*sub).runtime as *const core::ffi::c_void,
                       (*cdev).audio_out_buf_pos[index] as usize)
    } else {
        bytes_to_frames((*sub).runtime as *const core::ffi::c_void,
                       (*cdev).audio_in_buf_pos[index] as usize)
    }
}

static SND_USB_CAIAQ_OPS: SndPcmOps = SndPcmOps {
    open: snd_usb_caiaq_substream_open as *const _,
    close: snd_usb_caiaq_substream_close as *const _,
    hw_free: snd_usb_caiaq_pcm_hw_free as *const _,
    prepare: snd_usb_caiaq_pcm_prepare as *const _,
    trigger: snd_usb_caiaq_pcm_trigger as *const _,
    pointer: snd_usb_caiaq_pcm_pointer as *const _,
};

unsafe fn check_for_elapsed_periods(cdev: *mut SndUsbCaiaqdev,
                                     subs: *mut *mut SndPcmSubstream) {
    for stream in 0..(*cdev).n_streams as usize {
        let sub = *subs.add(stream);
        if sub.is_null() {
            continue;
        }

        let pb = snd_pcm_lib_period_bytes(sub);
        let cnt = if (*sub).stream == 0 { // SNDRV_PCM_STREAM_PLAYBACK
            &mut (*cdev).period_out_count[stream]
        } else {
            &mut (*cdev).period_in_count[stream]
        };

        if *cnt >= (pb as i32) {
            snd_pcm_period_elapsed(sub);
            *cnt %= pb as i32;
        }
    }
}

unsafe fn read_in_urb_mode0(cdev: *mut SndUsbCaiaqdev,
                            urb: *const Urb,
                            iso: *const UsbIsoPacketDescriptor) {
    let usb_buf = ((*urb).transfer_buffer as *const u8).add((*iso).offset);
    let mut i = 0;

    if all_substreams_zero((*cdev).sub_capture.as_ptr()) != 0 {
        return;
    }

    while i < (*iso).actual_length {
        for stream in 0..(*cdev).n_streams as usize {
            let sub = (*cdev).sub_capture[stream];
            if !sub.is_null() {
                let rt = (*sub).runtime;
                let audio_buf = (*rt).dma_area as *mut u8;
                let sz = frames_to_bytes(rt as *const core::ffi::c_void, (*rt).buffer_size);
                *audio_buf.add((*cdev).audio_in_buf_pos[stream] as usize) = *usb_buf.add(i);
                (*cdev).audio_in_buf_pos[stream] += 1;
                (*cdev).period_in_count[stream] += 1;
                if (*cdev).audio_in_buf_pos[stream] as usize == sz {
                    (*cdev).audio_in_buf_pos[stream] = 0;
                }
            }
            i += 1;
        }
    }
}

unsafe fn read_in_urb_mode2(cdev: *mut SndUsbCaiaqdev,
                            urb: *const Urb,
                            iso: *const UsbIsoPacketDescriptor) {
    let usb_buf = ((*urb).transfer_buffer as *const u8).add((*iso).offset);
    let mut i = 0;

    while i < (*iso).actual_length {
        if i % ((*cdev).n_streams as usize * BYTES_PER_SAMPLE_USB) == 0 {
            for stream in 0..(*cdev).n_streams as usize {
                if (*cdev).first_packet == 0 {
                    let check_byte = make_checkbyte(cdev, stream as u32, i);

                    if (*usb_buf.add(i) & 0x3f) != check_byte {
                        (*cdev).input_panic = 1;
                    }

                    if (*usb_buf.add(i) & 0x80) != 0 {
                        (*cdev).output_panic = 1;
                    }
                }
                i += 1;
            }
        }
        (*cdev).first_packet = 0;

        for stream in 0..(*cdev).n_streams as usize {
            let sub = (*cdev).sub_capture[stream];
            if (*cdev).input_panic != 0 {
                *(usb_buf as *mut u8).add(i) = 0;
            }

            if !sub.is_null() {
                let rt = (*sub).runtime;
                let audio_buf = (*rt).dma_area as *mut u8;
                let sz = frames_to_bytes(rt as *const core::ffi::c_void, (*rt).buffer_size);
                *audio_buf.add((*cdev).audio_in_buf_pos[stream] as usize) = *usb_buf.add(i);
                (*cdev).audio_in_buf_pos[stream] += 1;
                (*cdev).period_in_count[stream] += 1;
                if (*cdev).audio_in_buf_pos[stream] as usize == sz {
                    (*cdev).audio_in_buf_pos[stream] = 0;
                }
            }
            i += 1;
        }
    }
}

unsafe fn read_in_urb_mode3(cdev: *mut SndUsbCaiaqdev,
                            urb: *const Urb,
                            iso: *const UsbIsoPacketDescriptor) {
    let usb_buf = ((*urb).transfer_buffer as *const u8).add((*iso).offset);
    let dev = caiaqdev_to_dev(cdev);
    let mut i = 0;

    if (*iso).actual_length % (BYTES_PER_SAMPLE_USB * CHANNELS_PER_STREAM) != 0 {
        return;
    }

    while i < (*iso).actual_length {
        for stream in 0..(*cdev).n_streams as usize {
            let sub = (*cdev).sub_capture[stream];
            let mut audio_buf: *mut u8 = core::ptr::null_mut();
            let mut sz: usize = 0;

            if !sub.is_null() && (*cdev).input_panic == 0 {
                let rt = (*sub).runtime;
                audio_buf = (*rt).dma_area as *mut u8;
                sz = frames_to_bytes(rt as *const core::ffi::c_void, (*rt).buffer_size);
            }

            for c in 0..CHANNELS_PER_STREAM {
                if !audio_buf.is_null() {
                    for n in 0..BYTES_PER_SAMPLE {
                        *audio_buf.add((*cdev).audio_in_buf_pos[stream] as usize) = *usb_buf.add(i + n);
                        (*cdev).audio_in_buf_pos[stream] += 1;

                        if (*cdev).audio_in_buf_pos[stream] as usize == sz {
                            (*cdev).audio_in_buf_pos[stream] = 0;
                        }
                    }
                    (*cdev).period_in_count[stream] += BYTES_PER_SAMPLE as i32;
                }

                i += BYTES_PER_SAMPLE;

                if (*usb_buf.add(i) != (((stream as u32) << 1) | c as u32) as u8) &&
                   (*cdev).first_packet == 0 {
                    if (*cdev).input_panic == 0 {
                        dev_warn(dev, b" EXPECTED: %02x got %02x, c %d, stream %d, i %d\n\0".as_ptr() as *const i8,
                                (((stream as u32) << 1) | c as u32) as u8, *usb_buf.add(i), c, stream, i);
                    }
                    (*cdev).input_panic = 1;
                }

                i += 1;
            }
        }
    }

    if (*cdev).first_packet > 0 {
        (*cdev).first_packet -= 1;
    }
}

unsafe fn read_in_urb(cdev: *mut SndUsbCaiaqdev,
                      urb: *const Urb,
                      iso: *const UsbIsoPacketDescriptor) {
    let dev = caiaqdev_to_dev(cdev);

    if (*cdev).streaming == 0 {
        return;
    }

    if (*iso).actual_length < (*cdev).bpp {
        return;
    }

    match (*cdev).spec.data_alignment {
        0 => {
            read_in_urb_mode0(cdev, urb, iso);
        }
        2 => {
            read_in_urb_mode2(cdev, urb, iso);
        }
        3 => {
            read_in_urb_mode3(cdev, urb, iso);
        }
        _ => {}
    }

    if ((*cdev).input_panic != 0 || (*cdev).output_panic != 0) && (*cdev).warned == 0 {
        dev_warn(dev, b"streaming error detected %s %s\n\0".as_ptr() as *const i8,
                if (*cdev).input_panic != 0 { "(input)\0" } else { "\0" }.as_ptr() as *const i8,
                if (*cdev).output_panic != 0 { "(output)\0" } else { "\0" }.as_ptr() as *const i8);
        (*cdev).warned = 1;
    }
}

unsafe fn fill_out_urb_mode_0(cdev: *mut SndUsbCaiaqdev,
                              urb: *mut Urb,
                              iso: *const UsbIsoPacketDescriptor) {
    let usb_buf = ((*urb).transfer_buffer as *mut u8).add((*iso).offset);
    let mut i = 0;

    while i < (*iso).length {
        for stream in 0..(*cdev).n_streams as usize {
            let sub = (*cdev).sub_playback[stream];
            if !sub.is_null() {
                let rt = (*sub).runtime;
                let audio_buf = (*rt).dma_area as *const u8;
                let sz = frames_to_bytes(rt as *const core::ffi::c_void, (*rt).buffer_size);
                *usb_buf.add(i) = *audio_buf.add((*cdev).audio_out_buf_pos[stream] as usize);
                (*cdev).period_out_count[stream] += 1;
                (*cdev).audio_out_buf_pos[stream] += 1;
                if (*cdev).audio_out_buf_pos[stream] as usize == sz {
                    (*cdev).audio_out_buf_pos[stream] = 0;
                }
            } else {
                *usb_buf.add(i) = 0;
            }
            i += 1;
        }

        if (*cdev).spec.data_alignment == 2 &&
           i % ((*cdev).n_streams as usize * BYTES_PER_SAMPLE_USB) ==
               ((*cdev).n_streams as usize * CHANNELS_PER_STREAM) {
            for stream in 0..(*cdev).n_streams as usize {
                *usb_buf.add(i) = make_checkbyte(cdev, stream as u32, i);
                i += 1;
            }
        }
    }
}

unsafe fn fill_out_urb_mode_3(cdev: *mut SndUsbCaiaqdev,
                              urb: *mut Urb,
                              iso: *const UsbIsoPacketDescriptor) {
    let usb_buf = ((*urb).transfer_buffer as *mut u8).add((*iso).offset);
    let mut i = 0;

    while i < (*iso).length {
        for stream in 0..(*cdev).n_streams as usize {
            let sub = (*cdev).sub_playback[stream];
            let mut audio_buf: *const u8 = core::ptr::null();
            let mut sz: usize = 0;

            if !sub.is_null() {
                let rt = (*sub).runtime;
                audio_buf = (*rt).dma_area as *const u8;
                sz = frames_to_bytes(rt as *const core::ffi::c_void, (*rt).buffer_size);
            }

            for c in 0..CHANNELS_PER_STREAM {
                for n in 0..BYTES_PER_SAMPLE {
                    if !audio_buf.is_null() {
                        *usb_buf.add(i + n) = *audio_buf.add((*cdev).audio_out_buf_pos[stream] as usize);
                        (*cdev).audio_out_buf_pos[stream] += 1;

                        if (*cdev).audio_out_buf_pos[stream] as usize == sz {
                            (*cdev).audio_out_buf_pos[stream] = 0;
                        }
                    } else {
                        *usb_buf.add(i + n) = 0;
                    }
                }

                if !audio_buf.is_null() {
                    (*cdev).period_out_count[stream] += BYTES_PER_SAMPLE as i32;
                }

                i += BYTES_PER_SAMPLE;

                *usb_buf.add(i) = (((stream as u32) << 1) | c as u32) as u8;
                i += 1;
            }
        }
    }
}

unsafe fn fill_out_urb(cdev: *mut SndUsbCaiaqdev,
                       urb: *mut Urb,
                       iso: *const UsbIsoPacketDescriptor) {
    match (*cdev).spec.data_alignment {
        0 | 2 => {
            fill_out_urb_mode_0(cdev, urb, iso);
        }
        3 => {
            fill_out_urb_mode_3(cdev, urb, iso);
        }
        _ => {}
    }
}

unsafe extern "C" fn read_completed(urb: *mut Urb) {
    let info = (*urb).context as *const SndUsbCaiaqCbInfo;
    let cdev: *mut SndUsbCaiaqdev;
    let dev: *const Device;
    let mut out: *mut Urb = core::ptr::null_mut();
    let mut i: usize;
    let mut frame: usize;
    let mut len: usize;
    let mut send_it: i32 = 0;
    let mut outframe: usize = 0;
    let mut offset: usize = 0;

    if (*urb).status != 0 || info.is_null() {
        return;
    }

    cdev = (*info).cdev;
    dev = caiaqdev_to_dev(cdev);

    if (*cdev).streaming == 0 {
        return;
    }

    for i in 0..N_URBS {
        if test_and_set_bit(i, &(*cdev).outurb_active_mask as *const _ as *mut core::ffi::c_void) == 0 {
            out = (*cdev).data_urbs_out[i];
            break;
        }
    }

    if out.is_null() {
        dev_err(dev, b"Unable to find an output urb to use\n\0".as_ptr() as *const i8);
        goto_requeue(urb);
        return;
    }

    for frame in 0..FRAMES_PER_URB {
        if (*(*urb).iso_frame_desc.add(frame)).status != 0 {
            continue;
        }

        len = (*(*urb).iso_frame_desc.add(outframe)).actual_length;
        (*(*out).iso_frame_desc.add(outframe)).length = len;
        (*(*out).iso_frame_desc.add(outframe)).actual_length = 0;
        (*(*out).iso_frame_desc.add(outframe)).offset = offset;
        offset += len;

        if len > 0 {
            fill_out_urb(cdev, out, &*(*out).iso_frame_desc.add(outframe));
            read_in_urb(cdev, urb, &*(*urb).iso_frame_desc.add(frame));
            check_for_elapsed_periods(cdev, (*cdev).sub_playback.as_mut_ptr());
            check_for_elapsed_periods(cdev, (*cdev).sub_capture.as_mut_ptr());
            send_it = 1;
        }

        outframe += 1;
    }

    if send_it != 0 {
        (*out).number_of_packets = outframe;
        usb_submit_urb(out, 0x20); // GFP_ATOMIC
    } else {
        let oinfo = (*out).context as *const SndUsbCaiaqCbInfo;
        clear_bit((*oinfo).index, &(*cdev).outurb_active_mask as *const _ as *mut core::ffi::c_void);
    }

    goto_requeue(urb);
}

unsafe fn goto_requeue(urb: *mut Urb) {
    for frame in 0..FRAMES_PER_URB {
        (*(*urb).iso_frame_desc.add(frame)).offset = BYTES_PER_FRAME * frame;
        (*(*urb).iso_frame_desc.add(frame)).length = BYTES_PER_FRAME;
        (*(*urb).iso_frame_desc.add(frame)).actual_length = 0;
    }

    (*urb).number_of_packets = FRAMES_PER_URB;
    usb_submit_urb(urb, 0x20); // GFP_ATOMIC
}

unsafe extern "C" fn write_completed(urb: *mut Urb) {
    let info = (*urb).context as *const SndUsbCaiaqCbInfo;
    let cdev = (*info).cdev;

    if (*cdev).output_running == 0 {
        (*cdev).output_running = 1;
        wake_up((*cdev).prepare_wait_queue as *mut core::ffi::c_void);
    }

    clear_bit((*info).index, &(*cdev).outurb_active_mask as *const _ as *mut core::ffi::c_void);
}

unsafe fn alloc_urbs(cdev: *mut SndUsbCaiaqdev, dir: i32, ret: *mut i32) -> *mut *mut Urb {
    let mut i: usize;
    let mut frame: usize;
    let urbs: *mut *mut Urb;
    let usb_dev = (*(*cdev).chip).dev;
    let pipe: u32;

    pipe = if dir == 0 { // SNDRV_PCM_STREAM_PLAYBACK
        usb_sndisocpipe(usb_dev, ENDPOINT_PLAYBACK)
    } else {
        usb_rcvisocpipe(usb_dev, ENDPOINT_CAPTURE)
    };

    urbs = kmalloc_objs(std::mem::size_of::<*mut Urb>() * N_URBS) as *mut *mut Urb;
    if urbs.is_null() {
        *ret = -12; // -ENOMEM
        return core::ptr::null_mut();
    }

    for i in 0..N_URBS {
        *urbs.add(i) = usb_alloc_urb(FRAMES_PER_URB as i32, 0x10); // GFP_KERNEL
        if (*urbs.add(i)).is_null() {
            *ret = -12; // -ENOMEM
            return urbs;
        }

        (*(*urbs.add(i))).transfer_buffer = kmalloc_array(BYTES_PER_FRAME, FRAMES_PER_URB, 0x10);
        if (*(*urbs.add(i))).transfer_buffer.is_null() {
            *ret = -12; // -ENOMEM
            return urbs;
        }

        for frame in 0..FRAMES_PER_URB {
            let iso = &mut (*(*urbs.add(i))).iso_frame_desc[frame];
            (*iso).offset = BYTES_PER_FRAME * frame;
            (*iso).length = BYTES_PER_FRAME;
        }

        (*(*urbs.add(i))).dev = usb_dev;
        (*(*urbs.add(i))).pipe = pipe;
        (*(*urbs.add(i))).transfer_buffer_length = FRAMES_PER_URB * BYTES_PER_FRAME;
        (*(*urbs.add(i))).context = &(*cdev).data_cb_info[i] as *const _ as *mut core::ffi::c_void;
        (*(*urbs.add(i))).interval = 1;
        (*(*urbs.add(i))).number_of_packets = FRAMES_PER_URB;
        (*(*urbs.add(i))).complete = if dir == 1 { // SNDRV_PCM_STREAM_CAPTURE
            read_completed as *const _
        } else {
            write_completed as *const _
        };
    }

    *ret = 0;
    urbs
}

unsafe fn free_urbs(urbs: *mut *mut Urb) {
    if urbs.is_null() {
        return;
    }

    for i in 0..N_URBS {
        let urb = *urbs.add(i);
        if !urb.is_null() {
            usb_kill_urb(urb);
            kfree((*urb).transfer_buffer);
            usb_free_urb(urb);
        }
    }

    kfree(urbs as *mut core::ffi::c_void);
}

pub unsafe extern "C" fn snd_usb_caiaq_audio_init(cdev: *mut SndUsbCaiaqdev) -> i32 {
    let mut i: usize;
    let mut ret: i32;
    let dev = caiaqdev_to_dev(cdev);

    (*cdev).n_audio_in = ((*(*cdev).spec).num_analog_audio_in.max((*(*cdev).spec).num_digital_audio_in)) / CHANNELS_PER_STREAM as u32;
    (*cdev).n_audio_out = ((*(*cdev).spec).num_analog_audio_out.max((*(*cdev).spec).num_digital_audio_out)) / CHANNELS_PER_STREAM as u32;
    (*cdev).n_streams = (*cdev).n_audio_in.max((*cdev).n_audio_out);

    dev_dbg(dev, b"cdev->n_audio_in = %d\n\0".as_ptr() as *const i8, (*cdev).n_audio_in);
    dev_dbg(dev, b"cdev->n_audio_out = %d\n\0".as_ptr() as *const i8, (*cdev).n_audio_out);
    dev_dbg(dev, b"cdev->n_streams = %d\n\0".as_ptr() as *const i8, (*cdev).n_streams);

    if (*cdev).n_streams > (MAX_STREAMS as u32) {
        dev_err(dev, b"unable to initialize device, too many streams.\n\0".as_ptr() as *const i8);
        return -22; // -EINVAL
    }

    if (*cdev).n_streams < 1 {
        dev_err(dev, b"bogus number of streams: %d\n\0".as_ptr() as *const i8, (*cdev).n_streams);
        return -22; // -EINVAL
    }

    let pcm_ptr: *mut *mut core::ffi::c_void = &mut (*cdev).pcm as *mut _ as *mut *mut core::ffi::c_void;
    ret = snd_pcm_new((*(*cdev).chip).card, (*cdev).product_name as *const i8, 0,
                     (*cdev).n_audio_out as i32, (*cdev).n_audio_in as i32, pcm_ptr);

    if ret < 0 {
        dev_err(dev, b"snd_pcm_new() returned %d\n\0".as_ptr() as *const i8, ret);
        return ret;
    }

    let pcm = *pcm_ptr as *mut core::ffi::c_void;
    // (*pcm).private_data = cdev as *mut core::ffi::c_void;
    strscpy((*pcm as *mut i8).add(/* offset to name field */), (*cdev).product_name as *const i8, 80);

    memset(&mut (*cdev).sub_playback as *mut _ as *mut core::ffi::c_void, 0,
           std::mem::size_of_val(&(*cdev).sub_playback));
    memset(&mut (*cdev).sub_capture as *mut _ as *mut core::ffi::c_void, 0,
           std::mem::size_of_val(&(*cdev).sub_capture));

    memcpy(&mut (*cdev).pcm_info as *mut _ as *mut core::ffi::c_void,
           &SND_USB_CAIAQ_PCM_HARDWARE as *const _ as *const core::ffi::c_void,
           std::mem::size_of_val(&SND_USB_CAIAQ_PCM_HARDWARE));

    (*cdev).samplerates = (*cdev).pcm_info.rates;
    match (*(*cdev).chip).usb_id {
        0x17cc0001 | 0x17cc0003 | 0x17cc0006 | 0x17cc0008 => {
            (*cdev).samplerates |= 0x00100000; // SNDRV_PCM_RATE_192000
            match (*(*cdev).chip).usb_id {
                0x17cc0009 | 0x17cc000a | 0x17cc000b | 0x17cc0015 => {
                    (*cdev).samplerates |= 0x00010000; // SNDRV_PCM_RATE_88200
                }
                _ => {}
            }
        }
        _ => {}
    }

    snd_pcm_set_ops(pcm, 0, &SND_USB_CAIAQ_OPS); // SNDRV_PCM_STREAM_PLAYBACK
    snd_pcm_set_ops(pcm, 1, &SND_USB_CAIAQ_OPS); // SNDRV_PCM_STREAM_CAPTURE
    snd_pcm_set_managed_buffer_all(pcm, 2, core::ptr::null_mut(), 0, 0); // SNDRV_DMA_TYPE_VMALLOC

    (*cdev).data_cb_info = kmalloc_objs(std::mem::size_of::<SndUsbCaiaqCbInfo>() * N_URBS) as *mut SndUsbCaiaqCbInfo;

    if (*cdev).data_cb_info.is_null() {
        return -12; // -ENOMEM
    }

    (*cdev).outurb_active_mask = 0;
    // BUILD_BUG_ON check not needed in Rust

    for i in 0..N_URBS {
        (*(*cdev).data_cb_info.add(i)).cdev = cdev;
        (*(*cdev).data_cb_info.add(i)).index = i;
    }

    ret = 0;
    (*cdev).data_urbs_in = alloc_urbs(cdev, 1, &mut ret); // SNDRV_PCM_STREAM_CAPTURE
    if ret < 0 {
        kfree((*cdev).data_cb_info as *mut core::ffi::c_void);
        free_urbs((*cdev).data_urbs_in);
        return ret;
    }

    (*cdev).data_urbs_out = alloc_urbs(cdev, 0, &mut ret); // SNDRV_PCM_STREAM_PLAYBACK
    if ret < 0 {
        kfree((*cdev).data_cb_info as *mut core::ffi::c_void);
        free_urbs((*cdev).data_urbs_in);
        free_urbs((*cdev).data_urbs_out);
        return ret;
    }

    0
}

pub unsafe extern "C" fn snd_usb_caiaq_audio_disconnect(cdev: *mut SndUsbCaiaqdev) {
    let dev = caiaqdev_to_dev(cdev);

    dev_dbg(dev, b"%s(%p)\n\0".as_ptr() as *const i8, cdev);
    stream_stop(cdev);
}

pub unsafe extern "C" fn snd_usb_caiaq_audio_free(cdev: *mut SndUsbCaiaqdev) {
    let dev = caiaqdev_to_dev(cdev);

    dev_dbg(dev, b"%s(%p)\n\0".as_ptr() as *const i8, cdev);
    free_urbs((*cdev).data_urbs_in);
    free_urbs((*cdev).data_urbs_out);
    kfree((*cdev).data_cb_info as *mut core::ffi::c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
