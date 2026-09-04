// SPDX-License-Identifier: GPL-2.0-only
/*
 * Line 6 Linux USB driver
 *
 * Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
 */

// Dependencies: linux/slab.h, linux/export.h, sound/core.h, sound/control.h, sound/pcm.h, sound/pcm_params.h
// Custom dependencies: capture.h, driver.h, playback.h

use core::mem;
use core::ptr::{self, null_mut};

// External kernel types and functions
extern "C" {
    type snd_kcontrol;
    type snd_ctl_elem_info;
    type snd_ctl_elem_value;
    type snd_line6_pcm;
    type line6_pcm_stream;
    type usb_line6;
    type snd_pcm;
    type snd_pcm_substream;
    type snd_pcm_hw_params;
    type line6_pcm_properties;
    type snd_kcontrol_new;
    type usb_device;

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_line6_pcm;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_line6_pcm;
    fn snd_pcm_chip(pcm: *mut snd_pcm) -> *mut snd_line6_pcm;
    fn usb_unlink_urb(urb: *mut core::ffi::c_void) -> i32;
    fn usb_kill_urb(urb: *mut core::ffi::c_void);
    fn usb_free_urb(urb: *mut core::ffi::c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn mutex_init(mutex: *mut core::ffi::c_void);
    fn spin_lock_init(lock: *mut core::ffi::c_void);
    fn set_current_state(state: i32);
    fn schedule_timeout(timeout: i32) -> i32;
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn snd_pcm_new(
        card: *mut core::ffi::c_void,
        id: *const u8,
        device: i32,
        playback_count: i32,
        capture_count: i32,
        rpcm: *mut *mut snd_pcm,
    ) -> i32;
    fn snd_pcm_set_ops(
        pcm: *mut snd_pcm,
        direction: i32,
        ops: *const core::ffi::c_void,
    );
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: i32,
        dev: *mut core::ffi::c_void,
        prealloc: usize,
        max: usize,
    );
    fn snd_ctl_add(card: *mut core::ffi::c_void, kcontrol: *mut snd_kcontrol) -> i32;
    fn snd_ctl_new1(
        kcontrolp: *const snd_kcontrol_new,
        private_data: *mut core::ffi::c_void,
    ) -> *mut snd_kcontrol;
    fn snd_pcm_group_for_each_entry(s: *mut *mut snd_pcm_substream, substream: *mut snd_pcm_substream);
    fn params_period_bytes(hw_params: *const snd_pcm_hw_params) -> i32;
    fn usb_rcvisocpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    fn usb_sndisocpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    fn usb_maxpacket(dev: *mut usb_device, pipe: u32) -> usize;
    fn strscpy(dest: *mut u8, src: *const u8) -> isize;

    fn line6_submit_audio_out_all_urbs(line6pcm: *mut snd_line6_pcm) -> i32;
    fn line6_submit_audio_in_all_urbs(line6pcm: *mut snd_line6_pcm) -> i32;
    fn line6_create_audio_out_urbs(line6pcm: *mut snd_line6_pcm) -> i32;
    fn line6_create_audio_in_urbs(line6pcm: *mut snd_line6_pcm) -> i32;

    static snd_line6_playback_ops: core::ffi::c_void;
    static snd_line6_capture_ops: core::ffi::c_void;
}

