// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) by Paul Barton-Davis 1998-1999
 *
 * Some portions of this file are taken from work that is
 * copyright (C) by Hannu Savolainen 1993-1996
 */

/*
 * Rust source-level translation of wavefront_synth.c.
 * Kernel/header supplied items from linux/* and sound/snd_wavefront.h are
 * declared or referenced here as external dependencies.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{copy_nonoverlapping, null_mut};

type u8 = u8;
type u16 = u16;
type u32 = u32;

const EACCES: c_int = 13;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;

const GFP_KERNEL: c_int = 0;
const HZ: c_int = 100;

const DEFAULT_OSPATH: &[u8] = b"wavefront.os\0";

static mut wf_raw: c_int = 0;
static mut fx_raw: c_int = 1;
static mut debug_default: c_int = 0;
static mut ospath: *mut c_char = DEFAULT_OSPATH.as_ptr() as *mut c_char;
static mut wait_usecs: c_int = 150;
static mut sleep_interval: c_int = 100;
static mut sleep_tries: c_int = 50;
static mut reset_time: c_int = 2;
static mut ramcheck_time: c_int = 20;
static mut osrun_time: c_int = 10;

/* module_param/MODULE_PARM_DESC/MODULE_FIRMWARE metadata is supplied by the
 * kernel module layer in C; preserved here as source intent only.
 */

const WF_DEBUG: c_int = 1;
const LOGNAME: &str = "WaveFront: ";

const STAT_RINTR_ENABLED: c_int = 0x01;
const STAT_CAN_READ: c_int = 0x02;
const STAT_INTR_READ: c_int = 0x04;
const STAT_WINTR_ENABLED: c_int = 0x10;
const STAT_CAN_WRITE: c_int = 0x20;
const STAT_INTR_WRITE: c_int = 0x40;

const NEEDS_ACK: c_int = 1;

/* Constants normally supplied by snd_wavefront.h. Values are declarations of
 * dependency intent for this isolated translation.
 */
const WFC_SET_SYNTHVOL: c_int = 0x01;
const WFC_GET_SYNTHVOL: c_int = 0x02;
const WFC_SET_NVOICES: c_int = 0x03;
const WFC_GET_NVOICES: c_int = 0x04;
const WFC_SET_TUNING: c_int = 0x05;
const WFC_GET_TUNING: c_int = 0x06;
const WFC_DISABLE_CHANNEL: c_int = 0x07;
const WFC_ENABLE_CHANNEL: c_int = 0x08;
const WFC_GET_CHANNEL_STATUS: c_int = 0x09;
const WFC_MISYNTH_OFF: c_int = 0x0a;
const WFC_MISYNTH_ON: c_int = 0x0b;
const WFC_VMIDI_ON: c_int = 0x0c;
const WFC_VMIDI_OFF: c_int = 0x0d;
const WFC_MIDI_STATUS: c_int = 0x0e;
const WFC_FIRMWARE_VERSION: c_int = 0x0f;
const WFC_HARDWARE_VERSION: c_int = 0x10;
const WFC_GET_NSAMPLES: c_int = 0x11;
const WFC_INSTOUT_LEVELS: c_int = 0x12;
const WFC_PEAKOUT_LEVELS: c_int = 0x13;
const WFC_DOWNLOAD_SAMPLE: c_int = 0x14;
const WFC_DOWNLOAD_BLOCK: c_int = 0x15;
const WFC_DOWNLOAD_SAMPLE_HEADER: c_int = 0x16;
const WFC_UPLOAD_SAMPLE_HEADER: c_int = 0x17;
const WFC_DOWNLOAD_MULTISAMPLE: c_int = 0x18;
const WFC_UPLOAD_MULTISAMPLE: c_int = 0x19;
const WFC_DOWNLOAD_SAMPLE_ALIAS: c_int = 0x1a;
const WFC_UPLOAD_SAMPLE_ALIAS: c_int = 0x1b;
const WFC_DELETE_SAMPLE: c_int = 0x1c;
const WFC_IDENTIFY_SAMPLE_TYPE: c_int = 0x1d;
const WFC_UPLOAD_SAMPLE_PARAMS: c_int = 0x1e;
const WFC_REPORT_FREE_MEMORY: c_int = 0x1f;
const WFC_DOWNLOAD_PATCH: c_int = 0x20;
const WFC_UPLOAD_PATCH: c_int = 0x21;
const WFC_DOWNLOAD_PROGRAM: c_int = 0x22;
const WFC_UPLOAD_PROGRAM: c_int = 0x23;
const WFC_DOWNLOAD_EDRUM_PROGRAM: c_int = 0x24;
const WFC_UPLOAD_EDRUM_PROGRAM: c_int = 0x25;
const WFC_SET_EDRUM_CHANNEL: c_int = 0x26;
const WFC_DISABLE_DRUM_PROGRAM: c_int = 0x27;
const WFC_REPORT_CHANNEL_PROGRAMS: c_int = 0x28;
const WFC_NOOP: c_int = 0x29;
const WFC_DISABLE_INTERRUPTS: c_int = 0x2a;
const WFC_ENABLE_INTERRUPTS: c_int = 0x2b;
const WFC_INTERRUPT_STATUS: c_int = 0x2c;
const WFC_ROMSAMPLES_RDONLY: c_int = 0x2d;
const WFC_IDENTIFY_SLOT_TYPE: c_int = 0x2e;
const WFC_DEBUG_DRIVER: c_int = 0x2f;
const WFC_DOWNLOAD_OS: c_int = 0x30;

const WF_SAMPLE_BYTES: usize = 0;
const WF_SAMPLE_HDR_BYTES: usize = 25;
const WF_ALIAS_BYTES: usize = 25;
const WF_PATCH_BYTES: usize = 132;
const WF_PROGRAM_BYTES: usize = 32;
const WF_DRUM_BYTES: usize = 9;
const WF_MSAMPLE_BYTES: usize = 259;
const WF_SECTION_MAX: c_int = 128;
const WF_MAX_SAMPLE: usize = 512;
const WF_MAX_PATCH: usize = 256;
const WF_MAX_PROGRAM: usize = 128;
const WF_NUM_LAYERS: usize = 4;
const WF_ST_EMPTY: u8 = 0;
const WF_ST_SAMPLE: u8 = 1;
const WF_ST_MULTISAMPLE: u8 = 2;
const WF_ST_ALIAS: u8 = 3;
const WF_ST_DRUM: u8 = 4;
const WF_ST_PATCH: u8 = 5;
const WF_ST_PROGRAM: u8 = 6;
const WF_ST_MASK: u8 = 0x0f;
const WF_SLOT_FILLED: u8 = 0x10;
const WF_SLOT_ROM: u8 = 0x20;
const WF_SLOT_USED: u8 = 0x40;
const WF_ACK: c_int = 0x80;
const WF_DMA_ACK: c_int = 0x81;
const LINEAR_16BIT: u8 = 1;
const WAVEFRONT_FIND_FREE_SAMPLE_SLOT: c_int = -1;
const WFCTL_LOAD_SPP: c_uint = 0;
const WFCTL_WFCMD: c_uint = 1;
const WF_DEBUG_DATA: c_int = 0x01;
const WF_DEBUG_CMD: c_int = 0x02;
const WF_DEBUG_IO: c_int = 0x04;
const WF_DEBUG_LOAD_PATCH: c_int = 0x08;
type c_uint = u32;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct module { _private: [u8; 0] }
#[repr(C)]
pub struct file { pub private_data: *mut c_void }
#[repr(C)]
pub struct snd_card { pub dev: *mut device, pub module: *mut module, pub private_data: *mut c_void }
#[repr(C)]
pub struct snd_hwdep { pub card: *mut snd_card }
#[repr(C)]
pub struct firmware { pub size: usize, pub data: *const u8 }
#[repr(C)]
pub struct wait_queue_entry_t { _private: [u8; 0] }

