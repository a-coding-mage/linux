/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OSS compatible sequencer driver
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Dependencies from the original header:
 * <linux/time.h>, <linux/wait.h>, <linux/slab.h>,
 * <linux/sched/signal.h>, <sound/core.h>, <sound/seq_oss.h>,
 * <sound/rawmidi.h>, <sound/seq_kernel.h>, <sound/info.h>,
 * and "../seq_clientmgr.h".
 */

/* max. applications */
pub const SNDRV_SEQ_OSS_MAX_CLIENTS: c_int = 16;
pub const SNDRV_SEQ_OSS_MAX_SYNTH_DEVS: usize = 16;
pub const SNDRV_SEQ_OSS_MAX_MIDI_DEVS: c_int = 32;

/* version */
pub const SNDRV_SEQ_OSS_MAJOR_VERSION: c_int = 0;
pub const SNDRV_SEQ_OSS_MINOR_VERSION: c_int = 1;
pub const SNDRV_SEQ_OSS_TINY_VERSION: c_int = 8;
pub const SNDRV_SEQ_OSS_VERSION_STR: &[u8; 6] = b"0.1.8\0";

/* device and proc interface name */
pub const SNDRV_SEQ_OSS_PROCNAME: &[u8; 4] = b"oss\0";

/*
 * type definitions
 */

pub type reltime_t = c_uint;
pub type abstime_t = c_uint;

/*
 * synthesizer channel information
 */
#[repr(C)]
pub struct seq_oss_chinfo {
    pub note: c_int,
    pub vel: c_int,
}

/*
 * synthesizer information
 */
#[repr(C)]
pub struct seq_oss_synthinfo {
    pub arg: snd_seq_oss_arg,
    pub ch: *mut seq_oss_chinfo,
    pub nr_voices: c_int,
    pub opened: c_int,
    pub is_midi: c_int,
    pub midi_mapped: c_int,
}

/*
 * sequencer client information
 */

#[repr(C)]
pub struct seq_oss_devinfo {
    pub index: c_int, /* application index */
    pub cseq: c_int,  /* sequencer client number */
    pub port: c_int,  /* sequencer port number */
    pub queue: c_int, /* sequencer queue number */

    pub addr: snd_seq_addr, /* address of this device */

    pub seq_mode: c_int,  /* sequencer mode */
    pub file_mode: c_int, /* file access */

    /* midi device table */
    pub max_mididev: c_int,

    /* synth device table */
    pub max_synthdev: c_int,
    pub synths: [seq_oss_synthinfo; SNDRV_SEQ_OSS_MAX_SYNTH_DEVS],
    pub synth_opened: c_int,

    /* output queue */
    pub writeq: *mut seq_oss_writeq,

    /* midi input queue */
    pub readq: *mut seq_oss_readq,

    /* timer */
    pub timer: *mut seq_oss_timer,
}

/*
 * function prototypes
 */

unsafe extern "C" {
    /* create/delete OSS sequencer client */
    pub fn snd_seq_oss_create_client() -> c_int;
    pub fn snd_seq_oss_delete_client() -> c_int;

    /* device file interface */
    pub fn snd_seq_oss_open(file: *mut file, level: c_int) -> c_int;
    pub fn snd_seq_oss_release(dp: *mut seq_oss_devinfo);
    pub fn snd_seq_oss_ioctl(dp: *mut seq_oss_devinfo, cmd: c_uint, arg: c_ulong) -> c_int;
    pub fn snd_seq_oss_read(dev: *mut seq_oss_devinfo, buf: *mut c_char, count: c_int) -> c_int;
    pub fn snd_seq_oss_write(
        dp: *mut seq_oss_devinfo,
        buf: *const c_char,
        count: c_int,
        opt: *mut file,
    ) -> c_int;
    pub fn snd_seq_oss_poll(
        dp: *mut seq_oss_devinfo,
        file: *mut file,
        wait: *mut poll_table,
    ) -> __poll_t;

    pub fn snd_seq_oss_reset(dp: *mut seq_oss_devinfo);

    /* proc interface */
    pub fn snd_seq_oss_system_info_read(buf: *mut snd_info_buffer);
    pub fn snd_seq_oss_midi_info_read(buf: *mut snd_info_buffer);
    pub fn snd_seq_oss_synth_info_read(buf: *mut snd_info_buffer);
    pub fn snd_seq_oss_readq_info_read(q: *mut seq_oss_readq, buf: *mut snd_info_buffer);

    pub fn snd_seq_kernel_client_dispatch(
        client: c_int,
        ev: *mut snd_seq_event,
        atomic: c_int,
        hop: c_int,
    ) -> c_int;
    pub fn snd_seq_kernel_client_ioctl(client: c_int, type_: c_uint, arg: *mut c_void) -> c_int;
}

/* file mode macros */
#[inline]
pub unsafe fn is_read_mode(mode: c_int) -> c_int {
    mode & SNDRV_SEQ_OSS_FILE_READ
}

#[inline]
pub unsafe fn is_write_mode(mode: c_int) -> c_int {
    mode & SNDRV_SEQ_OSS_FILE_WRITE
}

#[inline]
pub unsafe fn is_nonblock_mode(mode: c_int) -> c_int {
    mode & SNDRV_SEQ_OSS_FILE_NONBLOCK
}

/* dispatch event */
#[inline]
pub unsafe fn snd_seq_oss_dispatch(
    dp: *mut seq_oss_devinfo,
    ev: *mut snd_seq_event,
    atomic: c_int,
    hop: c_int,
) -> c_int {
    unsafe { snd_seq_kernel_client_dispatch((*dp).cseq, ev, atomic, hop) }
}

/* ioctl for writeq */
#[inline]
pub unsafe fn snd_seq_oss_control(
    dp: *mut seq_oss_devinfo,
    type_: c_uint,
    arg: *mut c_void,
) -> c_int {
    unsafe { snd_seq_kernel_client_ioctl((*dp).cseq, type_, arg) }
}

/* fill the addresses in header */
#[inline]
pub unsafe fn snd_seq_oss_fill_addr(
    dp: *mut seq_oss_devinfo,
    ev: *mut snd_seq_event,
    dest_client: c_int,
    dest_port: c_int,
) {
    unsafe {
        (*ev).queue = (*dp).queue;
        (*ev).source = (*dp).addr;
        (*ev).dest.client = dest_client;
        (*ev).dest.port = dest_port;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