// Constants
const SNDRV_CTL_ELEM_TYPE_INTEGER: i32 = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: i32 = 2;
const SNDRV_PCM_STREAM_PLAYBACK: i32 = 0;
const SNDRV_PCM_STREAM_CAPTURE: i32 = 1;
const SNDRV_PCM_TRIGGER_START: i32 = 0;
const SNDRV_PCM_TRIGGER_STOP: i32 = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: i32 = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: i32 = 4;
const SNDRV_PCM_TRIGGER_RESUME: i32 = 5;
const SNDRV_PCM_TRIGGER_SUSPEND: i32 = 6;
const SNDRV_DMA_TYPE_CONTINUOUS: i32 = 0;
const GFP_KERNEL: u32 = 0xd0;
const HZ: i32 = 100;
const TASK_UNINTERRUPTIBLE: i32 = 2;
const EINVAL: i32 = -22;
const ENOMEM: i32 = -12;
const LINE6_ISO_PACKETS: usize = 2;
const LINE6_IMPULSE_DEFAULT_PERIOD: i32 = 0;
const LINE6_STREAM_IMPULSE: i32 = 1;
const LINE6_STREAM_PCM: i32 = 0;
const LINE6_STREAM_CAPTURE_HELPER: i32 = 2;
const LINE6_STREAM_MONITOR: i32 = 3;
const LINE6_FLAG_PREPARED: i32 = 0;
const LINE6_FLAG_PAUSE_PLAYBACK: i32 = 1;
const LINE6_CAP_PCM: i32 = 0x00000001;
const LINE6_CAP_IN_NEEDS_OUT: i32 = 0x00010000;

unsafe fn test_bit(nr: i32, addr: *const u64) -> bool {
    let word = *(addr as *const u64).add((nr as usize) / 64);
    (word & (1u64 << ((nr as u64) % 64))) != 0
}

unsafe fn test_and_set_bit(nr: i32, addr: *mut u64) -> bool {
    let word_ptr = (addr as *mut u64).add((nr as usize) / 64);
    let word = *word_ptr;
    let mask = 1u64 << ((nr as u64) % 64);
    let old_bit = (word & mask) != 0;
    *word_ptr = word | mask;
    old_bit
}

unsafe fn set_bit(nr: i32, addr: *mut u64) {
    let word_ptr = (addr as *mut u64).add((nr as usize) / 64);
    let mask = 1u64 << ((nr as u64) % 64);
    *word_ptr = *word_ptr | mask;
}

unsafe fn clear_bit(nr: i32, addr: *mut u64) {
    let word_ptr = (addr as *mut u64).add((nr as usize) / 64);
    let mask = 1u64 << ((nr as u64) % 64);
    *word_ptr = *word_ptr & !mask;
}

unsafe fn array3_size(a: usize, b: usize, c: usize) -> usize {
    a.wrapping_mul(b).wrapping_mul(c)
}

// guard(spinlock_irqsave) - simulate with closure in Rust
// This is a simplified representation since we can't fully replicate kernel locking in this context
// The actual implementation would need kernel spinlock types

// impulse response volume controls
unsafe extern "C" fn snd_line6_impulse_volume_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    let uinfo = &mut *uinfo;
    (*((uinfo as *mut u8) as *mut i32)) = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*((uinfo as *mut u8).add(4) as *mut i32)) = 1;
    (*((uinfo as *mut u8).add(8) as *mut i32)) = 0;
    (*((uinfo as *mut u8).add(12) as *mut i32)) = 255;
    0
}

unsafe extern "C" fn snd_line6_impulse_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol);
    let impulse_volume = (*(line6pcm as *const u8).add(512) as *const i32).read();
    (*((ucontrol as *mut u8).add(8) as *mut i32)) = impulse_volume;
    0
}

unsafe extern "C" fn snd_line6_impulse_volume_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol);
    let value = (*((ucontrol as *const u8).add(8) as *const i32)).read();
    let impulse_volume_ptr = (line6pcm as *mut u8).add(512) as *mut i32;
    let current_volume = (*impulse_volume_ptr).read();

    if current_volume == value {
        return 0;
    }

    *impulse_volume_ptr = value;
    if value > 0 {
        let err = line6_pcm_acquire(line6pcm, LINE6_STREAM_IMPULSE, true);
        if err < 0 {
            *impulse_volume_ptr = 0;
            return err;
        }
    } else {
        line6_pcm_release(line6pcm, LINE6_STREAM_IMPULSE);
    }
    1
}