#[repr(C)]
pub struct wavefront_midi { pub isvirtual: c_char, pub base: c_ulong }

#[repr(C)]
pub struct snd_wavefront_t {
    pub status_port: c_ulong,
    pub data_port: c_ulong,
    pub control_port: c_ulong,
    pub block_port: c_ulong,
    pub last_block_port: c_ulong,
    pub card: *mut snd_card,
    pub sample_status: [u8; WF_MAX_SAMPLE],
    pub patch_status: [u8; WF_MAX_PATCH],
    pub prog_status: [u8; WF_MAX_PROGRAM],
    pub samples_used: c_uint,
    pub freemem: c_int,
    pub rom_samples_rdonly: c_char,
    pub interrupts_are_midi: c_char,
    pub midi_in_to_synth: c_char,
    pub midi: wavefront_midi,
    pub base: c_ulong,
    pub israw: c_char,
    pub fx_initialized: c_char,
    pub has_fx: c_char,
    pub debug: c_int,
    pub irq_cnt: c_int,
    pub irq_ok: c_int,
    pub irq: c_int,
    pub fw_version: [u8; 2],
    pub hw_version: [u8; 2],
    pub irq_lock: c_ulong,
    pub interrupt_sleeper: c_ulong,
}

#[repr(C)]
pub struct snd_wavefront_card_t { pub wavefront: snd_wavefront_t }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wavefront_layer { pub mute: u8, pub patch_number: usize }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wavefront_program { pub layer: [wavefront_layer; WF_NUM_LAYERS] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wavefront_patch { pub sample_number: u8, pub sample_msb: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wavefront_sample {
    pub sampleStartOffset: [u8; 4],
    pub loopStartOffset: [u8; 4],
    pub loopEndOffset: [u8; 4],
    pub sampleEndOffset: [u8; 4],
    pub FrequencyBias: u16,
    pub SampleResolution: u8,
    pub Loop: u8,
    pub Bidirectional: u8,
    pub Reverse: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wavefront_alias {
    pub OriginalSample: c_uint,
    pub sampleStartOffset: [u8; 4],
    pub loopStartOffset: [u8; 4],
    pub loopEndOffset: [u8; 4],
    pub sampleEndOffset: [u8; 4],
    pub FrequencyBias: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wavefront_multisample { pub NumberOfSamples: u8, pub SampleNumber: [c_uint; 128] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wavefront_drum { pub PatchNumber: u8 }
#[repr(C)]
pub union wavefront_any {
    pub s: wavefront_sample,
    pub a: wavefront_alias,
    pub ms: wavefront_multisample,
    pub d: wavefront_drum,
    pub p: wavefront_patch,
    pub pr: wavefront_program,
}
#[repr(C)]
pub struct wavefront_patch_info {
    pub number: usize,
    pub subkey: u8,
    pub size: u32,
    pub dataptr: *mut u16,
    pub hdrptr: *const c_void,
    pub hdr: wavefront_any,
}
#[repr(C)]
pub struct wavefront_control {
    pub cmd: c_int,
    pub rbuf: [u8; 512],
    pub wbuf: [u8; 512],
    pub status: c_int,
}

#[repr(C)]
struct wavefront_command {
    cmd: c_int,
    action: *const c_char,
    read_cnt: c_uint,
    write_cnt: c_uint,
    need_ack: c_int,
}
#[repr(C)]
struct wavefront_error {
    errno: c_int,
    errstr: *const c_char,
}

unsafe extern "C" {
    fn inb(port: c_ulong) -> c_int;
    fn outb(value: c_int, port: c_ulong);
    fn outw(value: u16, port: c_ulong);
    fn udelay(usecs: c_ulong);
    fn schedule_timeout_interruptible(limit: c_int) -> c_int;
    fn schedule_timeout_uninterruptible(limit: c_int) -> c_int;
    fn signal_pending(task: *mut c_void) -> c_int;
    static mut current: *mut c_void;
    static mut jiffies: c_ulong;
    fn time_before(a: c_ulong, b: c_ulong) -> bool;
    fn barrier();
    fn kmalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_int;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_int;
    fn memdup_user(from: *const c_void, n: usize) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn firmware_request_cache(dev: *mut device, name: *const c_char) -> c_int;
    fn init_waitqueue_entry(wait: *mut wait_queue_entry_t, task: *mut c_void);
    fn add_wait_queue(head: *mut c_ulong, wait: *mut wait_queue_entry_t);
    fn wake_up(head: *mut c_ulong);
    fn snd_BUG_ON(cond: bool) -> bool;
    fn snd_wavefront_midi_disable_virtual(card: *mut snd_wavefront_card_t);
    fn snd_wavefront_midi_enable_virtual(card: *mut snd_wavefront_card_t);
    fn snd_wavefront_midi_start(card: *mut snd_wavefront_card_t) -> c_int;
    fn snd_wavefront_midi_resume(card: *mut snd_wavefront_card_t);
    fn snd_wavefront_fx_detect(dev: *mut snd_wavefront_t) -> c_int;
    fn snd_wavefront_fx_start(dev: *mut snd_wavefront_t);
}

macro_rules! cstr { ($s:expr) => { concat!($s, "\0").as_ptr() as *const c_char } }
macro_rules! DPRINT { ($dev:expr, $cond:expr, $($arg:tt)*) => {{ let _ = ($dev, $cond); }} }
macro_rules! dev_err { ($($arg:tt)*) => {{}} }
macro_rules! dev_warn { ($($arg:tt)*) => {{}} }
macro_rules! dev_info { ($($arg:tt)*) => {{}} }
macro_rules! dev_dbg { ($($arg:tt)*) => {{}} }

static mut wavefront_errors: [wavefront_error; 10] = [
    wavefront_error { errno: 0x01, errstr: cstr!("Bad sample number") },
    wavefront_error { errno: 0x02, errstr: cstr!("Out of sample memory") },
    wavefront_error { errno: 0x03, errstr: cstr!("Bad patch number") },
    wavefront_error { errno: 0x04, errstr: cstr!("Error in number of voices") },
    wavefront_error { errno: 0x06, errstr: cstr!("Sample load already in progress") },
    wavefront_error { errno: 0x0B, errstr: cstr!("No sample load request pending") },
    wavefront_error { errno: 0x0E, errstr: cstr!("Bad MIDI channel number") },
    wavefront_error { errno: 0x10, errstr: cstr!("Download Record Error") },
    wavefront_error { errno: 0x80, errstr: cstr!("Success") },
    wavefront_error { errno: 0x0, errstr: core::ptr::null() },
];

static mut wavefront_commands: [wavefront_command; 41] = [
    wavefront_command { cmd: WFC_SET_SYNTHVOL, action: cstr!("set synthesizer volume"), read_cnt: 0, write_cnt: 1, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_GET_SYNTHVOL, action: cstr!("get synthesizer volume"), read_cnt: 1, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_SET_NVOICES, action: cstr!("set number of voices"), read_cnt: 0, write_cnt: 1, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_GET_NVOICES, action: cstr!("get number of voices"), read_cnt: 1, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_SET_TUNING, action: cstr!("set synthesizer tuning"), read_cnt: 0, write_cnt: 2, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_GET_TUNING, action: cstr!("get synthesizer tuning"), read_cnt: 2, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_DISABLE_CHANNEL, action: cstr!("disable synth channel"), read_cnt: 0, write_cnt: 1, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_ENABLE_CHANNEL, action: cstr!("enable synth channel"), read_cnt: 0, write_cnt: 1, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_GET_CHANNEL_STATUS, action: cstr!("get synth channel status"), read_cnt: 3, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_MISYNTH_OFF, action: cstr!("disable midi-in to synth"), read_cnt: 0, write_cnt: 0, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_MISYNTH_ON, action: cstr!("enable midi-in to synth"), read_cnt: 0, write_cnt: 0, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_VMIDI_ON, action: cstr!("enable virtual midi mode"), read_cnt: 0, write_cnt: 0, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_VMIDI_OFF, action: cstr!("disable virtual midi mode"), read_cnt: 0, write_cnt: 0, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_MIDI_STATUS, action: cstr!("report midi status"), read_cnt: 1, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_FIRMWARE_VERSION, action: cstr!("report firmware version"), read_cnt: 2, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_HARDWARE_VERSION, action: cstr!("report hardware version"), read_cnt: 2, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_GET_NSAMPLES, action: cstr!("report number of samples"), read_cnt: 2, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_INSTOUT_LEVELS, action: cstr!("report instantaneous output levels"), read_cnt: 7, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_PEAKOUT_LEVELS, action: cstr!("report peak output levels"), read_cnt: 7, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_DOWNLOAD_SAMPLE, action: cstr!("download sample"), read_cnt: 0, write_cnt: WF_SAMPLE_BYTES as c_uint, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_DOWNLOAD_BLOCK, action: cstr!("download block"), read_cnt: 0, write_cnt: 0, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_DOWNLOAD_SAMPLE_HEADER, action: cstr!("download sample header"), read_cnt: 0, write_cnt: WF_SAMPLE_HDR_BYTES as c_uint, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_UPLOAD_SAMPLE_HEADER, action: cstr!("upload sample header"), read_cnt: 13, write_cnt: 2, need_ack: 0 },
    wavefront_command { cmd: WFC_DOWNLOAD_MULTISAMPLE, action: cstr!("download multisample"), read_cnt: 0, write_cnt: 0, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_UPLOAD_MULTISAMPLE, action: cstr!("upload multisample"), read_cnt: 2, write_cnt: 1, need_ack: 0 },
    wavefront_command { cmd: WFC_DOWNLOAD_SAMPLE_ALIAS, action: cstr!("download sample alias"), read_cnt: 0, write_cnt: WF_ALIAS_BYTES as c_uint, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_UPLOAD_SAMPLE_ALIAS, action: cstr!("upload sample alias"), read_cnt: WF_ALIAS_BYTES as c_uint, write_cnt: 2, need_ack: 0 },
    wavefront_command { cmd: WFC_DELETE_SAMPLE, action: cstr!("delete sample"), read_cnt: 0, write_cnt: 2, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_IDENTIFY_SAMPLE_TYPE, action: cstr!("identify sample type"), read_cnt: 5, write_cnt: 2, need_ack: 0 },
    wavefront_command { cmd: WFC_UPLOAD_SAMPLE_PARAMS, action: cstr!("upload sample parameters"), read_cnt: 0, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_REPORT_FREE_MEMORY, action: cstr!("report free memory"), read_cnt: 4, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_DOWNLOAD_PATCH, action: cstr!("download patch"), read_cnt: 0, write_cnt: 134, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_UPLOAD_PATCH, action: cstr!("upload patch"), read_cnt: 132, write_cnt: 2, need_ack: 0 },
    wavefront_command { cmd: WFC_DOWNLOAD_PROGRAM, action: cstr!("download program"), read_cnt: 0, write_cnt: 33, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_UPLOAD_PROGRAM, action: cstr!("upload program"), read_cnt: 32, write_cnt: 1, need_ack: 0 },
    wavefront_command { cmd: WFC_DOWNLOAD_EDRUM_PROGRAM, action: cstr!("download enhanced drum program"), read_cnt: 0, write_cnt: 9, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_UPLOAD_EDRUM_PROGRAM, action: cstr!("upload enhanced drum program"), read_cnt: 8, write_cnt: 1, need_ack: 0 },
    wavefront_command { cmd: WFC_SET_EDRUM_CHANNEL, action: cstr!("set enhanced drum program channel"), read_cnt: 0, write_cnt: 1, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_DISABLE_DRUM_PROGRAM, action: cstr!("disable drum program"), read_cnt: 0, write_cnt: 1, need_ack: NEEDS_ACK },
    wavefront_command { cmd: WFC_REPORT_CHANNEL_PROGRAMS, action: cstr!("report channel program numbers"), read_cnt: 32, write_cnt: 0, need_ack: 0 },
    wavefront_command { cmd: WFC_NOOP, action: cstr!("the no-op command"), read_cnt: 0, write_cnt: 0, need_ack: NEEDS_ACK },
];

unsafe fn wavefront_errorstr(errnum: c_int) -> *const c_char {
    let mut i = 0usize;
    while !wavefront_errors[i].errstr.is_null() {
        if wavefront_errors[i].errno == errnum {
            return wavefront_errors[i].errstr;
        }
        i += 1;
    }
    cstr!("Unknown WaveFront error")
}

unsafe fn wavefront_get_command(cmd: c_int) -> *mut wavefront_command {
    let mut i = 0usize;
    while i < wavefront_commands.len() {
        if cmd == wavefront_commands[i].cmd {
            return &mut wavefront_commands[i];
        }
        i += 1;
    }
    null_mut()
}

#[inline]
unsafe fn wavefront_status(dev: *mut snd_wavefront_t) -> c_int {
    inb((*dev).status_port)
}

unsafe fn wavefront_sleep(limit: c_int) -> c_int {
    schedule_timeout_interruptible(limit);
    signal_pending(current)
}

unsafe fn wavefront_wait(dev: *mut snd_wavefront_t, mask: c_int) -> c_int {
    let mut i = 0;
    while i < wait_usecs {
        if wavefront_status(dev) & mask != 0 { return 1; }
        udelay(5);
        i += 5;
    }
    i = 0;
    while i < sleep_tries {
        if wavefront_status(dev) & mask != 0 { return 1; }
        if wavefront_sleep(HZ / sleep_interval) != 0 { return 0; }
        i += 1;
    }
    0
}

unsafe fn wavefront_read(dev: *mut snd_wavefront_t) -> c_int {
    if wavefront_wait(dev, STAT_CAN_READ) != 0 {
        return inb((*dev).data_port);
    }
    DPRINT!(dev, WF_DEBUG_DATA, "read timeout.\n");
    -1
}

unsafe fn wavefront_write(dev: *mut snd_wavefront_t, data: u8) -> c_int {
    if wavefront_wait(dev, STAT_CAN_WRITE) != 0 {
        outb(data as c_int, (*dev).data_port);
        return 0;
    }
    DPRINT!(dev, WF_DEBUG_DATA, "write timeout.\n");
    -1
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_cmd(dev: *mut snd_wavefront_t, cmd: c_int, mut rbuf: *mut u8, wbuf: *mut u8) -> c_int {
    let mut ack: c_int;
    let mut c: c_int;
    let wfcmd = wavefront_get_command(cmd);
    if wfcmd.is_null() { return 1; }
    if cmd == WFC_DOWNLOAD_MULTISAMPLE {
        (*wfcmd).write_cnt = rbuf as c_ulong as c_uint;
        rbuf = null_mut();
    }
    if wavefront_write(dev, cmd as u8) != 0 { return 1; }
    let mut i: c_uint = 0;
    while i < (*wfcmd).write_cnt {
        if wavefront_write(dev, *wbuf.add(i as usize)) != 0 { return 1; }
        i += 1;
    }
    i = 0;
    while i < (*wfcmd).read_cnt {
        c = wavefront_read(dev);
        if c == -1 { return 1; }
        if c == 0xff {
            c = wavefront_read(dev);
            if c == -1 { return 1; }
            if c == 1 && (*wfcmd).cmd == WFC_IDENTIFY_SAMPLE_TYPE {
                *rbuf = WF_ST_EMPTY;
                return 0;
            } else if c == 3 && (*wfcmd).cmd == WFC_UPLOAD_PATCH {
                return 3;
            } else if c == 1 && (*wfcmd).cmd == WFC_UPLOAD_PROGRAM {
                return 1;
            } else {
                let _ = wavefront_errorstr(c);
                return 1;
            }
        } else {
            *rbuf.add(i as usize) = c as u8;
        }
        i += 1;
    }
    if ((*wfcmd).read_cnt == 0 && (*wfcmd).write_cnt == 0) || (*wfcmd).need_ack != 0 {
        ack = wavefront_read(dev);
        if ack == 0 { ack = WF_ACK; }
        if ack != WF_ACK {
            if ack == -1 { return 1; }
            let mut err = -1;
            if ack == 0xff {
                err = wavefront_read(dev);
            }
            return -err;
        }
    }
    0
}

unsafe fn munge_int32(mut src: c_uint, mut dst: *mut u8, dst_size: c_uint) -> *mut u8 {
    let mut i = 0;
    while i < dst_size {
        *dst = (src & 0x7f) as u8;
        src >>= 7;
        dst = dst.add(1);
        i += 1;
    }
    dst
}

unsafe fn demunge_int32(src: *mut u8, src_size: c_int) -> c_int {
    let mut i = src_size - 1;
    let mut outval = 0;
    while i >= 0 {
        outval = (outval << 7) + *src.add(i as usize) as c_int;
        i -= 1;
    }
    outval
}

unsafe fn munge_buf(src: *mut u8, mut dst: *mut u8, dst_size: c_uint) -> *mut u8 {
    let last = dst_size / 2;
    let mut i = 0;
    while i < last {
        *dst = *src.add(i as usize) & 0x7f; dst = dst.add(1);
        *dst = *src.add(i as usize) >> 7; dst = dst.add(1);
        i += 1;
    }
    dst
}

unsafe fn demunge_buf(mut src: *mut u8, dst: *mut u8, src_bytes: c_uint) -> *mut u8 {
    let end = src.add(src_bytes as usize);
    let mut i = 0usize;
    while src != end {
        *dst.add(i) = *src; src = src.add(1);
        *dst.add(i) |= (*src) << 7; src = src.add(1);
        i += 1;
    }
    dst
}

unsafe fn wavefront_delete_sample(dev: *mut snd_wavefront_t, sample_num: c_int) -> c_int {
    let mut wbuf = [0u8; 2];
    wbuf[0] = (sample_num & 0x7f) as u8;
    wbuf[1] = (sample_num >> 7) as u8;
    let x = snd_wavefront_cmd(dev, WFC_DELETE_SAMPLE, null_mut(), wbuf.as_mut_ptr());
    if x == 0 { (*dev).sample_status[sample_num as usize] = WF_ST_EMPTY; }
    x
}

unsafe fn wavefront_get_sample_status(dev: *mut snd_wavefront_t, assume_rom: c_int) -> c_int {
    let mut rbuf = [0u8; 32];
    let mut wbuf = [0u8; 32];
    let mut sc_real: c_uint = 0;
    let mut sc_alias: c_uint = 0;
    let mut sc_multi: c_uint = 0;
    if snd_wavefront_cmd(dev, WFC_GET_NSAMPLES, rbuf.as_mut_ptr(), wbuf.as_mut_ptr()) != 0 { return -1; }
    (*dev).samples_used = 0;
    let mut i = 0usize;
    while i < WF_MAX_SAMPLE {
        wbuf[0] = (i & 0x7f) as u8;
        wbuf[1] = (i >> 7) as u8;
        if snd_wavefront_cmd(dev, WFC_IDENTIFY_SAMPLE_TYPE, rbuf.as_mut_ptr(), wbuf.as_mut_ptr()) != 0 {
            (*dev).sample_status[i] = WF_ST_EMPTY;
            i += 1;
            continue;
        }
        (*dev).sample_status[i] = WF_SLOT_FILLED | rbuf[0];
        if assume_rom != 0 { (*dev).sample_status[i] |= WF_SLOT_ROM; }
        match rbuf[0] & WF_ST_MASK {
            WF_ST_SAMPLE => sc_real += 1,
            WF_ST_MULTISAMPLE => sc_multi += 1,
            WF_ST_ALIAS => sc_alias += 1,
            WF_ST_EMPTY => {}
            _ => {}
        }
        if rbuf[0] != WF_ST_EMPTY { (*dev).samples_used += 1; }
        i += 1;
    }
    let _ = (sc_real, sc_alias, sc_multi);
    0
}

unsafe fn wavefront_get_patch_status(dev: *mut snd_wavefront_t) -> c_int {
    let mut patchbuf = [0u8; WF_PATCH_BYTES];
    let mut patchnum = [0u8; 2];
    let mut i = 0usize;
    while i < WF_MAX_PATCH {
        patchnum[0] = (i & 0x7f) as u8;
        patchnum[1] = (i >> 7) as u8;
        let x = snd_wavefront_cmd(dev, WFC_UPLOAD_PATCH, patchbuf.as_mut_ptr(), patchnum.as_mut_ptr());
        if x == 0 {
            (*dev).patch_status[i] |= WF_SLOT_FILLED;
            let p = patchbuf.as_mut_ptr() as *mut wavefront_patch;
            (*dev).sample_status[((*p).sample_number | ((*p).sample_msb << 7)) as usize] |= WF_SLOT_USED;
        } else if x == 3 {
            (*dev).patch_status[i] = 0;
        } else {
            (*dev).patch_status[i] = 0;
            return 1;
        }
        i += 1;
    }
    0
}

unsafe fn wavefront_get_program_status(dev: *mut snd_wavefront_t) -> c_int {
    let mut progbuf = [0u8; WF_PROGRAM_BYTES];
    let mut prog: wavefront_program = zeroed();
    let mut prognum: u8;
    let mut i = 0usize;
    while i < WF_MAX_PROGRAM {
        prognum = i as u8;
        let x = snd_wavefront_cmd(dev, WFC_UPLOAD_PROGRAM, progbuf.as_mut_ptr(), &mut prognum);
        if x == 0 {
            (*dev).prog_status[i] |= WF_SLOT_USED;
            demunge_buf(progbuf.as_mut_ptr(), (&mut prog as *mut wavefront_program).cast(), WF_PROGRAM_BYTES as c_uint);
            let mut l = 0usize;
            while l < WF_NUM_LAYERS {
                if prog.layer[l].mute != 0 {
                    (*dev).patch_status[prog.layer[l].patch_number] |= WF_SLOT_USED;
                }
                l += 1;
            }
        } else if x == 1 {
            (*dev).prog_status[i] = 0;
        } else {
            (*dev).prog_status[i] = 0;
        }
        i += 1;
    }
    0
}

unsafe fn wavefront_send_patch(dev: *mut snd_wavefront_t, header: *mut wavefront_patch_info) -> c_int {
    let mut buf = [0u8; WF_PATCH_BYTES + 2];
    if (*header).number >= (*dev).patch_status.len() { return -EINVAL; }
    (*dev).patch_status[(*header).number] |= WF_SLOT_FILLED;
    let bptr = munge_int32((*header).number as c_uint, buf.as_mut_ptr(), 2);
    munge_buf((&mut (*header).hdr.p as *mut wavefront_patch).cast(), bptr, WF_PATCH_BYTES as c_uint);
    if snd_wavefront_cmd(dev, WFC_DOWNLOAD_PATCH, null_mut(), buf.as_mut_ptr()) != 0 { return -EIO; }
    0
}

unsafe fn wavefront_send_program(dev: *mut snd_wavefront_t, header: *mut wavefront_patch_info) -> c_int {
    let mut buf = [0u8; WF_PROGRAM_BYTES + 1];
    if (*header).number >= (*dev).prog_status.len() { return -EINVAL; }
    (*dev).prog_status[(*header).number] = WF_SLOT_USED;
    let mut i = 0usize;
    while i < WF_NUM_LAYERS {
        if (*header).hdr.pr.layer[i].mute != 0 {
            (*dev).patch_status[(*header).hdr.pr.layer[i].patch_number] |= WF_SLOT_USED;
        }
        i += 1;
    }
    buf[0] = (*header).number as u8;
    munge_buf((&mut (*header).hdr.pr as *mut wavefront_program).cast(), buf.as_mut_ptr().add(1), WF_PROGRAM_BYTES as c_uint);
    if snd_wavefront_cmd(dev, WFC_DOWNLOAD_PROGRAM, null_mut(), buf.as_mut_ptr()) != 0 { return -EIO; }
    0
}

unsafe fn wavefront_freemem(dev: *mut snd_wavefront_t) -> c_int {
    let mut rbuf = [0u8; 8];
    if snd_wavefront_cmd(dev, WFC_REPORT_FREE_MEMORY, rbuf.as_mut_ptr(), null_mut()) != 0 { -1 } else { demunge_int32(rbuf.as_mut_ptr(), 4) }
}

#[inline] unsafe fn WF_GET_CHANNEL(_s: *mut wavefront_sample) -> c_int { 0 }
#[inline] unsafe fn WF_SET_CHANNEL(_s: *mut wavefront_sample, _v: c_int) {}
#[inline] unsafe fn WF_SAMPLE_IS_8BIT(s: *mut wavefront_sample) -> bool { (*s).SampleResolution != LINEAR_16BIT }
#[inline] fn ALIGN(v: u32, a: u32) -> u32 { (v + a - 1) & !(a - 1) }

unsafe fn wavefront_send_sample(dev: *mut snd_wavefront_t, header: *mut wavefront_patch_info, mut dataptr: *mut u16, data_is_unsigned: c_int) -> c_int {
    let mut sample_short: u16 = 0;
    let mut sample_hdr = [0u8; WF_SAMPLE_HDR_BYTES];
    let max_blksize: u32 = 4096 / 2;
    let mut skip = 0;
    let mut initial_skip = 0;
    if (*header).number as c_int == WAVEFRONT_FIND_FREE_SAMPLE_SLOT {
        let x = wavefront_find_free_sample(dev);
        if x < 0 { return -ENOMEM; }
        (*header).number = x as usize;
    }
    if (*header).number >= WF_MAX_SAMPLE { return -EINVAL; }
    if (*header).size != 0 {
        if (*dev).rom_samples_rdonly != 0 && ((*dev).sample_status[(*header).number] & WF_SLOT_ROM) != 0 { return -EACCES; }
        wavefront_delete_sample(dev, (*header).number as c_int);
        (*dev).freemem = wavefront_freemem(dev);
        if (*dev).freemem < 0 || (*dev).freemem < (*header).size as c_int { return -ENOMEM; }
    }
    skip = WF_GET_CHANNEL(&mut (*header).hdr.s);
    if skip > 0 && (*header).hdr.s.SampleResolution != LINEAR_16BIT { return -EINVAL; }
    match skip {
        0 => { initial_skip = 0; skip = 1; }
        1 => { initial_skip = 0; skip = 2; }
        2 => { initial_skip = 1; skip = 2; }
        3 => { initial_skip = 2; skip = 3; }
        4 => { initial_skip = 3; skip = 4; }
        5 => { initial_skip = 4; skip = 5; }
        6 => { initial_skip = 5; skip = 6; }
        _ => {}
    }
    WF_SET_CHANNEL(&mut (*header).hdr.s, 0);
    let length = (*header).size / 2;
    let mut shptr = sample_hdr.as_mut_ptr();
    shptr = munge_int32((*header).number as c_uint, shptr, 2);
    if (*header).size != 0 { shptr = munge_int32(length, shptr, 4); }
    shptr = munge_int32(*((&mut (*header).hdr.s.sampleStartOffset as *mut [u8; 4]).cast::<u32>()), shptr, 4);
    shptr = munge_int32(*((&mut (*header).hdr.s.loopStartOffset as *mut [u8; 4]).cast::<u32>()), shptr, 4);
    shptr = munge_int32(*((&mut (*header).hdr.s.loopEndOffset as *mut [u8; 4]).cast::<u32>()), shptr, 4);
    shptr = munge_int32(*((&mut (*header).hdr.s.sampleEndOffset as *mut [u8; 4]).cast::<u32>()), shptr, 4);
    shptr = munge_int32((*header).hdr.s.FrequencyBias as c_uint, shptr, 3);
    let fb_next = (&mut (*header).hdr.s.FrequencyBias as *mut u16).add(1) as *mut u8;
    let _ = munge_int32(*fb_next as c_uint, shptr, 2);
    if snd_wavefront_cmd(dev, if (*header).size != 0 { WFC_DOWNLOAD_SAMPLE } else { WFC_DOWNLOAD_SAMPLE_HEADER }, null_mut(), sample_hdr.as_mut_ptr()) != 0 { return -EIO; }
    if (*header).size == 0 { return 0; }
    let data_end = dataptr.add(length as usize);
    dataptr = dataptr.add(initial_skip as usize);
    let mut written: u32 = 0;
    while written < length {
        let blocksize = if length - written > max_blksize { max_blksize } else { ALIGN(length - written, 8) };
        if snd_wavefront_cmd(dev, WFC_DOWNLOAD_BLOCK, null_mut(), null_mut()) != 0 { return -EIO; }
        let mut i = 0u32;
        while i < blocksize {
            if dataptr < data_end {
                sample_short = *dataptr;
                dataptr = dataptr.add(skip as usize);
                if data_is_unsigned != 0 {
                    if WF_SAMPLE_IS_8BIT(&mut (*header).hdr.s) {
                        let p = (&mut sample_short as *mut u16).cast::<u8>();
                        *p = (*p).wrapping_add(0x7f);
                        *p.add(1) = (*p.add(1)).wrapping_add(0x7f);
                    } else {
                        sample_short = sample_short.wrapping_add(0x7fff);
                    }
                }
            }
            if i < blocksize - 1 { outw(sample_short, (*dev).block_port); } else { outw(sample_short, (*dev).last_block_port); }
            i += 1;
        }
        let dma_ack = wavefront_read(dev);
        if dma_ack != WF_DMA_ACK { return -EIO; }
        written += max_blksize;
    }
    (*dev).sample_status[(*header).number] = WF_SLOT_FILLED | WF_ST_SAMPLE;
    0
}

unsafe fn wavefront_send_alias(dev: *mut snd_wavefront_t, header: *mut wavefront_patch_info) -> c_int {
    let mut alias_hdr = [0u8; WF_ALIAS_BYTES];
    if (*header).number >= WF_MAX_SAMPLE { return -EINVAL; }
    munge_int32((*header).number as c_uint, alias_hdr.as_mut_ptr(), 2);
    munge_int32((*header).hdr.a.OriginalSample, alias_hdr.as_mut_ptr().add(2), 2);
    munge_int32(*((&mut (*header).hdr.a.sampleStartOffset as *mut [u8; 4]).cast::<u32>()), alias_hdr.as_mut_ptr().add(4), 4);
    munge_int32(*((&mut (*header).hdr.a.loopStartOffset as *mut [u8; 4]).cast::<u32>()), alias_hdr.as_mut_ptr().add(8), 4);
    munge_int32(*((&mut (*header).hdr.a.loopEndOffset as *mut [u8; 4]).cast::<u32>()), alias_hdr.as_mut_ptr().add(12), 4);
    munge_int32(*((&mut (*header).hdr.a.sampleEndOffset as *mut [u8; 4]).cast::<u32>()), alias_hdr.as_mut_ptr().add(16), 4);
    munge_int32((*header).hdr.a.FrequencyBias as c_uint, alias_hdr.as_mut_ptr().add(20), 3);
    munge_int32(*((&mut (*header).hdr.a.FrequencyBias as *mut u16).add(1) as *mut u8) as c_uint, alias_hdr.as_mut_ptr().add(23), 2);
    if snd_wavefront_cmd(dev, WFC_DOWNLOAD_SAMPLE_ALIAS, null_mut(), alias_hdr.as_mut_ptr()) != 0 { return -EIO; }
    (*dev).sample_status[(*header).number] = WF_SLOT_FILLED | WF_ST_ALIAS;
    0
}

unsafe fn wavefront_send_multisample(dev: *mut snd_wavefront_t, header: *mut wavefront_patch_info) -> c_int {
    if (*header).number >= WF_MAX_SAMPLE { return -EINVAL; }
    let msample_hdr = kmalloc(WF_MSAMPLE_BYTES, GFP_KERNEL) as *mut u8;
    if msample_hdr.is_null() { return -ENOMEM; }
    munge_int32((*header).number as c_uint, msample_hdr, 2);
    let num_samples = 1 << ((*header).hdr.ms.NumberOfSamples & 7);
    *msample_hdr.add(2) = (*header).hdr.ms.NumberOfSamples;
    let mut i = 0usize;
    while i < num_samples {
        munge_int32((*header).hdr.ms.SampleNumber[i], msample_hdr.add(3 + i * 2), 2);
        i += 1;
    }
    if snd_wavefront_cmd(dev, WFC_DOWNLOAD_MULTISAMPLE, ((num_samples * 2 + 3) as c_ulong) as *mut u8, msample_hdr) != 0 {
        kfree(msample_hdr.cast());
        return -EIO;
    }
    (*dev).sample_status[(*header).number] = WF_SLOT_FILLED | WF_ST_MULTISAMPLE;
    kfree(msample_hdr.cast());
    0
}

unsafe fn wavefront_fetch_multisample(dev: *mut snd_wavefront_t, header: *mut wavefront_patch_info) -> c_int {
    let mut log_ns = [0u8; 1];
    let mut number = [0u8; 2];
    munge_int32((*header).number as c_uint, number.as_mut_ptr(), 2);
    if snd_wavefront_cmd(dev, WFC_UPLOAD_MULTISAMPLE, log_ns.as_mut_ptr(), number.as_mut_ptr()) != 0 { return -EIO; }
    (*header).hdr.ms.NumberOfSamples = log_ns[0];
    let num_samples = 1 << log_ns[0];
    let mut i = 0usize;
    while i < num_samples {
        let mut d = [0u8; 2];
        let mut val = wavefront_read(dev); if val == -1 { return -EIO; } d[0] = val as u8;
        val = wavefront_read(dev); if val == -1 { return -EIO; } d[1] = val as u8;
        (*header).hdr.ms.SampleNumber[i] = demunge_int32(d.as_mut_ptr(), 2) as c_uint;
        i += 1;
    }
    0
}

unsafe fn wavefront_send_drum(dev: *mut snd_wavefront_t, header: *mut wavefront_patch_info) -> c_int {
    let mut drumbuf = [0u8; WF_DRUM_BYTES];
    drumbuf[0] = ((*header).number & 0x7f) as u8;
    let drum = &mut (*header).hdr.d as *mut wavefront_drum;
    let mut i = 0usize;
    while i < 4 {
        munge_int32(*drum.cast::<u8>().add(i) as c_uint, drumbuf.as_mut_ptr().add(1 + i * 2), 2);
        i += 1;
    }
    if snd_wavefront_cmd(dev, WFC_DOWNLOAD_EDRUM_PROGRAM, null_mut(), drumbuf.as_mut_ptr()) != 0 { return -EIO; }
    0
}

unsafe fn wavefront_find_free_sample(dev: *mut snd_wavefront_t) -> c_int {
    let mut i = 0usize;
    while i < WF_MAX_SAMPLE {
        if ((*dev).sample_status[i] & WF_SLOT_FILLED) == 0 { return i as c_int; }
        i += 1;
    }
    -1
}

/* #if 0 wavefront_find_free_patch omitted by the C preprocessor in original. */

unsafe fn wavefront_load_patch(dev: *mut snd_wavefront_t, addr: *const c_char) -> c_int {
    let header = kmalloc(size_of::<wavefront_patch_info>(), GFP_KERNEL) as *mut wavefront_patch_info;
    if header.is_null() { return -ENOMEM; }
    let mut err: c_int;
    if copy_from_user(header.cast(), addr.cast(), size_of::<wavefront_patch_info>() - size_of::<wavefront_any>()) != 0 {
        err = -EFAULT;
        kfree(header.cast());
        return err;
    }
    match (*header).subkey {
        WF_ST_SAMPLE => {
            if copy_from_user((&mut (*header).hdr.s as *mut wavefront_sample).cast(), (*header).hdrptr, size_of::<wavefront_sample>()) != 0 { err = -EFAULT; }
            else { err = wavefront_send_sample(dev, header, (*header).dataptr, 0); }
        }
        WF_ST_MULTISAMPLE => {
            if copy_from_user((&mut (*header).hdr.ms as *mut wavefront_multisample).cast(), (*header).hdrptr, size_of::<wavefront_multisample>()) != 0 { err = -EFAULT; }
            else { err = wavefront_send_multisample(dev, header); }
        }
        WF_ST_ALIAS => {
            if copy_from_user((&mut (*header).hdr.a as *mut wavefront_alias).cast(), (*header).hdrptr, size_of::<wavefront_alias>()) != 0 { err = -EFAULT; }
            else { err = wavefront_send_alias(dev, header); }
        }
        WF_ST_DRUM => {
            if copy_from_user((&mut (*header).hdr.d as *mut wavefront_drum).cast(), (*header).hdrptr, size_of::<wavefront_drum>()) != 0 { err = -EFAULT; }
            else { err = wavefront_send_drum(dev, header); }
        }
        WF_ST_PATCH => {
            if copy_from_user((&mut (*header).hdr.p as *mut wavefront_patch).cast(), (*header).hdrptr, size_of::<wavefront_patch>()) != 0 { err = -EFAULT; }
            else { err = wavefront_send_patch(dev, header); }
        }
        WF_ST_PROGRAM => {
            if copy_from_user((&mut (*header).hdr.pr as *mut wavefront_program).cast(), (*header).hdrptr, size_of::<wavefront_program>()) != 0 { err = -EFAULT; }
            else { err = wavefront_send_program(dev, header); }
        }
        _ => err = -EINVAL,
    }
    kfree(header.cast());
    err
}

unsafe fn process_sample_hdr(buf: *mut u8) {
    let mut s: wavefront_sample = zeroed();
    let mut ptr = buf;
    *(&mut s.sampleStartOffset as *mut [u8; 4]).cast::<u32>() = demunge_int32(ptr, 4) as u32; ptr = ptr.add(4);
    *(&mut s.loopStartOffset as *mut [u8; 4]).cast::<u32>() = demunge_int32(ptr, 4) as u32; ptr = ptr.add(4);
    *(&mut s.loopEndOffset as *mut [u8; 4]).cast::<u32>() = demunge_int32(ptr, 4) as u32; ptr = ptr.add(4);
    *(&mut s.sampleEndOffset as *mut [u8; 4]).cast::<u32>() = demunge_int32(ptr, 4) as u32; ptr = ptr.add(4);
    *(&mut s.FrequencyBias as *mut u16).cast::<u32>() = demunge_int32(ptr, 3) as u32; ptr = ptr.add(3);
    s.SampleResolution = *ptr & 0x3;
    s.Loop = *ptr & 0x8;
    s.Bidirectional = *ptr & 0x10;
    s.Reverse = *ptr & 0x40;
    copy_nonoverlapping((&s as *const wavefront_sample).cast::<u8>(), buf, size_of::<wavefront_sample>());
}

unsafe fn wavefront_synth_control(acard: *mut snd_wavefront_card_t, wc: *mut wavefront_control) -> c_int {
    let dev = &mut (*acard).wavefront as *mut snd_wavefront_t;
    let mut patchnumbuf = [0u8; 2];
    match (*wc).cmd {
        WFC_DISABLE_INTERRUPTS => { outb(0x80 | 0x20, (*dev).control_port); (*dev).interrupts_are_midi = 1; return 0; }
        WFC_ENABLE_INTERRUPTS => { outb(0x80 | 0x40 | 0x20, (*dev).control_port); (*dev).interrupts_are_midi = 1; return 0; }
        WFC_INTERRUPT_STATUS => { (*wc).rbuf[0] = (*dev).interrupts_are_midi as u8; return 0; }
        WFC_ROMSAMPLES_RDONLY => { (*dev).rom_samples_rdonly = (*wc).wbuf[0] as c_char; (*wc).status = 0; return 0; }
        WFC_IDENTIFY_SLOT_TYPE => {
            let i = ((*wc).wbuf[0] as c_int) | (((*wc).wbuf[1] as c_int) << 7);
            if i < 0 || i >= WF_MAX_SAMPLE as c_int { (*wc).status = EINVAL; return -EINVAL; }
            (*wc).rbuf[0] = (*dev).sample_status[i as usize]; (*wc).status = 0; return 0;
        }
        WFC_DEBUG_DRIVER => { (*dev).debug = (*wc).wbuf[0] as c_int; return 0; }
        WFC_UPLOAD_PATCH => { munge_int32(*((*wc).wbuf.as_mut_ptr()).cast::<u32>(), patchnumbuf.as_mut_ptr(), 2); copy_nonoverlapping(patchnumbuf.as_ptr(), (*wc).wbuf.as_mut_ptr(), 2); }
        WFC_UPLOAD_MULTISAMPLE => { (*wc).status = wavefront_fetch_multisample(dev, (*wc).rbuf.as_mut_ptr().cast()); return 0; }
        WFC_UPLOAD_SAMPLE_ALIAS => { (*wc).status = EINVAL; return -EINVAL; }
        _ => {}
    }
    (*wc).status = snd_wavefront_cmd(dev, (*wc).cmd, (*wc).rbuf.as_mut_ptr(), (*wc).wbuf.as_mut_ptr());
    if (*wc).status == 0 {
        match (*wc).cmd {
            WFC_REPORT_FREE_MEMORY => (*dev).freemem = demunge_int32((*wc).rbuf.as_mut_ptr(), 4),
            WFC_UPLOAD_PATCH => { demunge_buf((*wc).rbuf.as_mut_ptr(), (*wc).rbuf.as_mut_ptr(), WF_PATCH_BYTES as c_uint); }
            WFC_UPLOAD_PROGRAM => { demunge_buf((*wc).rbuf.as_mut_ptr(), (*wc).rbuf.as_mut_ptr(), WF_PROGRAM_BYTES as c_uint); }
            WFC_UPLOAD_EDRUM_PROGRAM => { demunge_buf((*wc).rbuf.as_mut_ptr(), (*wc).rbuf.as_mut_ptr(), (WF_DRUM_BYTES - 1) as c_uint); }
            WFC_UPLOAD_SAMPLE_HEADER => process_sample_hdr((*wc).rbuf.as_mut_ptr()),
            WFC_MISYNTH_OFF => (*dev).midi_in_to_synth = 0,
            WFC_MISYNTH_ON => (*dev).midi_in_to_synth = 1,
            WFC_VMIDI_OFF => snd_wavefront_midi_disable_virtual(acard),
            WFC_VMIDI_ON => snd_wavefront_midi_enable_virtual(acard),
            _ => {}
        }
    }
    0
}

unsafe fn wavefront_restore_midi_state(acard: *mut snd_wavefront_card_t, isvirtual: c_char, midi_in_to_synth: c_char) -> c_int {
    let dev = &mut (*acard).wavefront as *mut snd_wavefront_t;
    let mut rbuf = [0u8; 4];
    let mut wbuf = [0u8; 4];
    if (*dev).midi_in_to_synth != midi_in_to_synth {
        if snd_wavefront_cmd(dev, if midi_in_to_synth != 0 { WFC_MISYNTH_ON } else { WFC_MISYNTH_OFF }, rbuf.as_mut_ptr(), wbuf.as_mut_ptr()) != 0 { return -EIO; }
        (*dev).midi_in_to_synth = midi_in_to_synth;
    }
    if (*dev).midi.isvirtual != isvirtual {
        if snd_wavefront_cmd(dev, if isvirtual != 0 { WFC_VMIDI_ON } else { WFC_VMIDI_OFF }, rbuf.as_mut_ptr(), wbuf.as_mut_ptr()) != 0 { return -EIO; }
        if isvirtual != 0 { snd_wavefront_midi_enable_virtual(acard); } else { snd_wavefront_midi_disable_virtual(acard); }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_resume_synth(acard: *mut snd_wavefront_card_t) -> c_int {
    let dev = &mut (*acard).wavefront as *mut snd_wavefront_t;
    let was_virtual = (*dev).midi.isvirtual;
    let midi_in_to_synth = (*dev).midi_in_to_synth;
    let rom_samples_rdonly = (*dev).rom_samples_rdonly;
    let mut err = snd_wavefront_detect(acard);
    if err < 0 { (*dev).israw = 1; }
    if (*dev).israw != 0 {
        (*dev).fx_initialized = 0;
        err = snd_wavefront_start(dev);
        if err < 0 { return err; }
    } else {
        (*dev).has_fx = (snd_wavefront_fx_detect(dev) == 0) as c_char;
        wavefront_get_sample_status(dev, 0);
        wavefront_get_program_status(dev);
        wavefront_get_patch_status(dev);
        outb(0x80 | 0x40 | 0x20, (*dev).control_port);
    }
    (*dev).rom_samples_rdonly = rom_samples_rdonly;
    (*dev).midi.base = (*dev).base;
    err = snd_wavefront_midi_start(acard);
    if err < 0 { return err; }
    err = wavefront_restore_midi_state(acard, was_virtual, midi_in_to_synth);
    if err < 0 { return err; }
    snd_wavefront_midi_resume(acard);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_synth_open(hw: *mut snd_hwdep, file: *mut file) -> c_int {
    if !try_module_get((*(*hw).card).module) { return -EFAULT; }
    (*file).private_data = hw.cast();
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_synth_release(hw: *mut snd_hwdep, _file: *mut file) -> c_int {
    module_put((*(*hw).card).module);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_synth_ioctl(hw: *mut snd_hwdep, _file: *mut file, cmd: c_uint, arg: c_ulong) -> c_int {
    let card = (*hw).card;
    let argp = arg as *mut c_void;
    let mut err: c_int;
    if snd_BUG_ON(card.is_null()) { return -ENODEV; }
    if snd_BUG_ON((*card).private_data.is_null()) { return -ENODEV; }
    let acard = (*card).private_data as *mut snd_wavefront_card_t;
    let dev = &mut (*acard).wavefront as *mut snd_wavefront_t;
    match cmd {
        WFCTL_LOAD_SPP => {
            if wavefront_load_patch(dev, argp.cast()) != 0 { return -EIO; }
        }
        WFCTL_WFCMD => {
            let wc = memdup_user(argp, size_of::<wavefront_control>()) as *mut wavefront_control;
            if IS_ERR(wc.cast()) { return PTR_ERR(wc.cast()); }
            if wavefront_synth_control(acard, wc) < 0 { err = -EIO; }
            else if copy_to_user(argp, wc.cast(), size_of::<wavefront_control>()) != 0 { err = -EFAULT; }
            else { err = 0; }
            kfree(wc.cast());
            return err;
        }
        _ => return -EINVAL,
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_internal_interrupt(card: *mut snd_wavefront_card_t) {
    let dev = &mut (*card).wavefront as *mut snd_wavefront_t;
    if (wavefront_status(dev) & (STAT_INTR_READ | STAT_INTR_WRITE)) == 0 { return; }
    (*dev).irq_ok = 1;
    (*dev).irq_cnt += 1;
    wake_up(&mut (*dev).interrupt_sleeper);
}

unsafe fn snd_wavefront_interrupt_bits(dev: *mut snd_wavefront_t, irq: c_int) -> c_int {
    match irq {
        9 => 0x00,
        5 => 0x08,
        12 => 0x10,
        15 => 0x18,
        _ => { let _ = dev; -1 }
    }
}

unsafe fn wavefront_should_cause_interrupt(dev: *mut snd_wavefront_t, val: c_int, port: c_ulong, timeout: c_ulong) {
    let mut wait: wait_queue_entry_t = zeroed();
    init_waitqueue_entry(&mut wait, current);
    add_wait_queue(&mut (*dev).interrupt_sleeper, &mut wait);
    (*dev).irq_ok = 0;
    outb(val, port);
    while (*dev).irq_ok == 0 && time_before(jiffies, timeout) {
        schedule_timeout_uninterruptible(1);
        barrier();
    }
}

unsafe fn wavefront_reset_to_cleanliness(dev: *mut snd_wavefront_t) -> c_int {
    let bits = snd_wavefront_interrupt_bits(dev, (*dev).irq);
    let mut hwv = [0i32; 2];
    outb(0x0, (*dev).control_port);
    outb(0x80 | 0x40 | bits, (*dev).data_port);
    wavefront_should_cause_interrupt(dev, 0x80 | 0x40 | 0x10 | 0x1, (*dev).control_port, ((reset_time * HZ) / 100) as c_ulong);
    if (*dev).irq_ok == 0 { return 1; }
    wavefront_should_cause_interrupt(dev, WFC_HARDWARE_VERSION, (*dev).data_port, (ramcheck_time * HZ) as c_ulong);
    if (*dev).irq_ok == 0 { return 1; }
    if wavefront_wait(dev, STAT_CAN_READ) == 0 { return 1; }
    hwv[0] = wavefront_read(dev);
    if hwv[0] == -1 { return 1; }
    if hwv[0] == 0xff {
        hwv[0] = wavefront_read(dev);
        return 1;
    }
    hwv[1] = wavefront_read(dev);
    if hwv[1] == -1 { return 1; }
    0
}

unsafe fn wavefront_download_firmware(dev: *mut snd_wavefront_t, path: *mut c_char) -> c_int {
    let mut fw: *const firmware = core::ptr::null();
    let mut err = request_firmware(&mut fw, path, (*(*dev).card).dev);
    if err < 0 { return 1; }
    let firmware = &*fw;
    let mut len: c_int = 0;
    let mut buf = firmware.data;
    let mut section_cnt_downloaded = 0;
    loop {
        let mut section_length = *(buf as *const i8) as c_int;
        if section_length == 0 { break; }
        if section_length < 0 || section_length > WF_SECTION_MAX { return 1; }
        buf = buf.add(1); len += 1;
        if firmware.size < (len + section_length) as usize { return 1; }
        if wavefront_write(dev, WFC_DOWNLOAD_OS as u8) != 0 { return 1; }
        while section_length != 0 {
            if wavefront_write(dev, *buf) != 0 { return 1; }
            buf = buf.add(1); len += 1; section_length -= 1;
        }
        if wavefront_wait(dev, STAT_CAN_READ) == 0 { return 1; }
        err = inb((*dev).data_port);
        if err != WF_ACK { let _ = section_cnt_downloaded; return 1; }
        section_cnt_downloaded += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_cache_firmware(dev: *mut snd_wavefront_t) {
    let err = firmware_request_cache((*(*dev).card).dev, ospath);
    if err < 0 { dev_warn!((*(*dev).card).dev, "unable to cache firmware"); }
}

unsafe fn wavefront_do_reset(dev: *mut snd_wavefront_t) -> c_int {
    let mut voices = [0u8; 1];
    if wavefront_reset_to_cleanliness(dev) != 0 { return 1; }
    if (*dev).israw != 0 {
        if wavefront_download_firmware(dev, ospath) != 0 { return 1; }
        (*dev).israw = 0;
        wavefront_should_cause_interrupt(dev, WFC_NOOP, (*dev).data_port, (osrun_time * HZ) as c_ulong);
        if (*dev).irq_ok == 0 { return 1; }
        wavefront_should_cause_interrupt(dev, WFC_NOOP, (*dev).data_port, (10 * HZ) as c_ulong);
        if (*dev).irq_ok == 0 { return 1; }
        outb(0x80 | 0x40, (*dev).control_port);
    }
    (*dev).freemem = wavefront_freemem(dev);
    if (*dev).freemem < 0 { return 1; }
    if wavefront_write(dev, 0xf0) != 0 || wavefront_write(dev, 1) != 0 || wavefront_read(dev) < 0 {
        (*dev).debug = 0;
        outb(0x0, (*dev).control_port);
        (*dev).interrupts_are_midi = 0;
        return 1;
    }
    voices[0] = 32;
    if snd_wavefront_cmd(dev, WFC_SET_NVOICES, null_mut(), voices.as_mut_ptr()) != 0 {
        outb(0x0, (*dev).control_port);
        (*dev).interrupts_are_midi = 0;
        return 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_start(dev: *mut snd_wavefront_t) -> c_int {
    let samples_are_from_rom = if (*dev).israw != 0 { 1 } else { 0 };
    if (*dev).israw != 0 || fx_raw != 0 {
        if wavefront_do_reset(dev) != 0 { return -1; }
    }
    (*dev).has_fx = (snd_wavefront_fx_detect(dev) == 0) as c_char;
    if (*dev).has_fx != 0 && fx_raw != 0 { snd_wavefront_fx_start(dev); }
    wavefront_get_sample_status(dev, samples_are_from_rom);
    wavefront_get_program_status(dev);
    wavefront_get_patch_status(dev);
    outb(0x80 | 0x40 | 0x20, (*dev).control_port);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_detect(card: *mut snd_wavefront_card_t) -> c_int {
    let mut rbuf = [0u8; 4];
    let mut wbuf = [0u8; 4];
    let dev = &mut (*card).wavefront as *mut snd_wavefront_t;
    (*dev).israw = 0;
    (*dev).has_fx = 0;
    (*dev).debug = debug_default;
    (*dev).interrupts_are_midi = 0;
    (*dev).irq_cnt = 0;
    (*dev).rom_samples_rdonly = 1;
    if snd_wavefront_cmd(dev, WFC_FIRMWARE_VERSION, rbuf.as_mut_ptr(), wbuf.as_mut_ptr()) == 0 {
        (*dev).fw_version[0] = rbuf[0];
        (*dev).fw_version[1] = rbuf[1];
        if snd_wavefront_cmd(dev, WFC_HARDWARE_VERSION, rbuf.as_mut_ptr(), wbuf.as_mut_ptr()) == 0 {
            (*dev).hw_version[0] = rbuf[0];
            (*dev).hw_version[1] = rbuf[1];
        } else {
            return -1;
        }
        if wf_raw == 0 { return 0; } else { (*dev).israw = 1; }
    } else {
        (*dev).israw = 1;
    }
    0
}

/* MODULE_FIRMWARE(DEFAULT_OSPATH); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
