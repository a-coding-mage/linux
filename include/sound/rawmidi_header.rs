/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from sound/rawmidi.h. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

/* Dependencies supplied by the surrounding kernel translation. */
pub enum snd_rawmidi {}
pub enum snd_rawmidi_substream {}
pub enum snd_seq_port_info {}
pub enum pid {}
pub enum snd_info_entry {}
pub enum snd_info_buffer {}
pub enum snd_card {}
pub enum snd_rawmidi_info {}
pub enum snd_rawmidi_params {}
pub enum list_head {}
pub enum spinlock_t {}
pub enum wait_queue_head_t {}
pub enum work_struct {}
pub enum mutex {}
pub enum device {}
pub enum snd_seq_device {}

pub const SNDRV_RAWMIDI_DEVICES: c_uint = 8;
pub const SNDRV_RAWMIDI_LFLG_OUTPUT: c_uint = 1 << 0;
pub const SNDRV_RAWMIDI_LFLG_INPUT: c_uint = 1 << 1;
pub const SNDRV_RAWMIDI_LFLG_OPEN: c_uint = 3 << 0;
pub const SNDRV_RAWMIDI_LFLG_APPEND: c_uint = 1 << 2;

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
    pub drain: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
}

#[repr(C)]
pub struct snd_rawmidi_global_ops {
    pub dev_register: Option<unsafe extern "C" fn(*mut snd_rawmidi) -> c_int>,
    pub dev_unregister: Option<unsafe extern "C" fn(*mut snd_rawmidi) -> c_int>,
    pub get_port_info: Option<unsafe extern "C" fn(*mut snd_rawmidi, c_int, *mut snd_seq_port_info)>,
    pub ioctl: Option<unsafe extern "C" fn(*mut snd_rawmidi, c_uint, *mut c_void) -> c_long>,
    pub proc_read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub struct snd_rawmidi_runtime {
    pub substream: *mut snd_rawmidi_substream,
    /* C bitfields: drain:1 and oss:1. Kept in their original unsigned word. */
    pub drain_oss: c_uint,
    pub buffer: *mut u8,
    pub buffer_size: usize,
    pub appl_ptr: usize,
    pub hw_ptr: usize,
    pub avail_min: usize,
    pub avail: usize,
    pub xruns: usize,
    pub align: usize,
    pub buffer_ref: c_int,
    pub sleep: wait_queue_head_t,
    pub event: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
    pub event_work: work_struct,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub list: list_head,
    pub stream: c_int,
    pub number: c_int,
    pub opened: bool,
    pub append: bool,
    pub active_sensing: bool,
    pub framing: c_uint,
    pub clock_type: c_uint,
    pub use_count: c_int,
    pub inactive: bool,
    pub bytes: usize,
    pub lock: spinlock_t,
    pub rmidi: *mut snd_rawmidi,
    pub pstr: *mut snd_rawmidi_str,
    pub name: [c_char; 32],
    pub runtime: *mut snd_rawmidi_runtime,
    pub pid: *mut pid,
    pub ops: *const snd_rawmidi_ops,
}

#[repr(C)]
pub struct snd_rawmidi_file {
    pub rmidi: *mut snd_rawmidi,
    pub input: *mut snd_rawmidi_substream,
    pub output: *mut snd_rawmidi_substream,
    pub user_pversion: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi_str {
    pub substream_count: c_uint,
    pub substream_opened: c_uint,
    pub substreams: list_head,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub card: *mut snd_card,
    pub list: list_head,
    pub device: c_uint,
    pub info_flags: c_uint,
    pub tied_device: c_uint,
    pub id: [c_char; 64],
    pub name: [c_char; 80],
    #[cfg(CONFIG_SND_OSSEMUL)]
    pub ossreg: c_int,
    pub ops: *const snd_rawmidi_global_ops,
    pub streams: [snd_rawmidi_str; 2],
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_rawmidi)>,
    pub open_mutex: mutex,
    pub open_wait: wait_queue_head_t,
    pub dev: *mut device,
    pub proc_entry: *mut snd_info_entry,
    #[cfg(CONFIG_SND_SEQUENCER)]
    pub seq_dev: *mut snd_seq_device,
}

extern "C" {
    pub fn snd_rawmidi_new(card: *mut snd_card, id: *mut c_char, device: c_int,
                           output_count: c_int, input_count: c_int,
                           rmidi: *mut *mut snd_rawmidi) -> c_int;
    pub fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int,
                               ops: *const snd_rawmidi_ops);
    pub fn snd_rawmidi_init(rmidi: *mut snd_rawmidi, card: *mut snd_card, id: *mut c_char,
                            device: c_int, output_count: c_int, input_count: c_int,
                            info_flags: c_uint) -> c_int;
    pub fn snd_rawmidi_free(rmidi: *mut snd_rawmidi) -> c_int;
    pub fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buffer: *const u8, count: c_int) -> c_int;
    pub fn snd_rawmidi_transmit_empty(substream: *mut snd_rawmidi_substream) -> c_int;
    pub fn snd_rawmidi_transmit_peek(substream: *mut snd_rawmidi_substream, buffer: *mut u8, count: c_int) -> c_int;
    pub fn snd_rawmidi_transmit_ack(substream: *mut snd_rawmidi_substream, count: c_int) -> c_int;
    pub fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buffer: *mut u8, count: c_int) -> c_int;
    pub fn snd_rawmidi_proceed(substream: *mut snd_rawmidi_substream) -> c_int;
    pub fn snd_rawmidi_info_select(card: *mut snd_card, info: *mut snd_rawmidi_info) -> c_int;
    pub fn snd_rawmidi_kernel_open_nested(rmidi: *mut snd_rawmidi, subdevice: c_int, mode: c_int, rfile: *mut snd_rawmidi_file, depth: c_int) -> c_int;
    pub fn snd_rawmidi_kernel_release_nested(rfile: *mut snd_rawmidi_file, depth: c_int) -> c_int;
    pub fn snd_rawmidi_output_params(substream: *mut snd_rawmidi_substream, params: *mut snd_rawmidi_params) -> c_int;
    pub fn snd_rawmidi_input_params(substream: *mut snd_rawmidi_substream, params: *mut snd_rawmidi_params) -> c_int;
    pub fn snd_rawmidi_drop_output(substream: *mut snd_rawmidi_substream) -> c_int;
    pub fn snd_rawmidi_drain_output(substream: *mut snd_rawmidi_substream) -> c_int;
    pub fn snd_rawmidi_drain_input(substream: *mut snd_rawmidi_substream) -> c_int;
    pub fn snd_rawmidi_kernel_read(substream: *mut snd_rawmidi_substream, buf: *mut u8, count: c_long) -> c_long;
    pub fn snd_rawmidi_kernel_write(substream: *mut snd_rawmidi_substream, buf: *const u8, count: c_long) -> c_long;
}

#[inline]
pub unsafe fn snd_rawmidi_kernel_open(rmidi: *mut snd_rawmidi, subdevice: c_int, mode: c_int, rfile: *mut snd_rawmidi_file) -> c_int {
    snd_rawmidi_kernel_open_nested(rmidi, subdevice, mode, rfile, 0)
}

#[inline]
pub unsafe fn snd_rawmidi_kernel_release(rfile: *mut snd_rawmidi_file) -> c_int {
    snd_rawmidi_kernel_release_nested(rfile, 0)
}

#[inline]
pub unsafe fn snd_rawmidi_tie_devices(r1: *mut snd_rawmidi, r2: *mut snd_rawmidi) {
    (*r1).tied_device = (*r2).device + 1;
    (*r2).tied_device = (*r1).device + 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
