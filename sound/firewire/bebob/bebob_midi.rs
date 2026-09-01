// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob_midi.c - a part of driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Rust translation of the implementation that depends on declarations from "bebob.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of_val;
use core::ptr::{null_mut, NonNull};

const SNDRV_RAWMIDI_STREAM_INPUT: usize = 0;
const SNDRV_RAWMIDI_STREAM_OUTPUT: usize = 1;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x0000_0001;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x0000_0002;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x0000_0004;

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
pub struct amdtp_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
}

#[repr(C)]
pub struct snd_bebob {
    pub card: *mut snd_card,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub substreams_counter: c_uint,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub midi_output_ports: c_uint,
    pub midi_input_ports: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub name: [c_char; 80],
    pub streams: [snd_rawmidi_str; 2],
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
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
    fn snd_bebob_stream_lock_try(bebob: *mut snd_bebob) -> c_int;
    fn snd_bebob_stream_lock_release(bebob: *mut snd_bebob);
    fn snd_bebob_stream_reserve_duplex(
        bebob: *mut snd_bebob,
        rate: c_uint,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    fn snd_bebob_stream_start_duplex(bebob: *mut snd_bebob) -> c_int;
    fn snd_bebob_stream_stop_duplex(bebob: *mut snd_bebob);
    fn amdtp_am824_midi_trigger(
        stream: *mut amdtp_stream,
        port: c_int,
        substream: *mut snd_rawmidi_substream,
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
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_uint);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_uint);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

struct MutexGuard {
    mutex: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(mutex: *mut mutex) -> Self {
        unsafe {
            mutex_lock(mutex);
        }
        Self { mutex }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe {
            mutex_unlock(self.mutex);
        }
    }
}

struct SpinlockIrqsaveGuard {
    lock: *mut spinlock_t,
    flags: c_uint,
}

impl SpinlockIrqsaveGuard {
    unsafe fn new(lock: *mut spinlock_t) -> Self {
        let mut flags = 0;
        unsafe {
            spin_lock_irqsave(lock, &mut flags);
        }
        Self { lock, flags }
    }
}

impl Drop for SpinlockIrqsaveGuard {
    fn drop(&mut self) {
        unsafe {
            spin_unlock_irqrestore(self.lock, self.flags);
        }
    }
}

unsafe extern "C" fn midi_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let bebob = unsafe { (*(*substream).rmidi).private_data as *mut snd_bebob };
    let mut err: c_int;

    err = unsafe { snd_bebob_stream_lock_try(bebob) };
    if err < 0 {
        return err;
    }

    {
        let _guard = unsafe { MutexGuard::new(&mut (*bebob).mutex) };
        err = unsafe { snd_bebob_stream_reserve_duplex(bebob, 0, 0, 0) };
        if err >= 0 {
            unsafe {
                (*bebob).substreams_counter = (*bebob).substreams_counter.wrapping_add(1);
            }
            err = unsafe { snd_bebob_stream_start_duplex(bebob) };
            if err < 0 {
                unsafe {
                    (*bebob).substreams_counter = (*bebob).substreams_counter.wrapping_sub(1);
                }
            }
        }
    }
    if err < 0 {
        unsafe {
            snd_bebob_stream_lock_release(bebob);
        }
    }

    err
}

unsafe extern "C" fn midi_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let bebob = unsafe { (*(*substream).rmidi).private_data as *mut snd_bebob };

    {
        let _guard = unsafe { MutexGuard::new(&mut (*bebob).mutex) };
        unsafe {
            (*bebob).substreams_counter = (*bebob).substreams_counter.wrapping_sub(1);
            snd_bebob_stream_stop_duplex(bebob);
        }
    }

    unsafe {
        snd_bebob_stream_lock_release(bebob);
    }
    0
}

unsafe extern "C" fn midi_capture_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    let bebob = unsafe { (*(*substrm).rmidi).private_data as *mut snd_bebob };

    let _guard = unsafe { SpinlockIrqsaveGuard::new(&mut (*bebob).lock) };

    if up != 0 {
        unsafe {
            amdtp_am824_midi_trigger(&mut (*bebob).tx_stream, (*substrm).number, substrm);
        }
    } else {
        unsafe {
            amdtp_am824_midi_trigger(&mut (*bebob).tx_stream, (*substrm).number, null_mut());
        }
    }
}