// impulse response period controls
unsafe extern "C" fn snd_line6_impulse_period_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    let uinfo = &mut *uinfo;
    (*((uinfo as *mut u8) as *mut i32)) = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*((uinfo as *mut u8).add(4) as *mut i32)) = 1;
    (*((uinfo as *mut u8).add(8) as *mut i32)) = 0;
    (*((uinfo as *mut u8).add(12) as *mut i32)) = 2000;
    0
}

unsafe extern "C" fn snd_line6_impulse_period_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol);
    let impulse_period = (*(line6pcm as *const u8).add(516) as *const i32).read();
    (*((ucontrol as *mut u8).add(8) as *mut i32)) = impulse_period;
    0
}

unsafe extern "C" fn snd_line6_impulse_period_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol);
    let value = (*((ucontrol as *const u8).add(8) as *const i32)).read();
    let impulse_period_ptr = (line6pcm as *mut u8).add(516) as *mut i32;
    let current_period = (*impulse_period_ptr).read();

    if current_period == value {
        return 0;
    }

    *impulse_period_ptr = value;
    1
}

// Unlink all currently active URBs.
unsafe fn line6_unlink_audio_urbs(line6pcm: *mut snd_line6_pcm, pcms: *mut line6_pcm_stream) {
    let line6pcm = &mut *line6pcm;
    let pcms = &mut *pcms;
    let iso_buffers = (*(line6pcm as *const u8 as *const usb_line6)).iso_buffers as i32;

    for i in 0..iso_buffers {
        let active_urbs_ptr = (pcms as *const u8 as *const u64).add(0);
        if test_bit(i, active_urbs_ptr) {
            let unlink_urbs_ptr = (pcms as *mut u8 as *mut u64).add(1);
            if !test_and_set_bit(i, unlink_urbs_ptr) {
                let urbs_ptr = (pcms as *const u8 as *const *mut *mut core::ffi::c_void).add(2);
                let urb = (**urbs_ptr).add(i as usize).read();
                usb_unlink_urb(urb);
            }
        }
    }
}

// Wait until unlinking of all currently active URBs has been finished.
unsafe fn line6_wait_clear_audio_urbs(line6pcm: *mut snd_line6_pcm, pcms: *mut line6_pcm_stream) {
    let line6pcm = &mut *line6pcm;
    let pcms = &mut *pcms;
    let iso_buffers = (*(line6pcm as *const u8 as *const usb_line6)).iso_buffers as i32;
    let mut timeout = HZ;

    loop {
        let mut alive = 0;
        for i in 0..iso_buffers {
            let active_urbs_ptr = (pcms as *const u8 as *const u64).add(0);
            if test_bit(i, active_urbs_ptr) {
                alive += 1;
            }
        }
        if alive == 0 {
            break;
        }
        set_current_state(TASK_UNINTERRUPTIBLE);
        schedule_timeout(1);
        timeout -= 1;
        if timeout <= 0 {
            break;
        }
    }

    let mut alive = 0;
    for i in 0..iso_buffers {
        let active_urbs_ptr = (pcms as *const u8 as *const u64).add(0);
        if test_bit(i, active_urbs_ptr) {
            alive += 1;
        }
    }
    if alive != 0 {
        dev_err(
            (line6pcm as *const u8 as *const usb_line6).ifcdev as *mut core::ffi::c_void,
            b"timeout: still %d active urbs..\n\0".as_ptr(),
            alive,
        );
    }
}

unsafe fn get_stream(line6pcm: *mut snd_line6_pcm, direction: i32) -> *mut line6_pcm_stream {
    let line6pcm = &*line6pcm;
    if direction == SNDRV_PCM_STREAM_PLAYBACK {
        &line6pcm.out as *const line6_pcm_stream as *mut line6_pcm_stream
    } else {
        &line6pcm.in_ as *const line6_pcm_stream as *mut line6_pcm_stream
    }
}

