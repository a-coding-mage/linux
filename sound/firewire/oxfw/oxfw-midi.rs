// SPDX-License-Identifier: GPL-2.0-only
/*
 * oxfw_midi.c - a part of driver for OXFW970/971 based devices
 *
 * Copyright (c) 2014 Takashi Sakamoto
 */

// Dependency intent from C source: #include "oxfw.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x00000001;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x00000002;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x00000004;
const SNDRV_RAWMIDI_STREAM_OUTPUT: usize = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: usize = 1;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
}

#[repr(C)]
pub struct amdtp_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_oxfw {
    pub card: *mut snd_card,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub substreams_count: c_uint,
    pub midi_input_ports: c_uint,
    pub midi_output_ports: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub name: [c_char; 80],
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub streams: [snd_rawmidi_str; 2],
}

#[repr(C)]
pub struct snd_rawmidi_str {
    pub substreams: list_head,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub list: list_head,
    pub rmidi: *mut snd_rawmidi,
    pub number: c_int,
    pub name: [c_char; 32],
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

unsafe extern "C" {
    fn snd_oxfw_stream_lock_try(oxfw: *mut snd_oxfw) -> c_int;
    fn snd_oxfw_stream_lock_release(oxfw: *mut snd_oxfw);
    fn snd_oxfw_stream_reserve_duplex(
        oxfw: *mut snd_oxfw,
        stream: *mut amdtp_stream,
        rate: c_uint,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
        channels: c_uint,
    ) -> c_int;
    fn snd_oxfw_stream_start_duplex(oxfw: *mut snd_oxfw) -> c_int;
    fn snd_oxfw_stream_stop_duplex(oxfw: *mut snd_oxfw);
    fn amdtp_am824_midi_trigger(
        stream: *mut amdtp_stream,
        port: c_int,
        substrm: *mut snd_rawmidi_substream,
    );
    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        output_count: c_uint,
        input_count: c_uint,
        rmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_rawmidi_set_ops(
        rmidi: *mut snd_rawmidi,
        stream: c_int,
        ops: *const snd_rawmidi_ops,
    );
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
}

type c_ulong = core::ffi::c_ulong;

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        unsafe {
            mutex_lock(lock);
        }
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe {
            mutex_unlock(self.lock);
        }
    }
}

struct SpinLockIrqsaveGuard {
    lock: *mut spinlock_t,
    flags: c_ulong,
}

impl SpinLockIrqsaveGuard {
    unsafe fn new(lock: *mut spinlock_t) -> Self {
        let mut flags = 0;
        unsafe {
            spin_lock_irqsave(lock, &mut flags);
        }
        Self { lock, flags }
    }
}

impl Drop for SpinLockIrqsaveGuard {
    fn drop(&mut self) {
        unsafe {
            spin_unlock_irqrestore(self.lock, self.flags);
        }
    }
}

unsafe extern "C" fn midi_capture_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let oxfw = unsafe { (*(*substream).rmidi).private_data as *mut snd_oxfw };
    let mut err: c_int;

    err = unsafe { snd_oxfw_stream_lock_try(oxfw) };
    if err < 0 {
        return err;
    }

    {
        let _guard = unsafe { MutexGuard::new(&mut (*oxfw).mutex) };
        err = unsafe {
            snd_oxfw_stream_reserve_duplex(oxfw, &mut (*oxfw).tx_stream, 0, 0, 0, 0)
        };
        if err >= 0 {
            unsafe {
                (*oxfw).substreams_count = (*oxfw).substreams_count.wrapping_add(1);
            }
            err = unsafe { snd_oxfw_stream_start_duplex(oxfw) };
            if err < 0 {
                unsafe {
                    (*oxfw).substreams_count = (*oxfw).substreams_count.wrapping_sub(1);
                }
            }
        }
    }

    if err < 0 {
        unsafe {
            snd_oxfw_stream_lock_release(oxfw);
        }
    }

    err
}

unsafe extern "C" fn midi_playback_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let oxfw = unsafe { (*(*substream).rmidi).private_data as *mut snd_oxfw };
    let mut err: c_int;

    err = unsafe { snd_oxfw_stream_lock_try(oxfw) };
    if err < 0 {
        return err;
    }

    {
        let _guard = unsafe { MutexGuard::new(&mut (*oxfw).mutex) };
        err = unsafe {
            snd_oxfw_stream_reserve_duplex(oxfw, &mut (*oxfw).rx_stream, 0, 0, 0, 0)
        };
        if err >= 0 {
            unsafe {
                (*oxfw).substreams_count = (*oxfw).substreams_count.wrapping_add(1);
            }
            err = unsafe { snd_oxfw_stream_start_duplex(oxfw) };
        }
    }

    if err < 0 {
        unsafe {
            snd_oxfw_stream_lock_release(oxfw);
        }
    }

    err
}