unsafe extern "C" fn midi_playback_trigger(substrm: *mut snd_rawmidi_substream, up: c_int) {
    let bebob = unsafe { (*(*substrm).rmidi).private_data as *mut snd_bebob };

    let _guard = unsafe { SpinlockIrqsaveGuard::new(&mut (*bebob).lock) };

    if up != 0 {
        unsafe {
            amdtp_am824_midi_trigger(&mut (*bebob).rx_stream, (*substrm).number, substrm);
        }
    } else {
        unsafe {
            amdtp_am824_midi_trigger(&mut (*bebob).rx_stream, (*substrm).number, null_mut());
        }
    }
}

unsafe fn container_of_substream_list(ptr: *mut list_head) -> *mut snd_rawmidi_substream {
    (ptr as *mut u8).sub((unsafe {
        &(*(core::ptr::null::<snd_rawmidi_substream>())).list as *const list_head as usize
    })) as *mut snd_rawmidi_substream
}

unsafe fn set_midi_substream_names(bebob: *mut snd_bebob, str_: *mut snd_rawmidi_str) {
    let mut pos = unsafe { (*str_).substreams.next };

    while pos != unsafe { &mut (*str_).substreams } {
        let subs = unsafe { container_of_substream_list(pos) };
        unsafe {
            scnprintf(
                (*subs).name.as_mut_ptr(),
                size_of_val(&(*subs).name),
                c"%s MIDI %d".as_ptr(),
                (*(*bebob).card).shortname.as_ptr(),
                (*subs).number + 1,
            );
            pos = (*pos).next;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_bebob_create_midi_devices(bebob: *mut snd_bebob) -> c_int {
    static CAPTURE_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_open),
        close: Some(midi_close),
        trigger: Some(midi_capture_trigger),
    };
    static PLAYBACK_OPS: snd_rawmidi_ops = snd_rawmidi_ops {
        open: Some(midi_open),
        close: Some(midi_close),
        trigger: Some(midi_playback_trigger),
    };
    let mut rmidi: *mut snd_rawmidi = null_mut();
    let mut str_: *mut snd_rawmidi_str;
    let mut err: c_int;

    /* create midi ports */
    err = unsafe {
        snd_rawmidi_new(
            (*bebob).card,
            (*(*bebob).card).driver.as_ptr(),
            0,
            (*bebob).midi_output_ports,
            (*bebob).midi_input_ports,
            &mut rmidi,
        )
    };
    if err < 0 {
        return err;
    }

    unsafe {
        snprintf(
            (*rmidi).name.as_mut_ptr(),
            size_of_val(&(*rmidi).name),
            c"%s MIDI".as_ptr(),
            (*(*bebob).card).shortname.as_ptr(),
        );
        (*rmidi).private_data = bebob as *mut c_void;
    }

    if unsafe { (*bebob).midi_input_ports > 0 } {
        unsafe {
            (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_INPUT;

            snd_rawmidi_set_ops(
                rmidi,
                SNDRV_RAWMIDI_STREAM_INPUT as c_int,
                &CAPTURE_OPS,
            );

            str_ = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_INPUT];

            set_midi_substream_names(bebob, str_);
        }
    }

    if unsafe { (*bebob).midi_output_ports > 0 } {
        unsafe {
            (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT;

            snd_rawmidi_set_ops(
                rmidi,
                SNDRV_RAWMIDI_STREAM_OUTPUT as c_int,
                &PLAYBACK_OPS,
            );

            str_ = &mut (*rmidi).streams[SNDRV_RAWMIDI_STREAM_OUTPUT];

            set_midi_substream_names(bebob, str_);
        }
    }

    if unsafe { ((*bebob).midi_output_ports > 0) && ((*bebob).midi_input_ports > 0) } {
        unsafe {
            (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_DUPLEX;
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