// allocate a buffer if not opened yet;
// call this in line6pcm.state_mutex
unsafe fn line6_buffer_acquire(
    line6pcm: *mut snd_line6_pcm,
    pstr: *mut line6_pcm_stream,
    direction: i32,
    type_: i32,
) -> i32 {
    let line6pcm = &*line6pcm;
    let pstr = &mut *pstr;
    let pkt_size = if direction == SNDRV_PCM_STREAM_PLAYBACK {
        line6pcm.max_packet_size_out
    } else {
        line6pcm.max_packet_size_in
    };

    let opened_ptr = &pstr.opened as *const i32 as *mut i32;
    if !test_and_set_bit(type_, opened_ptr as *mut u64) && pstr.buffer.is_null() {
        let size = array3_size(line6pcm.line6.iso_buffers, LINE6_ISO_PACKETS, pkt_size);
        let buffer = kmalloc(size, GFP_KERNEL);
        if buffer.is_null() {
            return ENOMEM;
        }
        pstr.buffer = buffer;
    }
    0
}

// free a buffer if all streams are closed;
// call this in line6pcm.state_mutex
unsafe fn line6_buffer_release(line6pcm: *mut snd_line6_pcm, pstr: *mut line6_pcm_stream, type_: i32) {
    let pstr = &mut *pstr;
    clear_bit(type_, &pstr.opened as *const i32 as *mut u64);
    if pstr.opened == 0 {
        line6_wait_clear_audio_urbs(line6pcm, pstr);
        kfree(pstr.buffer);
        pstr.buffer = null_mut();
    }
}

// start a PCM stream
unsafe fn line6_stream_start(line6pcm: *mut snd_line6_pcm, direction: i32, type_: i32) -> i32 {
    let pstr = get_stream(line6pcm, direction);
    let pstr = &mut *pstr;
    let mut ret = 0;

    // guard(spinlock_irqsave) - acquire lock here
    if !test_and_set_bit(type_, &pstr.running as *const i32 as *mut u64) {
        if pstr.active_urbs == 0 && pstr.unlink_urbs == 0 {
            pstr.count = 0;
            if direction == SNDRV_PCM_STREAM_PLAYBACK {
                ret = line6_submit_audio_out_all_urbs(line6pcm);
            } else {
                ret = line6_submit_audio_in_all_urbs(line6pcm);
            }
        }
    }

    if ret < 0 {
        clear_bit(type_, &pstr.running as *const i32 as *mut u64);
    }
    // release lock here
    ret
}

// stop a PCM stream; this doesn't sync with the unlinked URBs
unsafe fn line6_stream_stop(line6pcm: *mut snd_line6_pcm, direction: i32, type_: i32) {
    let pstr = get_stream(line6pcm, direction);
    let pstr = &mut *pstr;

    // scoped_guard(spinlock_irqsave, ...) block
    {
        clear_bit(type_, &pstr.running as *const i32 as *mut u64);
        if pstr.running != 0 {
            return;
        }
    }

    line6_unlink_audio_urbs(line6pcm, pstr);
    if direction == SNDRV_PCM_STREAM_CAPTURE {
        // guard(spinlock_irqsave) block
        {
            (line6pcm as *mut u8 as *mut *const u8).add(100).write(null_mut());
            ((line6pcm as *mut u8).add(104) as *mut i32).write(0);
        }
    }
}