unsafe extern "C" fn midi_capture_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let oxfw = unsafe { (*(*substream).rmidi).private_data as *mut snd_oxfw };

    {
        let _guard = unsafe { MutexGuard::new(&mut (*oxfw).mutex) };
        unsafe {
            (*oxfw).substreams_count = (*oxfw).substreams_count.wrapping_sub(1);
            snd_oxfw_stream_stop_duplex(oxfw);
        }
    }

    unsafe {
        snd_oxfw_stream_lock_release(oxfw);
    }
    0
}

unsafe extern "C" fn midi_playback_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let oxfw = unsafe { (*(*substream).rmidi).private_data as *mut snd_oxfw };

    {
        let _guard = unsafe { MutexGuard::new(&mut (*oxfw).mutex) };
        unsafe {
            (*oxfw).substreams_count = (*oxfw).substreams_count.wrapping_sub(1);
            snd_oxfw_stream_stop_duplex(oxfw);
        }
    }

    unsafe {
        snd_oxfw_stream_lock_release(oxfw);
    }
    0
}

unsafe extern "C" fn midi_capture_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    let oxfw = unsafe { (*(*substrm).rmidi).private_data as *mut snd_oxfw };

    let _guard = unsafe { SpinLockIrqsaveGuard::new(&mut (*oxfw).lock) };

    if up != 0 {
        unsafe {
            amdtp_am824_midi_trigger(&mut (*oxfw).tx_stream, (*substrm).number, substrm);
        }
    } else {
        unsafe {
            amdtp_am824_midi_trigger(&mut (*oxfw).tx_stream, (*substrm).number, ptr::null_mut());
        }
    }
}

unsafe extern "C" fn midi_playback_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    let oxfw = unsafe { (*(*substrm).rmidi).private_data as *mut snd_oxfw };

    let _guard = unsafe { SpinLockIrqsaveGuard::new(&mut (*oxfw).lock) };

    if up != 0 {
        unsafe {
            amdtp_am824_midi_trigger(&mut (*oxfw).rx_stream, (*substrm).number, substrm);
        }
    } else {
        unsafe {
            amdtp_am824_midi_trigger(&mut (*oxfw).rx_stream, (*substrm).number, ptr::null_mut());
        }
    }
}

unsafe fn set_midi_substream_names(oxfw: *mut snd_oxfw, str_: *mut snd_rawmidi_str) {
    let mut pos = unsafe { (*str_).substreams.next };

    while pos != unsafe { &mut (*str_).substreams as *mut list_head } {
        let subs = pos as *mut snd_rawmidi_substream;
        unsafe {
            scnprintf(
                (*subs).name.as_mut_ptr(),
                size_of::<[c_char; 32]>(),
                c"%s MIDI %d".as_ptr(),
                (*(*oxfw).card).shortname.as_ptr(),
                (*subs).number + 1,
            );
            pos = (*pos).next;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_oxfw_create_midi(oxfw: *mut snd_oxfw) -> c_int {
    static CAPTURE_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_capture_open),
        close: Some(midi_capture_close),
        trigger: Some(midi_capture_trigger),
    };
    static PLAYBACK_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_playback_open),
        close: Some(midi_playback_close),
        trigger: Some(midi_playback_trigger),
    };
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let mut str_: *mut snd_rawmidi_str;
    let mut err: c_int;

    if unsafe { (*oxfw).midi_input_ports == 0 && (*oxfw).midi_output_ports == 0 } {
        return 0;
    }

    /* create midi ports */
    err = unsafe {
        snd_rawmidi_new(
            (*oxfw).card,
            (*(*oxfw).card).driver.as_ptr(),
            0,
            (*oxfw).midi_output_ports,
            (*oxfw).midi_input_ports,
            &mut rmidi,
        )
    };
    if err < 0 {
        return err;
    }

    unsafe {
        snprintf(
            (*rmidi).name.as_mut_ptr(),
            size_of::<[c_char; 80]>(),
            c"%s MIDI".as_ptr(),
            (*(*oxfw).card).shortname.as_ptr(),
        );
        (*rmidi).private_data = oxfw as *mut c_void;
    }

    if unsafe { (*oxfw).midi_input_ports > 0 } {
        unsafe {
            (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_INPUT;

            snd_rawmidi_set_ops(
                rmidi,
                SNDRV_RAWMIDI_STREAM_INPUT as c_int,
                &CAPTURE_OPS,
            );

            str_ = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT];

            set_midi_substream_names(oxfw, str_);
        }
    }

    if unsafe { (*oxfw).midi_output_ports > 0 } {
        unsafe {
            (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;

            snd_rawmidi_set_ops(
                rmidi,
                SNDRV_RAWMIDI_STREAM_OUTPUT as c_int,
                &PLAYBACK_OPS,
            );

            str_ = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT];

            set_midi_substream_names(oxfw, str_);
        }
    }

    if unsafe { ((*oxfw).midi_output_ports > 0) && ((*oxfw).midi_input_ports > 0) } {
        unsafe {
            (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