// common PCM trigger callback
pub unsafe extern "C" fn snd_line6_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32 {
    let line6pcm = snd_pcm_substream_chip(substream);
    let substream = &*substream;
    let mut err;

    clear_bit(LINE6_FLAG_PREPARED, &(line6pcm as *const u8 as *const u64).add(200).read());

    // snd_pcm_group_for_each_entry loop - simplified
    let mut s = substream as *const snd_pcm_substream as *mut snd_pcm_substream;
    loop {
        let s_ref = &*s;
        if (s_ref as *const u8 as *const i32).read() != (substream as *const u8 as *const i32).read() {
            // Different card, skip
            break;
        }

        match cmd {
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
                if s_ref.stream == SNDRV_PCM_STREAM_CAPTURE {
                    let capabilities = ((line6pcm as *const u8 as *const usb_line6).properties as *const i32).add(2).read();
                    if (capabilities & LINE6_CAP_IN_NEEDS_OUT) != 0 {
                        err = line6_stream_start(line6pcm, SNDRV_PCM_STREAM_PLAYBACK, LINE6_STREAM_CAPTURE_HELPER);
                        if err < 0 {
                            return err;
                        }
                    }
                }
                err = line6_stream_start(line6pcm, s_ref.stream, LINE6_STREAM_PCM);
                if err < 0 {
                    return err;
                }
            }

            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
                if s_ref.stream == SNDRV_PCM_STREAM_CAPTURE {
                    let capabilities = ((line6pcm as *const u8 as *const usb_line6).properties as *const i32).add(2).read();
                    if (capabilities & LINE6_CAP_IN_NEEDS_OUT) != 0 {
                        line6_stream_stop(line6pcm, SNDRV_PCM_STREAM_PLAYBACK, LINE6_STREAM_CAPTURE_HELPER);
                    }
                }
                line6_stream_stop(line6pcm, s_ref.stream, LINE6_STREAM_PCM);
            }

            SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                if s_ref.stream != SNDRV_PCM_STREAM_PLAYBACK {
                    return EINVAL;
                }
                set_bit(LINE6_FLAG_PAUSE_PLAYBACK, &(line6pcm as *const u8 as *const u64).add(200).read());
            }

            SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                if s_ref.stream != SNDRV_PCM_STREAM_PLAYBACK {
                    return EINVAL;
                }
                clear_bit(LINE6_FLAG_PAUSE_PLAYBACK, &(line6pcm as *const u8 as *const u64).add(200).read());
            }

            _ => {
                return EINVAL;
            }
        }
        break;
    }

    0
}

// common PCM pointer callback
pub unsafe extern "C" fn snd_line6_pointer(substream: *mut snd_pcm_substream) -> u64 {
    let line6pcm = snd_pcm_substream_chip(substream);
    let substream = &*substream;
    let pstr = get_stream(line6pcm, substream.stream);
    let pstr = &*pstr;

    pstr.pos_done as u64
}

// Stop and release duplex streams
unsafe fn __line6_pcm_release(line6pcm: *mut snd_line6_pcm, type_: i32) {
    for dir in 0..2 {
        line6_stream_stop(line6pcm, dir, type_);
    }
    for dir in 0..2 {
        let pstr = get_stream(line6pcm, dir);
        line6_buffer_release(line6pcm, pstr, type_);
    }
}

// Stop and release duplex streams
pub unsafe extern "C" fn line6_pcm_release(line6pcm: *mut snd_line6_pcm, type_: i32) {
    // guard(mutex)(&line6pcm->state_mutex)
    __line6_pcm_release(line6pcm, type_);
}

// EXPORT_SYMBOL_GPL(line6_pcm_release);

// Acquire and optionally start duplex streams:
// type is either LINE6_STREAM_IMPULSE or LINE6_STREAM_MONITOR
pub unsafe extern "C" fn line6_pcm_acquire(line6pcm: *mut snd_line6_pcm, type_: i32, start: bool) -> i32 {
    let mut ret = 0;

    // guard(mutex)(&line6pcm->state_mutex)
    // TODO: We should assert SNDRV_PCM_STREAM_PLAYBACK/CAPTURE == 0/1
    for dir in 0..2 {
        let pstr = get_stream(line6pcm, dir);
        ret = line6_buffer_acquire(line6pcm, pstr, dir, type_);
        if ret < 0 {
            goto_error(line6pcm, type_);
            return ret;
        }
        let pstr_ref = &*pstr;
        if pstr_ref.running == 0 {
            line6_wait_clear_audio_urbs(line6pcm, pstr);
        }
    }
    if start {
        for dir in 0..2 {
            ret = line6_stream_start(line6pcm, dir, type_);
            if ret < 0 {
                goto_error(line6pcm, type_);
                return ret;
            }
        }
    }

    ret
}

unsafe fn goto_error(line6pcm: *mut snd_line6_pcm, type_: i32) {
    __line6_pcm_release(line6pcm, type_);
}

// EXPORT_SYMBOL_GPL(line6_pcm_acquire);

// common PCM hw_params callback
pub unsafe extern "C" fn snd_line6_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> i32 {
    let line6pcm = snd_pcm_substream_chip(substream);
    let substream = &*substream;
    let pstr = get_stream(line6pcm, substream.stream);

    // guard(mutex)(&line6pcm->state_mutex)
    let ret = line6_buffer_acquire(line6pcm, pstr, substream.stream, LINE6_STREAM_PCM);
    if ret < 0 {
        return ret;
    }

    let pstr = &mut *pstr;
    pstr.period = params_period_bytes(hw_params);
    0
}

// common PCM hw_free callback
pub unsafe extern "C" fn snd_line6_hw_free(substream: *mut snd_pcm_substream) -> i32 {
    let line6pcm = snd_pcm_substream_chip(substream);
    let substream = &*substream;
    let pstr = get_stream(line6pcm, substream.stream);

    // guard(mutex)(&line6pcm->state_mutex)
    line6_buffer_release(line6pcm, pstr, LINE6_STREAM_PCM);
    0
}

// control info callback
unsafe extern "C" fn snd_line6_control_playback_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    let uinfo = &mut *uinfo;
    (*((uinfo as *mut u8) as *mut i32)) = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*((uinfo as *mut u8).add(4) as *mut i32)) = 2;
    (*((uinfo as *mut u8).add(8) as *mut i32)) = 0;
    (*((uinfo as *mut u8).add(12) as *mut i32)) = 256;
    0
}

// control get callback
unsafe extern "C" fn snd_line6_control_playback_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol);

    for i in 0..2 {
        let volume = (*(line6pcm as *const u8).add(300) as *const i32).add(i).read();
        (*((ucontrol as *mut u8).add(8) as *mut i32)).add(i).write(volume);
    }

    0
}

// control put callback
unsafe extern "C" fn snd_line6_control_playback_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let line6pcm = snd_kcontrol_chip(kcontrol);
    let mut changed = 0;

    for i in 0..2 {
        let current = (*(line6pcm as *const u8).add(300) as *const i32).add(i).read();
        let new_val = (*((ucontrol as *const u8).add(8) as *const i32)).add(i).read();
        if current != new_val {
            (*(line6pcm as *mut u8).add(300) as *mut i32).add(i).write(new_val);
            changed = 1;
        }
    }

    changed
}

// control definition
#[repr(C)]
struct SndKcontrolNew {
    iface: i32,
    name: *const u8,
    info: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> i32,
    get: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32,
    put: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32,
}

static LINE6_CONTROLS: [SndKcontrolNew; 3] = [
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"PCM Playback Volume\0".as_ptr(),
        info: snd_line6_control_playback_info,
        get: snd_line6_control_playback_get,
        put: snd_line6_control_playback_put,
    },
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Impulse Response Volume\0".as_ptr(),
        info: snd_line6_impulse_volume_info,
        get: snd_line6_impulse_volume_get,
        put: snd_line6_impulse_volume_put,
    },
    SndKcontrolNew {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Impulse Response Period\0".as_ptr(),
        info: snd_line6_impulse_period_info,
        get: snd_line6_impulse_period_get,
        put: snd_line6_impulse_period_put,
    },
];

// Cleanup the PCM device.
unsafe fn cleanup_urbs(pcms: *mut line6_pcm_stream, iso_buffers: i32) {
    let pcms = &mut *pcms;

    if pcms.urbs.is_null() {
        return;
    }

    for i in 0..iso_buffers {
        let urb = (pcms.urbs as *const *mut core::ffi::c_void).add(i as usize).read();
        if !urb.is_null() {
            usb_kill_urb(urb);
            usb_free_urb(urb);
        }
    }
    kfree(pcms.urbs as *mut core::ffi::c_void);
    pcms.urbs = null_mut();
}

unsafe fn line6_cleanup_pcm(pcm: *mut snd_pcm) {
    let line6pcm = snd_pcm_chip(pcm);
    let line6pcm = &*line6pcm;

    cleanup_urbs(&line6pcm.out as *const line6_pcm_stream as *mut line6_pcm_stream, line6pcm.line6.iso_buffers as i32);
    cleanup_urbs(&line6pcm.in_ as *const line6_pcm_stream as *mut line6_pcm_stream, line6pcm.line6.iso_buffers as i32);
    kfree(line6pcm as *const u8 as *mut core::ffi::c_void);
}

// create a PCM device
unsafe fn snd_line6_new_pcm(line6: *mut usb_line6, pcm_ret: *mut *mut snd_pcm) -> i32 {
    let line6 = &*line6;
    let mut err;

    err = snd_pcm_new(
        line6.card as *mut core::ffi::c_void,
        line6.properties.name as *const u8,
        0,
        1,
        1,
        pcm_ret,
    );
    if err < 0 {
        return err;
    }
    let pcm = *pcm_ret;
    strscpy(
        (pcm as *mut u8).add(0) as *mut u8,
        line6.properties.name as *const u8,
    );

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_line6_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_line6_capture_ops);

    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_CONTINUOUS, null_mut(), 64 * 1024, 128 * 1024);
    0
}

// Sync with PCM stream stops.
pub unsafe extern "C" fn line6_pcm_disconnect(line6pcm: *mut snd_line6_pcm) {
    let line6pcm = &*line6pcm;
    line6_unlink_audio_urbs(line6pcm as *const u8 as *mut snd_line6_pcm, &line6pcm.out as *const line6_pcm_stream as *mut line6_pcm_stream);
    line6_unlink_audio_urbs(line6pcm as *const u8 as *mut snd_line6_pcm, &line6pcm.in_ as *const line6_pcm_stream as *mut line6_pcm_stream);
    line6_wait_clear_audio_urbs(line6pcm as *const u8 as *mut snd_line6_pcm, &line6pcm.out as *const line6_pcm_stream as *mut line6_pcm_stream);
    line6_wait_clear_audio_urbs(line6pcm as *const u8 as *mut snd_line6_pcm, &line6pcm.in_ as *const line6_pcm_stream as *mut line6_pcm_stream);
}

// Create and register the PCM device and mixer entries.
// Create URBs for playback and capture.
pub unsafe extern "C" fn line6_init_pcm(
    line6: *mut usb_line6,
    properties: *mut line6_pcm_properties,
) -> i32 {
    let line6 = &mut *line6;
    let mut pcm: *mut snd_pcm = null_mut();
    let mut err;

    if (line6.properties.capabilities & LINE6_CAP_PCM) == 0 {
        return 0;
    }

    err = snd_line6_new_pcm(line6, &mut pcm);
    if err < 0 {
        return err;
    }

    let line6pcm = kzalloc(mem::size_of::<snd_line6_pcm>(), GFP_KERNEL) as *mut snd_line6_pcm;
    if line6pcm.is_null() {
        return ENOMEM;
    }

    let line6pcm_ref = &mut *line6pcm;
    mutex_init(&line6pcm_ref.state_mutex as *const u8 as *mut core::ffi::c_void);
    line6pcm_ref.pcm = pcm;
    line6pcm_ref.properties = properties;
    line6pcm_ref.volume_playback[0] = 255;
    line6pcm_ref.volume_playback[1] = 255;
    line6pcm_ref.volume_monitor = 255;
    line6pcm_ref.line6 = line6;

    spin_lock_init(&line6pcm_ref.out.lock as *const u8 as *mut core::ffi::c_void);
    spin_lock_init(&line6pcm_ref.in_.lock as *const u8 as *mut core::ffi::c_void);
    line6pcm_ref.impulse_period = LINE6_IMPULSE_DEFAULT_PERIOD;

    line6.line6pcm = line6pcm;

    (pcm as *mut u8 as *mut *const core::ffi::c_void).add(8).write(line6pcm as *const core::ffi::c_void);
    (pcm as *mut u8 as *mut unsafe extern "C" fn(*mut snd_pcm)).add(16).write(line6_cleanup_pcm);

    let ep_read = line6.properties.ep_audio_r;
    let ep_write = line6.properties.ep_audio_w;

    line6pcm_ref.max_packet_size_in = usb_maxpacket(line6.usbdev, usb_rcvisocpipe(line6.usbdev, ep_read));
    line6pcm_ref.max_packet_size_out = usb_maxpacket(line6.usbdev, usb_sndisocpipe(line6.usbdev, ep_write));
    if line6pcm_ref.max_packet_size_in == 0 || line6pcm_ref.max_packet_size_out == 0 {
        dev_err(
            line6pcm_ref.line6.ifcdev as *mut core::ffi::c_void,
            b"cannot get proper max packet size\n\0".as_ptr(),
        );
        return EINVAL;
    }

    err = line6_create_audio_out_urbs(line6pcm);
    if err < 0 {
        return err;
    }

    err = line6_create_audio_in_urbs(line6pcm);
    if err < 0 {
        return err;
    }

    for i in 0..LINE6_CONTROLS.len() {
        err = snd_ctl_add(
            line6.card as *mut core::ffi::c_void,
            snd_ctl_new1(&LINE6_CONTROLS[i] as *const SndKcontrolNew as *const snd_kcontrol_new, line6pcm as *mut core::ffi::c_void),
        );
        if err < 0 {
            return err;
        }
    }

    0
}

// EXPORT_SYMBOL_GPL(line6_init_pcm);

// prepare pcm callback
pub unsafe extern "C" fn snd_line6_prepare(substream: *mut snd_pcm_substream) -> i32 {
    let line6pcm = snd_pcm_substream_chip(substream);
    let substream = &*substream;
    let pstr = get_stream(line6pcm, substream.stream);

    // guard(mutex)(&line6pcm->state_mutex)
    let pstr_ref = &*pstr;
    if pstr_ref.running == 0 {
        line6_wait_clear_audio_urbs(line6pcm, pstr);
    }

    let line6pcm = &*line6pcm;
    if !test_and_set_bit(LINE6_FLAG_PREPARED, &line6pcm.flags as *const i32 as *mut u64) {
        let out = &line6pcm.out;
        (out as *const line6_pcm_stream as *mut line6_pcm_stream as *mut u8 as *mut i32).add(0).write(0);
        (out as *const line6_pcm_stream as *mut line6_pcm_stream as *mut u8 as *mut i32).add(1).write(0);
        (out as *const line6_pcm_stream as *mut line6_pcm_stream as *mut u8 as *mut i32).add(2).write(0);
        (out as *const line6_pcm_stream as *mut line6_pcm_stream as *mut u8 as *mut i32).add(3).write(0);
        let in_ = &line6pcm.in_;
        (in_ as *const line6_pcm_stream as *mut line6_pcm_stream as *mut u8 as *mut i32).add(0).write(0);
        (in_ as *const line6_pcm_stream as *mut line6_pcm_stream as *mut u8 as *mut i32).add(2).write(0);
        (in_ as *const line6_pcm_stream as *mut line6_pcm_stream as *mut u8 as *mut i32).add(3).write(0);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
